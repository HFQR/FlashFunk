use core::{
    fmt::{Debug, Display, Formatter, Result as FmtResult},
    ops::{Deref, DerefMut},
};

use alloc::vec::Vec;

#[cfg(feature = "async")]
use {alloc::sync::Arc, futures_core::task::__internal::AtomicWaker};

use super::{spmc, spsc};

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
    tx: spsc::Producer<M>,
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
    rx: spsc::Consumer<M>,
    #[cfg(feature = "async")]
    waker: Arc<AtomicWaker>,
}

pub struct BroadcastSender<M> {
    tx: spmc::Producer<M>,
    #[cfg(feature = "async")]
    waker: Arc<AtomicWaker>,
}

impl<M> BroadcastSender<M> {
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

pub struct BroadcastReceiver<M> {
    rx: spmc::Consumer<M>,
    #[cfg(feature = "async")]
    waker: Arc<AtomicWaker>,
}

impl<M> Clone for BroadcastReceiver<M> {
    fn clone(&self) -> Self {
        Self {
            rx: self.rx.clone(),
            #[cfg(feature = "async")]
            waker: Arc::new(AtomicWaker::new()),
        }
    }
}

#[cfg(not(feature = "async"))]
pub use r#sync::*;

#[cfg(not(feature = "async"))]
mod r#sync {
    use super::*;

    pub fn channel<M>(cap: usize) -> (Sender<M>, Receiver<M>) {
        let (tx, rx) = spsc::new(cap);
        (Sender { tx }, Receiver { rx })
    }

    pub fn broadcast<M>(cap: usize) -> (BroadcastSender<M>, BroadcastReceiver<M>) {
        let (tx, rx) = spmc::new(cap);
        (BroadcastSender { tx }, BroadcastReceiver { rx })
    }

    impl<M> Receiver<M> {
        #[inline]
        pub fn recv(&mut self) -> Result<M, ChannelError<M>> {
            self.rx.pop().map_err(|_| ChannelError::RecvError)
        }
    }

    impl<M> BroadcastReceiver<M> {
        #[inline]
        pub fn recv(&mut self) -> Result<&M, ChannelError<M>> {
            self.rx.pop().ok_or(ChannelError::RecvError)
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
        let (tx, rx) = spsc::new(cap);
        let waker = Arc::new(AtomicWaker::new());
        (
            Sender {
                tx,
                waker: waker.clone(),
            },
            Receiver { rx, waker },
        )
    }

    pub fn broadcast<M>(cap: usize) -> (BroadcastSender<M>, BroadcastReceiver<M>) {
        let (tx, rx) = spmc::new(cap);
        let waker = Arc::new(AtomicWaker::new());
        (
            BroadcastSender {
                tx,
                waker: waker.clone(),
            },
            BroadcastReceiver { rx, waker },
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

    impl<M> BroadcastReceiver<M> {
        #[inline]
        pub async fn recv(&mut self) -> Result<&M, ChannelError<M>> {
            todo!()
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

pub struct GroupReceiver<M, const N: usize> {
    receivers: [Receiver<M>; N],
}

impl<M, const N: usize> GroupReceiver<M, N> {
    pub(crate) fn from_vec(vec: Vec<Receiver<M>>) -> Self {
        Self {
            receivers: vec.try_into().ok().unwrap(),
        }
    }
}

impl<M, const N: usize> Deref for GroupReceiver<M, N> {
    type Target = [Receiver<M>];

    fn deref(&self) -> &Self::Target {
        &self.receivers
    }
}

impl<M, const N: usize> DerefMut for GroupReceiver<M, N> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.receivers
    }
}
