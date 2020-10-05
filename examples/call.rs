#![allow(dead_code, unused_imports, unused_must_use, unused_variables)]

use chrono::Local;
use flashfunk::app::{CtpbeeR, StrategyMessage, StrategyWorkerContext};
use flashfunk::constants::{Direction, Exchange, Offset, OrderType};
use flashfunk::ctp::md_api::MdApi;
use flashfunk::ctp::td_api::TdApi;
use flashfunk::interface::Interface;
use flashfunk::structs::{CancelRequest, LoginForm, OrderData, OrderRequest, TickData};
use flashfunk::{Ac, IntoStrategy};
use flashfunk_codegen::Strategy;
use std::fmt::Pointer;
use std::thread;

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
            println!("price changed at {}", tick.last_price - self.last_price);
            self.bid = tick.bid_price(0);
            self.ask = tick.ask_price(0);
            self.last_price = tick.last_price;
        };
    }
}

#[derive(Strategy)]
#[name("阿呆")]
#[symbol("rb2101")]
struct Strategy {
    quote: Quote,
}

impl Ac for Strategy {
    fn on_tick(&mut self, tick: &TickData, ctx: &mut StrategyWorkerContext) {
        let req = OrderRequest {
            symbol: "rb2101".to_string(),
            exchange: Exchange::SHFE,
            direction: Direction::LONG,
            order_type: OrderType::LIMIT,
            volume: 1.0,
            price: tick.last_price - 20.0,
            offset: Offset::OPEN,
            reference: None,
        };
        println!(
            "行情 : {} 買一:{}  賣一:{} 買一量: {} 賣一量:{}",
            tick.last_price,
            tick.bid_price(0),
            tick.ask_price(0),
            tick.bid_volume(0),
            tick.ask_volume(0)
        );

        self.quote.update_tick(tick);
        // ctx.send(req.into());
        let cancel_reqs: Vec<StrategyMessage> = ctx.get_active_orders().iter().map(|f| {
            CancelRequest {
                order_id: f.orderid.clone().unwrap(),
                exchange: Exchange::SHFE,
                symbol: f.symbol.to_string(),
            }.into()
        }).collect();

        for order in cancel_reqs {
            ctx.send(order);
        }
        println!("{:?}", ctx.get_active_ids());
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
    CtpbeeR::new("ctpbee")
        .strategies(vec![strategy_1.into_str()])
        .id("name")
        .pwd("id")
        .path("bug")
        .login_form(login_form)
        .start();
}
