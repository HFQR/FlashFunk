use std::collections::hash_map::RandomState;
use std::collections::HashMap;
use std::ops::{DerefMut, Deref};

use crate::structs::{AccountData, OrderData, Params, TickData, TradeData};
use std::borrow::Borrow;
use crate::constants::{Offset, Direction};

pub struct DailyResult {
    available: f64,
    balance: f64,
    fee: f64,
    margin: f64,
    date: String,
}

///
pub struct Account {
    name: f64,
    pre_balance: f64,
    // 手续费占用率
    pub commission_ratio: HashMap<String, f64>,
    // 保证金占用率
    pub margin_ratio: HashMap<String, f64>,
    pub size_map: HashMap<String, f64>,
    // 冻结的手续费
    pub frozen_fee: HashMap<String, Vec<OrderData>>,
    // 手续费
    pub fee: HashMap<String, f64>,
    // 平仓盈亏
    pub close_profit: HashMap<String, f64>,
    // 计数器
    pub count: f64,
    // 持仓冻结
    pub margin_frozen_container: HashMap<String, f64>,
}


impl Account {
    /// 返回账户可用资金
    fn available(&mut self) -> f64 {
        let frozen_fee: f64 = self.frozen_fee.values().into_iter().map(|x| {
            let mut sum = 0.0;
            for c in x {
                sum += c.price * c.volume * self.commission_ratio.get(c.symbol.clone().unwrap().as_str()).unwrap().clone();
            }
            sum
        }).collect::<Vec<f64>>().iter().sum();
        let fee: f64 = self.fee.values().into_iter().map(|x| { x.clone() }).collect::<Vec<f64>>().iter().sum();
        let close_profit: f64 = self.close_profit.values().into_iter().map(|x| { x.clone() }).collect::<Vec<f64>>().iter().sum();
        self.pre_balance + self.float_pnl() + close_profit - frozen_fee - fee - self.margin() - self.frozen_margin()
    }
    /// 净值
    fn balance(&mut self) -> f64 {
        self.available() + self.margin()
    }
    /// 更新成交单
    /// 此处根据成交单更新成交
    fn update_trade(&mut self, data: TradeData) {
        let symbol: String = data.symbol.clone().unwrap().to_string();
        // 手续费更新
        let commision = data.volume * data.price * *self.get_commission_ratio(symbol.as_str());
        if let Some(t) = self.fee.get(symbol.as_str()) {
            self.fee.insert(symbol.clone(), commision + *t);
        } else {
            self.fee.insert(symbol.clone(), commision);
        }
        // 成交单更新
        match data.offset.unwrap() {
            Offset::OPEN => {
                self.count += 1.0;
                if let Some(order_id) = &data.orderid {
                    if self.margin_frozen_container.contains_key(order_id) {
                        self.margin_frozen_container.remove(order_id);
                    }
                }
            }
            Offset::CLOSE => {
                // 平仓
                // let pos =
                // let symbol = &data.symbol.clone().unwrap();
                let close_profit = match data.direction.unwrap() {
                    Direction::LONG => {
                        // 计算平仓盈亏
                        (0.0 - data.price) * data.volume * *self.get_size_map(&symbol)
                    }
                    Direction::SHORT => {
                        (data.price - 0.0) * data.volume * *self.get_size_map(&symbol)
                    }
                    Direction::NET => { 0.0 }
                };
                if let Some(t) = self.close_profit.get(&symbol) {
                    self.close_profit.insert(symbol.clone(), close_profit + *t);
                } else {
                    self.close_profit.insert(symbol.clone(), close_profit);
                }
            }
            Offset::CLOSETODAY => {
                match data.direction.unwrap() {
                    Direction::LONG => {}
                    Direction::SHORT => {}
                    _ => {}
                }
            }
            Offset::CLOSEYESTERDAY => {}
            _ => { panic!("At Bad TradeData offset setting,  please checking your code first") }
        }
    }
    fn get_size_map(&mut self, symbol: &str) -> &f64 {
        self.size_map.get(symbol).unwrap_or(0.0.borrow())
    }

    fn get_commission_ratio(&mut self, symbol: &str) -> &f64 {
        self.commission_ratio.get(symbol).unwrap_or(0.0.borrow())
    }

    fn get_margin_ratio(&mut self, symbol: &str) -> &f64 {
        self.margin_ratio.get(symbol).unwrap_or(0.0.borrow())
    }
    /// 更新报单
    fn update_order(&mut self, data: OrderData) { unimplemented!() }
    /// 更新仓位,主要用于更新仓位的实时价格
    fn update_tick(&mut self, tick: TickData) { unimplemented!() }
    /// 浮动盈亏
    fn float_pnl(&mut self) -> f64 { unimplemented!() }
    /// 保证金占用
    fn margin(&mut self) -> f64 { unimplemented!() }
    /// 结算
    fn settle(&mut self) -> bool { unimplemented!() }
    /// 更新参数
    fn update_params(&mut self, params: Params) { unimplemented!() }
    /// 冻结保证金
    fn frozen_margin(&mut self) -> f64 { unimplemented!() }


    fn generate_self(&mut self) -> DailyResult { unimplemented!() }
}

impl From<HashMap<String, f64>> for Account {
    fn from(_: HashMap<String, f64, RandomState>) -> Self {
        unimplemented!()
    }
}

impl From<AccountData> for Account {
    fn from(_: AccountData) -> Self {
        unimplemented!()
    }
}

