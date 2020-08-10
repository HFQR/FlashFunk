//! # General Structs
//! Author: Aaron Qiu
use crate::constants::*;
use chrono::{DateTime, Utc};
use std::option::Option;

#[allow(dead_code)]
pub struct TickData {
    symbol: Option<String>,
    exchange: Option<Exchange>,
    datetime: Option<DateTime<Utc>>,
    name: Option<String>,
    volume: f64,
    open_interest: f64,
    last_price: f64,
    last_volume: f64,
    limit_up: f64,
    limit_down: f64,
    open_price: f64,
    high_price: f64,
    low_price: f64,
    pre_close: f64,
    bid_price_1: f64,
    bid_price_2: f64,
    bid_price_3: f64,
    bid_price_4: f64,
    bid_price_5: f64,
    ask_price_1: f64,
    ask_price_2: f64,
    ask_price_3: f64,
    ask_price_4: f64,
    ask_price_5: f64,
    bid_volume_1: f64,
    bid_volume_2: f64,
    bid_volume_3: f64,
    bid_volume_4: f64,
    bid_volume_5: f64,
    ask_volume_1: f64,
    ask_volume_2: f64,
    ask_volume_3: f64,
    ask_volume_4: f64,
    ask_volume_5: f64,
}

impl Default for TickData {
    fn default() -> TickData {
        TickData {
            symbol: None,
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

#[allow(dead_code)]
pub struct BarData {
    symbol: Option<String>,
    exchange: Option<Exchange>,
    datetime: Option<DateTime<Utc>>,
    interval: Option<Interval>,
    volume: f64,
    open_interest: f64,
    open_price: f64,
    high_price: f64,
    low_price: f64,
    close_price: f64,
}

impl Default for BarData {
    fn default() -> BarData {
        BarData {
            symbol: None,
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

#[allow(dead_code)]
pub struct OrderData {
    symbol: Option<String>,
    exchange: Option<Exchange>,
    datetime: Option<DateTime<Utc>>,
    orderid: Option<String>,
    order_type: OrderType,
    direction: Option<Direction>,
    offset: Offset,
    price: f64,
    volume: f64,
    traded: f64,
    status: Option<Status>,
}

impl Default for OrderData {
    fn default() -> OrderData {
        OrderData {
            symbol: None,
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

#[allow(dead_code)]
pub struct TradeData {
    symbol: Option<String>,
    exchange: Option<Exchange>,
    datetime: Option<DateTime<Utc>>,
    orderid: Option<String>,
    tradeid: Option<String>,
    direction: Option<Direction>,
    offset: Option<Offset>,
    price: f64,
    volume: f64,
}

impl Default for TradeData {
    fn default() -> TradeData {
        TradeData {
            symbol: None,
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

#[allow(dead_code)]
pub struct PositionData {
    symbol: Option<String>,
    exchange: Option<Exchange>,
    direction: Option<Direction>,
    volume: f64,
    frozen: f64,
    price: f64,
    pnl: f64,
    yd_volume: f64,
}

impl Default for PositionData {
    fn default() -> PositionData {
        PositionData {
            symbol: None,
            exchange: None,
            direction: None,
            volume: 0.0,
            frozen: 0.0,
            price: 0.0,
            pnl: 0.0,
            yd_volume: 0.0,
        }
    }
}

#[allow(dead_code)]
pub struct AccountData {
    accountid: String,
    balance: f64,
    frozen: f64,
}

#[allow(dead_code)]
pub struct ContractData {
    symbol: Option<String>,
    exchange: Option<Exchange>,
    name: Option<String>,
    product: Option<Product>,
    size: Option<i32>,
    pricetick: f64,
    min_volume: f64,
    stop_supported: bool,
    net_position: bool,
    history_data: bool,
    option_strike: f64,
    option_underlying: Option<String>,
    option_type: Option<OptionType>,
    option_expiry: Option<DateTime<Utc>>,
    option_portfolio: Option<String>,
    option_index: Option<String>,
}

impl Default for ContractData {
    fn default() -> ContractData {
        ContractData {
            symbol: None,
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

#[allow(dead_code)]
pub struct SubscribeRequest {
    symbol: String,
    exchange: Exchange,
}

#[allow(dead_code)]
pub struct OrderRequest {
    symbol: String,
    exchange: Exchange,
    direction: Direction,
    order_type: OrderType,
    volume: f64,
    price: f64,
    offset: Offset,
    reference: Option<String>,
}

#[allow(dead_code)]
pub struct CancelRequest {
    orderid: String,
    symbol: String,
    exchange: Exchange,
}

#[allow(dead_code)]
pub struct HistoryRequest {
    symbol: String,
    exchange: Exchange,
    start: DateTime<Utc>,
    end: DateTime<Utc>,
    interval: Interval,
}
