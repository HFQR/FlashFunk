#![allow(dead_code, unused_imports, unused_must_use, unused_variables)]

use std::fmt::Pointer;
use std::thread;

use chrono::Local;
use flashfunk::constants::{Direction, Exchange, Offset, OrderType};
use flashfunk::ctp::md_api::MdApi;
use flashfunk::ctp::td_api::TdApi;
use flashfunk::interface::Interface;
use flashfunk::prelude::*;
use flashfunk::structs::{CancelRequest, LoginForm, OrderData, OrderRequest, TickData};
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
#[symbol("ag2101")]
struct Strategy {
    quote: Quote,
}

impl Ac for Strategy {
    fn on_tick(&mut self, tick: &TickData, ctx: &mut Context) {
        let req = OrderRequest {
            symbol: "ag2101".to_string(),
            exchange: Exchange::SHFE,
            direction: Direction::LONG,
            order_type: OrderType::LIMIT,
            volume: 1.0,
            price: tick.last_price + 1.0,
            offset: Offset::OPEN,
            reference: None,
        };
        // println!(
        //     "行情 : {} 買一:{}  賣一:{} 買一量: {} 賣一量:{}",
        //     tick.last_price,
        //     tick.bid_price(0),
        //     tick.ask_price(0),
        //     tick.bid_volume(0),
        //     tick.ask_volume(0)
        // );
        // print the active order's id
        // println!("{:?}", ctx.get_active_ids().collect::<Vec<_>>());

        // get the pos infomation
        let pos = ctx.get_position("ag2101");

        println!("{:?}", pos);

        // send  a close position request
        if pos.long_volume != 0.0 {
            let req = OrderRequest {
                symbol: "ag2101".to_string(),
                exchange: Exchange::SHFE,
                direction: Direction::SHORT,
                order_type: OrderType::LIMIT,
                volume: pos.long_volume.clone(),
                price: tick.last_price - 2.0,
                offset: Offset::CLOSETODAY,
                reference: None,
            };
            // ctx.send(req);
        }
        self.quote.update_tick(tick);

        let acc = ctx.get_account();
        // println!("{:?}", acc);

        // send order reuqest right now
        // ctx.send(req);

        // // 当我们需要同时引用上下文的不同状态时，我们可以使用Context::enter方法
        // ctx.enter(|sender, ctx| {
        //     ctx.get_active_orders().for_each(|f| {
        //         let order = CancelRequest {
        //             order_id: f.orderid.clone().unwrap(),
        //             exchange: Exchange::SHFE,
        //             symbol: f.symbol.to_string(),
        //         };
        //         sender.send(order);
        //     });
        // });
    }

    fn on_order(&mut self, order: &OrderData, ctx: &mut Context) {
        // println!("{:?}", order);
    }
}

fn main() {
    let login_form = LoginForm::new()
        .user_id("170874")
        .password("wi1015..")
        .broke_id("9999")
        .app_id("simnow_client_test")
        .md_address("tcp://218.202.237.33:10112")
        .td_address("tcp://218.202.237.33:10102")
        .auth_code("0000000000000000")
        .production_info("");
    let strategy_1 = Strategy {
        quote: Quote::new(),
    };
    CtpbeeR::builder("ctpbee")
        .strategies(vec![strategy_1.into_str()])
        .id("name")
        .pwd("id")
        .path("bug")
        .login_form(login_form)
        .start();
}
