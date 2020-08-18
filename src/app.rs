#![allow(dead_code, unused_variables)]

use actix::{Actor, Handler, Context, Addr, AsyncContext, Arbiter};
use super::interface::Interface;
use crate::constants::{OrderType, Direction};
use crate::structs::{OrderData, PositionData, TradeData, AccountData, ContractData};
use crate::account::Account;
use std::collections::HashMap;
use crate::ac::{Ac, BoxedAc};

/// ctpbee核心运行器
/// 作为该运行器
pub struct CtpbeeR {
    name: String,
    md: Option<Box<dyn Interface>>,
    td: Option<Box<dyn Interface>>,
    acc: Account,
    addr: Option<Addr<Self>>,
    login_info: HashMap<String, String>,
    strategies: Vec<Addr<BoxedAc>>,
}

impl Actor for CtpbeeR {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        println!("Ctpbee started, Application: {:?}", self.name);
        self.addr = Some(ctx.address());
    }

    fn stopped(&mut self, ctx: &mut Self::Context) {
        self.md.as_mut().unwrap().exit();
        self.td.as_mut().unwrap().exit();
        println!("Task stopped, Exit successful ")
    }
}

impl CtpbeeR {
    pub fn new(name: String) -> CtpbeeR {
        CtpbeeR {
            name,
            md: None,
            td: None,
            acc: Default::default(),
            addr: None,
            login_info: HashMap::new(),
            strategies: vec![],
        }
    }
    pub fn login(&mut self) -> bool {
        true
    }
    /// 增加策略, 注意会被call, 他是一个Actor，等待所有相关实现,
    pub fn add_strategy(&mut self, strategy: Box<dyn Ac + Send>) {
        let arbiter = Arbiter::new();
        let addr = BoxedAc::start_in_arbiter(&arbiter, |_| { BoxedAc(strategy) });
        self.strategies.push(addr);
    }
    /// 从HashMap载入登录信息
    pub fn load_setting(&mut self, map: HashMap<String, String>) {
        let mut dt = HashMap::new();
        for (key, value) in map {
            dt.insert(key, value);
        }
        self.login_info = dt;
    }

    /// 获取当前的持仓信息
    fn get_positions(&mut self, symbol: &str, direction: &Direction) -> Option<PositionData> {
        unimplemented!()
    }
    /// 获取所有的活跃报单
    fn get_active_orders(&mut self) -> Vec<OrderData> {
        unimplemented!()
    }
    /// 多开
    fn buy(&mut self, price: f64, volume: f64, price_type: OrderType) {
        unimplemented!()
    }
    /// 空开
    fn short(&mut self, price: f64, volume: f64, price_type: OrderType) {
        unimplemented!()
    }
    /// 平多头
    fn cover(&mut self, price: f64, volume: f64, price_type: OrderType) {
        unimplemented!()
    }
    /// 平空头
    fn sell(&mut self, price: f64, volume: f64, price_type: OrderType) {
        unimplemented!()
    }
    /// 撤单
    fn cancel(&mut self, order_id: &str) {
        unimplemented!()
    }
}

impl Handler<TradeData> for CtpbeeR {
    type Result = ();

    fn handle(&mut self, msg: TradeData, ctx: &mut Context<Self>) -> Self::Result {
        unimplemented!()
    }
}

impl Handler<OrderData> for CtpbeeR {
    type Result = ();

    fn handle(&mut self, msg: OrderData, ctx: &mut Context<Self>) -> Self::Result {
        unimplemented!()
    }
}


impl Handler<AccountData> for CtpbeeR {
    type Result = ();

    fn handle(&mut self, msg: AccountData, ctx: &mut Context<Self>) -> Self::Result {
        unimplemented!()
    }
}

impl Handler<PositionData> for CtpbeeR {
    type Result = ();

    fn handle(&mut self, msg: PositionData, ctx: &mut Context<Self>) -> Self::Result {
        unimplemented!()
    }
}

impl Handler<ContractData> for CtpbeeR {
    type Result = ();

    fn handle(&mut self, msg: ContractData, ctx: &mut Context<Self>) -> Self::Result {
        unimplemented!()
    }
}