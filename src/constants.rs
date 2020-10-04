//! # General Enum
//! Author: Aaron Qiu
#![allow(dead_code)]

use derive_more::Display;

/// Direction of order/trade/position.
#[derive(Clone, Debug, PartialEq, Display)]
pub enum Direction {
    /// 多
    #[display(fmt = "LONG")]
    LONG,
    /// 净
    #[display(fmt = "NET")]
    NET,
    /// 空
    #[display(fmt = "SHORT")]
    SHORT,
}

/// 对冲
#[derive(Clone, Debug, PartialEq, Display)]
pub enum Offset {
    /// 无
    #[display(fmt = "None")]
    NONE,
    /// 平
    #[display(fmt = "CLOSE")]
    CLOSE,
    /// 平今
    #[display(fmt = "CLOSETODAY")]
    CLOSETODAY,
    /// 平昨
    #[display(fmt = "CLOSEYESTERDAY")]
    CLOSEYESTERDAY,
    /// 开
    #[display(fmt = "OPEN")]
    OPEN,
}

// #[derive(Clone, Debug, Display, PartialEq)]
// pub enum Status {
//     #[display(fmt = "Submiting")]
//     SUBMITTING,
//     #[display(fmt = "NotTraded")]
//     NOTTRADED,
//     #[display(fmt = "PartTraded")]
//     PARTTRADED,
//     #[display(fmt = "AllTraded")]
//     ALLTRADED,
//     #[display(fmt = "Cancelled")]
//     CANCELLED,
//     #[display(fmt = "Rejected")]
//     REJECTED,
// }

bitflags! {
    /// 状态
    pub struct Status: u8 {
        const INIT =        0b_0000_0001;
        const SUBMITTING =  0b_0000_0010;
        const NOTTRADED =   0b_0000_0100;
        const PARTTRADED =  0b_0000_1000;
        const ALLTRADED =   0b_0001_0000;
        const CANCELLED =   0b_0010_0000;
        const REJECTED =    0b_0100_0000;
        const ACTIVE_IN = Self::NOTTRADED.bits | Self::SUBMITTING.bits | Self::PARTTRADED.bits;
    }
}

/// 产品
#[derive(Clone, Debug, PartialEq, Display)]
pub enum Product {
    /// 股票
    #[display(fmt = "EQUITY")]
    EQUITY,
    /// 期货
    #[display(fmt = "FUTURES")]
    FUTURES,
    /// 期权
    #[display(fmt = "OPTION")]
    OPTION,
}

/// 订单类型
#[derive(Clone, Debug, PartialEq, Display)]
pub enum OrderType {
    /// 限价
    #[display(fmt = "LIMIT")]
    LIMIT,
    /// 市价
    #[display(fmt = "MARKET")]
    MARKET,
    /// STOP
    #[display(fmt = "STOP")]
    STOP,
    /// FAK
    #[display(fmt = "FAK")]
    FAK,
    /// FOK
    #[display(fmt = "FOK")]
    FOK,
    /// 询价
    #[display(fmt = "RFQ")]
    RFQ,
}

/// 期权类型
#[derive(Clone, Debug, PartialEq, Display)]
pub enum OptionType {
    /// 看涨期权
    #[display(fmt = "CALL")]
    CALL,
    /// 看跌期权
    #[display(fmt = "PUT")]
    PUT,
}

// /// 交易所
// /// 暫時只支持國內期貨交易
// #[derive(Copy, Clone, Debug, PartialEq, Display)]
// pub enum Exchange {
//     // Chinese
//     #[display(fmt = "CFETS")]
//     CFETS,
//     #[display(fmt = "CFFEX")]
//     CFFEX,
//     #[display(fmt = "CZCE")]
//     CZCE,
//     #[display(fmt = "DCE")]
//     DCE,
//     #[display(fmt = "INE")]
//     INE,
//     #[display(fmt = "SGE")]
//     SGE,
//     #[display(fmt = "SHFE")]
//     SHFE,
//     #[display(fmt = "SSE")]
//     SSE,
//     #[display(fmt = "SZSE")]
//     SZSE,
//     #[display(fmt = "WXE")]
//     WXE,
// }

bitflags! {
    /// 交易所
    /// 暫時只支持國內期貨交易
    pub struct Exchange: u16 {
        const CFETS =   0b_0000_0000_0001;
        const CFFEX =   0b_0000_0000_0010;
        const CZCE =    0b_0000_0000_0100;
        const DCE =     0b_0000_0000_1000;
        const INE =     0b_0000_0001_0000;
        const SGE =     0b_0000_0010_0000;
        const SHFE =    0b_0000_0100_0000;
        const SSE =     0b_0000_1000_0000;
        const SZSE =    0b_0001_0000_0000;
        const WXE =     0b_0010_0000_0000;
    }
}
