#![allow(dead_code, unused_variables)]

use std::collections::hash_map::RandomState;
use std::collections::HashMap;

use crate::structs::{AccountData, OrderData, Params, TickData, TradeData, DailyResult};
use std::borrow::Borrow;
use crate::constants::{Offset, Direction};

use crate::structs::PositionData;
use std::sync::Arc;

pub struct PositionManager {
    pub account: Box<Arc<Account>>
}

impl PositionManager {
    fn get_all_positions(&mut self) -> Vec<PositionData> {
        unimplemented!()
    }
}


/// Account Instance
/// It provides most public API to Accept data or solve data

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
    pub frozen_fee: HashMap<String, f64>,
    // 手续费
    pub fee: HashMap<String, f64>,
    // 平仓盈亏
    pub close_profit: HashMap<String, f64>,
    // 计数器
    pub count: f64,
    // 持仓冻结
    pub margin_frozen_container: HashMap<String, f64>,
    pub position_manager: PositionManager,
}


impl Account {
    pub fn balance(&mut self) -> f64 {
        self.available() + self.margin()
    }
    pub fn available(&mut self) -> f64 {
        self.pre_balance + self.float_pnl() + self.get_close_profit_sum() - self.get_frozen_fee_sum() - self.get_fee_sum() - self.margin() - self.frozen_margin()
    }
    pub fn get_fee_sum(&mut self) -> f64 {
        self.fee.values().into_iter().map(|x| { x.clone() }).collect::<Vec<f64>>().iter().sum()
    }

    pub fn get_frozen_fee_sum(&mut self) -> f64 {
        self.frozen_fee.values().into_iter().sum()
    }

    pub fn get_close_profit_sum(&mut self) -> f64 {
        self.close_profit.values().into_iter().map(|x| { x.clone() }).collect::<Vec<f64>>().iter().sum()
    }
    /// update trade
    /// 1. add fee to actual fee
    /// 2.remove frozen_fee if exist
    /// 3.remove frozen if exist
    /// 4. add close_profit
    pub fn update_trade(&mut self, data: TradeData) {
        let symbol: String = data.symbol.clone().unwrap().to_string();
        // calculate fee for trade_data
        let commision = data.volume * data.price * *self.get_commission_ratio(symbol.as_str());

        // Check the orderid if has been frozen
        if let Some(order_id) = &data.orderid {
            if self.frozen_fee.contains_key(order_id) {
                self.frozen_fee.remove(order_id);
            }
        }
        // insert fee to fact
        if let Some(t) = self.fee.get(symbol.as_str()) {
            let sum = commision + *t;
            self.fee.insert(symbol.clone(), sum);
        } else {
            self.fee.insert(symbol.clone(), commision);
        }
        // update margin_frozen if open else add close_profit for close action
        match data.offset.unwrap() {
            Offset::OPEN => {
                self.count += 1.0;
                if let Some(order_id) = &data.orderid {
                    if self.margin_frozen_container.contains_key(order_id) {
                        self.margin_frozen_container.remove(order_id);
                    }
                }
            }
            _ => {
                // todo : let pos =
                let close_profit = match data.direction.unwrap() {
                    Direction::LONG => {
                        //  replace 0.0 with  position avg price
                        (0.0 - data.price) * data.volume * *self.get_size_map(&symbol)
                    }
                    Direction::SHORT => {
                        (data.price - 0.0) * data.volume * *self.get_size_map(&symbol)
                    }
                    Direction::NET => { 0.0 }
                };
                if let Some(t) = self.close_profit.get(&symbol) {
                    let sum = close_profit + *t;
                    self.close_profit.insert(symbol.clone(), sum);
                } else {
                    self.close_profit.insert(symbol.clone(), close_profit);
                }
            }
        }
    }
    /// return size by passed symbol
    fn get_size_map(&mut self, symbol: &str) -> &f64 {
        self.size_map.get(symbol).unwrap_or(0.0.borrow())
    }
    /// return commission_ration by passed symbol
    fn get_commission_ratio(&mut self, symbol: &str) -> &f64 {
        self.commission_ratio.get(symbol).unwrap_or(0.0.borrow())
    }
    /// return margin_ratio by passed symbol
    fn get_margin_ratio(&mut self, symbol: &str) -> &f64 {
        self.margin_ratio.get(symbol).unwrap_or(0.0.borrow())
    }
    /// update order
    /// 1.add frozen fee if open
    /// 2.add margin_frozen if open
    pub fn update_order(&mut self, data: OrderData) {
        let symbol: String = data.symbol.clone().unwrap().to_string();
        let commission_ratio = self.get_commission_ratio(&symbol).clone();
        self.frozen_fee.insert(symbol.clone(), commission_ratio * data.volume * data.price);
        match data.offset {
            Offset::OPEN => {
                // Add Margin frozen
                let margin_ratio = self.get_margin_ratio(&symbol).clone();
                self.margin_frozen_container.insert(data.orderid.unwrap().clone(), data.volume * data.price * margin_ratio);
            }
            _ => {}
        }
    }
    /// update position by tick
    /// refresh pnl in time
    pub fn update_tick(&mut self, tick: TickData) { unimplemented!() }
    ///  get the float pnl for account
    pub fn float_pnl(&mut self) -> f64 { unimplemented!() }
    ///  get the margin of position for the account
    pub fn margin(&mut self) -> f64 {
        let mut rs = 0.0;

        for pos in self.position_manager.get_all_positions() {
            if let Some(t) = &pos.symbol {
                let size = *self.get_size_map(t.as_str());
                rs += pos.price * pos.volume * *self.get_margin_ratio(t.as_str()) * size
            }
        };
        rs
    }
    /// settle the account by passed a datetime
    pub fn settle(&mut self) -> bool { unimplemented!() }
    /// update the params by pass a Params
    /// it looks like hard to understand
    fn update_params(&mut self, params: Params) { unimplemented!() }
    /// get the frozen , when day,end ,it will zero
    pub fn frozen_margin(&mut self) -> f64 { unimplemented!() }

    /// generator a Account object named DailyResult, it will be written into database
    fn generate_self(&mut self) -> DailyResult {
        DailyResult {
            available: self.available(),
            balance: self.balance(),
            fee: self.get_fee_sum(),
            margin: self.margin(),
            date: "".to_string(),
        }
    }
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

