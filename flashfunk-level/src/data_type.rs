use std::borrow::{Borrow, BorrowMut, Cow};

use crate::constant::*;
use bitflags::_core::cmp::{max, min};
use bitflags::_core::ops::{Deref, DerefMut};
use chrono::{Date, DateTime, NaiveDateTime, Timelike, Utc};
use chrono_tz::Tz;
use clickhouse_rs::types::{Block, Complex, Row};
use std::time::Instant;

/// Tick Data
#[derive(Clone)]
pub struct TickData {
    pub symbol: String,
    pub exchange: Exchange,
    pub datetime: NaiveDateTime,
    pub name: String,
    pub volume: i32,
    pub amount: f64,
    pub open_interest: f64,
    pub last_price: f64,
    pub last_volume: i32,
    pub limit_up: f64,
    pub limit_down: f64,
    pub open_price: f64,
    pub high_price: f64,
    pub low_price: f64,
    pub pre_close: f64,
    pub bid_price: [f64; 5],
    pub ask_price: [f64; 5],
    pub bid_volume: [i32; 5],
    pub ask_volume: [i32; 5],
    pub instant: Instant,
}

impl Default for TickData {
    fn default() -> TickData {
        TickData {
            symbol: String::from(""),
            exchange: Exchange::INIT,
            datetime: chrono::Utc::now().naive_utc(),
            name: String::from(""),
            volume: 0,
            amount: 0.0,
            open_interest: 0.0,
            last_price: 0.0,
            last_volume: 0,
            limit_up: 0.0,
            limit_down: 0.0,
            open_price: 0.0,
            high_price: 0.0,
            low_price: 0.0,
            pre_close: 0.0,
            bid_price: [0.0; 5],
            ask_price: [0.0; 5],
            bid_volume: [0; 5],
            ask_volume: [0; 5],
            instant: Instant::now(),
        }
    }
}

impl TickData {
    pub fn bid_price(&self, index: usize) -> f64 {
        *self.bid_price.get(index).unwrap()
    }

    pub fn ask_price(&self, index: usize) -> f64 {
        *self.ask_price.get(index).unwrap()
    }

    pub fn bid_volume(&self, index: usize) -> f64 {
        *self.bid_volume.get(index).unwrap() as f64
    }

    pub fn ask_volume(&self, index: usize) -> f64 {
        *self.ask_volume.get(index).unwrap() as f64
    }
}

/// Order Data
#[derive(Clone, Debug)]
pub struct OrderData {
    pub symbol: String,
    pub exchange: Exchange,
    pub datetime: NaiveDateTime,
    pub orderid: Option<String>,
    pub order_type: OrderType,
    pub direction: Option<Direction>,
    pub offset: Offset,
    pub price: f64,
    pub volume: f64,
    pub traded: f64,
    pub status: Status,
}

pub struct ExtraOrder {
    n: OrderData,
}

impl From<OrderData> for ExtraOrder {
    fn from(data: OrderData) -> Self {
        Self { n: data }
    }
}

impl Deref for ExtraOrder {
    type Target = OrderData;

    fn deref(&self) -> &Self::Target {
        self.n.borrow()
    }
}

impl DerefMut for ExtraOrder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.n.borrow_mut()
    }
}

pub struct ExtraTrade {
    n: TradeData,
}

impl From<TradeData> for ExtraTrade {
    fn from(data: TradeData) -> Self {
        Self { n: data }
    }
}

impl Deref for ExtraTrade {
    type Target = TradeData;

    fn deref(&self) -> &Self::Target {
        self.n.borrow()
    }
}

impl DerefMut for ExtraTrade {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.n.borrow_mut()
    }
}

impl From<&Tick> for TickData {
    fn from(tick: &Tick) -> Self {
        TickData {
            symbol: tick.local_symbol.clone(),
            exchange: Exchange::INIT,
            datetime: tick.datetime.clone(),
            name: "".to_string(),
            volume: tick.volume.clone(),
            amount: tick.amount.clone(),
            open_interest: tick.open_interest.clone(),
            last_price: tick.last_price.clone(),
            last_volume: 0,
            limit_up: tick.limit_up.clone(),
            limit_down: tick.limit_down.clone(),
            open_price: 0.0,
            high_price: 0.0,
            low_price: 0.0,
            pre_close: 0.0,
            bid_price: [
                tick.bid_price_1.clone(),
                tick.bid_price_2.clone(),
                tick.bid_price_3.clone(),
                tick.bid_price_4.clone(),
                tick.bid_price_5.clone(),
            ],
            ask_price: [
                tick.ask_price_1.clone(),
                tick.ask_price_2.clone(),
                tick.ask_price_3.clone(),
                tick.ask_price_4.clone(),
                tick.ask_price_5.clone(),
            ],
            bid_volume: [
                tick.bid_volume_1.clone(),
                tick.bid_volume_2.clone(),
                tick.bid_volume_3.clone(),
                tick.bid_volume_4.clone(),
                tick.bid_volume_5.clone(),
            ],
            ask_volume: [
                tick.ask_volume_1.clone(),
                tick.ask_volume_2.clone(),
                tick.ask_volume_3.clone(),
                tick.ask_volume_4.clone(),
                tick.ask_volume_5.clone(),
            ],
            instant: Instant::now(),
        }
    }
}

/// Trade Data
#[derive(Clone, Debug)]
pub struct TradeData {
    pub symbol: Cow<'static, str>,
    pub exchange: Option<Exchange>,
    pub datetime: NaiveDateTime,
    pub orderid: Option<String>,
    pub tradeid: Option<String>,
    pub direction: Option<Direction>,
    pub offset: Option<Offset>,
    pub price: f64,
    pub volume: i32,
}

impl Default for TradeData {
    fn default() -> TradeData {
        TradeData {
            symbol: Cow::Borrowed(""),
            exchange: None,
            datetime: chrono::Utc::now().naive_utc(),
            orderid: None,
            tradeid: None,
            direction: None,
            offset: None,
            price: 0.0,
            volume: 0,
        }
    }
}

/// Position Data
#[derive(Clone, Debug)]
pub struct PositionData {
    pub symbol: Cow<'static, str>,
    pub exchange: Option<Exchange>,
    pub direction: Option<Direction>,
    pub volume: f64,
    pub frozen: f64,
    pub price: f64,
    pub pnl: f64,
    pub yd_volume: f64,
    pub available: f64,
}

impl PositionData {
    pub fn new_with_long(symbol: &str) -> Self {
        PositionData {
            symbol: Cow::Owned(symbol.to_owned()),
            exchange: None,
            direction: Option::from(Direction::LONG),
            volume: 0.0,
            frozen: 0.0,
            price: 0.0,
            pnl: 0.0,
            yd_volume: 0.0,
            available: 0.0,
        }
    }
    pub fn new_with_short(symbol: &str) -> Self {
        PositionData {
            symbol: Cow::Owned(symbol.to_owned()),
            exchange: None,
            direction: Option::from(Direction::SHORT),
            volume: 0.0,
            frozen: 0.0,
            price: 0.0,
            pnl: 0.0,
            yd_volume: 0.0,
            available: 0.0,
        }
    }
}

impl Default for PositionData {
    fn default() -> PositionData {
        PositionData {
            symbol: Cow::Borrowed(""),
            exchange: None,
            direction: None,
            volume: 0.0,
            frozen: 0.0,
            price: 0.0,
            pnl: 0.0,
            yd_volume: 0.0,
            available: 0.0,
        }
    }
}

/// Position Data
/// fixme 針對於與這種數據做數組優化
#[derive(Clone, Debug, Default)]
pub struct Position {
    pub symbol: Cow<'static, str>,
    pub exchange: Option<Exchange>,
    pub long_volume: f64,
    pub short_volume: f64,
    pub long_frozen: f64,
    pub short_frozen: f64,
    pub long_price: f64,
    pub short_price: f64,
    pub long_pnl: f64,
    pub short_pnl: f64,
    pub long_yd_volume: f64,
    pub short_yd_volume: f64,
    pub long_available: f64,
    pub short_available: f64,
}

impl Position {
    pub(crate) fn new_with_symbol(symbol: &str) -> Self {
        Self {
            symbol: Cow::Owned(symbol.to_owned()),
            exchange: None,
            long_volume: 0.0,
            short_volume: 0.0,
            long_frozen: 0.0,
            short_frozen: 0.0,
            long_price: 0.0,
            short_price: 0.0,
            long_pnl: 0.0,
            short_pnl: 0.0,
            long_yd_volume: 0.0,
            short_yd_volume: 0.0,
            long_available: 0.0,
            short_available: 0.0,
        }
    }

    pub(crate) fn new_with_long(
        symbol: &str,
        long_volume: f64,
        long_price: f64,
        long_available: f64,
        long_frozen: f64,
        long_yd_volume: f64,
        long_pnl: f64,
    ) -> Self {
        Self {
            symbol: Cow::Owned(symbol.to_owned()),
            exchange: None,
            long_volume,
            short_volume: 0.0,
            long_frozen,
            short_frozen: 0.0,
            long_price,
            short_price: 0.0,
            long_pnl,
            short_pnl: 0.0,
            long_yd_volume,
            short_yd_volume: 0.0,
            long_available,
            short_available: 0.0,
        }
    }

    pub(crate) fn new_with_short(
        symbol: &str,
        short_volume: f64,
        short_price: f64,
        short_available: f64,
        short_frozen: f64,
        short_yd_volume: f64,
        short_pnl: f64,
    ) -> Self {
        Self {
            symbol: Cow::Owned(symbol.to_owned()),
            exchange: None,
            long_volume: 0.0,
            short_volume,
            long_frozen: 0.0,
            short_frozen,
            long_price: 0.0,
            short_price,
            long_pnl: 0.0,
            short_pnl,
            long_yd_volume: 0.0,
            short_yd_volume,
            long_available: 0.0,
            short_available,
        }
    }

    pub fn long_today_volume(&self) -> f64 {
        self.long_volume - self.long_yd_volume
    }

    pub fn short_today_volume(&self) -> f64 {
        self.short_volume - self.short_yd_volume
    }
}

/// Account Data
#[derive(Clone, Debug)]
pub struct AccountData {
    pub accountid: String,
    pub balance: f64,
    pub frozen: f64,
    pub date: Date<Utc>,
}

impl Default for AccountData {
    fn default() -> Self {
        AccountData {
            accountid: "".to_string(),
            balance: 0.0,
            frozen: 0.0,
            date: Utc::today(),
        }
    }
}

/// Contract Data
#[derive(Clone, Debug)]
pub struct ContractData {
    pub symbol: String,
    pub exchange: Option<Exchange>,
    pub name: Option<String>,
    pub product: Option<Product>,
    pub size: f64,
    pub pricetick: f64,
    pub min_volume: f64,
    pub stop_supported: bool,
    pub net_position: bool,
    pub history_data: bool,
    pub option_strike: f64,
    pub option_underlying: Option<String>,
    pub option_type: Option<OptionType>,
    pub option_expiry: Option<DateTime<Utc>>,
    pub option_portfolio: Option<String>,
    pub option_index: Option<String>,
}

impl Default for ContractData {
    fn default() -> ContractData {
        ContractData {
            symbol: "".to_string(),
            exchange: None,
            name: None,
            product: None,
            size: 0.0,
            pricetick: 0.0,
            min_volume: 1.0,
            stop_supported: false,
            net_position: false,
            history_data: false,
            option_strike: 0.0,
            option_underlying: None,
            option_type: None,
            option_expiry: None,
            option_portfolio: None,
            option_index: None,
        }
    }
}

pub struct ContractVec {
    n: Vec<ContractData>,
}

impl From<Vec<ContractData>> for ContractVec {
    fn from(n: Vec<ContractData>) -> Self {
        ContractVec { n }
    }
}

impl Deref for ContractVec {
    type Target = Vec<ContractData>;

    fn deref(&self) -> &Self::Target {
        &self.n
    }
}

/// Subscribe Request
#[derive(Clone, Debug)]
pub struct SubscribeRequest {
    pub symbol: String,
}

/// Order Request
#[derive(Clone, Debug)]
pub struct OrderRequest {
    pub symbol: String,
    pub exchange: Exchange,
    pub direction: Direction,
    pub order_type: OrderType,
    pub volume: f64,
    pub price: f64,
    pub offset: Offset,
    pub reference: Option<String>,
}

/// Cancel Request
#[derive(Clone)]
pub struct CancelRequest {
    pub order_id: String,
    pub symbol: String,
    pub exchange: Exchange,
}

#[derive(Clone, Debug)]
pub struct ConnectInfo {}

#[derive(Clone, Debug)]
pub struct Params {
    pub connect_info: ConnectInfo,
}

#[derive(Clone, Debug)]
pub struct DailyResult {
    pub available: f64,
    pub balance: f64,
    pub fee: f64,
    pub margin: f64,
    pub date: String,
}

#[derive(Clone, Default)]
pub struct LoginForm {
    user_id: Cow<'static, str>,
    password: Cow<'static, str>,
    broke_id: Cow<'static, str>,
    app_id: Cow<'static, str>,
    md_address: Cow<'static, str>,
    td_address: Cow<'static, str>,
    auth_code: Cow<'static, str>,
    production_info: Cow<'static, str>,
}

impl LoginForm {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn user_id(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.user_id = value.into();
        self
    }

    pub fn password(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.password = value.into();
        self
    }

    pub fn broke_id(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.broke_id = value.into();
        self
    }

    pub fn app_id(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.app_id = value.into();
        self
    }

    pub fn md_address(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.md_address = value.into();
        self
    }

    pub fn td_address(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.td_address = value.into();
        self
    }

    pub fn auth_code(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.auth_code = value.into();
        self
    }

    pub fn production_info(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.production_info = value.into();
        self
    }

    pub(super) fn _user_id(&self) -> &str {
        &self.user_id
    }

    pub(super) fn _password(&self) -> &str {
        &self.password
    }

    pub(super) fn _broke_id(&self) -> &str {
        &self.broke_id
    }

    pub(super) fn _app_id(&self) -> &str {
        &self.app_id
    }

    pub(super) fn _md_address(&self) -> &str {
        &self.md_address
    }

    pub(super) fn _td_address(&self) -> &str {
        &self.td_address
    }

    pub(super) fn _auth_code(&self) -> &str {
        &self.auth_code
    }

    pub(super) fn _production_info(&self) -> &str {
        &self.production_info
    }
}

pub struct QueryRequest {}

pub struct Bar {
    pub high: f64,
    pub volume: f64,
    pub amount: f64,
    pub low: f64,
    pub open: f64,
    pub close: f64,
    pub datetime: NaiveDateTime,
    pub symbol: String,
    pub frq_sec: i32,
}

impl Bar {
    pub fn new(symbol: String, frq: i32, navie: NaiveDateTime, open: f64) -> Self {
        Bar {
            high: 0.0,
            volume: 0.0,
            amount: 0.0,
            low: 0.0,
            open,
            close: 0.0,
            datetime: navie,
            symbol,
            frq_sec: frq,
        }
    }
}

pub struct Generator {
    symbol: String,
    last: Option<Bar>,
    frq: i32,
}

impl Generator {
    pub fn new(symbol: String, frq: i32) -> Self {
        Generator {
            symbol,
            last: None,
            frq,
        }
    }

    pub fn update_tick<F>(&mut self, tick: &TickData, f: F)
    where
        F: FnOnce(&mut Self, Bar),
    {
        let time = tick.datetime;
        if self.last.is_none() {
            self.last = Some(Bar::new(
                self.symbol.clone(),
                self.frq,
                time,
                tick.last_price,
            ));
            return;
        }
        let bar = self.last.as_mut().unwrap();
        if time.second() - bar.datetime.second() >= self.frq as u32 {
            let x = self.last.take().unwrap();
            f(self, x);
            self.last = Some(Bar::new(
                self.symbol.clone(),
                self.frq,
                time,
                tick.last_price,
            ));
        } else {
            bar.close = tick.last_price;
            bar.high = Generator::max(tick.last_price, bar.high);
            bar.low = Generator::min(tick.last_price, tick.low_price);
        }
    }

    fn max(i: f64, v: f64) -> f64 {
        max((i * 10000.0) as i32, (v * 10000.0) as i32) as f64 / 10000.0
    }

    fn min(i: f64, v: f64) -> f64 {
        min((i * 10000.0) as i32, (v * 10000.0) as i32) as f64 / 10000.0
    }
}

#[derive(Clone, Debug)]
pub struct Tick {
    pub local_symbol: String,
    pub volume: i32,
    pub amount: f64,
    pub open_interest: f64,
    pub low_price: f64,
    pub high_price: f64,
    pub limit_up: f64,
    pub limit_down: f64,
    pub last_price: f64,
    pub ask_price_1: f64,
    pub ask_price_2: f64,
    pub ask_price_3: f64,
    pub ask_price_4: f64,
    pub ask_price_5: f64,
    pub bid_price_1: f64,
    pub bid_price_2: f64,
    pub bid_price_3: f64,
    pub bid_price_4: f64,
    pub bid_price_5: f64,
    pub datetime: NaiveDateTime,
    pub ask_volume_1: i32,
    pub ask_volume_2: i32,
    pub ask_volume_3: i32,
    pub ask_volume_4: i32,
    pub ask_volume_5: i32,
    pub bid_volume_1: i32,
    pub bid_volume_2: i32,
    pub bid_volume_3: i32,
    pub bid_volume_4: i32,
    pub bid_volume_5: i32,
}

impl Tick {
    // 遍历column
    pub fn new_with_block(block: Block<Complex>) -> Vec<Tick> {
        block
            .rows()
            .into_iter()
            .map(|x| Tick::new(x))
            .collect::<Vec<Tick>>()
    }
    pub fn new(row: Row<Complex>) -> Tick {
        let x: DateTime<Tz> = row.get("datetime").unwrap();
        Tick {
            local_symbol: row.get("local_symbol").unwrap(),
            volume: row.get("volume").unwrap(),
            amount: row.get("amount").unwrap(),
            open_interest: row.get("open_interest").unwrap(),
            low_price: row.get("low_price").unwrap(),
            high_price: row.get("high_price").unwrap(),
            limit_up: row.get("limit_up").unwrap(),
            limit_down: row.get("limit_down").unwrap(),
            last_price: row.get("last_price").unwrap(),
            ask_price_1: row.get("ask_price_1").unwrap(),
            ask_price_2: row.get("ask_price_2").unwrap(),
            ask_price_3: row.get("ask_price_3").unwrap(),
            ask_price_4: row.get("ask_price_4").unwrap(),
            ask_price_5: row.get("ask_price_5").unwrap(),
            bid_price_1: row.get("bid_price_1").unwrap(),
            bid_price_2: row.get("bid_price_2").unwrap(),
            bid_price_3: row.get("bid_price_3").unwrap(),
            bid_price_4: row.get("bid_price_4").unwrap(),
            bid_price_5: row.get("bid_price_5").unwrap(),
            datetime: x.naive_local(),
            ask_volume_1: row.get("ask_volume_1").unwrap(),
            ask_volume_2: row.get("ask_volume_2").unwrap(),
            ask_volume_3: row.get("ask_volume_3").unwrap(),
            ask_volume_4: row.get("ask_volume_4").unwrap(),
            ask_volume_5: row.get("ask_volume_5").unwrap(),
            bid_volume_1: row.get("bid_volume_1").unwrap(),
            bid_volume_2: row.get("bid_volume_2").unwrap(),
            bid_volume_3: row.get("bid_volume_3").unwrap(),
            bid_volume_4: row.get("bid_volume_4").unwrap(),
            bid_volume_5: row.get("bid_volume_5").unwrap(),
        }
    }
}
