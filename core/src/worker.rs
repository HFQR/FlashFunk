use super::api::API;
use super::strategy::{Strategy, StrategyCtx};
use super::util::{
    channel::{Receiver, Sender},
    pin_to_core::{self, CoreId},
};

pub struct Worker<S, A>
where
    S: Strategy<A>,
    A: API,
{
    strategy: S,
    sender: Sender<A::RecvMessage>,
    receiver: Receiver<A::SndMessage>,
}

impl<S, A> Worker<S, A>
where
    S: Strategy<A> + 'static,
    A: API + 'static,
{
    pub(super) fn new(
        strategy: S,
        sender: Sender<A::RecvMessage>,
        receiver: Receiver<A::SndMessage>,
    ) -> Self {
        Self {
            strategy,
            sender,
            receiver,
        }
    }

    #[cfg(not(feature = "async"))]
    #[inline]
    pub(super) fn run_in_core(self, id: Option<CoreId>) {
        std::thread::spawn(move || {
            pin_to_core::pin_to_core(id);
            self.run()
        });
    }

    #[cfg(not(feature = "async"))]
    #[inline(always)]
    pub(super) fn run(self) {
        let Self {
            mut strategy,
            sender,
            receiver,
        } = self;

        let ctx = &mut StrategyCtx::new(sender);

        loop {
            if let Ok(msg) = receiver.recv() {
                strategy.call(msg, ctx);
            }
            strategy.on_idle(ctx);
        }
    }

    #[cfg(feature = "async")]
    #[inline]
    pub(super) fn run_in_core(self, id: Option<CoreId>) {
        std::thread::spawn(move || {
            tokio::runtime::Builder::new_current_thread()
                .on_thread_start(move || pin_to_core::pin_to_core(id))
                .enable_all()
                .build()
                .unwrap()
                .block_on(self.run())
        });
    }

    #[cfg(feature = "async")]
    #[inline(always)]
    pub(super) async fn run(self) {
        let Self {
            mut strategy,
            sender,
            receiver,
        } = self;

        let ctx = &mut StrategyCtx::new(sender);

        loop {
            if let Ok(msg) = receiver.recv().await {
                strategy.call(msg, ctx);
            }
            strategy.on_idle(ctx);
        }
    }
}
