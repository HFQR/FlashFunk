//! # General Structs
//! Author: Aaron Qiu

use enums::*;
use chrono::DateTime;

pub struct TickData {
    symbol: String,
    exchange: Exchange,
    datetime: DateTime,
    name: String = "",
    volume: f64 = 0.0,
    open_interest: f64 = 0.0,
    last_price: f64 = 0.0,
    last_volume: f64 = 0.0,
    limit_up: f64 = 0.0,
    limit_down: f64 = 0.0,
    open_price: f64 = 0.0,
    high_price: f64 = 0.0,
    low_price: f64 = 0.0,
    pre_close: f64 = 0.0,
    bid_price_1: f64 = 0.0,
    bid_price_2: f64 = 0.0,
    bid_price_3: f64 = 0.0,
    bid_price_4: f64 = 0.0,
    bid_price_5: f64 = 0.0,
    ask_price_1: f64 = 0.0,
    ask_price_2: f64 = 0.0,
    ask_price_3: f64 = 0.0,
    ask_price_4: f64 = 0.0,
    ask_price_5: f64 = 0.0,
    bid_volume_1: f64 = 0.0,
    bid_volume_2: f64 = 0.0,
    bid_volume_3: f64 = 0.0,
    bid_volume_4: f64 = 0.0,
    bid_volume_5: f64 = 0.0,
    ask_volume_1: f64 = 0.0,
    ask_volume_2: f64 = 0.0,
    ask_volume_3: f64 = 0.0,
    ask_volume_4: f64 = 0.0,
    ask_volume_5: f64 = 0.0,
}

pub struct BarData {
    symbol: String,
    exchange: Exchange,
    datetime: DateTime,
    interval: Interval = None,
    volume: f64 = 0.0,
    open_interest: f64 = 0.0,
    open_price: f64 = 0.0,
    high_price: f64 = 0.0,
    low_price: f64 = 0.0,
    close_price: f64 = 0.0,
}

pub struct OrderData {
    symbol: String,
    exchange: Exchange,
    datetime: DateTime,
    orderid: String,
    type: OrderType = OrderType::LIMIT,
    direction: Direction = None,
    offset: Offset = Offset::NONE,
    price: f64 = 0.0,
    volume: f64 = 0.0,
    traded: f64 = 0.0,
    status: Status = Status::SUBMITTING,
}

pub struct TradeData {
    symbol: String,
    exchange: Exchange,
    datetime: DateTime,
    orderid: String,
    tradeid: String,
    direction: Direction = None,
    offset: Offset::None,
    price: f64,
    volume: f64,
}

pub struct PositionData {
    symbol: String,
    exchange: Exchange,
    direction: Direction,
    volume: f64 = 0.0,
    frozen: f64 = 0.0,
    price: f64 = 0.0,
    pnl: f64 = 0.0,
    yd_volume: f64 = 0.0,
}

pub struct AccountData {
    accountid: String,
    balance: f64 = 0.0,
    frozen: f64 = 0.0,
}

pub struct ContractData {
    symbol: String,
    exchange: Exchange,
    name: String,
    product: Product,
    size: i32,
    pricetick: f64,
    min_volume: f64 = 1,
    stop_supported: bool = false,
    net_position: bool = false,
    history_data: bool = false,
    option_strike: f64 = 0,
    option_underlying: String = "",
    option_type: OptionType,
    option_expiry: Datetime,
    option_portfolio: String = "",
    option_index: String = "",
}

pub struct SubscribeRequest {
    symbol: String,
    exchange: Exchange,
}

pub struct OrderRequest {
    symbol: String,
    exchange: Exchange,
    direction: Direction,
    type: OrderType,
    volume: f64,
    price: f64 = 0,
    offset: Offset::None,
    reference: String = "",
}

pub struct CancelRequest {
    orderid: String,
    symbol: String,
    exchange: Exchange,
}

pub struct HistoryRequest {
    symbol: String,
    exchange: Exchange,
    start: Datetime,
    end: Datetime = None,
    interval: Interval = None,
}