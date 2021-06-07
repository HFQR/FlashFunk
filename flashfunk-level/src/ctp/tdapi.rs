use core::ffi::c_void;
use core::fmt;

use std::borrow::Cow;
use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int, c_uchar};
use std::process::id;
use std::sync::Arc;
use std::time::Instant;

use chrono::{DateTime, Local, NaiveDate, NaiveDateTime, NaiveTime, Timelike};
// use encoding::all::GB18030;
// use encoding::{DecoderTrap, Encoding};

use crate::ctp::func::QuoteApi;
use crate::ctp::sys::{
    check_slice_to_string, slice_to_string, CThostFtdcDepthMarketDataField,
    CThostFtdcFensUserInfoField, CThostFtdcForQuoteRspField, CThostFtdcMdApi,
    CThostFtdcMdApi_GetTradingDay, CThostFtdcMdApi_Init, CThostFtdcMdApi_RegisterFront,
    CThostFtdcMdApi_RegisterSpi, CThostFtdcMdApi_ReqUserLogin, CThostFtdcMdApi_SubscribeMarketData,
    CThostFtdcMdSpi, CThostFtdcReqUserLoginField, CThostFtdcRspInfoField,
    CThostFtdcRspUserLoginField, CThostFtdcSpecificInstrumentField, CThostFtdcTraderApi,
    CThostFtdcUserLogoutField, DisconnectionReason, QuoteSpi, QuoteSpi_Destructor,
    TThostFtdcErrorIDType, TThostFtdcRequestIDType, ToCSlice,
};
use crate::c_func::parse_datetime_from_str;
use crate::data_type::{CancelRequest, LoginForm, OrderRequest, TickData};
use crate::interface::Interface;
use crate::types::message::MdApiMessage;
use crate::util::blocker::Blocker;
use crate::util::channel::GroupSender;
use crate::{get_interface_path, os_path};
use std::fs::create_dir;
use std::path::PathBuf;
use crate::ctp::CtpMd::CtpMdCApi;
use crate::ctp::CtpTd::CtpTdCApi;

#[allow(non_camel_case_types)]
type c_bool = std::os::raw::c_uchar;

struct MdApiBlocker(Arc<MdApiBlockerInner>);

impl Clone for MdApiBlocker {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl MdApiBlocker {
    fn new() -> Self {
        Self(Arc::new(MdApiBlockerInner {
            step1: Default::default(),
            step2: Default::default(),
        }))
    }
}

struct MdApiBlockerInner {
    step1: Blocker,
    step2: Blocker,
}

pub struct CtpTdApi {}

struct Level {
    sender: GroupSender<MdApiMessage>,
    symbols: Vec<&'static str>,
    blocker: Option<MdApiBlocker>,
    login_form: LoginForm,
    market_pointer: *mut CThostFtdcMdApi,
    request_id: i32,
}

/// 此处我们实现种种方法来构建ctp的登录流程
impl CtpTdCApi for Level {}

unsafe impl Send for CtpTdApi {}

/// Now we get a very useful spi, and we get use the most important things to let everything works well
impl Interface for CtpTdApi {
    type Message = MdApiMessage;

    fn new(
        id: impl Into<Vec<u8>>,
        pwd: impl Into<Vec<u8>>,
        symbols: Vec<&'static str>,
        req: &LoginForm,
        sender: GroupSender<Self::Message>,
    ) -> Self {
        CtpTdApi {}
    }

    fn connect(&mut self) {
        // self.collector.connect();
    }

    fn subscribe(&mut self) {}
}

