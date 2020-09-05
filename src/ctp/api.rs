#![allow(dead_code, unused_variables, unused_imports)]


use super::interface::Interface;
use std::ffi::{CStr, CString};
use std::os::raw::{c_void, c_char, c_int, c_uchar};
use crate::ctp::sys::{CThostFtdcMdApi, CThostFtdcTraderApi, CThostFtdcMdApi_Init, QuoteSpi, CThostFtdcMdSpi,
                      CThostFtdcMdApi_RegisterFront, CThostFtdcMdApi_SubscribeMarketData,
                      CThostFtdcMdApi_RegisterSpi, QuoteSpi_Destructor,
                      CThostFtdcMdApi_GetTradingDay, CThostFtdcMdApi_ReqUserLogin,
                      CThostFtdcReqUserLoginField, CThostFtdcUserLogoutField, CThostFtdcFensUserInfoField,
                      CThostFtdcSpecificInstrumentField, CThostFtdcRspInfoField, CThostFtdcDepthMarketDataField,
                      CThostFtdcForQuoteRspField, CThostFtdcRspUserLoginField, TThostFtdcRequestIDType,
                      TThostFtdcErrorIDType};
use std::process::id;
use actix::Addr;
use crate::app::CtpbeeR;
use std::fmt;
use std::borrow::{Cow, BorrowMut, Borrow};

use encoding::{DecoderTrap, Encoding};
use encoding::all::GB18030;
use failure::_core::str::Utf8Error;
use crate::structs::{OrderRequest, CancelRequest, LoginForm, TickData};
use crate::ctp::func::QuoteApi;
use failure::_core::cell::RefCell;
use std::rc::Rc;
use core::mem;

#[allow(non_camel_case_types)]
type c_bool = std::os::raw::c_uchar;

#[link(name = "thostmduserapi_se")]
extern "C" {}

pub struct MdApi {
    user_id: CString,
    password: CString,
    path: CString,
    market_api: *mut CThostFtdcMdApi,
    market_spi: Option<*mut QuoteSpi>,
    addr: Addr<CtpbeeR>,
    login_info: Option<LoginForm>,
    request_id: i32,
    this: Option<Rc<RefCell<MdApi>>>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DisconnectionReason {
    ReadError = 0x1001,
    WriteError = 0x1002,
    HeartbeatTimeout = 0x2001,
    HeartbeatSendError = 0x2002,
    ErrorMessageReceived = 0x2003,
    Unknown = 0x0000,
}

impl std::convert::From<c_int> for DisconnectionReason {
    fn from(reason: c_int) -> DisconnectionReason {
        match reason {
            0x1001 => DisconnectionReason::ReadError,
            0x1002 => DisconnectionReason::WriteError,
            0x2001 => DisconnectionReason::HeartbeatTimeout,
            0x2002 => DisconnectionReason::HeartbeatSendError,
            0x2003 => DisconnectionReason::ErrorMessageReceived,
            _ => DisconnectionReason::Unknown,
        }
    }
}

#[must_use]
pub type RspResult = Result<(), RspError>;

#[derive(Clone, Debug, PartialEq)]
pub struct RspError {
    pub id: TThostFtdcErrorIDType,
    pub msg: String,
}

impl std::error::Error for RspError {}

impl fmt::Display for RspError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.id, self.msg)
    }
}


pub fn result_to_string(rsp_result: &RspResult) -> String {
    match rsp_result {
        Ok(()) => "Ok(())".to_string(),
        Err(err) => format!("Err(RspError{{ id: {}, msg: {} }})", err.id, err.msg),
    }
}

pub unsafe fn info_to_result(rsp_info: *const CThostFtdcRspInfoField) -> RspResult {
    #[allow(unused_unsafe)] // for future "unsafe blocks in unsafe fn" feature
    match unsafe { rsp_info.as_ref() } {
        Some(info) => match info {
            CThostFtdcRspInfoField { ErrorID: 0, .. } => {
                Ok(())
            }
            CThostFtdcRspInfoField { ErrorID: id, ErrorMsg: msg } => {
                Err(RspError { id: *id, msg: covert_cstr_to_str(msg).into_owned() })
            }
        },
        None => {
            Ok(())
        }
    }
}

// pub extern fn covert_to_str(to: *const c_char) -> String {
//     let c_str = unsafe { CString::fr(to) };
//     c_str.to_str().unwrap().to_string()
// }
/// todo: 下面有问题描述
pub fn covert_cstr_to_str(v: &[i8]) -> Cow<str> {
    Cow::from("这里有严重的问题， 我不知道怎么把i8的c_char转换为 String")
}


struct DataCollector {
    addr: Addr<CtpbeeR>,
    login_status: bool,
    connect_status: bool,
    api: Rc<RefCell<MdApi>>,
}

/// 此处我们实现种种方法来构建ctp的登录流程
impl QuoteApi for DataCollector {
    fn on_front_connected(&mut self) {
        self.connect_status = true;
        (*self.api).borrow_mut().login();
    }

    fn on_front_disconnected(&mut self, reason: DisconnectionReason) {
        self.login_status = false;
        self.connect_status = false;
    }

    fn on_rsp_user_login(&mut self, pRspUserLogin: *mut CThostFtdcRspUserLoginField, pRspInfo: *mut CThostFtdcRspInfoField, request_id: TThostFtdcRequestIDType, is_last: bool) {
        self.login_status = true;
    }
    fn on_rtn_depth_market_data(&mut self, pDepthMarketData: *mut CThostFtdcDepthMarketDataField) {
        unsafe {
            self.addr.do_send(TickData {
                symbol: "".to_string(),
                exchange: None,
                datetime: None,
                name: None,
                volume: (*pDepthMarketData).Volume as f64,
                open_interest: (*pDepthMarketData).OpenInterest as f64,
                last_price: (*pDepthMarketData).LastPrice as f64,
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
            });
        }
    }

    fn get_addr(&self) -> &Addr<CtpbeeR> {
        self.addr.borrow()
    }
}

unsafe impl Send for MdApi {}

/// Now we get a very useful spi, and we get use the most important things to let everything works well
/// the code is from ctp-rs
///
/// 实现行情API的一些主动基准调用方法
impl MdApi {
    pub fn new(id: String, pwd: String, path: String, addr: Addr<CtpbeeR>) -> Rc<RefCell<MdApi>> {
        let ids = CString::new(id).unwrap();
        let pwds = CString::new(pwd).unwrap();
        let paths = CString::new(path).unwrap();
        let flow_path_ptr = paths.as_ptr();
        // 创建了行情对象
        let api = unsafe { CThostFtdcMdApi::CreateFtdcMdApi(flow_path_ptr, true, true) };
        let x = MdApi {
            user_id: ids,
            password: pwds,
            path: paths,
            market_api: api,
            market_spi: None,
            addr,
            login_info: None,
            request_id: 0,
            this: None,
        };
        let rc = Rc::new(RefCell::new(x));
        (*rc).borrow_mut().this = Some(rc.clone());
        rc
    }
    /// 初始化调用
    pub fn init(&mut self) -> bool {
        unsafe { CThostFtdcMdApi_Init(self.market_api) };
        true
    }
    /// 获取交易日
    pub fn get_trading_day<'a>(&mut self) -> &'a str {
        let trading_day_cstr = unsafe { CThostFtdcMdApi_GetTradingDay(self.market_api) };
        unsafe { CStr::from_ptr(trading_day_cstr as *const i8).to_str().unwrap() }
    }

    fn login(&mut self) {
        let login_form = self.login_info.take().unwrap();
        let user_id = CString::new(login_form.user_id).unwrap();
        let pwd = CString::new(login_form.password).unwrap();
        let broker_id = CString::new(login_form.broke_id).unwrap();
        let auth_code = CString::new(login_form.auth_code).unwrap();
        let app_id = CString::new(login_form.app_id.clone()).unwrap();
        let production_info = CString::new(login_form.production_info.clone()).unwrap();
        unsafe { CThostFtdcMdApi_ReqUserLogin(self.market_api, CThostFtdcReqUserLoginField::default().borrow_mut(), self.request_id.clone()) };
    }

    /// 注册前置地址
    fn register_fronted_address(&mut self, register_addr: CString) {
        let front_socket_address_ptr = register_addr.into_raw();
        unsafe { CThostFtdcMdApi_RegisterFront(self.market_api, front_socket_address_ptr) };
    }

    /// 注册回调
    fn register_spi(&mut self, login_info: LoginForm) {
        self.login_info = Some(login_info);
        let collector = DataCollector { addr: self.addr.clone(), login_status: false, connect_status: false, api: self.this.take().unwrap().clone() };
        // rust object
        let trait_object_box: Box<Box<dyn QuoteApi>> = Box::new(Box::new(collector));
        let trait_object_pointer =
            Box::into_raw(trait_object_box) as *mut Box<dyn QuoteApi> as *mut c_void;
        // 把 rust对象 传给回调SPI
        let quote_spi = unsafe { QuoteSpi::new(trait_object_pointer) };
        let ptr = Box::into_raw(Box::new(quote_spi));
        self.market_spi = Some(ptr);
        unsafe { CThostFtdcMdApi_RegisterSpi(self.market_api, ptr as *mut CThostFtdcMdSpi) };
    }

    fn release(&mut self) {
        println!("正在释放接口")
    }
}

impl Interface for MdApi {
    fn send_order(&mut self, order: OrderRequest) -> String {
        unimplemented!("行情接口无此功能")
    }

    fn cancel_order(&mut self, req: CancelRequest) {
        unimplemented!("行情接口无此功能")
    }

    fn connect(&mut self, req: &LoginForm) {
        let address = (&req.md_address).to_string();
        self.register_spi(req.clone());
        let addr = CString::new(address).unwrap();
        self.register_fronted_address(addr);
        self.init();
        println!("初始化成功");
    }

    fn subscribe(&mut self, symbol: String) {
        let code = CString::new(symbol).unwrap();
        let mut c = code.into_raw();
        unsafe { CThostFtdcMdApi_SubscribeMarketData(self.market_api, c.borrow_mut(), 1) };
    }

    fn unsubscribe(&mut self, symbol: String) {
        unimplemented!()
    }

    fn exit(&mut self) {
        unimplemented!()
    }

    fn get_app(&mut self) -> &Addr<CtpbeeR> {
        self.addr.borrow()
    }
}

impl Drop for MdApi {
    fn drop(&mut self) {
        self.release();
        if let Some(spi_stub) = self.market_spi {
            unsafe { QuoteSpi_Destructor(spi_stub) };
        }
    }
}