#![allow(dead_code, unused_imports, unused_must_use, unused_variables)]

use chrono::Local;
use flashfunk::ac::OrderManager;
use flashfunk::app::{CtpbeeR, StrategyMessage};
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
}

impl Quote{
    /// 計算盤口信息
    pub fn update_tick(&mut self){

    }

}

#[derive(Strategy)]
#[name("阿呆")]
#[symbol("rb2101")]
struct Strategy {
    order_manager: OrderManager,
    is_send: bool,
}

impl Ac for Strategy {
    fn on_tick(&mut self, tick: &TickData) -> Vec<StrategyMessage> {
        let mut res = Vec::new();
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
        res.push(req.into());
        for order in self.order_manager.get_active_orders() {
            res.push(
                CancelRequest {
                    sysid: order.sysid.as_ref().unwrap().to_string(),
                    exchange: Exchange::SHFE,
                    symbol: order.symbol.to_string(),
                }
                .into(),
            );
        }
        res
    }

    fn on_order(&mut self, order: &OrderData) {
        self.order_manager.add_order(order.clone()

        );
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
        order_manager: OrderManager::new(),
        is_send: false,
    };
    CtpbeeR::new("ctpbee")
        .strategies(vec![strategy_1.into_str()])
        .id("name")
        .pwd("id")
        .path("bug")
        .login_form(login_form)
        .start();
}
