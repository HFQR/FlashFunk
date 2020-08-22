#![allow(dead_code, unused_imports, unused_variables)]

use actix::prelude::*;
use super::interface::Interface;
use crate::constants::{OrderType, Direction};
use crate::structs::{OrderData, PositionData, TradeData, AccountData, ContractData, TickData, BarData, OrderRequest, CancelRequest, SubscribeRequest};
use crate::account::Account;
use std::collections::HashMap;
use crate::ac::{Ac, BoxedAc};
use std::cell::RefCell;
use std::rc::Rc;
use futures::SinkExt;
use std::mem::swap;
use std::fmt::Debug;
use core::fmt;

/// ctpbee核心运行器
/// 作为该运行器
pub struct CtpbeeR {
    name: String,
    md: Option<Box<dyn Interface>>,
    td: Option<Box<dyn Interface>>,
    acc: Rc<RefCell<Account>>,
    addr: Option<Addr<Self>>,
    login_info: HashMap<String, String>,
    strategy_addrs: Vec<Addr<BoxedAc>>,
    str: Vec<BoxedAc>,
    sender: Option<futures::channel::oneshot::Sender<()>>,
    receiver: Option<futures::channel::oneshot::Receiver<()>>,
}

impl Actor for CtpbeeR {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        println!(">>> Ctpbee started, Application: {:?}", self.name);
        self.addr = Some(ctx.address());
    }

    fn stopped(&mut self, ctx: &mut Self::Context) {
        self.md.as_mut().unwrap().exit();
        self.td.as_mut().unwrap().exit();
        let _ = self.sender.take().unwrap().send(());
        println!("Task stopped, Exit successful ")
    }
}

impl CtpbeeR {
    pub fn new(name: String) -> CtpbeeR {
        CtpbeeR {
            name,
            md: None,
            td: None,
            acc: Account::new(),
            addr: None,
            login_info: HashMap::new(),
            strategy_addrs: vec![],
            str: vec![],
            sender: None,
            receiver: None,
        }
    }
    pub fn login(&mut self) -> bool {
        true
    }

    /// 从HashMap载入登录信息
    pub fn load_setting(&mut self, map: HashMap<String, String>) {
        let mut dt = HashMap::new();
        for (key, value) in map {
            dt.insert(key, value);
        }
        self.login_info = dt;
    }
    /// 增加策略, 注意会被call, 他是一个Actor，等待所有相关实现,
    pub fn add_strategy(&mut self, strategy: Box<dyn Ac + Send>) {
        self.str.push(BoxedAc(strategy));
    }
    pub fn run_forever(mut self) -> (Addr<CtpbeeR>, futures::channel::oneshot::Receiver<()>) {
        let (tx, rx) = futures::channel::oneshot::channel::<()>();
        self.sender = Some(tx);
        let mut temp_acs = vec![];
        swap(&mut self.str, &mut temp_acs);
        let addr = self.start();
        temp_acs.into_iter().for_each(|x| addr.do_send(x));
        (addr, rx)
    }
}

impl Debug for CtpbeeR {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Ctpbee >>> : {}", self.name)
    }
}

impl Handler<TradeData> for CtpbeeR {
    type Result = ();

    fn handle(&mut self, msg: TradeData, ctx: &mut Context<Self>) -> Self::Result {
        for x in &self.strategy_addrs {
            x.do_send(msg.clone());
        }
    }
}

impl Handler<OrderData> for CtpbeeR {
    type Result = ();

    fn handle(&mut self, msg: OrderData, ctx: &mut Context<Self>) -> Self::Result {
        for x in &self.strategy_addrs {
            x.do_send(msg.clone());
        }
    }
}


impl Handler<TickData> for CtpbeeR {
    type Result = ();

    fn handle(&mut self, msg: TickData, ctx: &mut Context<Self>) -> Self::Result {
        for x in &self.strategy_addrs {
            x.do_send(msg.clone());
        }
    }
}


impl Handler<BarData> for CtpbeeR {
    type Result = ();

    fn handle(&mut self, msg: BarData, ctx: &mut Context<Self>) -> Self::Result {
        for x in &self.strategy_addrs {
            x.do_send(msg.clone());
        }
    }
}


impl Handler<AccountData> for CtpbeeR {
    type Result = ();

    fn handle(&mut self, msg: AccountData, ctx: &mut Context<Self>) -> Self::Result {
        for x in &self.strategy_addrs {
            x.do_send(msg.clone());
        }
    }
}

impl Handler<PositionData> for CtpbeeR {
    type Result = ();

    fn handle(&mut self, msg: PositionData, ctx: &mut Context<Self>) -> Self::Result {
        for x in &self.strategy_addrs {
            x.do_send(msg.clone());
        }
    }
}

impl Handler<ContractData> for CtpbeeR {
    type Result = ();

    fn handle(&mut self, msg: ContractData, ctx: &mut Context<Self>) -> Self::Result {
        for x in &self.strategy_addrs {
            x.do_send(msg.clone());
        }
    }
}

impl Handler<BoxedAc> for CtpbeeR {
    type Result = ();

    fn handle(&mut self, mut msg: BoxedAc, ctx: &mut Context<Self>) -> Self::Result {
        let arbiter = Arbiter::new();
        msg.0.init(ctx.address().clone());
        let addr = BoxedAc::start_in_arbiter(&arbiter, |_| { msg });
        self.strategy_addrs.push(addr);
    }
}

impl Handler<OrderRequest> for CtpbeeR {
    type Result = ();

    fn handle(&mut self, msg: OrderRequest, ctx: &mut Context<Self>) -> Self::Result {
        unimplemented!()
    }
}

impl Handler<CancelRequest> for CtpbeeR {
    type Result = ();

    fn handle(&mut self, msg: CancelRequest, ctx: &mut Context<Self>) -> Self::Result {
        unimplemented!()
    }
}

impl Handler<SubscribeRequest> for CtpbeeR {
    type Result = ();

    fn handle(&mut self, msg: SubscribeRequest, ctx: &mut Context<Self>) -> Self::Result {
        unimplemented!()
    }
}