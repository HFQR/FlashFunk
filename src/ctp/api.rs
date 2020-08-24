#![allow(dead_code, unused_variables, unused_imports)]


use super::interface::Interface;
use std::ffi::{CStr, CString};
use std::os::raw::{c_void, c_char, c_int};
use crate::ctp::sys::{CThostFtdcMdApi, CThostFtdcTraderApi, CThostFtdcMdSpi, CThostFtdcReqUserLoginField, CThostFtdcUserLogoutField, CThostFtdcFensUserInfoField};


#[allow(dead_code)]
#[link(name = "thostmduserapi_se")]
extern "C" {
    #[link_name = "_ZN15CThostFtdcMdApi15CreateFtdcMdApiEPKcbb"]
    fn CThostFtdcMdApiCreateFtdcMdApi(pszFlowPath: *const c_char, bIsUsingUdp: c_bool, bIsMulticast: c_bool) -> *mut c_void;
    #[link_name = "_ZN15CThostFtdcMdApi13GetApiVersionEv"]
    fn CThostFtdcMdApiGetApiVersion() -> *const c_char;
    #[link_name = "_ZN14CFtdcMdApiImpl7ReleaseEv"]
    fn CFtdcMdApiImplRelease(api: *mut c_void);
    #[link_name = "_ZN14CFtdcMdApiImpl4InitEv"]
    fn CFtdcMdApiImplInit(api: *mut c_void);
    #[link_name = "_ZN14CFtdcMdApiImpl4JoinEv"]
    fn CFtdcMdApiImplJoin(api: *mut c_void) -> c_int;
    #[link_name = "_ZN14CFtdcMdApiImpl13GetTradingDayEv"]
    fn CFtdcMdApiImplGetTradingDay(api: *mut c_void) -> *const c_char;
    #[link_name = "_ZN14CFtdcMdApiImpl13RegisterFrontEPc"]
    fn CFtdcMdApiImplRegisterFront(api: *mut c_void, pszFrontAddress: *const c_char);
    #[link_name = "_ZN14CFtdcMdApiImpl18RegisterNameServerEPc"]
    fn CFtdcMdApiImplRegisterNameServer(api: *mut c_void, pszNsAddress: *const c_char);
    #[link_name = "_ZN14CFtdcMdApiImpl20RegisterFensUserInfoEP27CThostFtdcFensUserInfoField"]
    fn CFtdcMdApiImplRegisterFensUserInfo(api: *mut c_void, pFensUserInfo: *const CThostFtdcFensUserInfoField);
    #[link_name = "_ZN14CFtdcMdApiImpl11RegisterSpiEP15CThostFtdcMdSpi"]
    fn CFtdcMdApiImplRegisterSpi(api: *mut c_void, pSpi: *mut c_void);
    #[link_name = "_ZN14CFtdcMdApiImpl19SubscribeMarketDataEPPci"]
    fn CFtdcMdApiImplSubscribeMarketData(api: *mut c_void, ppInstrumentID: *const *const c_char, nCount: c_int) -> c_int;
    #[link_name = "_ZN14CFtdcMdApiImpl21UnSubscribeMarketDataEPPci"]
    fn CFtdcMdApiImplUnSubscribeMarketData(api: *mut c_void, ppInstrumentID: *const *const c_char, nCount: c_int) -> c_int;
    #[link_name = "_ZN14CFtdcMdApiImpl20SubscribeForQuoteRspEPPci"]
    fn CFtdcMdApiImplSubscribeForQuoteRsp(api: *mut c_void, ppInstrumentID: *const *const c_char, nCount: c_int) -> c_int;
    #[link_name = "_ZN14CFtdcMdApiImpl22UnSubscribeForQuoteRspEPPci"]
    fn CFtdcMdApiImplUnSubscribeForQuoteRsp(api: *mut c_void, ppInstrumentID: *const *const c_char, nCount: c_int) -> c_int;
    #[link_name = "_ZN14CFtdcMdApiImpl12ReqUserLoginEP27CThostFtdcReqUserLoginFieldi"]
    fn CFtdcMdApiImplReqUserLogin(api: *mut c_void, pReqUserLoginField: *const CThostFtdcReqUserLoginField, nRequestID: c_int) -> c_int;
    #[link_name = "_ZN14CFtdcMdApiImpl13ReqUserLogoutEP25CThostFtdcUserLogoutFieldi"]
    fn CFtdcMdApiImplReqUserLogout(api: *mut c_void, pUserLogoutField: *const CThostFtdcUserLogoutField, nRequestID: c_int) -> c_int;
}


/// the implement of market api
pub struct MdApi {
    user_id: CString,
    password: CString,
    path: CString,
    market_api: Option<*mut CThostFtdcMdSpi>,
}

impl MdApi {
    fn new(id: &str, pwd: &str, path: &str) -> MdApi {
        MdApi {
            user_id: CString"".to_string(),
            password: "".to_string(),
            path: "".to_string(),
            market_api: None,
        }
    }

    fn init(&mut self) {
        unsafe { CFtdcMdApiImplInit(self.user_id.clone()) };
    }
}

impl Interface for MdApi {}
