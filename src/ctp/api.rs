#![allow(dead_code, unused_variables, unused_imports)]


use super::interface::Interface;
use std::ffi::{CStr, CString};
use std::os::raw::{c_void, c_char, c_int};
use crate::ctp::sys::{CThostFtdcMdApi, CThostFtdcTraderApi, CThostFtdcMdSpi, CThostFtdcReqUserLoginField, CThostFtdcUserLogoutField, CThostFtdcFensUserInfoField};
use std::process::id;

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

    fn init(&mut self) -> bool {
        unsafe { CFtdcMdApiImplInit(self.market_api) };
        true
    }
}