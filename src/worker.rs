use core::time::Duration;

use std::time::Instant;

use crate::ac::__Strategy;
use crate::builder::CtpBuilder;
use crate::context::{new_context, ContextTrait};
use crate::ctp::md_api::MdApi;
use crate::ctp::td_api::TdApi;
use crate::interface::Interface;
use crate::types::message::{MdApiMessage, StrategyMessage, TdApiMessage};
use crate::util::channel::{channel, GroupSender, Receiver, Sender};

// 主线程工人。阻塞主线程，接收策略消息并发起api的回调。
struct MainWorker {
    receiver: Vec<Receiver<StrategyMessage>>,
}

impl MainWorker {
    fn new(receiver: Vec<Receiver<StrategyMessage>>) -> Self {
        Self { receiver }
    }

    fn block_handle(&self, mut td_api: TdApi) {
        const INTERVAL: Duration = Duration::from_secs(1);

        let mut r = true;
        let mut now = Instant::now();
        loop {
            self.receiver
                .iter()
                .enumerate()
                .filter_map(|(idx, c)| c.recv().map(|msg| (idx, msg)).ok())
                // 此处idx为策略通道的index，可以交给回调用以确认回程消息的策略
                .for_each(|(idx, msg)| match msg {
                    StrategyMessage::OrderRequest(req) => {
                        td_api.send_order(idx, req);
                    }
                    StrategyMessage::CancelRequest(req) => {
                        td_api.cancel_order(req);
                    }
                    _ => {}
                });

            if Instant::now().duration_since(now) >= INTERVAL {
                if r {
                    td_api.req_position();
                } else {
                    td_api.req_account();
                }
                r = !r;
                now = Instant::now();
            }
        }
    }
}

// 策略工人，接收api的回调并处理，之后可发送消息给主线程工人。每个工人占据一个线程
struct StrategyWorker {
    st: __Strategy,
    c_md: Receiver<MdApiMessage>,
    c_td: Receiver<TdApiMessage>,
    pub p_st: Sender<StrategyMessage>,
}

impl StrategyWorker {
    fn new(
        st: __Strategy,
        c_md: Receiver<MdApiMessage>,
        c_td: Receiver<TdApiMessage>,
        p_st: Sender<StrategyMessage>,
    ) -> Self {
        Self {
            st,
            c_md,
            c_td,
            p_st,
        }
    }

    // 初次启动
    fn start(mut self) {
        std::thread::spawn(move || self._start());
    }

    fn _start(&mut self) {
        // worker上下文. 保存order_id和其对应的exchange
        let mut ctx = new_context(&self.p_st);
        loop {
            // 接收md_api的消息并处理。
            match self.c_md.recv() {
                Ok(msg) => match msg {
                    // tick会返回订单vec，我们转发至td api
                    MdApiMessage::TickData(data) => self.st.on_tick(&data, &mut ctx),
                    _ => {}
                },
                Err(_) => (),
            }
            // 接收td_api的消息并处理。
            match self.c_td.recv() {
                Ok(msg) => match msg {
                    TdApiMessage::OrderData(data) => {
                        // 可以在这里插入信息至ctx
                        // 目前所有order都会被加入map，需要过滤我们建立的单子。
                        // 目前没有删除机制，此insert会泄露内存。
                        ctx.add_order(data.clone());
                        ctx.insert_order(&data);
                        self.st.on_order(&data, &mut ctx);
                    }
                    TdApiMessage::TradeData(data) => self.st.on_trade(&data, &mut ctx),
                    TdApiMessage::AccountData(data) => {
                        ctx.update_account(&data);
                        self.st.on_account(&data, &mut ctx)
                    }
                    TdApiMessage::PositionData(data) => {
                        ctx.update_position_by_pos(&data);
                        self.st.on_position(&data, &mut ctx);
                    }
                    TdApiMessage::ContractData(data) => {
                        ctx.1
                            .insert_exchange(data.symbol.as_str(), data.exchange.unwrap());
                        self.st.on_contract(&data, &mut ctx);
                    }
                },
                Err(_) => (),
            }
        }
    }
}

impl Drop for StrategyWorker {
    fn drop(&mut self) {
        // worker退出时如果线程panic则恢复运行
        if std::thread::panicking() {
            self._start();
        }
    }
}

// 为builder建立工人并启动
pub(crate) fn start_workers(mut builder: CtpBuilder<'_>) {
    // 准备所有策略工人，并返回对应的通道制造（消费）端 和订阅symbols集合
    // * 策略已被此函数消费无法再次调用。
    let (symbols, workers, s_md, s_td, c_st) = prepare_worker_channel(&mut builder);

    let id = builder.id.into();
    let pwd = builder.pwd.into();
    let path = builder.path.into();

    // 构造api
    let mut md_api = MdApi::new(id, pwd, path, symbols);
    let mut td_api = TdApi::new("".parse().unwrap());

    let login_form = builder.login_form;

    // 连接
    md_api.connect(&login_form, s_md);

    // 订阅
    md_api.subscribe();

    td_api.connect(&login_form, s_td);

    // 启动策略工人线程。
    workers.into_iter().for_each(|worker| worker.start());

    // 此处策略消费端会阻塞线程并发送消息给td_api。
    c_st.block_handle(td_api);
}

// 通道容量设为1024.如果单tick中每个策略的消息数量超过这个数值（或者有消息积压），可以考虑放松此上限。
// 只影响内存占用。
const MESSAGE_LIMIT: usize = 1024usize;

// 帮助函数
fn prepare_worker_channel(
    builder: &mut CtpBuilder<'_>,
) -> (
    Vec<&'static str>,
    Vec<StrategyWorker>,
    GroupSender<MdApiMessage>,
    GroupSender<TdApiMessage>,
    MainWorker,
) {
    let sts = std::mem::take(&mut builder.strategy);

    let cap = sts.len();

    // 单向spsc:
    // MpApi -> Strategies.
    let mut producer_md = Vec::with_capacity(cap);
    // Strategies -> TdApi.
    let mut producer_td = Vec::with_capacity(cap);
    // TdApi -> Strategies.
    let mut consumer_st = Vec::with_capacity(cap);

    // 策略工人集合
    let mut workers = Vec::with_capacity(cap);

    // symbols为订阅symbol &str的非重复vec集合
    let mut symbols = Vec::new();
    // groups为与symbols相对应(vec index)的策略们的发送端vec，该vec保存策略发送端在producer_md vec中的index
    let mut groups: Vec<Vec<usize>> = Vec::new();

    sts.into_iter().enumerate().for_each(|(st_index, st)| {
        st.symbols().iter().for_each(|symbol| {
            symbols
                .iter()
                .enumerate()
                .find_map(|(index, s)| if s == symbol { Some(index) } else { None })
                .map(|index| {
                    groups.get_mut(index).unwrap().push(st_index);
                })
                .unwrap_or_else(|| {
                    groups.push(vec![st_index]);
                    symbols.push(*symbol);
                });
        });

        // MpApi -> Strategies
        let (s_md, r_md) = channel(MESSAGE_LIMIT);

        // Strategies -> TdApi.
        let (s_st, r_st) = channel(MESSAGE_LIMIT);

        // TdApi -> Strategies.
        let (s_td, r_td) = channel(MESSAGE_LIMIT);

        let worker = StrategyWorker::new(st, r_md, r_td, s_st);

        workers.push(worker);
        producer_md.push(s_md);
        producer_td.push(s_td);
        consumer_st.push(r_st);
    });

    (
        symbols,
        workers,
        GroupSender::new(producer_md, groups),
        GroupSender::new(producer_td, vec![]),
        MainWorker::new(consumer_st),
    )
}
