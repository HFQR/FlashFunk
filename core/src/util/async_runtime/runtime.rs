use core::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};

use super::{
    park::{Park, Unpark},
    waker::waker_fn,
};

pub struct Runtime<P: Park> {
    parker: P,
}

impl<P: Park> Runtime<P> {
    pub fn new(parker: P) -> Self {
        Self { parker }
    }

    pub fn block_on<Fut: Future>(&mut self, mut fut: Fut) -> Fut::Output {
        let unparker = self.parker.unparker();

        let waker = waker_fn(move || unparker.unpark());

        // SAFETY
        // Pinning is safe. Future is shadow named after Pin.
        // It's not possible to move Fut without going through the Pin.
        let mut fut = unsafe { Pin::new_unchecked(&mut fut) };

        loop {
            let cx = &mut Context::from_waker(&waker);

            match fut.as_mut().poll(cx) {
                Poll::Ready(res) => return res,
                Poll::Pending => self.parker.park(),
            }
        }
    }
}
