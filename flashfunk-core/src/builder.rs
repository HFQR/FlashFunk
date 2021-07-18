use super::api::API;
use super::strategy::Strategy;
use super::util::channel::{channel, GroupReceiver, GroupSender};
use super::worker::Worker;

// 通道容量设为1024.如果单tick中每个策略的消息数量超过这个数值（或者有消息积压），可以考虑放松此上限。
// 只影响内存占用。 fixme:  开始启动的时候会导致消息过多 造成pusherror
const MESSAGE_LIMIT: usize = 3024usize;

pub struct APIBuilder<A, S, const N: usize> {
    pub(crate) api: A,
    pub(crate) strategies: [S; N],
}

impl<A, S, const N: usize> APIBuilder<A, S, N>
where
    A: API + 'static,
    S: Strategy<A> + 'static,
{
    pub fn build(self) {
        let Self { api, strategies } = self;

        // 收集核心cid
        let mut cores = core_affinity::get_core_ids().unwrap();

        // symbols为订阅symbol &str的非重复vec集合
        let mut symbols = Vec::new();

        // groups为与symbols相对应(vec index)的策略们的发送端vec.
        let mut group = Vec::<Vec<usize>>::new();

        // 单向spsc:
        // API -> Strategies.
        let mut senders = Vec::new();
        // Strategies -> API.
        let mut receivers = Vec::new();

        let mut st_index = 0;

        for st in strategies {
            st.symbol().iter().for_each(|symbol| {
                symbols
                    .iter()
                    .enumerate()
                    .find_map(|(index, s)| if s == symbol { Some(index) } else { None })
                    .map(|index| {
                        group.get_mut(index).unwrap().push(st_index);
                    })
                    .unwrap_or_else(|| {
                        group.push(vec![st_index]);
                        symbols.push(*symbol);
                    });
            });

            // API -> Strategies
            let (s1, r1) = channel(MESSAGE_LIMIT);

            // Strategies -> API.
            let (s2, r2) = channel(MESSAGE_LIMIT);

            senders.push(s1);
            receivers.push(r2);

            let core = cores.pop().unwrap();
            Worker::new(st, s2, r1).run_in_core(core);

            st_index += 1;
        }

        let group_senders = GroupSender::new(senders, group);
        let group_receivers = GroupReceiver::<_, N>::from_vec(receivers);

        // 分配最后一个核心给主线程
        let core = cores.pop().unwrap();
        core_affinity::set_for_current(core);

        api.run(symbols, group_senders, group_receivers);
    }
}
