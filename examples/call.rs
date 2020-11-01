#![allow(dead_code, unused_imports, unused_must_use, unused_variables)]

use std::fmt::Pointer;
use std::thread;

use chrono::{Local, NaiveDateTime};
use flashfunk::constants::{Direction, Exchange, Offset, OrderType};
use flashfunk::ctp::md_api::MdApi;
use flashfunk::ctp::td_api::TdApi;
use flashfunk::interface::Interface;
use flashfunk::prelude::*;
use flashfunk::structs::{CancelRequest, Generator, LoginForm, OrderData, OrderRequest, TickData};
use flashfunk::types::message::TdApiMessage;
use flashfunk::MockMdApi;
use flashfunk_codegen::Strategy;

/// 價格
/// Now we build a order book and calculate the  trend power
struct Quote {
    ask: f64,
    bid: f64,
    bid_volume: f64,
    ask_volume: f64,
    thread: f64,
    last_price: f64,
    value: f64,
}

impl Quote {
    pub fn new() -> Quote {
        Quote {
            ask: 0.0,
            bid: 0.0,
            bid_volume: 0.0,
            ask_volume: 0.0,
            thread: 0.0,
            last_price: 0.0,
            value: 0.0,
        }
    }
    /// 計算盤口信息
    pub fn update_tick(&mut self, tick: &TickData) -> f64 {
        let ask_1 = tick.ask_price(0);
        let bid_1 = tick.bid_price(0);
        let ask_vol_1 = tick.ask_volume(0);
        let bid_vol_1 = tick.bid_volume(0);
        // when price is down just, increase\+
        // let value  = if ask_1 > self.ask {
                // price u


        self.value = self.value;
        self.value
    }
}

#[derive(Strategy)]
#[name("阿呆")]
#[symbol("OI101")]
struct Strategy {
    quote: Quote,
}

impl Ac for Strategy {
    fn on_tick(&mut self, tick: &TickData, ctx: &mut Context) {
        let is_send_long = true;
        let is_send_short = true;
        ctx.enter(|x, v| {
            let pos = v.position_mut("OI101");
            if pos.short_volume != 0.0 && is_send_long == false {
                let req = OrderRequest {
                    symbol: "OI101".to_string(),
                    exchange: Exchange::CZCE,
                    direction: Direction::LONG,
                    order_type: OrderType::LIMIT,
                    volume: pos.short_volume,
                    price: tick.last_price + 5.0,
                    offset: Offset::CLOSETODAY,
                    reference: None,
                };
                x.send(req);
            }

            if pos.long_volume != 0.0 && is_send_short == false {
                let req = OrderRequest {
                    symbol: "OI101".to_string(),
                    exchange: Exchange::CZCE,
                    direction: Direction::SHORT,
                    order_type: OrderType::LIMIT,
                    volume: pos.long_volume,
                    price: tick.last_price - 3.0,
                    offset: Offset::CLOSETODAY,
                    reference: None,
                };
                x.send(req);
            }
        });
        self.quote.update_tick(tick);
        println!("dur: {}", tick.instant.elapsed().as_nanos());
    }

    fn on_order(&mut self, order: &OrderData, ctx: &mut Context) {
        println!("Order 回報: {:?}", order);
    }
}

fn main() {
    let login_form = LoginForm::new()
        .user_id("170874")
        .password("wi1015..")
        .broke_id("9999")
        .app_id("simnow_client_test")
        .md_address("tcp://180.168.146.187:10131")
        .td_address("tcp://180.168.146.187:10130")
        .auth_code("0000000000000000")
        .production_info("");
    let strategy_1 = Strategy {
        quote: Quote::new(),
    };
    CtpbeeR::builder::<MdApi, TdApi, _>("ctpbee")
        .strategies(vec![strategy_1.into_str()])
        .id("name")
        .pwd("id")
        .path("bug")
        .login_form(login_form)
        .start();
}
