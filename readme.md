## FlashFunk 

> High Frequency Trading Framework


## 为什么选择`Rust`

速度快 安全 写的爽

## 任务队列 

- fix backtest and add test.

- 測試覆盖

- 日志

- 移除所有警告以及修复clippy错误

### 接口编译  
本项目最终采用`rust-indgen` 作为`c++`接口封装

### 如何安装
 
参见 [Install](./install.md)

### 系统要求

- linux 提供了安装教程 

- windows 需要安装llvm环境, 自行折腾 ~~ 


### 简单上手
无须你处理任何事情 专注写策略即可  我们已经为你实现绝大部分功能, 唯一需要你关注的是如何写出赚钱的策略

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
    fn on_tick(&mut self, tick: &TickData, ctx: &mut Context) {
        println!("dur: {}", tick.instant.elapsed().as_nanos());
    }

    // 此处我们需要返还静态的引用 推荐使用此函数即可 
    fn local_symbols<'a>(&mut self) -> Vec<&'a str> { 
        let mut strs: Vec<&'static str> = Vec::new();
        self.local_symbol.iter().for_each(|x| strs.push(Box::leak(x.clone().into_boxed_str())));
        strs
    }

    fn name<'a>(&mut self) -> &'a str { "oi" }
}

fn main() {
    let login_form = LoginForm::new()
        .user_id("170874")
        .password("wi1015..")
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










