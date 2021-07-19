use core::fmt::{Debug, Display, Formatter, Result as FmtResult};
use core::mem::{ManuallyDrop, MaybeUninit};
use core::ops::{Deref, DerefMut};
use core::ptr;
use core::slice;

use super::spsc::{new, Consumer, Producer};

pub fn channel<M>(cap: usize) -> (Sender<M>, Receiver<M>) {
    let (tx, rx) = new(cap);
    (Sender(tx), Receiver(rx))
}

pub struct Sender<M>(Producer<M>);

impl<M> Sender<M> {
    // 发送失败会panic
    #[inline]
    pub fn send(&self, m: impl Into<M>) {
        self.0.push(m.into()).unwrap();
    }

    // 发送失败返回消息
    #[inline]
    pub fn try_send(&self, m: M) -> Result<(), ChannelError<M>> {
        self.0.push(m).map_err(|e| ChannelError::TrySendError(e.0))
    }
}

pub struct Receiver<M>(Consumer<M>);

impl<M> Receiver<M> {
    #[inline]
    pub fn recv(&self) -> Result<M, ChannelError<M>> {
        self.0.pop().map_err(|_| ChannelError::RecvError)
    }
}

pub enum ChannelError<M> {
    RecvError,
    TrySendError(M),
    SenderOverFlow(M),
    SenderGroupNotFound(M),
}

impl<M> Debug for ChannelError<M> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let mut fmt = f.debug_struct("ChannelError");

        match self {
            ChannelError::SenderOverFlow(_) => fmt
                .field("cause", &"ChannelGroupSender")
                .field("description", &"Overflow on group sender's sender index"),
            ChannelError::SenderGroupNotFound(_) => {
                fmt.field("cause", &"ChannelGroupSender").field(
                    "description",
                    &"Overflow on group sender's group index(group not found)",
                )
            }
            ChannelError::RecvError => fmt
                .field("cause", &"ChannelReceiver")
                .field("description", &"Failed to receive message from channel"),
            ChannelError::TrySendError(_) => fmt
                .field("cause", &"ChannelSender")
                .field("description", &"Failed to send message through channel"),
        };

        fmt.finish()
    }
}

impl<M> Display for ChannelError<M> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{:?}", self)
    }
}

pub struct GroupSender<M, const N: usize> {
    senders: StackGroup<Sender<M>, N>,
    group: Vec<Vec<usize>>,
}

impl<M, const N: usize> GroupSender<M, N> {
    pub fn new(sender: Vec<Sender<M>>, group: Vec<Vec<usize>>) -> Self {
        Self {
            senders: StackGroup::from_vec(sender),
            group,
        }
    }

    // 发送至所有sender
    #[inline]
    pub fn send_all<MM>(&self, mm: MM)
    where
        MM: Into<M> + Clone,
    {
        self.senders.iter().for_each(|s| s.send(mm.clone().into()))
    }

    // 发送至指定index的sender. 失败会panic
    #[inline]
    pub fn send_to(&self, m: impl Into<M>, sender_index: usize) {
        self.senders[sender_index].send(m.into());
    }

    // 发送至指定index的sender. 失败会返回消息
    #[inline]
    pub fn try_send_to<MM>(&self, m: MM, sender_index: usize) -> Result<(), ChannelError<MM>>
    where
        MM: Into<M>,
    {
        match self.senders.get(sender_index) {
            Some(s) => {
                s.send(m.into());
                Ok(())
            }
            None => Err(ChannelError::SenderOverFlow(m)),
        }
    }

    // 发送至指定group. group查找失败失败会返回消息.(group内的sender发送失败会panic)
    #[inline]
    pub fn try_send_group<MM>(&self, mm: MM, group_index: usize) -> Result<(), ChannelError<MM>>
    where
        MM: Into<M> + Clone,
    {
        match self.group.get(group_index) {
            Some(g) => {
                g.iter().for_each(|i| self.send_to(mm.clone(), *i));
                Ok(())
            }
            None => Err(ChannelError::SenderGroupNotFound(mm)),
        }
    }
}

pub struct GroupReceiver<M, const N: usize> {
    receivers: StackGroup<Receiver<M>, N>,
}

impl<M, const N: usize> GroupReceiver<M, N> {
    pub(crate) fn from_vec(vec: Vec<Receiver<M>>) -> Self {
        Self {
            receivers: StackGroup::from_vec(vec),
        }
    }
}

impl<M, const N: usize> Deref for GroupReceiver<M, N> {
    type Target = [Receiver<M>];

    fn deref(&self) -> &Self::Target {
        self.receivers.deref()
    }
}

struct StackGroup<T, const N: usize> {
    group: ManuallyDrop<MaybeUninit<[T; N]>>,
}

impl<T, const N: usize> StackGroup<T, N> {
    fn from_vec(mut vec: Vec<T>) -> Self {
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

impl<T, const N: usize> Deref for StackGroup<T, N> {
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

impl<T, const N: usize> DerefMut for StackGroup<T, N> {
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

impl<T, const N: usize> Drop for StackGroup<T, N> {
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

    use std::cell::Cell;
    use std::rc::Rc;

    #[test]
    fn stack_group_iter() {
        let v = vec![1, 2, 3];

        let group = StackGroup::<_, 3>::from_vec(v);

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

        let group = StackGroup::<_, 4>::from_vec(v);

        drop(group);

        assert_eq!(rc.get(), 4);
    }

    #[test]
    fn stack_group_get_idx() {
        let v = vec![1, 2, 3];

        let group = StackGroup::<_, 3>::from_vec(v);

        assert_eq!(group[0], 1);
        assert_eq!(group[1], 2);
        assert_eq!(group[2], 3);
    }

    #[test]
    #[should_panic]
    fn stack_group_get_idx_out_of_range() {
        let v = vec![1, 2, 3];

        let group = StackGroup::<_, 3>::from_vec(v);

        let _ = group[3];
    }
}
