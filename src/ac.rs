#![allow(dead_code, unused_variables)]

use crate::structs::{BarData, TickData, ContractData, PositionData, AccountData, TradeData, OrderData};
use actix::{Handler, Context, Message, Actor, Addr};
use crate::app::CtpbeeR;
use crate::constants::{OrderType, Direction};

pub trait Ac {
    fn on_bar(&mut self, bar: BarData);

    fn on_tick(&mut self, tick: TickData);

    fn init(&mut self, runtime: Addr<CtpbeeR>);

    fn get_addr(&mut self) -> &Addr<CtpbeeR>;

    fn on_contract(&mut self, contract: ContractData) {}

    fn on_position(&mut self, position: PositionData) {}

    fn on_trade(&mut self, trade: TradeData) {}

    fn on_order(&mut self, order: OrderData) {}

    fn on_account(&mut self, account: AccountData) {}

    fn on_realtime(&mut self) {}


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

#[derive(Message)]
#[rtype(result = "()")]
pub struct BoxedAc(
    pub Box<dyn Ac + Send>,
);


impl Actor for BoxedAc {
    type Context = Context<Self>;
}

impl Handler<BarData> for BoxedAc {
    type Result = ();

    fn handle(&mut self, msg: BarData, _: &mut Context<Self>) -> Self::Result {
        self.0.on_bar(msg);
    }
}


impl Handler<TickData> for BoxedAc {
    type Result = ();

    fn handle(&mut self, msg: TickData, ctx: &mut Context<Self>) -> Self::Result {
        self.0.on_tick(msg)
    }
}


impl Handler<ContractData> for BoxedAc {
    type Result = ();

    fn handle(&mut self, msg: ContractData, ctx: &mut Context<Self>) -> Self::Result {
        self.0.on_contract(msg)
    }
}

impl Handler<PositionData> for BoxedAc {
    type Result = ();

    fn handle(&mut self, msg: PositionData, ctx: &mut Context<Self>) -> Self::Result {
        self.0.on_position(msg)
    }
}


impl Handler<AccountData> for BoxedAc {
    type Result = ();

    fn handle(&mut self, msg: AccountData, ctx: &mut Context<Self>) -> Self::Result {
        self.0.on_account(msg)
    }
}

impl Handler<TradeData> for BoxedAc {
    type Result = ();

    fn handle(&mut self, msg: TradeData, ctx: &mut Context<Self>) -> Self::Result {
        self.0.on_trade(msg)
    }
}


impl Handler<OrderData> for BoxedAc {
    type Result = ();

    fn handle(&mut self, msg: OrderData, ctx: &mut Context<Self>) -> Self::Result {
        self.0.on_order(msg)
    }
}





