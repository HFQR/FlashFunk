use std::collections::hash_map::RandomState;
use std::collections::HashMap;
use std::ops::{DerefMut, Deref};

use crate::structs::{AccountData, OrderData, Params, TickData, TradeData};
use std::borrow::Borrow;

///
pub struct Account {
    name: f64,
    pre_balance: f64,
    // 手续费占用率
    pub commission_ratio: HashMap<String, f64>,
    // 保证金占用率
    pub margin_ratio: HashMap<String, f64>,
    // 冻结的手续费
    pub frozen_fee: HashMap<String, Vec<OrderData>>,
    // 手续费
    pub fee: HashMap<String, f64>,

    pub close_profit: HashMap<String, f64>,
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
    fn balance() -> f64 { unimplemented!() }
    /// 更新成交单
    fn update_trade(&mut self, data: TradeData) { unimplemented!() }
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

