use std::collections::HashMap;
use super::structs::OrderData;
use crate::structs::{TradeData, AccountData};
use std::collections::hash_map::RandomState;

///
pub struct Account {
    pre_balance: f64,
    // 手续费占用率
    pub commission_ratio: HashMap<String, f64>,
    // 保证金占用率
    pub margin_ratio: HashMap<String, f64>,
    // 冻结的手续费
    pub frozen_fee: HashMap<String, Vec<OrderData>>,
    // 手续费
    pub fee: HashMap<String, f64>,
}


impl Account {
    /// 返回账户可用资金
    fn available() -> f64 { unimplemented!() }
    /// 净值
    fn balance() -> f64 { unimplemented!() }
    /// 更新成交单
    fn update_trade(&mut self, data: TradeData) { unimplemented!() }
    /// 更新报单
    fn update_order(&mut self, data: OrderData) { unimplemented!() }
    /// 浮动盈亏
    fn float_pnl(&mut self) -> f64 { unimplemented!() }
    /// 保证金占用
    fn margin(&mut self) -> f64 { unimplemented!() }
    /// 结算
    fn settle(&mut self) -> bool { unimplemented!() }

    fn update_params(&mut self, params: Params){unimplemented!()}

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

