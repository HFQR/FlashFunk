use core::{
    fmt::{Debug, Display, Formatter, Result as FmtResult},
    ops::{Deref, DerefMut},
};

use alloc::vec::Vec;

#[cfg(feature = "async")]
use {alloc::sync::Arc, futures_core::task::__internal::AtomicWaker};

use super::{
    spsc::{new, Consumer, Producer},
    stack_array::StackArray,
};

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

pub struct Sender<M> {
    tx: Producer<M>,
    #[cfg(feature = "async")]
    waker: Arc<AtomicWaker>,
}

impl<M> Sender<M> {
    // 发送失败会panic
    #[inline]
    pub fn send(&mut self, m: impl Into<M>) {
        self.tx.push(m.into()).unwrap();
        #[cfg(feature = "async")]
        self.waker.wake();
    }

    // 发送失败返回消息
    #[inline]
    pub fn try_send(&mut self, m: M) -> Result<(), ChannelError<M>> {
        match self.tx.push(m) {
            Ok(_) => {
                #[cfg(feature = "async")]
                self.waker.wake();
                Ok(())
            }
            Err(e) => Err(ChannelError::TrySendError(e.0)),
        }
    }
}

pub struct Receiver<M> {
    rx: Consumer<M>,
    #[cfg(feature = "async")]
    waker: Arc<AtomicWaker>,
}

#[cfg(not(feature = "async"))]
pub use r#sync::*;

#[cfg(not(feature = "async"))]
mod r#sync {
    use super::*;

    pub fn channel<M>(cap: usize) -> (Sender<M>, Receiver<M>) {
        let (tx, rx) = new(cap);
        (Sender { tx }, Receiver { rx })
    }

    impl<M> Receiver<M> {
        #[inline]
        pub fn recv(&mut self) -> Result<M, ChannelError<M>> {
            self.rx.pop().map_err(|_| ChannelError::RecvError)
        }
    }
}

#[cfg(feature = "async")]
pub use r#async::*;

#[cfg(feature = "async")]
mod r#async {
    use super::*;

    use core::{
        future::Future,
        pin::Pin,
        task::{Context, Poll},
    };

    pub fn channel<M>(cap: usize) -> (Sender<M>, Receiver<M>) {
        let (tx, rx) = new(cap);
        let waker = Arc::new(AtomicWaker::new());
        (
            Sender {
                tx,
                waker: waker.clone(),
            },
            Receiver { rx, waker },
        )
    }

    impl<M> Receiver<M> {
        #[inline]
        pub async fn recv(&mut self) -> Result<M, ChannelError<M>> {
            struct Recv<'a, M>(&'a mut Receiver<M>);

            impl<M> Future for Recv<'_, M> {
                type Output = Result<M, ChannelError<M>>;

                fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
                    let this = self.get_mut();
                    match this.0.rx.pop() {
                        Ok(msg) => Poll::Ready(Ok(msg)),
                        Err(_) => {
                            this.0.waker.register(cx.waker());
                            Poll::Pending
                        }
                    }
                }
            }

            Recv(self).await
        }
    }

    impl<M, const N: usize> GroupReceiver<M, N> {
        pub async fn recv(&mut self) -> Result<M, ChannelError<M>> {
            struct GroupReceiveFut<'a, M, const N: usize> {
                group: &'a mut GroupReceiver<M, N>,
            }

            impl<M, const N: usize> Future for GroupReceiveFut<'_, M, N> {
                type Output = Result<M, ChannelError<M>>;

                fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
                    for rx in self.get_mut().group.iter_mut() {
                        match rx.rx.pop() {
                            Ok(msg) => return Poll::Ready(Ok(msg)),
                            Err(_) => {
                                rx.waker.register(cx.waker());
                            }
                        }
                    }

                    Poll::Pending
                }
            }

            GroupReceiveFut { group: self }.await
        }
    }
}

#[cfg(feature = "small-symbol")]
pub(crate) type HashMap<const N: usize> = super::no_hasher::NoHashMap<u64, ([usize; N], usize)>;

#[cfg(feature = "small-symbol")]
type KeyRef<'a> = &'a u64;

#[cfg(not(feature = "small-symbol"))]
pub(crate) type HashMap<const N: usize> =
    super::fx_hasher::FxHashMap<&'static str, ([usize; N], usize)>;

#[cfg(not(feature = "small-symbol"))]
type KeyRef<'a> = &'a str;

pub struct GroupSender<M, const N: usize> {
    senders: StackArray<Sender<M>, N>,
    group: HashMap<N>,
}

impl<M, const N: usize> GroupSender<M, N> {
    pub fn new(sender: Vec<Sender<M>>, group: HashMap<N>) -> Self {
        Self {
            senders: StackArray::from_vec(sender),
            group,
        }
    }

    #[inline]
    pub fn group(&self) -> &HashMap<N> {
        &self.group
    }

    #[inline]
    pub fn senders(&self) -> &[Sender<M>] {
        &*self.senders
    }

    // 发送至所有sender
    #[inline]
    pub fn send_all<MM>(&mut self, mm: MM)
    where
        MM: Into<M> + Clone,
    {
        self.senders
            .iter_mut()
            .for_each(|s| s.send(mm.clone().into()))
    }

    // 发送至指定index的sender. 失败会panic
    #[inline]
    pub fn send_to(&mut self, m: impl Into<M>, sender_index: usize) {
        self.senders[sender_index].send(m.into());
    }

    // 发送至指定index的sender. 失败会返回消息
    #[inline]
    pub fn try_send_to<MM>(&mut self, m: MM, sender_index: usize) -> Result<(), ChannelError<MM>>
    where
        MM: Into<M>,
    {
        match self.senders.get_mut(sender_index) {
            Some(s) => {
                s.send(m.into());
                Ok(())
            }
            None => Err(ChannelError::SenderOverFlow(m)),
        }
    }

    // 发送至指定group. group查找失败失败会返回消息.(group内的sender发送失败会panic)
    #[inline]
    pub fn try_send_group<MM>(&mut self, mm: MM, symbol: KeyRef<'_>) -> Result<(), ChannelError<MM>>
    where
        MM: Into<M> + Clone,
    {
        match self.group.get_mut(symbol) {
            Some((arr, len)) => {
                arr[..*len]
                    .iter_mut()
                    .for_each(|i| self.senders[*i].send(mm.clone().into()));
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

impl<M, const N: usize> DerefMut for GroupReceiver<M, N> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.receivers.deref_mut()
    }
}
