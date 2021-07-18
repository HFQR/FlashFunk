use core::ops::{Deref, DerefMut};

use super::api::API;
use super::util::channel::Sender;

/// Trait for single strategy of given NAME.
pub trait Strategy<A: API>
where
    A: API,
    Self: Send,
{
    fn symbol(&self) -> &[&'static str];

    /// Method called when a new message is received by strategy.
    fn call(&mut self, msg: A::SndMessage, ctx: &mut StrategyCtx<A::RecvMessage, A::Context>);

    /// Method called when all message are processed by strategy and wait for next
    /// message to arrive.
    #[allow(unused_variables)]
    fn on_idle(&mut self, ctx: &mut StrategyCtx<A::RecvMessage, A::Context>) {}
}

impl<S, A> Strategy<A> for Box<S>
where
    S: Strategy<A> + ?Sized,
    A: API,
{
    #[inline]
    fn symbol(&self) -> &[&'static str] {
        (**self).symbol()
    }

    #[inline]
    fn call(&mut self, msg: A::SndMessage, ctx: &mut StrategyCtx<A::RecvMessage, A::Context>) {
        (**self).call(msg, ctx)
    }

    #[inline]
    fn on_idle(&mut self, ctx: &mut StrategyCtx<A::RecvMessage, A::Context>) {
        (**self).on_idle(ctx)
    }
}

/// Context type of a strategy.
pub struct StrategyCtx<R, I> {
    sender: Sender<R>,
    inner: I,
}

impl<R, I: Default> StrategyCtx<R, I> {
    pub(super) fn new(sender: Sender<R>) -> Self {
        Self {
            sender,
            inner: I::default(),
        }
    }

    /// Get a sender type from strategy which can be used to send message
    /// to API.
    #[inline]
    pub fn sender(&self) -> &Sender<R> {
        &self.sender
    }
}

impl<R, I> Deref for StrategyCtx<R, I> {
    type Target = I;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<R, I> DerefMut for StrategyCtx<R, I> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
