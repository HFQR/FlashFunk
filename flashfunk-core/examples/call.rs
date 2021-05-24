#![allow(dead_code, unused_imports, unused_must_use, unused_variables)]

use std::fmt::Pointer;
use std::thread;

use chrono::{Local, NaiveDateTime};
use flashfunk_core::prelude::*;
use flashfunk_level::CtpMdApi;
use flashfunk_level::CtpTdApi;

struct Strategy {
    local_symbol: Vec<String>,
}

impl Ac for Strategy {
    fn on_tick(&mut self, tick: &TickData, ctx: &mut Context) {
        println!("dur: {}", tick.instant.elapsed().as_nanos());
    }

    fn codes(&mut self) -> Vec<String> {
        self.local_symbol.iter().map(|x| x.clone()).collect()
    }
    fn flash_name(&mut self) -> String {
        "oi".to_string()
    }
}

fn main() {
    let login_form = LoginForm::new()
        .user_id("170874")
        .password("wi1015..")
        .broke_id("9999")
        .app_id("simnow_client_test")
        .md_address("tcp://180.168.146.187:10212")
        .td_address("tcp://180.168.146.187:10202")
        .auth_code("0000000000000000")
        .production_info("");
    let strategy_1 = Strategy {
        local_symbol: vec!["rb2110".to_string()],
    };
    Flash::builder::<CtpMdApi, CtpTdApi, _>("ctpbee")
        .strategies(vec![Box::new(strategy_1)])
        .id("flashfunk")
        .login_form(login_form)
        .start();
}
