use core::ops::{Deref, DerefMut};

use crate::app::StrategyMessage;
use crate::constants::{Direction, Exchange, Offset, OrderType, Status};
use crate::structs::{
    AccountData, BarData, ContractData, OrderData, PositionData, TickData, TradeData,
};
use std::collections::HashMap;

pub trait IntoStrategy: Sized + Send + Ac + 'static {
    fn into_str(self) -> __Strategy {
        __Strategy {
            str: Box::new(self),
            name: Self::name(),
            symbols: Self::local_symbol(),
        }
    }

    fn name() -> &'static str;

    fn local_symbol() -> Vec<&'static str>;
}

pub struct __Strategy {
    str: Box<dyn Ac + Send>,
    name: &'static str,
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

/// an easy order manager
#[derive(Default)]
pub struct OrderManager {
    map: HashMap<String, OrderData>,
    active_in: Vec<Status>,
}

impl OrderManager {
    pub fn new() -> OrderManager {
        OrderManager {
            map: Default::default(),
            active_in: vec![Status::NOTTRADED, Status::SUBMITTING, Status::PARTTRADED],
        }
    }

    pub fn add_order(&mut self, order: OrderData) {
        self.map.insert(order.orderid.clone().unwrap(), order);
    }
    /// 返回所有的活躍報單
    pub fn get_active_orders(&mut self) -> Vec<&OrderData> {
        self.map.iter().filter_map(|(id, v)|
            if self.active_in.contains(v.status.as_ref().unwrap()) { Some(v) } else { None }).collect()
    }
    pub fn get_order(&mut self, order_id: &str) -> Option<&OrderData> {
        self.map.get(order_id)
    }
    pub fn get_active_ids(&mut self) -> Vec<&str> {
        self.map.iter().filter_map(|(id, v)|
            if self.active_in.contains(v.status.as_ref().unwrap()) { Some(id.as_str()) } else { None }).collect()
    }
    pub fn get_ids(&mut self) -> Vec<&str> {
        self.map.iter().map(|x| { x.0.as_str() }).collect()
    }
}