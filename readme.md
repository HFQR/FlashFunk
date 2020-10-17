## ctpbee-rs 

> High Frequency Trading Framework


### Quick Start 
```rust
#![allow(dead_code, unused_imports, unused_must_use, unused_variables)]

use ctpbee_rs::app::CtpbeeR;
use ctpbee_rs::ac::Ac;
use ctpbee_rs::structs::{BarData, TickData};
use std::thread;
use actix::Addr;
use std::borrow::Borrow;

struct Strategy {
    pub name: String,
    pub addr: Option<Addr<CtpbeeR>>,
}

impl Ac for Strategy {
    fn on_bar(&mut self, bar: BarData) {
        println!("got bar {:?}", self.get_addr());
    }

    fn on_tick(&mut self, tick: TickData) {
        println!("got tick {:?}", self.get_addr());
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
    let mut account = CtpbeeR::new("ctpbee".to_string());
    let str = Strategy { name: "hello".to_string(), addr: None };
    account.add_strategy(Box::new(str));
    let (addr, x) = account.run_forever();
    let copy = addr.clone();
    thread::spawn(move || {
        loop {
            addr.do_send(TickData::default());
        }
    });
    x.await;
}
```


## Document 


## 前置声明

This project are contributed by  `somewheve` and `fakeshadow` and `qzm`


###  Task 

|  task   | contributor  |  status  |
|  ----  | ----  | ----  |
| 账户基类计算         | `somewheve` | `finished` |
| 整体架构实现  | `fakeshadow `|  `finished`  |
| 策略模板实现         |  `somewheve`  | `finished`   |  
| ctp c++接口对接      | `somewheve`   |  `finished`   |
|  盛立 c++ 接口对接    |  `somewheve`  |    `not started`  |
|  `clickhouse`数据获取 |   `somewheve` | `not started`  | 
|  持仓管理    |  `qzm`          |    `not started`  |


### Interface compile 
本项目最终采用`bindgen` 作为`c++`接口封装 


```

### 一些详细工作 
- 如何进行编译 

修改`build.rs`里面main函数 `build`函数参数支持以下  

`ctp`

- how to print detail output 

cargo --vv build 

### Environment setup
In order to run `call` example, for example in linux 

you should export absolute path of `/home/somewheve/Documents/ctpbee-rs/sdk_sources/ctp/linux/`
likes, do not work on windows =_=
```
export LD_LIBRARY_PATH="/home/somewheve/Documents/ctpbee-rs/sdk_sources/ctp/linux/:$LD_LIBRARY_PATH"
```
`rusct-link-search-dylib` do not work , I don't know why
