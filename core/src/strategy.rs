use super::{api::API, util::channel::Sender};

/// Trait for single strategy of given NAME.
pub trait Strategy<A>
where
    A: API,
    Self: Send,
{
    fn symbol(&self) -> &[&'static str];

    /// Method called when strategy is about to start.
    #[allow(unused_variables)]
    fn on_start(&mut self, ctx: &mut Context<A>) {}

    /// Method called when a new message is received by strategy.
    fn call(&mut self, msg: A::SndMessage, ctx: &mut Context<A>);

    /// Method called when all message are processed by strategy and wait for next
    /// message to arrive.
    #[allow(unused_variables)]
    fn on_idle(&mut self, ctx: &mut Context<A>) {}
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
    fn on_start(&mut self, ctx: &mut Context<A>) {
        (**self).on_start(ctx)
    }

    #[inline]
    fn call(&mut self, msg: A::SndMessage, ctx: &mut Context<A>) {
        (**self).call(msg, ctx)
    }

    #[inline]
    fn on_idle(&mut self, ctx: &mut Context<A>) {
        (**self).on_idle(ctx)
    }
}

/// Context type of a strategy.
pub struct Context<A>
where
    A: API,
{
    sender: Sender<A::RecvMessage>,
}

impl<A> Context<A>
where
    A: API,
{
    pub(super) fn new(sender: Sender<A::RecvMessage>) -> Self {
        Self { sender }
    }

    /// Get a sender type from strategy which can be used to send message
    /// to API.
    #[inline]
    pub fn sender(&mut self) -> &mut Sender<A::RecvMessage> {
        &mut self.sender
    }
}
