//! # General Enum
//! Author: Aaron Qiu

#[allow(dead_code)]
pub enum Direction {
    LONG,
    NET,
    SHORT,
}

#[allow(dead_code)]
pub enum Offset {
    NONE,
    CLOSE,
    CLOSETODAY,
    CLOSEYESTERDAY,
    OPEN,
}

#[allow(dead_code)]
pub enum Status {
    ALLTRADED,
    CANCELLED,
    NOTTRADED,
    PARTTRADED,
    REJECTED,
    SUBMITTING,
}

#[allow(dead_code)]
pub enum Product {
    BOND,
    EQUITY,
    ETF,
    FOREX,
    FUND,
    FUTURES,
    INDEX,
    OPTION,
    SPOT,
    SPREAD,
    WARRANT,
}

#[allow(dead_code)]
pub enum OrderType {
    FAK,
    FOK,
    LIMIT,
    MARKET,
    RFQ,
    STOP,
}

#[allow(dead_code)]
pub enum OptionType {
    CALL,
    PUT,
}

#[allow(dead_code)]
pub enum Exchange {
    // Chinese
    CFETS,
    CFFEX,
    CZCE,
    DCE,
    INE,
    SGE,
    SHFE,
    SSE,
    SZSE,
    WXE,
    // Global
    APEX,
    ARCA,
    BATS,
    BMD,
    CBOE,
    CBOT,
    CFE,
    CME,
    COMEX,
    DME,
    EDGEA,
    EUNX,
    EUREX,
    GLOBEX,
    HKFE,
    HKSE,
    IBKRATS,
    ICE,
    IDEALPRO,
    IEX,
    ISLAND,
    KRX,
    LME,
    NASDAQ,
    NYMEX,
    NYSE,
    OTC,
    SEHK,
    SGX,
    SMART,
    TOCOM,
    // CryptoCurrency
    BINANCE,
    BITFINEX,
    BITMEX,
    BITSTAMP,
    BYBIT,
    COINBASE,
    DERIBIT,
    GATEIO,
    HUOBI,
    LOCAL,
    OKEX,
}

#[allow(dead_code)]
pub enum Currency {
    CNY,
    HKD,
    USD,
}

#[allow(dead_code)]
pub enum Interval {
    MINUTE,
    HOUR,
    DAILY,
    WEEKLY,
}
