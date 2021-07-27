use core::{
    fmt::{Debug, Display, Formatter, Result as FmtResult},
    ops::Deref,
};

use ahash::AHashMap;
use alloc::vec::Vec;

use super::spsc::{new, Consumer, Producer};
use super::stack_array::StackArray;

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
    senders: StackArray<Sender<M>, N>,
    group: AHashMap<&'static str, Vec<usize>>,
}

impl<M, const N: usize> GroupSender<M, N> {
    pub fn new(sender: Vec<Sender<M>>, group: AHashMap<&'static str, Vec<usize>>) -> Self {
        Self {
            senders: StackArray::from_vec(sender),
            group,
        }
    }

    #[inline]
    pub fn group(&self) -> &AHashMap<&'static str, Vec<usize>> {
        &self.group
    }

    #[inline]
    pub fn senders(&self) -> &[Sender<M>] {
        &*self.senders
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
    pub fn try_send_group<MM>(&self, mm: MM, symbol: &str) -> Result<(), ChannelError<MM>>
    where
        MM: Into<M> + Clone,
    {
        match self.group.get(symbol) {
            Some(g) => {
                g.iter().for_each(|i| self.send_to(mm.clone(), *i));
                Ok(())
            }
            None => Err(ChannelError::SenderGroupNotFound(mm)),
        }
    }
}

pub struct GroupReceiver<M, const N: usize> {
    receivers: StackArray<Receiver<M>, N>,
}

impl<M, const N: usize> GroupReceiver<M, N> {
    pub(crate) fn from_vec(vec: Vec<Receiver<M>>) -> Self {
        Self {
            receivers: StackArray::from_vec(vec),
        }
    }
}

impl<M, const N: usize> Deref for GroupReceiver<M, N> {
    type Target = [Receiver<M>];

    fn deref(&self) -> &Self::Target {
        self.receivers.deref()
    }
}
