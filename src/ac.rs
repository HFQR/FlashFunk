#![allow(dead_code, unused_variables)]

use crate::app::{CtpbeeR, CtpbeeRMessage};
use crate::constants::{Direction, Exchange, Offset, OrderType};
use crate::structs::{
    AccountData, BarData, CancelRequest, ContractData, OrderData, OrderRequest, PositionData,
    SubscribeRequest, TickData, TradeData,
};
use actix::{Actor, Context, Handler, Message};

pub trait Ac {
    fn on_bar(&mut self, bar: &BarData);

    fn on_tick(&mut self, tick: &TickData);

    fn on_contract(&mut self, contract: &ContractData) {}

    fn on_position(&mut self, position: &PositionData) {}

    fn on_trade(&mut self, trade: &TradeData) {}

    fn on_order(&mut self, order: &OrderData) {}

    fn on_account(&mut self, account: &AccountData) {}

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
    /// 订阅行情
    fn subscribe(&mut self, symbol: &str) {
        let req = SubscribeRequest {
            symbol: symbol.to_string(),
        };
    }
    /// 取消订阅
    fn unsubscribe(&mut self, symbol: &str) {
        unimplemented!("暂未实现此API ")
    }
    /// 撤单
    fn cancel(&mut self, order_id: &str) {
        let req = CancelRequest {
            orderid: "".to_string(),
        };
    }
    /// 发送底层报单
    fn send_order(
        &mut self,
        symbol: &str,
        price: f64,
        volume: f64,
        exchange: Exchange,
        price_type: OrderType,
        offset: Offset,
        direction: Direction,
    ) {
        let order = OrderRequest {
            symbol: symbol.to_string(),
            exchange,
            direction,
            order_type: price_type,
            volume,
            price,
            offset,
            reference: None,
        };
    }
}

pub trait Ac2 {
    type Account;

    fn on_bar(&mut self, bar: &BarData, acc: &mut Self::Account);

    fn on_tick(&mut self, tick: &TickData, acc: &mut Self::Account);

    fn on_contract(&mut self, contract: &ContractData, acc: &mut Self::Account) {}

    fn on_position(&mut self, position: &PositionData, acc: &mut Self::Account) {}

    fn on_trade(&mut self, trade: &TradeData, acc: &mut Self::Account) {}

    fn on_order(&mut self, order: &OrderData, acc: &mut Self::Account) {}

    fn on_account(&mut self, account: &AccountData) {}

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
    /// 订阅行情
    fn subscribe(&mut self, symbol: &str) {
        let req = SubscribeRequest {
            symbol: symbol.to_string(),
        };
    }
    /// 取消订阅
    fn unsubscribe(&mut self, symbol: &str) {
        unimplemented!("暂未实现此API ")
    }
    /// 撤单
    fn cancel(&mut self, order_id: &str) {
        let req = CancelRequest {
            orderid: "".to_string(),
        };
    }
    /// 发送底层报单
    fn send_order(
        &mut self,
        symbol: &str,
        price: f64,
        volume: f64,
        exchange: Exchange,
        price_type: OrderType,
        offset: Offset,
        direction: Direction,
    ) {
        let order = OrderRequest {
            symbol: symbol.to_string(),
            exchange,
            direction,
            order_type: price_type,
            volume,
            price,
            offset,
            reference: None,
        };
    }
}