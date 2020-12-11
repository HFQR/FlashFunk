## FlashFunk 

> High Frequency Trading Framework


## 为什么选择`Rust`

速度快 安全 写的爽

## 任务队列 

- fix backtest and add test.

- 将`ctp`里面的代码替换到`flashfunk-interface`

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
use chrono::{Local, NaiveDateTime, Timelike};

use flashfunk_core::constants::{Direction, Offset, Exchange, OrderType, Status};
use flashfunk_core::interface::Interface;
use flashfunk_core::structs::{CancelRequest, Generator, LoginForm, OrderData, OrderRequest, TickData};
use flashfunk_core::prelude::{Ac, IntoStrategy, Context, ContextTrait, Flash};
use flashfunk_core::ctp::md_api::MdApi;
use flashfunk_core::ctp::td_api::TdApi;
use flashfunk_codegen::Strategy;

#[derive(Strategy)]
#[name("flash_ni")]
#[symbol("ni2102")]
pub struct Collector{
    ticks: Vec<TickData>
}

impl Ac for Collector {
    fn on_tick(&mut self, tick:&TickData, ctx: &mut Context){
        println!("{} {}", tick.datetime, tick.last_price);    
    }   
    
    fn on_order(&mut self, order: &OrderData, ctx: &mut Context){
        // get your order back infomation   
    }   

}

fn main() {
    // build a strategy instance 
    let tor = Collector {
            ticks: vec![]
    };
    // get the login infomation
    let login_form = LoginForm::new()
        .user_id("089131")
        .password("350888")
        .broke_id("9999")
        .app_id("simnow_client_test")
        .md_address("tcp://180.168.146.187:10131")   // simnow24小时环境
        .td_address("tcp://180.168.146.187:10130")
        // .md_address("tcp://218.202.237.33:10112")  // simnow移动正常环境
        // .td_address("tcp://218.202.237.33:10102")
        .auth_code("0000000000000000")
        .production_info("");
    
    // build running environment to run strategy and start
    Flash::builder::<MdApi, TdApi, _>("flash")
        .strategies(vec![tor.into_str()])
        .id("flash")
        .login_form(login_form)
        .start();
}


```










