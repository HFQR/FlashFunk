//! # General Enum
//! Author: Aaron Qiu

bitflags! {
    /// Direction of order/trade/position.
    pub struct Direction: u8 {
        /// 多
        const LONG =    0;
        /// 净
        const NET =     1;
        /// 空
        const SHORT =   2;
    }
}

bitflags! {
    /// 对冲
    pub struct Offset: u8 {
        /// 无
        const NONE =            0;
        /// 平
        const CLOSE =           1;
        /// 平今
        const CLOSETODAY =      2;
        /// 平昨
        const CLOSEYESTERDAY =  3;
        /// 开
        const OPEN =            4;
    }
}

bitflags! {
    /// 日志信息等级
    pub struct LogLevel: u8 {
        /// 无
        const ERROR =            0;
        /// 平
        const WARNING =           1;
        /// 平今
        const DEBUG =      2;
        /// 平昨
        const INFO =  3;
    }
}

bitflags! {
    /// 状态
    pub struct Status: u8 {
        // 重要: default类型需要为 Status::INIT
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

bitflags! {
    /// 产品
    pub struct Product: u8 {
        /// 股票
        const EQUITY =  0;
        /// 期货
        const FUTURES = 1;
        /// 期权
        const OPTION =  2;
    }
}

bitflags! {
    /// 订单类型
    pub struct OrderType: u8 {
        /// 限价
        const LIMIT =   0;
        /// 市价
        const MARKET =  1;
        /// STOP
        const STOP =    2;
        /// FAK
        const FAK =     3;
        /// FOK
        const FOK =     4;
        /// 询价
        const RFQ =     5;
    }

}

bitflags! {
    /// 期权类型
    pub struct OptionType: u8 {
        /// 看涨期权
        const CALL =    0;
        /// 看跌期权
        const PUT =     1;
    }
}

bitflags! {
    /// 交易所
    /// 暫時只支持國內期貨交易
    pub struct Exchange: u16 {
     // 重要: default类型需要为 Status::INIT
        const INIT =     0b_0000_0001;
        const CFETS =    0b_0000_0010;
        const CFFEX =    0b_0000_0100;
        const CZCE =     0b_0000_1000;
        const DCE =      0b_0001_0000;
        const INE =      0b_0010_0000;
        const SGE =      0b_0100_0000;
        const SHFE =     0b_1000_0000;
        const SSE =      0b_0001_0000_0000;
        const SZSE =     0b_0010_0000_0000;
        const WXE =      0b_0100_0000_0000;
        const ACTIVE_TODAY = Self::SHFE.bits | Self::INE.bits;
    }
}
