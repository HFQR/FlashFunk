//! # General Enum
//! Author: Aaron Qiu

pub enum Direction {
    LONG,
    NET,
    SHORT,
}

pub enum Offset {
    CLOSE,
    CLOSETODAY,
    CLOSEYESTERDAY,
    NONE,
    OPEN,
}

pub enum Status {
    ALLTRADED,
    CANCELLED,
    NOTTRADED,
    PARTTRADED,
    REJECTED,
    SUBMITTING,
}

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

pub enum OrderType {
    FAK,
    FOK,
    LIMIT,
    MARKET,
    RFQ,
    STOP,
}

pub enum OptionType {
    CALL,
    PUT,
}

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

pub enum Currency {
    CNY,
    HKD,
    USD,
}

pub enum Interval {
    MINUTE,
    HOUR,
    DAILY,
    WEEKLY,
}
