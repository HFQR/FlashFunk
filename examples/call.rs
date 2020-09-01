#![allow(dead_code, unused_imports, unused_must_use, unused_variables)]

use ctpbee_rs::app::CtpbeeR;
use ctpbee_rs::ac::Ac;
use ctpbee_rs::structs::{BarData, TickData};
use ctpbee_rs::ctp::api::MdApi;
use std::thread;
use actix::Addr;
use std::borrow::Borrow;

struct Strategy {
    pub name: String,
    pub addr: Option<Addr<CtpbeeR>>,
}

impl Ac for Strategy {
    fn on_bar(&mut self, bar: BarData) {
        let name = self.name.clone();
        println!("{} got bar", name);
    }

    fn on_tick(&mut self, tick: TickData) {
        let name = self.name.clone();
        println!("{} got tick {:?}", name, self.get_addr());
    }

    fn init(&mut self, runtime: Addr<CtpbeeR>) {
        self.addr = Some(runtime);
    }

    fn get_addr(&mut self) -> &Addr<CtpbeeR> {
        self.addr.as_ref().unwrap()
    }
}

#[actix_rt::main]
async fn main() {
    // create main actor
    let mut account = CtpbeeR::new("ctpbee".to_string());
    let str = Strategy { name: "hello".to_string(), addr: None };
    let str2 = Strategy { name: "bug".to_string(), addr: None };
    // ADD strategy to main Actor
    account.add_strategy(Box::new(str));
    account.add_strategy(Box::new(str2));
    let (addr, x) = account.run_forever();
    let copy = addr.clone();
    // here is the call c++ code
    let mut md_api = MdApi::new("name".to_string(), "id".to_string(), "bug".to_string());
    md_api.init();
    let trading_day = md_api.get_trading_day();
    println!("trading day:{} ", trading_day);
    // wait
    x.await;
}