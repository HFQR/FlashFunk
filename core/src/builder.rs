use alloc::vec::Vec;

use super::{
    api::API,
    strategy::Strategy,
    util::{
        channel::{broadcast, channel, GroupReceiver},
        pin_to_core,
    },
    worker::Worker,
};

// 通道容量设为1024.如果单tick中每个策略的消息数量超过这个数值（或者有消息积压），可以考虑放松此上限。
// 只影响内存占用。 fixme:  开始启动的时候会导致消息过多 造成pusherror
const MESSAGE_LIMIT: usize = 3024usize;

pub struct APIBuilder<A, S, const N: usize> {
    pub(crate) pin_to_core: bool,
    pub(crate) message_capacity: usize,
    pub(crate) api: A,
    pub(crate) strategies: [S; N],
}

impl<A, S, const N: usize> APIBuilder<A, S, N>
where
    A: API + 'static,
    S: Strategy<A> + 'static,
{
    pub(super) fn new(api: A, strategies: [S; N]) -> Self {
        Self {
            pin_to_core: true,
            message_capacity: MESSAGE_LIMIT,
            api,
            strategies,
        }
    }

    /// Do not pin strategy worker thread to cpu cores.
    pub fn disable_pin_to_core(mut self) -> Self {
        self.pin_to_core = false;
        self
    }

    /// Change capacity of message channel between API thread and strategy worker threads.
    ///
    /// Capacity is for per strategy.
    pub fn message_capacity(mut self, cap: usize) -> Self {
        assert_ne!(cap, 0);
        self.message_capacity = cap;
        self
    }

    /// Build and start API on current thread.
    /// [API::run](crate::api::API::run) would be called when build finished.
    ///
    /// Every strategy would run on it's own dedicated thread.
    pub fn build(self) {
        let Self {
            pin_to_core,
            message_capacity,
            api,
            strategies,
        } = self;

        // 收集核心cid
        let mut cores = pin_to_core::get_core_ids();

        // 单向spmc:
        // API -> Strategies.
        let (broadcast_tx, broadcast_rx) = broadcast(self.message_capacity);

        // Strategies -> API.
        let mut receivers = Vec::new();

        for st in strategies.into_iter() {
            // Strategies -> API.
            let (tx, rx) = channel(message_capacity);

            receivers.push(rx);

            let id = pin_to_core.then(|| cores.pop()).flatten();
            Worker::new(st, tx, broadcast_rx.clone()).run_in_core(id);
        }

        let group_receivers = GroupReceiver::<_, N>::from_vec(receivers);

        // 分配最后一个核心给主线程
        let id = pin_to_core.then(|| cores.pop()).flatten();
        pin_to_core::pin_to_core(id);

        api.run(broadcast_tx, group_receivers);
    }
}
