//! # General Enum
//! Author: Aaron Qiu

/// Direction of order/trade/position.
#[allow(dead_code)]
pub enum Direction {
    /// 多
    LONG,
    /// 净
    NET,
    /// 空
    SHORT,
}

/// 对冲
#[allow(dead_code)]
pub enum Offset {
    /// 无
    NONE,
    /// 平
    CLOSE,
    /// 平今
    CLOSETODAY,
    /// 平昨
    CLOSEYESTERDAY,
    /// 开
    OPEN,
}

/// 状态
#[allow(dead_code)]
pub enum Status {
    /// 提交中
    SUBMITTING,
    /// 未成交
    NOTTRADED,
    /// 部分成交
    PARTTRADED,
    /// 全部成交
    ALLTRADED,
    /// 已撤销
    CANCELLED,
    /// 拒单
    REJECTED,
}

/// 产品
#[allow(dead_code)]
pub enum Product {
    /// 股票
    EQUITY,
    /// 期货
    FUTURES,
    /// 期权
    OPTION,
    /// 指数
    INDEX,
    /// 外汇
    FOREX,
    /// 现货
    SPOT,
    /// ETF
    ETF,
    /// 债券
    BOND,
    /// 权证
    WARRANT,
    /// 价差
    SPREAD,
    /// 基金
    FUND,
}

/// 订单类型
#[allow(dead_code)]
pub enum OrderType {
    /// 限价
    LIMIT,
    /// 市价
    MARKET,
    /// STOP
    STOP,
    /// FAK
    FAK,
    /// FOK
    FOK,
    /// 询价
    RFQ,
}

/// 期权类型
#[allow(dead_code)]
pub enum OptionType {
    /// 看涨期权
    CALL,
    /// 看跌期权
    PUT,
}

/// 交易所
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

/// 货币
#[allow(dead_code)]
pub enum Currency {
    CNY,
    HKD,
    USD,
}

/// 时间周期
#[allow(dead_code)]
pub enum Interval {
    MINUTE,
    HOUR,
    DAILY,
    WEEKLY,
}
