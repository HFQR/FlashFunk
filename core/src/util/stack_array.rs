use core::{
    mem::{ManuallyDrop, MaybeUninit},
    ops::{Deref, DerefMut},
    ptr, slice,
};

pub struct StackArray<T, const N: usize> {
    group: ManuallyDrop<MaybeUninit<[T; N]>>,
}

impl<T, const N: usize> StackArray<T, N> {
    pub fn from_vec(mut vec: Vec<T>) -> Self {
        assert_eq!(vec.len(), N);

        let mut group = ManuallyDrop::new(MaybeUninit::uninit());

        unsafe {
            // SAFETY:
            //
            // Set len to zero is safe:
            //
            // vector is the same length as N. This is assert checked beforehand.
            vec.set_len(0);

            // SAFETY:
            //
            // pointer copy is safe:
            //
            // Vector is the same length as N and receivers are constructed in scope
            // with mut reference in unsafe block.
            let dst = group.as_mut_ptr() as *mut T;
            ptr::copy_nonoverlapping(vec.as_ptr(), dst, N);
        }

        Self { group }
    }
}

impl<T, const N: usize> Deref for StackArray<T, N> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        // SAFETY:
        //
        // Deref is safe:
        //
        // StackGroup is only constructed from a non empty Vec<T>.
        // Deref can only happen after.
        //
        // N is a const generic param inherent from [Strategy; N] and it always
        // equal to the input length of Vec<T>.
        unsafe {
            let ptr = self.group.as_ptr() as *const T;
            slice::from_raw_parts(ptr, N)
        }
    }
}

impl<T, const N: usize> DerefMut for StackArray<T, N> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        // SAFETY:
        //
        // DerefMut is safe:
        //
        // For the same reason of Deref
        unsafe {
            let ptr = self.group.as_mut_ptr() as *mut T;
            slice::from_raw_parts_mut(ptr, N)
        }
    }
}

impl<T, const N: usize> Drop for StackArray<T, N> {
    fn drop(&mut self) {
        // SAFETY:
        //
        // Drop is safe:
        //
        // StackGroup itself is stateless and only the T needed to be dropped.
        unsafe { ptr::drop_in_place(&mut self[..]) }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use core::cell::Cell;

    use alloc::rc::Rc;

    #[test]
    fn stack_group_iter() {
        let v = vec![1, 2, 3];

        let group = StackArray::<_, 3>::from_vec(v);

        let mut iter = group.iter();

        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn stack_group_drop() {
        let rc = Rc::new(Cell::new(0));

        struct DropItem(Rc<Cell<usize>>);

        impl Drop for DropItem {
            fn drop(&mut self) {
                self.0.set(self.0.get() + 1);
            }
        }

        let v = vec![
            DropItem(rc.clone()),
            DropItem(rc.clone()),
            DropItem(rc.clone()),
            DropItem(rc.clone()),
        ];

        let group = StackArray::<_, 4>::from_vec(v);

        drop(group);

        assert_eq!(rc.get(), 4);
    }

    #[test]
    fn stack_group_get_idx() {
        let v = vec![1, 2, 3];

        let group = StackArray::<_, 3>::from_vec(v);

        assert_eq!(group[0], 1);
        assert_eq!(group[1], 2);
        assert_eq!(group[2], 3);
    }

    #[test]
    #[should_panic]
    fn stack_group_get_idx_out_of_range() {
        let v = vec![1, 2, 3];

        let group = StackArray::<_, 3>::from_vec(v);

        let _ = group[3];
    }
}
