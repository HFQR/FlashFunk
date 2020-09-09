#![allow(dead_code, unused_imports, unused_must_use, unused_variables)]

use ctpbee_rs::ac::Ac;
use ctpbee_rs::app::{CtpbeeR, CtpbeeRMessage, TIMER, LoginForm2};
use ctpbee_rs::ctp::api::MdApi;
use ctpbee_rs::interface::Interface;
use ctpbee_rs::structs::{BarData, LoginForm, TickData};
use failure::_core::time::Duration;
use std::borrow::Borrow;
use std::thread;
use std::time::Instant;

struct Strategy {
    pub name: String,
}

impl Ac for Strategy {
    fn on_bar(&mut self, bar: &BarData) {
        let name = self.name.clone();
        println!("{} got bar", name);
    }

    fn on_tick(&mut self, tick: &TickData) {
        println!(
            "gap: {:?}",
            Instant::now()
                .duration_since(*TIMER.lock().unwrap())
                .as_nanos()
        );
    }
}

fn main() {
    let str = Strategy {
        name: "hello".to_string(),
    };

    let login_form = LoginForm2 {
        user_id: "089131",
        password: "350888",
        broke_id: "9999",
        app_id: "simnow_client_test",
        md_address: "tcp://180.168.146.187:10131",
        td_address: "tcp://218.202.237.33:10102",
        auth_code: "0000000000000000",
        production_info: "",
        _lifetime: Default::default()
    };

    CtpbeeR::new("ctpbee")
        .strategy(vec![str])
        .id("name")
        .pwd("id")
        .path("bug")
        .login_form(login_form)
        .subscribe("rb2101")
        .start();
}
