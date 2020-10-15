use core::sync::atomic::{AtomicBool, Ordering};
use core::time::Duration;

use std::sync::Arc;
use std::time::Instant;

use crate::interface::Interface;
use crate::structs::{LoginForm, TickData};
use crate::types::message::MdApiMessage;
use crate::util::channel::GroupSender;
use std::rc::Rc;
use std::thread::JoinHandle;

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

    fn new(_: String, _: String, _: String, symbols: Vec<&'static str>) -> Self {
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
            let mut rt = tokio::runtime::Builder::new()
                .basic_scheduler()
                .enable_all()
                .build()
                .unwrap();

            tokio::task::LocalSet::new().block_on(&mut rt, async move {
                let sender = Rc::new(sender);

                // 每个单独的回调任务都可以参照这个task.
                let a = tokio::task::spawn_local({
                    let sender = sender.clone();
                    async move {
                        loop {
                            if shutdown.load(Ordering::SeqCst) == true {
                                return;
                            }

                            // 每500ms发送一个tick data;
                            tokio::time::delay_for(Duration::from_millis(500)).await;

                            // 在这里修改tick data数据;

                            sender.send_all(TickData {
                                symbol: Default::default(),
                                exchange: None,
                                datetime: None,
                                name: None,
                                volume: 0.0,
                                open_interest: 0.0,
                                last_price: 0.0,
                                last_volume: 0.0,
                                limit_up: 0.0,
                                limit_down: 0.0,
                                open_price: 0.0,
                                high_price: 0.0,
                                low_price: 0.0,
                                pre_close: 0.0,
                                bid_price: [0.0, 0.0, 0.0, 0.0, 0.0],
                                ask_price: [0.0, 0.0, 0.0, 0.0, 0.0],
                                bid_volume: [0.0, 0.0, 0.0, 0.0, 0.0],
                                ask_volume: [0.0, 0.0, 0.0, 0.0, 0.0],
                                instant: Instant::now(),
                            });
                        }
                    }
                });

                a.await.unwrap();
            });
        });

        self.handle = Some(handle);
    }
}
