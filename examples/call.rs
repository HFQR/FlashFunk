#![allow(dead_code, unused_imports, unused_must_use, unused_variables)]

use chrono::Local;
use ctpbee_rs::ac::{Ac, IntoStrategy};
use ctpbee_rs::app::{CtpbeeR, StrategyMessage};
use ctpbee_rs::constants::{Direction, Exchange, Offset, OrderType};
use ctpbee_rs::ctp::md_api::MdApi;
use ctpbee_rs::ctp::td_api::TdApi;
use ctpbee_rs::interface::Interface;
use ctpbee_rs::structs::{BarData, LoginForm, OrderData, OrderRequest, TickData};
use std::thread;

struct Strategy;

impl IntoStrategy for Strategy {
    fn name() -> &'static str {
        "abc"
    }

    fn price() -> Vec<f64> {
        vec![100.0]
    }

    fn local_symbol() -> Vec<&'static str> {
        vec!["rb2101"]
    }
}

impl Ac for Strategy {
    fn on_bar(&mut self, bar: &BarData) {}

    fn on_tick(&mut self, tick: &TickData) -> Vec<StrategyMessage> {
        println!(
            "策略1收到行情 symbol: {}, time: {:?} price: {}",
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

            res.push(req.into());
        }

        res
    }
}

struct Strategy2;

impl IntoStrategy for Strategy2 {
    fn name() -> &'static str {
        "abc2"
    }

    fn price() -> Vec<f64> {
        vec![100.0]
    }

    fn local_symbol() -> Vec<&'static str> {
        vec!["rb2105"]
    }
}

impl Ac for Strategy2 {
    fn on_bar(&mut self, bar: &BarData) {}

    fn on_tick(&mut self, tick: &TickData) -> Vec<StrategyMessage> {
        println!(
            "策略2收到行情 symbol: {}, time: {:?} price: {}",
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

    CtpbeeR::new("ctpbee")
        .strategies(vec![Strategy.into_str(), Strategy2.into_str()])
        .id("name")
        .pwd("id")
        .path("bug")
        .login_form(login_form)
        .start();
}
