use core::ops::{Deref, DerefMut};

use crate::app::StrategyMessage;
use crate::constants::{Direction, Exchange, Offset, OrderType};
use crate::structs::{
    AccountData, BarData, ContractData, OrderData, PositionData, TickData, TradeData,
};

pub trait IntoStrategy: Sized + Send + Ac + 'static {
    fn into_str(self) -> __Strategy {
        __Strategy {
            str: Box::new(self),
            name: Self::name(),
            price: Self::price(),
            symbols: Self::local_symbol(),
        }
    }

    fn name() -> &'static str;

    fn price() -> Vec<f64>;

    fn local_symbol() -> Vec<&'static str>;
}

pub struct __Strategy {
    str: Box<dyn Ac + Send>,
    name: &'static str,
    price: Vec<f64>,
    symbols: Vec<&'static str>,
}

impl __Strategy {
    pub fn symbols(&self) -> &[&'static str] {
        &self.symbols
    }
}

impl Deref for __Strategy {
    type Target = Box<dyn Ac + Send>;

    fn deref(&self) -> &Self::Target {
        &self.str
    }
}

impl DerefMut for __Strategy {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.str
    }
}

#[allow(unused_variables)]
pub trait Ac {
    /// 订阅行情
    fn on_bar(&mut self, bar: &BarData);

    fn on_tick(&mut self, tick: &TickData) -> Vec<StrategyMessage>;

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
    /// 取消订阅
    fn unsubscribe(&mut self, symbol: &str) {
        unimplemented!("暂未实现此API ")
    }
    /// 撤单
    fn cancel(&mut self, order_id: &str) {
        unimplemented!();
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
        unimplemented!();
    }
}
