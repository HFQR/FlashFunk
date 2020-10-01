#![allow(dead_code, unused_imports, unused_must_use, unused_variables)]

use chrono::Local;
use ctpbee_rs::ac::Ac;
use ctpbee_rs::app::{CtpbeeR, LoginForm2, StrategyMessage};
use ctpbee_rs::constants::{Direction, Exchange, Offset, OrderType};
use ctpbee_rs::ctp::md_api::MdApi;
use ctpbee_rs::ctp::td_api::TdApi;
use ctpbee_rs::interface::Interface;
use ctpbee_rs::structs::{BarData, LoginForm, OrderData, OrderRequest, TickData};
use std::thread;

struct Strategy {
    pub name: String,
    pub price: Vec<f64>,
}

impl Ac for Strategy {
    type Symbol = Vec<&'static str>;

    fn local_symbol(&self) -> Self::Symbol {
        ["rb2010"].into()
    }

    fn on_bar(&mut self, bar: &BarData) {}

    fn on_tick(&mut self, tick: &TickData) -> Vec<Option<StrategyMessage>> {
        println!(
            "收到行情 symbol: {}, time: {:?} price: {}",
            tick.symbol,
            tick.datetime.unwrap(),
            tick.last_price
        );
        let req = if tick.last_price > 3520.0 {
            Some(StrategyMessage::OrderRequest(OrderRequest {
                symbol: "rb2010".to_string(),
                exchange: Exchange::SHFE,
                direction: Direction::SHORT,
                order_type: OrderType::LIMIT,
                volume: 1.0,
                price: tick.last_price,
                offset: Offset::OPEN,
                reference: None,
            }))
        } else {
            None
        };

        vec![req]
    }
}

struct Strategy2 {
    pub name: String,
    pub price: Vec<f64>,
}

impl Ac for Strategy2 {
    type Symbol = Vec<&'static str>;

    fn local_symbol(&self) -> Self::Symbol {
        ["rb2105"].into()
    }

    fn on_bar(&mut self, bar: &BarData) {}

    fn on_tick(&mut self, tick: &TickData) -> Vec<Option<StrategyMessage>> {
        println!(
            "收到行情 symbol: {}, time: {:?} price: {}",
            tick.symbol,
            tick.datetime.unwrap(),
            tick.last_price
        );
        let req = if tick.last_price > 3520.0 {
            Some(StrategyMessage::OrderRequest(OrderRequest {
                symbol: "rb2105".to_string(),
                exchange: Exchange::SHFE,
                direction: Direction::SHORT,
                order_type: OrderType::LIMIT,
                volume: 1.0,
                price: tick.last_price,
                offset: Offset::OPEN,
                reference: None,
            }))
        } else {
            None
        };

        vec![req]
    }
}

fn main() {
    let str = Strategy {
        name: "hello".to_string(),
        price: vec![],
    };
    let str2 = Strategy2 {
        name: "bug".to_string(),
        price: vec![],
    };

    let login_form = LoginForm2 {
        user_id: "170874",
        password: "wi1015..",
        broke_id: "9999",
        app_id: "simnow_client_test",
        md_address: "tcp://180.168.146.187:10131",
        td_address: "tcp://180.168.146.187:10130",
        auth_code: "0000000000000000",
        production_info: "",
        _lifetime: Default::default(),
    };

    CtpbeeR::new("ctpbee")
        .strategy(vec![Box::new(str), Box::new(str2)])
        .id("name")
        .pwd("id")
        .path("bug")
        .login_form(login_form)
        .start();
}
