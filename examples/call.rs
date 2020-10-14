#![allow(dead_code, unused_imports, unused_must_use, unused_variables)]

use std::fmt::Pointer;
use std::thread;

use chrono::Local;
use flashfunk::constants::{Direction, Exchange, Offset, OrderType};
use flashfunk::ctp::md_api::MdApi;
use flashfunk::ctp::td_api::TdApi;
use flashfunk::interface::Interface;
use flashfunk::prelude::*;
use flashfunk::structs::{CancelRequest, Generator, LoginForm, OrderData, OrderRequest, TickData};
use flashfunk::types::message::TdApiMessage;
use flashfunk_codegen::Strategy;

/// 價格
struct Quote {
    ask: f64,
    bid: f64,
    bid_volume: f64,
    ask_volume: f64,
    thread: f64,
    last_price: f64,
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
        }
    }
    /// 計算盤口信息
    pub fn update_tick(&mut self, tick: &TickData) {
        if tick.ask_price(0) != self.ask && tick.bid_price(0) != self.bid {
            self.bid = tick.bid_price(0);
            self.ask = tick.ask_price(0);
            self.last_price = tick.last_price;
        };
    }
}

#[derive(Strategy)]
#[name("阿呆")]
#[symbol("ag2012")]
struct Strategy {
    quote: Quote,
    gap: u128,
    dup: u128,
}

impl Ac for Strategy {
    fn on_tick(&mut self, tick: &TickData, ctx: &mut Context) {
        // let req = OrderRequest {
        //     symbol: "ag2012".to_string(),
        //     exchange: Exchange::SHFE,
        //     direction: Direction::LONG,
        //     order_type: OrderType::LIMIT,
        //     volume: 1.0,
        //     price: tick.last_price + 1.0,
        //     offset: Offset::OPEN,
        //     reference: None,
        // };
        //
        // let orders = ctx.get_active_orders();
        // let mut is_send_long = false;
        // let mut is_send_short = false;
        // for x in orders {
        //     if x.direction.as_ref().unwrap() == &Direction::LONG {
        //         is_send_long = true;
        //     } else {
        //         is_send_short = true
        //     }
        // }
        //
        let is_send_long = true;
        let is_send_short = true;
        ctx.enter(|x, v| {
            let pos = v.position_mut("ag2012");
            if pos.short_volume != 0.0 && is_send_long == false {
                let req = OrderRequest {
                    symbol: "ag2012".to_string(),
                    exchange: Exchange::SHFE,
                    direction: Direction::LONG,
                    order_type: OrderType::LIMIT,
                    volume: pos.short_volume.clone(),
                    price: tick.last_price + 5.0,
                    offset: Offset::CLOSETODAY,
                    reference: None,
                };
                x.send(req);
            }

            if pos.long_volume != 0.0 && is_send_short == false {
                let req = OrderRequest {
                    symbol: "ag2012".to_string(),
                    exchange: Exchange::SHFE,
                    direction: Direction::SHORT,
                    order_type: OrderType::LIMIT,
                    volume: pos.long_volume.clone(),
                    price: tick.last_price - 3.0,
                    offset: Offset::CLOSETODAY,
                    reference: None,
                };
                x.send(req);
            }
        });
        // self.quote.update_tick(tick);
        //
        // let acc = ctx.get_account();
        // println!("{:?}", acc);
        self.gap += 1;
        if self.gap > 10 {
            // self.dup += tick.instant.elapsed().as_nanos();
            // if self.gap % 10 == 0 {
            //     println!("Single transmission delay： {}， gap:{}", self.dup / (self.gap - 10), self.gap - 10);
            // }
            println!("dur: {}", tick.instant.elapsed().as_nanos());
        }
    }

    fn on_order(&mut self, order: &OrderData, ctx: &mut Context) {
        println!("Order 回報: {:?}", order);
    }
}

fn main() {
    let login_form = LoginForm::new()
        .user_id("089131")
        .password("350888")
        .broke_id("9999")
        .app_id("simnow_client_test")
        .md_address("tcp://180.168.146.187:10131")
        .td_address("tcp://180.168.146.187:10130")
        .auth_code("0000000000000000")
        .production_info("");
    let strategy_1 = Strategy {
        quote: Quote::new(),
        gap: 0,
        dup: 0,
    };
    CtpbeeR::builder::<MdApi, TdApi, _>("ctpbee")
        .strategies(vec![strategy_1.into_str()])
        .id("name")
        .pwd("id")
        .path("bug")
        .login_form(login_form)
        .start();
}
