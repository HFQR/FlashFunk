//! # General Structs
//! Author: Aaron Qiu
#![allow(dead_code)]

use crate::constants::*;
use chrono::{DateTime, Utc, Date};
use std::option::Option;
use actix::Message;

/// Tick Data
#[derive(Message, Clone)]
#[rtype(result = "()")]
pub struct TickData {
    pub symbol: String,
    pub exchange: Option<Exchange>,
    pub datetime: Option<DateTime<Utc>>,
    pub name: Option<String>,
    pub volume: f64,
    pub open_interest: f64,
    pub last_price: f64,
    pub last_volume: f64,
    pub limit_up: f64,
    pub limit_down: f64,
    pub open_price: f64,
    pub high_price: f64,
    pub low_price: f64,
    pub pre_close: f64,
    pub bid_price_1: f64,
    pub bid_price_2: f64,
    pub bid_price_3: f64,
    pub bid_price_4: f64,
    pub bid_price_5: f64,
    pub ask_price_1: f64,
    pub ask_price_2: f64,
    pub ask_price_3: f64,
    pub ask_price_4: f64,
    pub ask_price_5: f64,
    pub bid_volume_1: f64,
    pub bid_volume_2: f64,
    pub bid_volume_3: f64,
    pub bid_volume_4: f64,
    pub bid_volume_5: f64,
    pub ask_volume_1: f64,
    pub ask_volume_2: f64,
    pub ask_volume_3: f64,
    pub ask_volume_4: f64,
    pub ask_volume_5: f64,
}

impl Default for TickData {
    fn default() -> TickData {
        TickData {
            symbol: "".to_string(),
            exchange: None,
            datetime: None,
            name: None,
            volume: 0.0,
            open_interest: 0.0,
            last_price: 0.0,
            last_volume: 0.0,
            limit_up: 0.0,
            limit_down: 0.0,
            open_price: 0.0,
            high_price: 0.0,
            low_price: 0.0,
            pre_close: 0.0,
            bid_price_1: 0.0,
            bid_price_2: 0.0,
            bid_price_3: 0.0,
            bid_price_4: 0.0,
            bid_price_5: 0.0,
            ask_price_1: 0.0,
            ask_price_2: 0.0,
            ask_price_3: 0.0,
            ask_price_4: 0.0,
            ask_price_5: 0.0,
            bid_volume_1: 0.0,
            bid_volume_2: 0.0,
            bid_volume_3: 0.0,
            bid_volume_4: 0.0,
            bid_volume_5: 0.0,
            ask_volume_1: 0.0,
            ask_volume_2: 0.0,
            ask_volume_3: 0.0,
            ask_volume_4: 0.0,
            ask_volume_5: 0.0,
        }
    }
}

/// Bar Data
#[derive(Message, Clone)]
#[rtype(result = "()")]
pub struct BarData {
    pub symbol: String,
    pub exchange: Option<Exchange>,
    pub datetime: Option<DateTime<Utc>>,
    pub interval: Option<Interval>,
    pub volume: f64,
    pub open_interest: f64,
    pub open_price: f64,
    pub high_price: f64,
    pub low_price: f64,
    pub close_price: f64,
}

impl Default for BarData {
    fn default() -> BarData {
        BarData {
            symbol: "".to_string(),
            exchange: None,
            datetime: None,
            interval: None,
            volume: 0.0,
            open_interest: 0.0,
            open_price: 0.0,
            high_price: 0.0,
            low_price: 0.0,
            close_price: 0.0,
        }
    }
}

/// Order Data
#[derive(Message, Clone)]
#[rtype(result = "()")]
pub struct OrderData {
    pub symbol: String,
    pub exchange: Option<Exchange>,
    pub datetime: Option<DateTime<Utc>>,
    pub orderid: Option<String>,
    pub order_type: OrderType,
    pub direction: Option<Direction>,
    pub offset: Offset,
    pub price: f64,
    pub volume: f64,
    pub traded: f64,
    pub status: Option<Status>,
}

impl Default for OrderData {
    fn default() -> OrderData {
        OrderData {
            symbol: "".to_string(),
            exchange: None,
            datetime: None,
            orderid: None,
            order_type: OrderType::LIMIT,
            direction: None,
            offset: Offset::NONE,
            price: 0.0,
            volume: 0.0,
            traded: 0.0,
            status: None,
        }
    }
}

/// Trade Data
#[derive(Message, Clone)]
#[rtype(result = "()")]
pub struct TradeData {
    pub symbol: String,
    pub exchange: Option<Exchange>,
    pub datetime: Option<DateTime<Utc>>,
    pub orderid: Option<String>,
    pub tradeid: Option<String>,
    pub direction: Option<Direction>,
    pub offset: Option<Offset>,
    pub price: f64,
    pub volume: f64,
}

impl Default for TradeData {
    fn default() -> TradeData {
        TradeData {
            symbol: "".to_string(),
            exchange: None,
            datetime: None,
            orderid: None,
            tradeid: None,
            direction: None,
            offset: None,
            price: 0.0,
            volume: 0.0,
        }
    }
}

/// Position Data
#[derive(Message, Clone)]
#[rtype(result = "()")]
pub struct PositionData {
    pub symbol: String,
    pub exchange: Option<Exchange>,
    pub direction: Option<Direction>,
    pub volume: f64,
    pub frozen: f64,
    pub price: f64,
    pub pnl: f64,
    pub yd_volume: f64,
    pub available: f64,
}

impl Default for PositionData {
    fn default() -> PositionData {
        PositionData {
            symbol: "".to_string(),
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

/// Account Data
#[derive(Message, Clone)]
#[rtype(result = "()")]
pub struct AccountData {
    pub accountid: String,
    pub balance: f64,
    pub frozen: f64,
    pub date: Date<Utc>,
}

/// Contract Data
#[derive(Message, Clone)]
#[rtype(result = "()")]
pub struct ContractData {
    pub symbol: String,
    pub exchange: Option<Exchange>,
    pub name: Option<String>,
    pub product: Option<Product>,
    pub size: Option<i32>,
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
            size: None,
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

/// Subscribe Request
#[derive(Message, Clone)]
#[rtype(result = "()")]
pub struct SubscribeRequest {
    pub symbol: String,
}

/// Order Request
#[derive(Message, Clone)]
#[rtype(result = "()")]
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
#[derive(Message, Clone)]
#[rtype(result = "()")]
pub struct CancelRequest {
    pub orderid: String,
}

/// History Request
#[derive(Message, Clone)]
#[rtype(result = "()")]
pub struct HistoryRequest {
    pub symbol: String,
    pub exchange: Exchange,
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
    pub interval: Interval,
}

#[derive(Message, Clone)]
#[rtype(result = "()")]
pub struct ConnectInfo {}

#[derive(Message, Clone)]
#[rtype(result = "()")]
pub struct Params {
    pub connect_info: ConnectInfo
}

#[derive(Message, Clone)]
#[rtype(result = "()")]
pub struct DailyResult {
    pub available: f64,
    pub balance: f64,
    pub fee: f64,
    pub margin: f64,
    pub date: String,
}

#[derive(Message, Clone)]
#[rtype(result = "()")]
pub struct LoginForm {
    user_id: String,
    password: String,
    app_id: String,
    address: String,
    auth_code: String,
    production_info: String,
}
