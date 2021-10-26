## FlashFunk

> High Frequency Trading Framework

## Why choose `Rust`

fast and safe 

## Support API 
- [x] ctp 上期所标准API  
- [x] REM 盛立纯socket对接 -> 未开源  

## Todo List

- Test Coverage

- Logger Need

- fix waring and remove clippy error

### Install

See [Install](./install.md)

### 系统要求

- stable-x86_64-unknown-linux-gnu - rustc 1.56.0 (09c42c458 2021-10-18) and later

- linux

- windows visual studio and llvm

### Easy Start

```rust
#![allow(dead_code, unused_imports, unused_must_use, unused_variables)]

use std::fmt::Pointer;
use std::thread;

use chrono::{Local, NaiveDateTime};
use flashfunk_level::CtpMdApi;
use flashfunk_level::CtpTdApi;
use flashfunk_core::prelude::*;

struct Strategy {
    local_symbol: Vec<String>,
}

impl Ac for Strategy {
    // tick 信息回报 
    fn on_tick(&mut self, tick: &TickData, ctx: &mut Context) { 
        println!("price: {}", tick.last_price);
    }
    fn codes(&mut self) -> Vec<String> { // 订阅的代码 
        self.local_symbol.iter().clone()
    }
    fn flash_name(&mut self) -> String {  // 名称
        "flashfunk".to_string()
    }
}

fn main() {
    let login_form = LoginForm::new()
        .user_id("089131")
        .password("350888")
        .broke_id("9999")
        .app_id("simnow_client_test")
        .md_address("tcp://218.202.237.33:10213")
        .td_address("tcp://218.202.237.33:10203")
        .auth_code("0000000000000000")
        .production_info("");
    let strategy_1 = Strategy {
        local_symbol: vec!["rb2110".to_string()]
    };
    Flash::builder::<CtpMdApi, CtpTdApi, _>("ctpbee")
        .strategies(vec![Box::new(strategy_1)])
        .id("flashfunk")
        .login_form(login_form)
        .start();
}

```










