#![allow(dead_code, unused_imports, unused_must_use, unused_variables)]

use chrono::Local;
use flashfunk::ac::OrderManager;
use flashfunk::app::{CtpbeeR, StrategyMessage};
use flashfunk::constants::{Direction, Exchange, Offset, OrderType};
use flashfunk::ctp::md_api::MdApi;
use flashfunk::ctp::td_api::TdApi;
use flashfunk::interface::Interface;
use flashfunk::structs::{BarData, LoginForm, OrderData, OrderRequest, TickData};
use flashfunk::{Ac, IntoStrategy};
use flashfunk_codegen::Strategy;
use std::thread;

#[derive(Strategy)]
#[name("阿呆")]
#[symbol("r2101")]
struct Strategy {
    order_manager: OrderManager,
}

impl Ac for Strategy {
    fn on_bar(&mut self, bar: &BarData) {}

    fn on_tick(&mut self, tick: &TickData) -> Vec<StrategyMessage> {
        println!(
            "策略{}收到行情 symbol: {}, time: {:?} price: {}",
            Self::name(),
            tick.symbol,
            tick.datetime.unwrap(),
            tick.last_price
        );

        let mut res = Vec::new();

        if tick.last_price > 3520.0 {
            let req = OrderRequest {
                symbol: "rb2101".to_string(),
                exchange: Exchange::SHFE,
                direction: Direction::LONG,
                order_type: OrderType::LIMIT,
                volume: 1.0,
                price: tick.last_price + 10.0,
                offset: Offset::OPEN,
                reference: None,
            };

            // res.push(req.into());
        }

        res
    }
}

#[derive(Strategy)]
#[name("阿瓜")]
#[symbol("r2105")]
struct Strategy2;

impl Ac for Strategy2 {
    fn on_bar(&mut self, bar: &BarData) {}

    fn on_tick(&mut self, tick: &TickData) -> Vec<StrategyMessage> {
        println!(
            "策略{}收到行情 symbol: {}, time: {:?} price: {}",
            Self::name(),
            tick.symbol,
            tick.datetime.unwrap(),
            tick.last_price
        );

        let mut res = Vec::new();
        if tick.last_price > 3520.0 {
            let req = OrderRequest {
                symbol: "rb2105".to_string(),
                exchange: Exchange::SHFE,
                direction: Direction::SHORT,
                order_type: OrderType::LIMIT,
                volume: 1.0,
                price: tick.last_price,
                offset: Offset::OPEN,
                reference: None,
            };

            res.push(req.into());
        };
        res
    }

    fn on_order(&mut self, order: &OrderData) {
        println!("{} - {}", order.symbol, order.price)
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
        order_manager: Default::default(),
    };

    CtpbeeR::new("ctpbee")
        .strategies(vec![strategy_1.into_str(), Strategy2.into_str()])
        .id("name")
        .pwd("id")
        .path("bug")
        .login_form(login_form)
        .start();
}
