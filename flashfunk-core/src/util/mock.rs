use core::sync::atomic::{AtomicBool, Ordering};
use core::time::Duration;

use std::rc::Rc;
use std::sync::Arc;
use std::thread::JoinHandle;

use crate::interface::Interface;
use crate::structs::{LoginForm, TickData, CancelRequest, OrderRequest, OrderData};
use crate::types::message::{MdApiMessage, TdApiMessage};
use crate::util::channel::{GroupSender};
use crate::account::Account;
use chrono::{NaiveDate, Utc};

pub struct MockMdApi {
    symbols: Vec<&'static str>,
    sender: Option<GroupSender<MdApiMessage>>,
    shutdown: Arc<AtomicBool>,
    handle: Option<JoinHandle<()>>,
}

impl Drop for MockMdApi {
    fn drop(&mut self) {
        self.shutdown.store(true, Ordering::SeqCst);
        if let Some(handle) = self.handle.take() {
            handle.join().unwrap();
        }
    }
}

impl Interface for MockMdApi {
    type Message = MdApiMessage;

    fn new(
        _: impl Into<Vec<u8>>,
        _: impl Into<Vec<u8>>,
        symbols: Vec<&'static str>,
    ) -> Self {
        MockMdApi {
            symbols,
            sender: None,
            shutdown: Arc::new(AtomicBool::new(false)),
            handle: None,
        }
    }

    fn connect(&mut self, _: &LoginForm, sender: GroupSender<Self::Message>) {
        self.sender = Some(sender);
    }

    fn subscribe(&mut self) {
        let sender = self.sender.take().unwrap();
        let shutdown = self.shutdown.clone();
        let handle = std::thread::spawn(move || {
            let rt = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .unwrap();

            tokio::task::LocalSet::new().block_on(&rt, async move {
                let sender = Rc::new(sender);

                // 每个单独的回调任务都可以参照这个task.
                let a = tokio::task::spawn_local({
                    let sender = sender.clone();
                    async move {
                        loop {
                            if shutdown.load(Ordering::SeqCst) {
                                return;
                            }

                            // 每500ms发送一个tick data;
                            tokio::time::sleep(Duration::from_millis(500)).await;

                            // 在这里修改tick data数据;

                            let msg: &'static TickData = Box::leak(Box::new(TickData::default()));

                            sender.send_all(msg);
                        }
                    }
                });

                a.await.unwrap();
            });
        });

        self.handle = Some(handle);
    }
}


pub struct MockTdApi {
    acc: Account,
    date: NaiveDate,
    sender: Option<GroupSender<TdApiMessage>>,
    current_tick: TickData,
}

impl MockTdApi {
    fn change_req_to_data(&mut self, order_req: OrderRequest) -> OrderData {
        unimplemented!()
    }


    fn match_order(&mut self) {}
    /// todo:
    ///  1. 检查订单队列是否成交
    ///  2. 更新订单数据 通过self.sender将数据推送出去， 出现资金不足的情况下 可以先打印 我后面会补充log信息
    fn check(&mut self) {}
}

/// todo: we need to build a Local TdApi with Account\
/// It should provide a settle check
impl Interface for MockTdApi {
    type Message = TdApiMessage;

    fn new(id: impl Into<Vec<u8>>, pwd: impl Into<Vec<u8>>, symbols: Vec<&'static str>) -> Self {
        println!("{:?}", symbols);
        MockTdApi {
            acc: Account::new(),
            // fixme: 需要将这个更换为回测过程中的交易日， 每日阶段后进行切换
            date: Utc::today().naive_local(),
            sender: None,
            current_tick: Default::default(),
        }
    }

    /// 发单
    fn send_order(&mut self, idx: usize, order: OrderRequest) {
        //todo: 将报单更新到订单队列
        // self.acc.update_order(self.change_req_to_data(order))
    }

    /// 撤单
    fn cancel_order(&mut self, req: CancelRequest) {
        // todo： 如果req的id还保存在订单队列 需要进场撤单。然后更新报单的状态 发挥给策略
    }

    /// 登录接口
    fn connect(&mut self, req: &LoginForm, sender: GroupSender<Self::Message>) {
        self.sender = Some(sender)
    }

    /// 订阅行情
    fn subscribe(&mut self) {
        unimplemented!()
    }

    /// 取消订阅行情
    fn unsubscribe(&mut self, symbol: String) {
        unimplemented!()
    }

    /// 释放退出接口
    fn exit(&mut self) {
        unimplemented!()
    }

    /// 更新行情
    fn update_quote(&mut self, quote: &TickData) {
        self.current_tick = quote.clone();
        self.check();
    }
}