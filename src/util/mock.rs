use core::sync::atomic::{AtomicBool, Ordering};
use core::time::Duration;

use std::rc::Rc;
use std::sync::Arc;
use std::thread::JoinHandle;

use crate::interface::Interface;
use crate::structs::{LoginForm, TickData, CancelRequest, OrderRequest, OrderData};
use crate::types::message::{MdApiMessage, TdApiMessage};
use crate::util::channel::{GroupSender, Sender};
use crate::account::Account;
use chrono::NaiveDate;

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
}

impl MockTdApi {
    fn change_req_to_data(&mut self, order_req: OrderRequest) -> OrderData {
        unimplemented!()
    }
}

/// todo: we need to build a Local TdApi with Account\
/// It should provide a settle check
impl Interface for MockTdApi {
    type Message = TdApiMessage;

    fn new(id: impl Into<Vec<u8>>, pwd: impl Into<Vec<u8>>, path: impl Into<Vec<u8>>, symbols: Vec<&'static str>) -> Self {
        unimplemented!()
    }

    /// 发单
    fn send_order(&mut self, idx: usize, order: OrderRequest) {
        // self.acc.update_order(self.change_req_to_data(order))
    }

    /// 撤单
    fn cancel_order(&mut self, req: CancelRequest) {}

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
}