use core::{
    mem::MaybeUninit,
    ops::{Deref, DerefMut},
    ptr,
};

pub struct StackArray<T, const N: usize> {
    group: [MaybeUninit<T>; N],
}

impl<T, const N: usize> StackArray<T, N> {
    pub fn from_vec(vec: Vec<T>) -> Self {
        assert_eq!(vec.len(), N);

        // SAFETY:
        // assume_init is valid for [MaybeUninit<T>; N];
        let mut group: [MaybeUninit<T>; N] = unsafe { MaybeUninit::uninit().assume_init() };

        let n = vec
            .into_iter()
            .enumerate()
            .map(|(idx, value)| {
                group[idx].write(value);
            })
            .count();

        // make sure all items in group are initialized.
        assert_eq!(N, n);

        Self { group }
    }
}

impl<T, const N: usize> Deref for StackArray<T, N> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        // SAFETY:
        // from_vec method did double check making sure all items are initialized in constructor.
        unsafe { &*(&self.group as *const [MaybeUninit<T>] as *const [T]) }
    }
}

impl<T, const N: usize> DerefMut for StackArray<T, N> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        // SAFETY:
        // from_vec method did double check making sure all items are initialized in constructor.
        unsafe { &mut *(&mut self.group as *mut [MaybeUninit<T>] as *mut [T]) }
    }
}

impl<T, const N: usize> Drop for StackArray<T, N> {
    fn drop(&mut self) {
        // SAFETY:
        // This is safe for the the same reason of StackArray as DerefMut is safe.
        unsafe {
            ptr::drop_in_place(&mut *(&mut self.group as *mut [MaybeUninit<T>] as *mut [T]));
        }
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
