#![allow(dead_code, unused_variables, unused_imports)]


use super::interface::Interface;
use std::ffi::{CStr, CString};
use std::os::raw::{c_void, c_char, c_int};
use crate::ctp::sys::{CThostFtdcMdApi, CThostFtdcTraderApi, CThostFtdcReqUserLoginField, CThostFtdcUserLogoutField, CThostFtdcFensUserInfoField, CThostFtdcSpecificInstrumentField, CThostFtdcRspInfoField, CThostFtdcDepthMarketDataField, CThostFtdcForQuoteRspField, CThostFtdcRspUserLoginField, TThostFtdcRequestIDType, TThostFtdcErrorIDType};
use std::process::id;
use actix::Addr;
use crate::app::CtpbeeR;
use std::fmt;
use std::borrow::Cow;

use encoding::{DecoderTrap, Encoding};
use encoding::all::GB18030;
use failure::_core::str::Utf8Error;

#[link(name = "thostmduserapi_se")]
extern "C" {
    /// 行情API 初始化
    #[link_name = "_ZN15CThostFtdcMdApi15CreateFtdcMdApiEPKcbb"]
    fn CThostFtdcMdApiCreateFtdcMdApi(pszFlowPath: *const c_char, bIsUsingUdp: bool, bIsMulticast: bool) -> *mut c_void;
    /// 获取API版本信息
    #[link_name = "_ZN15CThostFtdcMdApi13GetApiVersionEv"]
    fn CThostFtdcMdApiGetApiVersion() -> *const c_char;
    /// 释放API
    #[link_name = "_ZN14CFtdcMdApiImpl7ReleaseEv"]
    fn CFtdcMdApiImplRelease(api: *mut c_void);
    /// 行情API初始化
    #[link_name = "_ZN14CFtdcMdApiImpl4InitEv"]
    fn CFtdcMdApiImplInit(api: *mut c_void);
    /// 阻塞等待结束
    #[link_name = "_ZN14CFtdcMdApiImpl4JoinEv"]
    fn CFtdcMdApiImplJoin(api: *mut c_void) -> c_int;
    /// 获取交易日
    #[link_name = "_ZN14CFtdcMdApiImpl13GetTradingDayEv"]
    fn CFtdcMdApiImplGetTradingDay(api: *mut c_void) -> *const c_char;
    /// 注册前置地址 接受api和前置地址
    #[link_name = "_ZN14CFtdcMdApiImpl13RegisterFrontEPc"]
    fn CFtdcMdApiImplRegisterFront(api: *mut c_void, pszFrontAddress: *const c_char);
    /// 注册到服务器
    #[link_name = "_ZN14CFtdcMdApiImpl18RegisterNameServerEPc"]
    fn CFtdcMdApiImplRegisterNameServer(api: *mut c_void, pszNsAddress: *const c_char);
    /// 注册用户信息
    #[link_name = "_ZN14CFtdcMdApiImpl20RegisterFensUserInfoEP27CThostFtdcFensUserInfoField"]
    fn CFtdcMdApiImplRegisterFensUserInfo(api: *mut c_void, pFensUserInfo: *const CThostFtdcFensUserInfoField);
    /// 注册回调信息
    #[link_name = "_ZN14CFtdcMdApiImpl11RegisterSpiEP15CThostFtdcMdSpi"]
    fn CFtdcMdApiImplRegisterSpi(api: *mut c_void, pSpi: *mut c_void);
    #[link_name = "_ZN14CFtdcMdApiImpl19SubscribeMarketDataEPPci"]
    /// 订阅深度行情API
    fn CFtdcMdApiImplSubscribeMarketData(api: *mut c_void, ppInstrumentID: *const *const c_char, nCount: c_int) -> c_int;
    /// 去掉订阅深度行情API
    #[link_name = "_ZN14CFtdcMdApiImpl21UnSubscribeMarketDataEPPci"]
    fn CFtdcMdApiImplUnSubscribeMarketData(api: *mut c_void, ppInstrumentID: *const *const c_char, nCount: c_int) -> c_int;
    #[link_name = "_ZN14CFtdcMdApiImpl20SubscribeForQuoteRspEPPci"]
    /// 订阅行情回报
    fn CFtdcMdApiImplSubscribeForQuoteRsp(api: *mut c_void, ppInstrumentID: *const *const c_char, nCount: c_int) -> c_int;
    /// 取消订阅行情回报
    #[link_name = "_ZN14CFtdcMdApiImpl22UnSubscribeForQuoteRspEPPci"]
    fn CFtdcMdApiImplUnSubscribeForQuoteRsp(api: *mut c_void, ppInstrumentID: *const *const c_char, nCount: c_int) -> c_int;
    /// 用户登录请求
    #[link_name = "_ZN14CFtdcMdApiImpl12ReqUserLoginEP27CThostFtdcReqUserLoginFieldi"]
    fn CFtdcMdApiImplReqUserLogin(api: *mut c_void, pReqUserLoginField: *const CThostFtdcReqUserLoginField, nRequestID: c_int) -> c_int;
    /// 用户退出登录请求
    #[link_name = "_ZN14CFtdcMdApiImpl13ReqUserLogoutEP25CThostFtdcUserLogoutFieldi"]
    fn CFtdcMdApiImplReqUserLogout(api: *mut c_void, pUserLogoutField: *const CThostFtdcUserLogoutField, nRequestID: c_int) -> c_int;
}

/// the implement of market api
/// user_id 用户名
/// password 密码
/// path 流文件存放地址
/// market_api 行情API 收
/// market_spi 行情API 回调
pub struct MdApi {
    user_id: CString,
    password: CString,
    path: CString,
    market_api: *mut c_void,
    market_spi: Option<*mut CThostFtdcMdSpi>,
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

// unsafe fn unwrap_quote_spi<'a>(spi: *mut c_void) -> &'a mut dyn QuoteSpi {
//     &mut **(spi as *mut *mut dyn QuoteSpi)
// }
/// 行情回调API 应该对API 实现下面所有办法， 然后将回调SPI 注入到MdApi里面去
///
pub trait QuoteApi: Send {
    fn on_front_connected(&mut self) {
        println!("on_front_connected");
    }

    fn on_front_disconnected(&mut self, reason: DisconnectionReason) {
        println!("on_front_disconnected: {:?}", reason);
    }

    #[allow(unused_variables)]
    fn on_rsp_user_login(&mut self, rsp_user_login: Option<&CThostFtdcRspUserLoginField>, result: RspResult, request_id: TThostFtdcRequestIDType, is_last: bool) {
        println!("用户登录 回调 ")
        // println!("on_rsp_user_login: {:?}, {}, {:?}, {:?}", rsp_user_login, result_to_string(&result), request_id, is_last);
    }

    #[allow(unused_variables)]
    fn on_rsp_user_logout(&mut self, rsp_user_logout: Option<&CThostFtdcUserLogoutField>, result: RspResult, request_id: TThostFtdcRequestIDType, is_last: bool) {
        println!("用户登出 回调")
        // println!("on_rsp_user_logout: {:?}, {}, {:?}, {:?}", rsp_user_logout, result_to_string(&result), request_id, is_last);
    }

    #[allow(unused_variables)]
    fn on_rsp_error(&mut self, result: RspResult, request_id: TThostFtdcRequestIDType, is_last: bool) {
        println!("on_rsp_error: {}, {:?}, {:?}", result_to_string(&result), request_id, is_last);
    }

    #[allow(unused_variables)]
    fn on_rsp_sub_market_data(&mut self, specific_instrument: Option<&CThostFtdcSpecificInstrumentField>, result: RspResult, request_id: TThostFtdcRequestIDType, is_last: bool) {
        println!("on_rsp_sub_market_data: {:?}, {}, {:?}, {:?}", specific_instrument, result_to_string(&result), request_id, is_last);
    }

    #[allow(unused_variables)]
    fn on_rsp_un_sub_market_data(&mut self, specific_instrument: Option<&CThostFtdcSpecificInstrumentField>, result: RspResult, request_id: TThostFtdcRequestIDType, is_last: bool) {
        println!("on_rsp_un_sub_market_data: {:?}, {}, {:?}, {:?}", specific_instrument, result_to_string(&result), request_id, is_last);
    }

    #[allow(unused_variables)]
    fn on_rsp_sub_for_quote_rsp(&mut self, specific_instrument: Option<&CThostFtdcSpecificInstrumentField>, result: RspResult, request_id: TThostFtdcRequestIDType, is_last: bool) {
        println!("on_rsp_sub_for_quote_rsp: {:?}, {}, {:?}, {:?}", specific_instrument, result_to_string(&result), request_id, is_last);
    }

    #[allow(unused_variables)]
    fn on_rsp_un_sub_for_quote_rsp(&mut self, specific_instrument: Option<&CThostFtdcSpecificInstrumentField>, result: RspResult, request_id: TThostFtdcRequestIDType, is_last: bool) {
        println!("on_rsp_un_sub_for_quote_rsp: {:?}, {}, {:?}, {:?}", specific_instrument, result_to_string(&result), request_id, is_last);
    }

    #[allow(unused_variables)]
    fn on_rtn_depth_market_data(&mut self, depth_market_data: Option<&CThostFtdcDepthMarketDataField>) {
        println!("on_rtn_depth_market_data: {:?}", depth_market_data);
    }

    #[allow(unused_variables)]
    fn on_rtn_for_quote_rsp(&mut self, for_quote_rsp: Option<&CThostFtdcForQuoteRspField>) {
        println!("on_rtn_for_quote_rsp: {:?}", for_quote_rsp);
    }
    fn get_addr(&self) -> &Addr<CtpbeeR>;
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

#[repr(C)]
pub struct CThostFtdcMdSpi {
    vtable: *const SpiVTable,
    pub spi: *mut dyn QuoteApi,
    addr: Addr<CtpbeeR>,
}

fn create_spi(md_spi: *mut dyn QuoteApi, addr: Addr<CtpbeeR>) -> CThostFtdcMdSpi {
    CThostFtdcMdSpi { vtable: &SPI_VTABLE, spi: md_spi, addr: addr }
}

#[allow(non_snake_case)]
extern "C" fn spi_on_front_connected(spi: *mut CThostFtdcMdSpi) {
    unsafe { (*(*spi).spi).on_front_connected() };
}

#[allow(non_snake_case)]
extern "C" fn spi_on_front_disconnected(spi: *mut CThostFtdcMdSpi, nReason: c_int) {
    let reason = std::convert::From::from(nReason);
    unsafe { (*(*spi).spi).on_front_disconnected(reason) };
}

#[allow(non_snake_case, unused_variables)]
extern "C" fn spi_on_heart_beat_warning(spi: *mut CThostFtdcMdSpi, nTimeLapse: c_int) {
    // CTP API specification shows this will never be called
    unreachable!();
}

#[allow(non_snake_case)]
extern "C" fn spi_on_rsp_user_login(spi: *mut CThostFtdcMdSpi, pRspUserLogin: *const CThostFtdcRspUserLoginField, pRspInfo: *const CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: bool) {
    unsafe {
        let rsp_info = info_to_result(pRspInfo);
        (*(*spi).spi).on_rsp_user_login(pRspUserLogin.as_ref(), rsp_info, nRequestID, bIsLast);
    }
}

#[allow(non_snake_case)]
extern "C" fn spi_on_rsp_user_logout(spi: *mut CThostFtdcMdSpi, pUserLogout: *const CThostFtdcUserLogoutField, pRspInfo: *const CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: bool) {
    unsafe {
        let rsp_info = info_to_result(pRspInfo);
        (*(*spi).spi).on_rsp_user_logout(pUserLogout.as_ref(), rsp_info, nRequestID, bIsLast);
    }
}

#[allow(non_snake_case)]
extern "C" fn spi_on_rsp_error(spi: *mut CThostFtdcMdSpi, pRspInfo: *const CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: bool) {
    unsafe {
        let rsp_info = info_to_result(pRspInfo);
        (*(*spi).spi).on_rsp_error(rsp_info, nRequestID, bIsLast);
    }
}

#[allow(non_snake_case)]
extern "C" fn spi_on_rsp_sub_market_data(spi: *mut CThostFtdcMdSpi, pSpecificInstrument: *const CThostFtdcSpecificInstrumentField, pRspInfo: *const CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: bool) {
    unsafe {
        let rsp_info = info_to_result(pRspInfo);
        (*(*spi).spi).on_rsp_sub_market_data(pSpecificInstrument.as_ref(), rsp_info, nRequestID, bIsLast);
    }
}

#[allow(non_snake_case)]
extern "C" fn spi_on_rsp_un_sub_market_data(spi: *mut CThostFtdcMdSpi, pSpecificInstrument: *const CThostFtdcSpecificInstrumentField, pRspInfo: *const CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: bool) {
    unsafe {
        let rsp_info = info_to_result(pRspInfo);
        (*(*spi).spi).on_rsp_un_sub_market_data(pSpecificInstrument.as_ref(), rsp_info, nRequestID, bIsLast);
    }
}

#[allow(non_snake_case)]
extern "C" fn spi_on_rsp_sub_for_quote_rsp(spi: *mut CThostFtdcMdSpi, pSpecificInstrument: *const CThostFtdcSpecificInstrumentField, pRspInfo: *const CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: bool) {
    unsafe {
        let rsp_info = info_to_result(pRspInfo);
        (*(*spi).spi).on_rsp_sub_for_quote_rsp(pSpecificInstrument.as_ref(), rsp_info, nRequestID, bIsLast);
    }
}

#[allow(non_snake_case)]
extern "C" fn spi_on_rsp_un_sub_for_quote_rsp(spi: *mut CThostFtdcMdSpi, pSpecificInstrument: *const CThostFtdcSpecificInstrumentField, pRspInfo: *const CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: bool) {
    unsafe {
        let rsp_info = info_to_result(pRspInfo);
        (*(*spi).spi).on_rsp_un_sub_for_quote_rsp(pSpecificInstrument.as_ref(), rsp_info, nRequestID, bIsLast);
    }
}

#[allow(non_snake_case)]
extern "C" fn spi_on_rtn_depth_market_data(spi: *mut CThostFtdcMdSpi, pDepthMarketData: *const CThostFtdcDepthMarketDataField) {
    unsafe { (*(*spi).spi).on_rtn_depth_market_data(pDepthMarketData.as_ref()) };
}

#[allow(non_snake_case)]
extern "C" fn spi_on_rtn_for_quote_rsp(spi: *mut CThostFtdcMdSpi, pForQuoteRsp: *const CThostFtdcForQuoteRspField) {
    unsafe { (*(*spi).spi).on_rtn_for_quote_rsp(pForQuoteRsp.as_ref()) };
}

/// 这里是底层关系映射表
/// 我个人猜测这里是ctp的Spi是一个流失关系表,所以在此我们需要把自己写的函数传进去，
/// 此处类型声明我尚不清楚, 但是应该可以按照同样规则实现
#[repr(C)]
#[derive(Debug)]
struct SpiVTable {
    /// 前置连接回调
    on_front_connected: extern "C" fn(spi: *mut CThostFtdcMdSpi),
    /// 前置断开连接回调
    on_front_disconnected: extern "C" fn(spi: *mut CThostFtdcMdSpi, nReason: c_int),
    /// 心跳警告
    on_heart_beat_warning: extern "C" fn(spi: *mut CThostFtdcMdSpi, nTimeLapse: c_int),
    /// 用户登录回调
    on_rsp_user_login: extern "C" fn(spi: *mut CThostFtdcMdSpi, pRspUserLogin: *const CThostFtdcRspUserLoginField, pRspInfo: *const CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: bool),
    /// 用户登出回调
    on_rsp_user_logout: extern "C" fn(spi: *mut CThostFtdcMdSpi, pUserLogout: *const CThostFtdcUserLogoutField, pRspInfo: *const CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: bool),
    /// 错误回调
    on_rsp_error: extern "C" fn(spi: *mut CThostFtdcMdSpi, pRspInfo: *const CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: bool),
    /// 订阅深度行情回调
    on_rsp_sub_market_data: extern "C" fn(spi: *mut CThostFtdcMdSpi, pSpecificInstrument: *const CThostFtdcSpecificInstrumentField, pRspInfo: *const CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: bool),
    /// 取消订阅深度行情回调
    on_rsp_un_sub_market_data: extern "C" fn(spi: *mut CThostFtdcMdSpi, pSpecificInstrument: *const CThostFtdcSpecificInstrumentField, pRspInfo: *const CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: bool),
    ///
    on_rsp_sub_for_quote_rsp: extern "C" fn(spi: *mut CThostFtdcMdSpi, pSpecificInstrument: *const CThostFtdcSpecificInstrumentField, pRspInfo: *const CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: bool),
    /// 忘记了
    on_rsp_un_sub_for_quote_rsp: extern "C" fn(spi: *mut CThostFtdcMdSpi, pSpecificInstrument: *const CThostFtdcSpecificInstrumentField, pRspInfo: *const CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: bool),
    /// 深度行情回调
    on_rtn_depth_market_data: extern "C" fn(spi: *mut CThostFtdcMdSpi, pDepthMarketData: *const CThostFtdcDepthMarketDataField),
    on_rtn_for_quote_rsp: extern "C" fn(spi: *mut CThostFtdcMdSpi, pForQuoteRsp: *const CThostFtdcForQuoteRspField),
}

static SPI_VTABLE: SpiVTable = SpiVTable {
    on_front_connected: spi_on_front_connected,
    on_front_disconnected: spi_on_front_disconnected,
    on_heart_beat_warning: spi_on_heart_beat_warning,
    on_rsp_user_login: spi_on_rsp_user_login,
    on_rsp_user_logout: spi_on_rsp_user_logout,
    on_rsp_error: spi_on_rsp_error,
    on_rsp_sub_market_data: spi_on_rsp_sub_market_data,
    on_rsp_un_sub_market_data: spi_on_rsp_un_sub_market_data,
    on_rsp_sub_for_quote_rsp: spi_on_rsp_sub_for_quote_rsp,
    on_rsp_un_sub_for_quote_rsp: spi_on_rsp_un_sub_for_quote_rsp,
    on_rtn_depth_market_data: spi_on_rtn_depth_market_data,
    on_rtn_for_quote_rsp: spi_on_rtn_for_quote_rsp,
};


/// Now we get a very useful spi, and we get use the most important things to let everything works well
/// the code is from ctp-rs
///
/// 实现行情API的一些主动基准调用方法
impl MdApi {
    fn new(id: CString, pwd: CString, path: CString) -> MdApi {
        let flow_path_ptr = path.clone().into_raw();
        let api = unsafe { CThostFtdcMdApiCreateFtdcMdApi(flow_path_ptr, true, true) };
        MdApi {
            user_id: id,
            password: pwd,
            path,
            market_api: api,
            market_spi: None,
        }
    }
    /// 初始化调用
    fn init(&mut self) -> bool {
        unsafe { CFtdcMdApiImplInit(self.market_api) };
        true
    }
    /// 获取交易日
    fn get_trading_day<'a>(&mut self) -> &'a CStr {
        let trading_day_cstr = unsafe { CFtdcMdApiImplGetTradingDay(self.market_api) };
        unsafe { CStr::from_ptr(trading_day_cstr) }
    }

    /// 注册前置地址
    fn register_fronted_address(&mut self, register_addr: CString) {
        let front_socket_address_ptr = register_addr.into_raw();
        unsafe { CFtdcMdApiImplRegisterFront(self.market_api, front_socket_address_ptr) };
    }

    /// 注册回调
    fn register_spi(&mut self, quo_api: Box<dyn QuoteApi>, addr: Addr<CtpbeeR>) {
        //解开引用
        let last_registered_spi_ptr = self.market_spi.take();
        // 获取回调操作结构体
        let md_spi_ptr = Box::into_raw(quo_api);
        // 创建我们需要的回调结构体
        let spi_ptr = Box::into_raw(Box::new(create_spi(md_spi_ptr, addr)));
        unsafe { CFtdcMdApiImplRegisterSpi(self.market_api, spi_ptr as *mut c_void) };
        // 更新到本地的结构体,注册成功
        self.market_spi = Some(spi_ptr);
        // 暂时不清楚作用 先注释
        // if let Some(last_registered_spi_ptr) = last_registered_spi_ptr {
        //     unsafe {
        //         let last_registered_spi = Box::from_raw(last_registered_spi_ptr);
        //         let md_spi = Box::from_raw(last_registered_spi.md_spi_ptr);
        //         drop(md_spi);
        //         drop(last_registered_spi);
        //     }
        // };
    }
}