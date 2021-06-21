use flashfunk_level::c_func::parse_datetime_from_str;
use flashfunk_level::ctp::sys::ToCSlice;
use flashfunk_level::data_type::TickData;
use chrono::NaiveDateTime;
use core::fmt;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::borrow::Cow;
use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int, c_uchar, c_void};
use std::time::Instant;

pub type TThostFtdcDateType = [c_char; 9usize];
pub type TThostFtdcInstrumentIDType = [c_char; 31usize];
pub type TThostFtdcExchangeIDType = [::std::os::raw::c_char; 9usize];
pub type TThostFtdcExchangeInstIDType = [::std::os::raw::c_char; 31usize];
pub type TThostFtdcPriceType = f64;
pub type TThostFtdcLargeVolumeType = f64;
pub type TThostFtdcMoneyType = f64;
pub type TThostFtdcMillisecType = ::std::os::raw::c_int;
pub type TThostFtdcVolumeType = ::std::os::raw::c_int;
pub type TThostFtdcRatioType = f64;
pub type TThostFtdcTimeType = [::std::os::raw::c_char; 9usize];

#[derive(Debug, Default, Copy, Clone)]
pub struct CThostFtdcDepthMarketDataField {
    pub TradingDay: TThostFtdcDateType,
    pub InstrumentID: TThostFtdcInstrumentIDType,
    pub ExchangeID: TThostFtdcExchangeIDType,
    pub ExchangeInstID: TThostFtdcExchangeInstIDType,
    pub LastPrice: TThostFtdcPriceType,
    pub PreSettlementPrice: TThostFtdcPriceType,
    pub PreClosePrice: TThostFtdcPriceType,
    pub PreOpenInterest: TThostFtdcLargeVolumeType,
    pub OpenPrice: TThostFtdcPriceType,
    pub HighestPrice: TThostFtdcPriceType,
    pub LowestPrice: TThostFtdcPriceType,
    pub Volume: TThostFtdcVolumeType,
    pub Turnover: TThostFtdcMoneyType,
    pub OpenInterest: TThostFtdcLargeVolumeType,
    pub ClosePrice: TThostFtdcPriceType,
    pub SettlementPrice: TThostFtdcPriceType,
    pub UpperLimitPrice: TThostFtdcPriceType,
    pub LowerLimitPrice: TThostFtdcPriceType,
    pub PreDelta: TThostFtdcRatioType,
    pub CurrDelta: TThostFtdcRatioType,
    pub UpdateTime: TThostFtdcTimeType,
    pub UpdateMillisec: TThostFtdcMillisecType,
    pub BidPrice1: TThostFtdcPriceType,
    pub BidVolume1: TThostFtdcVolumeType,
    pub AskPrice1: TThostFtdcPriceType,
    pub AskVolume1: TThostFtdcVolumeType,
    pub BidPrice2: TThostFtdcPriceType,
    pub BidVolume2: TThostFtdcVolumeType,
    pub AskPrice2: TThostFtdcPriceType,
    pub AskVolume2: TThostFtdcVolumeType,
    pub BidPrice3: TThostFtdcPriceType,
    pub BidVolume3: TThostFtdcVolumeType,
    pub AskPrice3: TThostFtdcPriceType,
    pub AskVolume3: TThostFtdcVolumeType,
    pub BidPrice4: TThostFtdcPriceType,
    pub BidVolume4: TThostFtdcVolumeType,
    pub AskPrice4: TThostFtdcPriceType,
    pub AskVolume4: TThostFtdcVolumeType,
    pub BidPrice5: TThostFtdcPriceType,
    pub BidVolume5: TThostFtdcVolumeType,
    pub AskPrice5: TThostFtdcPriceType,
    pub AskVolume5: TThostFtdcVolumeType,
    pub AveragePrice: TThostFtdcPriceType,
    pub ActionDay: TThostFtdcDateType,
}

fn generate_volume() -> c_int {
    300 as c_int
}

// we need to build the Entity
impl CThostFtdcDepthMarketDataField {
    pub fn new() -> Self {
        CThostFtdcDepthMarketDataField {
            TradingDay: "21060325".to_c_slice(),
            InstrumentID: "rb2110".to_c_slice(),
            ExchangeID: "SHFE".to_c_slice(),
            ExchangeInstID: "SHFE".to_c_slice(),
            LastPrice: 3500.0,
            PreSettlementPrice: 3510.0,
            PreClosePrice: 3505.0,
            PreOpenInterest: 56525.0,
            OpenPrice: 3530.0,
            HighestPrice: 3400.0,
            LowestPrice: 3500.0,
            Volume: generate_volume(),
            Turnover: 541541651.0,
            OpenInterest: 120000.0,
            ClosePrice: 3522.0,
            SettlementPrice: 0.0,
            UpperLimitPrice: 3600.0,
            LowerLimitPrice: 3000.0,
            PreDelta: 0.0,
            CurrDelta: 0.0,
            UpdateTime: "09:12:15".to_c_slice(),
            UpdateMillisec: 0,
            BidPrice1: 3500.0,
            BidVolume1: generate_volume(),
            AskPrice1: 3502.0,
            AskVolume1: generate_volume(),
            BidPrice2: 0.0,
            BidVolume2: generate_volume(),
            AskPrice2: 0.0,
            AskVolume2: generate_volume(),
            BidPrice3: 0.0,
            BidVolume3: generate_volume(),
            AskPrice3: 0.0,
            AskVolume3: generate_volume(),
            BidPrice4: 0.0,
            BidVolume4: generate_volume(),
            AskPrice4: 0.0,
            AskVolume4: generate_volume(),
            BidPrice5: 0.0,
            BidVolume5: generate_volume(),
            AskPrice5: 0.0,
            AskVolume5: generate_volume(),
            AveragePrice: 0.0,
            ActionDay: "21060325".to_c_slice(),
        }
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    let x = CThostFtdcDepthMarketDataField::new();
    let v = Box::into_raw(Box::new(x)) as *mut CThostFtdcDepthMarketDataField;
    c.bench_function("c tick data", |b| b.iter(|| convert_to(v.clone())));
}

#[doc(hidden)]
// 主要耗费延时函数 CThostFtdcDepthMarketDataField
fn convert_to(field: *mut CThostFtdcDepthMarketDataField) -> TickData {
    unsafe {
        let instant = Instant::now();
        let depth = *field;
        let v = depth.InstrumentID.as_ptr();
        let symbol = CStr::from_ptr(v).to_str().unwrap();
        let symbol = symbol.to_owned();
        let (date, time) = parse_datetime_from_str(
            depth.ActionDay.as_ptr(),
            depth.UpdateTime.as_ptr(),
            depth.UpdateMillisec,
        );

        TickData {
            symbol,
            datetime: NaiveDateTime::new(date, time),
            volume: depth.Volume,
            open_interest: depth.OpenInterest,
            last_price: depth.LastPrice,
            limit_up: depth.UpperLimitPrice,
            limit_down: depth.LowerLimitPrice,
            open_price: depth.OpenPrice,
            high_price: depth.HighestPrice,
            low_price: depth.LowestPrice,
            pre_close: depth.PreClosePrice,
            bid_price: [
                depth.BidPrice1,
                depth.BidPrice2,
                depth.BidPrice3,
                depth.BidPrice4,
                depth.BidPrice5,
            ],
            ask_price: [
                depth.AskPrice1,
                depth.AskPrice2,
                depth.AskPrice3,
                depth.AskPrice4,
                depth.AskPrice5,
            ],
            bid_volume: [
                depth.BidVolume1,
                depth.BidVolume2,
                depth.BidVolume3,
                depth.BidVolume4,
                depth.BidVolume5,
            ],
            ask_volume: [
                depth.AskVolume1,
                depth.AskVolume2,
                depth.AskVolume3,
                depth.AskVolume4,
                depth.AskVolume5,
            ],
            instant,
            ..TickData::default()
        }
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);