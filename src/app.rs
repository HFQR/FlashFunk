#![allow(dead_code, unused_variables)]

use actix::{Actor, Handler, Context};
use super::interface::Interface;
use crate::constants::{OrderType, Direction};
use crate::structs::{OrderData, PositionData, TradeData, AccountData, ContractData};
use crate::account::Account;

/// ctpbee核心运行器
/// 作为该运行器
pub struct CtpbeeR {
    name: String,
    md: Box<dyn Interface>,
    td: Box<dyn Interface>,
    acc: Account,
}

impl Actor for CtpbeeR {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        println!("Ctpbee started, Application: {:?}", self.name);
    }

    fn stopped(&mut self, ctx: &mut Self::Context) {
        self.md.exit();
        self.td.exit();
        println!("Task stopped, Exit successful ")
    }
}

impl CtpbeeR {
    pub fn new() -> CtpbeeR {
        unimplemented!()
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