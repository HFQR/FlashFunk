use chrono::{Date, Utc};
use std::borrow::Cow;

use flashfunk_level::constant::{Direction, Offset, Status};
use flashfunk_level::data_type::PositionData;
use flashfunk_level::data_type::{DailyResult, OrderData, Params, TickData, TradeData};
use flashfunk_level::util::hash::HashMap;
use std::io::prelude::*;

pub trait PositionManager {
    fn get_all_positions(&self) -> Vec<PositionData>;
}

impl PositionManager for Account {
    fn get_all_positions(&self) -> Vec<PositionData> {
        vec![]
    }
}

/// Account Instance
/// It provides most public ctp to Accept data or solve data
pub struct Account {
    name: String,
    pre_balance: f64,
    // 手续费占用率
    pub commission_ratio: HashMap<String, f64>,
    // 保证金占用率
    pub margin_ratio: HashMap<String, f64>,
    pub size_map: HashMap<String, f64>,
    // 冻结的手续费
    pub frozen_fee: HashMap<String, f64>,
    // 手续费
    pub fee: HashMap<Cow<'static, str>, f64>,
    // 平仓盈亏
    pub close_profit: HashMap<Cow<'static, str>, f64>,
    // 计数器
    pub count: f64,
    // 持仓冻结
    pub margin_frozen_container: HashMap<String, f64>,
    pub pre_close: HashMap<String, f64>,
    pub price_mapping: HashMap<String, f64>,
    pub date: Date<Utc>,
}

impl Account {
    pub(crate) fn new() -> Self {
        // read config
        let mut config_file = std::fs::File::open(
            std::env::var("CONFIG_FILE").expect("please input env:CONFIG_FILE"),
        )
        .expect("can not open config file");
        let mut content = String::new();
        config_file
            .read_to_string(&mut content)
            .expect("can not read config");
        let config_line = content
            .lines()
            .nth(1)
            .expect("can not get second line (config data line)");
        let mut iter = config_line.split(",");
        let symbol = iter.next().expect("can not find symbol");
        let _ = iter.next();
        let _ = iter.next();
        let size: f64 = iter
            .next()
            .expect("can not find size")
            .parse()
            .expect("size should be f64");
        let margin_ratio: f64 = iter
            .next()
            .expect("can not find margin_ratio")
            .parse()
            .expect("margin_ratio should be f64");
        let commission_ratio: f64 = iter
            .next()
            .expect("can not find commission_ratio")
            .parse()
            .expect("commission_ratio should be f64");
        let mut config_size_map: HashMap<String, f64> = HashMap::new();
        let mut config_commission_ratio_map: HashMap<String, f64> = HashMap::new();
        let mut config_margin_ratio_map: HashMap<String, f64> = HashMap::new();
        config_size_map.insert(symbol.to_string(), size);
        config_margin_ratio_map.insert(symbol.to_string(), size);
        config_commission_ratio_map.insert(symbol.to_string(), size);
        Account {
            name: "FlashFunk".to_owned(),
            pre_balance: 1_000_000.0,
            commission_ratio: config_commission_ratio_map,
            margin_ratio: config_margin_ratio_map,
            size_map: config_size_map,
            frozen_fee: Default::default(),
            fee: Default::default(),
            close_profit: Default::default(),
            count: 0.0,
            margin_frozen_container: Default::default(),
            pre_close: Default::default(),
            price_mapping: Default::default(),
            date: Utc::today(),
        }
    }

    pub fn balance(&self) -> f64 {
        self.available() + self.margin()
    }

    pub fn available(&self) -> f64 {
        self.pre_balance + self.float_pnl() + self.get_close_profit_sum()
            - self.get_frozen_fee_sum()
            - self.get_fee_sum()
            - self.margin()
            - self.frozen_margin()
    }

    pub fn get_fee_sum(&self) -> f64 {
        self.fee.values().sum()
    }

    pub fn get_frozen_fee_sum(&self) -> f64 {
        self.frozen_fee.values().sum()
    }

    pub fn get_close_profit_sum(&self) -> f64 {
        self.close_profit.values().sum()
    }
    /// update trade
    /// 1. add fee to actual fee
    /// 2.remove frozen_fee if exist
    /// 3.remove frozen if exist
    /// 4. add close_profit
    pub fn update_trade(&mut self, data: TradeData) {
        let symbol = data.symbol.as_str();
        let volume = data.volume as f64;
        // calculate fee for trade_data
        let commision = volume * data.price * self.get_commission_ratio(symbol);

        // Check the orderid if has been frozen
        self.frozen_fee.remove(&data.orderid);

        // insert fee to fact
        match self.fee.get_mut(symbol) {
            Some(t) => *t += commision,
            None => {
                let _ = self
                    .fee
                    .insert(Cow::Owned(data.symbol.to_owned()), commision);
            }
        }

        // update margin_frozen if open else add close_profit for close action
        match data.offset {
            Offset::OPEN => {
                self.count += 1.0;
                self.margin_frozen_container.remove(&data.orderid);
            }
            _ => {
                // todo : let pos =
                let close_profit = match data.direction {
                    Direction::LONG => {
                        //  replace 0.0 with  position avg price
                        (0.0 - data.price) * volume * self.get_size_map(symbol)
                    }
                    Direction::SHORT => (data.price - 0.0) * volume * self.get_size_map(symbol),
                    _ => 0.0,
                };

                match self.close_profit.get_mut(symbol) {
                    Some(t) => *t += close_profit,
                    None => {
                        let _ = self
                            .close_profit
                            .insert(Cow::Owned(data.symbol), close_profit);
                    }
                }
            }
        }
    }
    /// return size by passed symbol
    pub fn get_size_map(&self, symbol: &str) -> f64 {
        self.size_map.get(symbol).copied().unwrap_or(0.0)
    }
    /// return commission_ration by passed symbol
    pub fn get_commission_ratio(&self, symbol: &str) -> f64 {
        self.commission_ratio.get(symbol).copied().unwrap_or(0.0)
    }
    /// return margin_ratio by passed symbol
    fn get_margin_ratio(&self, symbol: &str) -> f64 {
        self.margin_ratio.get(symbol).copied().unwrap_or(0.0)
    }
    /// update order
    /// 1.add frozen fee if open
    /// 2.add margin_frozen if open
    pub fn update_order(&mut self, data: OrderData) {
        let symbol = data.symbol.as_str();
        let commission_ratio = self.get_commission_ratio(&symbol);

        match data.status {
            Status::CANCELLED => {
                // Remove Margin frozen
                self.margin_frozen_container.remove(&data.orderid.clone());
            }
            _ => {}
        }

        match data.offset {
            Offset::OPEN => {
                // Add Margin frozen
                let ratio = self.get_margin_ratio(&symbol);
                self.margin_frozen_container
                    .insert(data.orderid, data.volume * data.price * ratio);
            }
            _ => {}
        }

        self.frozen_fee
            .insert(data.symbol, commission_ratio * data.volume * data.price);
    }
    /// update position by tick
    /// refresh pnl in time
    pub fn update_tick(&mut self, _tick: TickData) {
        unimplemented!()
    }
    /// Get the float pnl for account!
    /// so the most important things is to calculate the float pnl. It should be regard as a  import things
    /// And we should use replace the price  with pre_close_price  for yd `position`, and `price` for today volume,
    /// so we had to maintain the pre_close and real_price both in looper or realtime trade
    pub fn float_pnl(&self) -> f64 {
        self.get_all_positions()
            .iter()
            .map(|x| {
                let real_price = self.get_real_price(x.symbol.as_ref());
                match x.direction.unwrap() {
                    Direction::LONG => {
                        if x.yd_volume.eq(&0.0) {
                            x.volume * (real_price - x.price) * self.get_size_map(x.symbol.as_ref())
                        } else {
                            let today = x.volume - x.yd_volume;
                            today * (real_price - x.price) * self.get_size_map(x.symbol.as_ref())
                                + x.yd_volume
                                    * (real_price - self.get_pre_price(x.symbol.as_ref()))
                                    * self.get_size_map(x.symbol.as_ref())
                        }
                    }
                    Direction::SHORT => {
                        if x.yd_volume.eq(&0.0) {
                            x.volume * (x.price - real_price) * self.get_size_map(x.symbol.as_ref())
                        } else {
                            let today = x.volume - x.yd_volume;
                            today * (x.price - real_price) * self.get_size_map(x.symbol.as_ref())
                                + x.yd_volume
                                    * (self.get_pre_price(x.symbol.as_ref()) - real_price)
                                    * self.get_size_map(x.symbol.as_ref())
                        }
                    }
                    _ => unimplemented!("暂不支持"),
                }
            })
            .sum()
    }
    /// 获取实时价格
    fn get_real_price(&self, symbol: &str) -> f64 {
        *self.price_mapping.get(symbol).unwrap_or(&0.0)
    }
    /// 获取昨日收盘价
    fn get_pre_price(&self, symbol: &str) -> f64 {
        *self.pre_close.get(symbol).unwrap_or(&0.0)
    }
    ///  get the margin of position for the account
    pub fn margin(&self) -> f64 {
        self.get_all_positions()
            .into_iter()
            .fold(0.0, |mut rs, pos| {
                let symbol = pos.symbol.as_ref();
                rs += pos.price
                    * pos.volume
                    * self.get_margin_ratio(symbol)
                    * self.get_size_map(symbol);

                rs
            })
    }
    /// settle the account by passed a datetime
    pub fn settle(&mut self, date: Date<Utc>) -> bool {
        if self.date == date {
            false
        } else {
            // let p = self.generate_self();
            self.date = date;
            true
        }
    }
    /// update the params by pass a Params
    /// it looks like hard to understand
    fn update_params(&mut self, _params: Params) {
        unimplemented!()
    }
    /// get the frozen , when day,end ,it will zero
    pub fn frozen_margin(&self) -> f64 {
        self.margin_frozen_container.values().sum()
    }

    /// generator a Account object named DailyResult, it will be written into database
    fn generate_self(&self) -> DailyResult {
        DailyResult {
            available: self.available(),
            balance: self.balance(),
            fee: self.get_fee_sum(),
            margin: self.margin(),
            date: self.date.to_string(),
        }
    }
}

impl From<HashMap<String, f64>> for Account {
    fn from(_: HashMap<String, f64>) -> Self {
        unimplemented!()
    }
}
