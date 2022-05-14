use alloc::vec::Vec;

use super::{
    api::API,
    strategy::Strategy,
    util::{
        channel::{channel, GroupReceiver, GroupSender},
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
    #[allow(clippy::explicit_counter_loop)]
    pub fn build(self) {
        let Self {
            pin_to_core,
            message_capacity,
            api,
            strategies,
        } = self;

        // 收集核心cid
        let mut cores = pin_to_core::get_core_ids();

        // 单向spsc:
        // API -> Strategies.
        let mut senders = Vec::new();
        // Strategies -> API.
        let mut receivers = Vec::new();

        // groups为与symbols相对应(vec index)的策略们的发送端vec.
        let mut group = {
            #[cfg(not(feature = "small-symbol"))]
            {
                crate::util::fx_hasher::FxHashMap::default()
            }

            #[cfg(feature = "small-symbol")]
            {
                crate::util::no_hasher::NoHashMap::default()
            }
        };

        let mut st_index = 0usize;
        for st in strategies {
            st.symbol().iter().for_each(|symbol| {
                let g = {
                    #[cfg(feature = "small-symbol")]
                    {
                        let bytes = symbol.as_bytes();

                        assert!(bytes.len() <= 8, "small-symbol feature require a symbol with no more than 8 bytes in length.");

                        let mut buf = [0; 8];
                        for (idx, char) in bytes.into_iter().enumerate() {
                            buf[idx] = *char;
                        }

                        let symbol = u64::from_le_bytes(buf.try_into().unwrap());

                        group.entry(symbol).or_insert_with(Vec::<usize>::new)
                    }

                    #[cfg(not(feature = "small-symbol"))]
                    {
                        group.entry(*symbol).or_insert_with(Vec::<usize>::new)
                    }
                };

                assert!(!g.contains(&st_index));

                g.push(st_index);
            });

            // API -> Strategies
            let (s1, r1) = channel(message_capacity);

            // Strategies -> API.
            let (s2, r2) = channel(message_capacity);

            senders.push(s1);
            receivers.push(r2);

            let id = pin_to_core.then(|| cores.pop()).flatten();
            Worker::new(st, s2, r1).run_in_core(id);

            st_index += 1;
        }

        // shrink the Vec<usize> to Box<[usize]>
        let group = group
            .into_iter()
            .map(|(k, v)| (k, v.into_boxed_slice()))
            .collect();

        let group_senders = GroupSender::<_, N>::new(senders, group);
        let group_receivers = GroupReceiver::<_, N>::from_vec(receivers);

        // 分配最后一个核心给主线程
        let id = pin_to_core.then(|| cores.pop()).flatten();
        pin_to_core::pin_to_core(id);

        api.run(group_senders, group_receivers);
    }
}
