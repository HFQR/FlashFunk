#![allow(dead_code, unused_imports, unused_must_use, unused_variables)]

use core::sync::atomic::{AtomicBool, Ordering};
use core::time::Duration;

use std::rc::Rc;
use std::sync::Arc;
use std::thread::JoinHandle;

use chrono::{Local, NaiveDateTime};
use flashfunk_core::interface::Interface;
use flashfunk_core::structs::{CancelRequest, Generator, LoginForm, OrderData, OrderRequest, TickData, TradeData};
use flashfunk_core::types::message::{TdApiMessage, MdApiMessage, StrategyMessage};
use flashfunk_core::prelude::*;
use flashfunk_core::ac::Ac;
use flashfunk_core::constants::{Status, Direction, Exchange, OrderType, Offset};
use flashfunk_core::{GroupSender,MockMdApi,MockTdApi};
use flashfunk_fetcher::{Tick, fetch_tick};
use flashfunk_codegen::Strategy;

/*
pub struct LocalMdApi {
    symbols: Vec<&'static str>,
    sender: Option<GroupSender<MdApiMessage>>,
    shutdown: Arc<AtomicBool>,
    handle: Option<JoinHandle<()>>,
}

impl Drop for LocalMdApi {
    fn drop(&mut self) {
        self.shutdown.store(true, Ordering::SeqCst);
        if let Some(handle) = self.handle.take() {
            handle.join().unwrap();
        }
    }
}

impl Interface for LocalMdApi {
    type Message = MdApiMessage;

    fn new(
        _: impl Into<Vec<u8>>,
        _: impl Into<Vec<u8>>,
        symbols: Vec<&'static str>,
        _: &LoginForm,
        group_sender: GroupSender<Self::Message>,
    ) -> Self {
        LocalMdApi {
            symbols,
            sender: Option::from(group_sender),
            shutdown: Arc::new(AtomicBool::new(false)),
            handle: None,
        }
    }

    fn subscribe(&mut self) {
        let mut ticks: Vec<Tick> = fetch_tick("rb2101.SHFE", "2020-11-05 09:00:00", "2020-11-05 11:00:00").unwrap();
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
                            let tick = TickData::from(&ticks.remove(0));
                            let msg: &'static TickData = Box::leak(Box::new(tick));
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
*/
#[derive(Strategy)]
#[name("阿呆")]
#[symbol("OI101")]
pub struct HelloFlash {


}


impl Ac for HelloFlash {
    fn on_tick(&mut self, tick: &TickData, ctx: &mut Context) {
        ctx.send(tick.clone());
        println!("{:?} -- ask:{} bid:{}", tick.symbol, tick.ask_price(0), tick.bid_price(0));
        let pos = ctx.get_position(&tick.symbol);
        println!("pos: -- long:{} short:{}",pos.long_volume, pos.short_volume);
        let req = OrderRequest {
            //symbol: "ni2102.SHFE".to_string(),
            symbol: tick.symbol.clone(),
            exchange: Exchange::SHFE,
            direction: Direction::LONG,
            order_type: OrderType::LIMIT,
            volume: 1.0,
            price: tick.ask_price(0),
            offset: Offset::OPEN,
            reference: None,
        };
        ctx.send(req);
        
    }

    fn on_order(&mut self, order: &OrderData, ctx: &mut Context) {
        println!("on order orderid:{} status:{:?}", order.orderid.as_ref().unwrap(), order.status);
    }

    fn on_trade(&mut self, trade: &TradeData, ctx: &mut Context) {
        println!("on trade tradeid:{} orderid:{}", trade.tradeid.as_ref().unwrap(), trade.orderid.as_ref().unwrap());
    }
}


fn main() {
    println!("{}",std::env::var("CLICK_HOUSE_URI").unwrap_or("tcp://127.0.0.1:9001/tick".parse().unwrap()));
    println!("{}",std::env::var("CONFIG_FILE").unwrap());
    let login_form = LoginForm::new()
        .user_id("170874")
        .password("wi1015..")
        .broke_id("9999")
        .app_id("simnow_client_test")
        .md_address("tcp://180.168.146.187:10131")
        .td_address("tcp://180.168.146.187:10130")
        .auth_code("0000000000000000")
        .production_info("");
    let hello = HelloFlash {
    };
    CtpbeeR::builder::<MockMdApi, MockTdApi, _>("ctpbee")
        .strategies(vec![hello.into_str()])
        .id("name")
        .pwd("id")
        .path("bug")
        .login_form(login_form)
        .start();
}