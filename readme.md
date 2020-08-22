## ctpbee-rs 

> 专为高频交易设计的程序化交易框架


### 快速开始 
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


## 文档 


## 前置声明
此项目作为ctpbee前置闭源项目,代码所有权归`somewheve`所有，目前仅限 代码贡献成员所属公司内部使用, 
不允许以任何方式私自出售源码或者泄露. 尊重劳动成果 ^_^. 
未来可能进行商业化开源/开源 

###  项目任务分配 

|  task   | contributor  |  status  |
|  ----  | ----  | ----  |
| 账户基类计算         | `somewheve` | `finished` |
| 整体架构实现  | `somewheve `|  `finished`  |
| 策略模板实现         |  `somewheve`  | `pending`   |  
| ctp c++接口对接      | `somewheve`   |  `running`   |
|  盛立 c++ 接口对接    |  `somewheve`  |    `not started`  |
|  `clickhouse`数据获取 |   `somewheve` | `not started`  | 
|  持仓管理    |  `qzm`          |    `not started`  |
|  k线生成 |   `qzm`           |  `running`  |


### 接口编译 
本项目最终采用`bindgen` 作为`c++`接口封装 
请安装llvm以支持RUST  C++ bindgen


for windows 前往[地址](https://releases.llvm.org/download.html) 下载windows安装包即可, 注意要加入系统环境，其他系统亦可 
 
 > 注意windows此安装包仅仅提供 clang等编译程序 并不提供llvm-config程序，需要手动下载llvm源码进行编译
Here are the explanation 
```
llvm-config does not exist in windows prebuilt binaries. You need to compile from the source code to get it.

1.Grab CMAKE > 3.5 , install it and make sure you add it to PATH.
2.Download Visual Studio 2019
3.Donwload the source code (9.0.1 is the latest as I'm writing this)
4.Extract the source code
5.Cd into the root of the llvm source-code
6.In cmd, type cmake . this will generate Visual Studio 2019 sln.
7.open sln file(LLVM.sln), change the build type to Rlease, build the whole project
8.navigate to your Rlease\bin, and there you have your llvm-config.exe
```

### 一些详细工作 
- 如何进行编译 
修改`build.rs`里面main函数 `build`函数参数支持以下  

`ctp`

- 如何打印相信的编译输出 
使用cargo --vv build 



