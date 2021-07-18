use core_affinity::CoreId;

use super::api::API;
use super::strategy::{Strategy, StrategyCtx};
use super::util::channel::{Receiver, Sender};

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

    #[inline]
    pub(super) fn run_in_core(self, core_id: CoreId) {
        std::thread::spawn(move || {
            core_affinity::set_for_current(core_id);
            self.run()
        });
    }

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
}
