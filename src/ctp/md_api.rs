use core::ffi::c_void;
use core::fmt;

use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int, c_uchar};
use std::process::id;

use chrono::{DateTime, Local, NaiveDateTime};
use encoding::all::GB18030;
use encoding::{DecoderTrap, Encoding};

use super::interface::Interface;
use crate::app::{CtpbeeR, MdApiMessage, ProducerMdApi};
use crate::ctp::func::QuoteApi;
use crate::ctp::sys::{
    slice_to_string, CThostFtdcDepthMarketDataField, CThostFtdcFensUserInfoField,
    CThostFtdcForQuoteRspField, CThostFtdcMdApi, CThostFtdcMdApi_GetTradingDay,
    CThostFtdcMdApi_Init, CThostFtdcMdApi_RegisterFront, CThostFtdcMdApi_RegisterSpi,
    CThostFtdcMdApi_ReqUserLogin, CThostFtdcMdApi_SubscribeMarketData, CThostFtdcMdSpi,
    CThostFtdcReqUserLoginField, CThostFtdcRspInfoField, CThostFtdcRspUserLoginField,
    CThostFtdcSpecificInstrumentField, CThostFtdcTraderApi, CThostFtdcUserLogoutField,
    DisconnectionReason, QuoteSpi, QuoteSpi_Destructor, TThostFtdcErrorIDType,
    TThostFtdcRequestIDType,
};
use crate::structs::{CancelRequest, LoginForm, OrderRequest, TickData};

#[allow(non_camel_case_types)]
type c_bool = std::os::raw::c_uchar;

pub struct MdApi {
    user_id: CString,
    password: CString,
    path: CString,
    market_api: *mut CThostFtdcMdApi,
    market_spi: Option<*mut QuoteSpi>,
    producer: Option<ProducerMdApi>,
    login_info: Option<LoginForm>,
    request_id: i32,
}

struct DataCollector<'a> {
    producer: ProducerMdApi,
    login_status: bool,
    connect_status: bool,
    api: &'a mut MdApi,
}

/// 此处我们实现种种方法来构建ctp的登录流程
impl QuoteApi for DataCollector<'_> {
    fn on_front_connected(&mut self) {
        self.connect_status = true;
        self.api.login();
    }

    fn on_front_disconnected(&mut self, reason: DisconnectionReason) {
        self.login_status = false;
        self.connect_status = false;
    }

    fn on_rsp_user_login(
        &mut self,
        pRspUserLogin: *mut CThostFtdcRspUserLoginField,
        pRspInfo: *mut CThostFtdcRspInfoField,
        request_id: TThostFtdcRequestIDType,
        is_last: bool,
    ) {
        self.login_status = true;
    }

    fn on_rtn_depth_market_data(&mut self, pDepthMarketData: *mut CThostFtdcDepthMarketDataField) {
        // 回调应该传回策略分组的index，然后用try_send_to来确定收取tick data的分组
        self.producer.send_all(unsafe {
            let datetime = format!(
                "{} {}.{}",
                slice_to_string(&(*pDepthMarketData).ActionDay),
                slice_to_string(&(*pDepthMarketData).UpdateTime),
                (*pDepthMarketData).UpdateMillisec as i32 * 1000
            );
            let naive =
                NaiveDateTime::parse_from_str(datetime.as_str(), "%Y%m%d %H:%M:%S.%f").unwrap();
            TickData {
                symbol: slice_to_string(&(*pDepthMarketData).InstrumentID),
                exchange: None,
                datetime: Option::from(naive),
                name: None,
                volume: (*pDepthMarketData).Volume as f64,
                open_interest: (*pDepthMarketData).OpenInterest as f64,
                last_price: (*pDepthMarketData).LastPrice as f64,
                last_volume: 0.0,
                limit_up: (*pDepthMarketData).UpperLimitPrice as f64,
                limit_down: (*pDepthMarketData).LowerLimitPrice as f64,
                open_price: (*pDepthMarketData).OpenPrice as f64,
                high_price: (*pDepthMarketData).HighestPrice as f64,
                low_price: (*pDepthMarketData).LowestPrice as f64,
                pre_close: (*pDepthMarketData).PreClosePrice as f64,
                bid_price_1: (*pDepthMarketData).BidPrice1 as f64,
                bid_price_2: (*pDepthMarketData).BidPrice2 as f64,
                bid_price_3: (*pDepthMarketData).BidPrice3 as f64,
                bid_price_4: (*pDepthMarketData).BidPrice4 as f64,
                bid_price_5: (*pDepthMarketData).BidPrice5 as f64,
                ask_price_1: (*pDepthMarketData).AskPrice1 as f64,
                ask_price_2: (*pDepthMarketData).AskPrice2 as f64,
                ask_price_3: (*pDepthMarketData).AskPrice3 as f64,
                ask_price_4: (*pDepthMarketData).AskPrice4 as f64,
                ask_price_5: (*pDepthMarketData).AskPrice5 as f64,
                bid_volume_1: (*pDepthMarketData).BidVolume1 as f64,
                bid_volume_2: (*pDepthMarketData).BidVolume2 as f64,
                bid_volume_3: (*pDepthMarketData).BidVolume3 as f64,
                bid_volume_4: (*pDepthMarketData).BidVolume4 as f64,
                bid_volume_5: (*pDepthMarketData).BidVolume5 as f64,
                ask_volume_1: (*pDepthMarketData).AskVolume1 as f64,
                ask_volume_2: (*pDepthMarketData).AskVolume2 as f64,
                ask_volume_3: (*pDepthMarketData).AskVolume3 as f64,
                ask_volume_4: (*pDepthMarketData).AskVolume4 as f64,
                ask_volume_5: (*pDepthMarketData).AskVolume5 as f64,
            }
        });
    }
}

unsafe impl Send for MdApi {}

/// Now we get a very useful spi, and we get use the most important things to let everything works well
/// the code is from ctp-rs
///
/// 实现行情API的一些主动基准调用方法
impl MdApi {
    pub fn new(id: String, pwd: String, path: String, producer: ProducerMdApi) -> Self {
        let ids = CString::new(id).unwrap();
        let pwds = CString::new(pwd).unwrap();
        let paths = CString::new(path).unwrap();
        let flow_path_ptr = paths.as_ptr();
        // 创建了行情对象
        let api = unsafe { CThostFtdcMdApi::CreateFtdcMdApi(flow_path_ptr, true, true) };
        MdApi {
            user_id: ids,
            password: pwds,
            path: paths,
            market_api: api,
            market_spi: None,
            producer: Some(producer),
            login_info: None,
            request_id: 0,
        }
    }

    /// 初始化调用
    pub fn init(&mut self) -> bool {
        unsafe { CThostFtdcMdApi_Init(self.market_api) };
        true
    }
    /// 获取交易日
    pub fn get_trading_day<'a>(&mut self) -> &'a str {
        let trading_day_cstr = unsafe { CThostFtdcMdApi_GetTradingDay(self.market_api) };
        unsafe {
            CStr::from_ptr(trading_day_cstr as *const i8)
                .to_str()
                .unwrap()
        }
    }

    fn login(&mut self) {
        let login_form = self.login_info.take().unwrap();
        let user_id = CString::new(login_form._user_id()).unwrap();
        let pwd = CString::new(login_form._password()).unwrap();
        let broker_id = CString::new(login_form._broke_id()).unwrap();
        let auth_code = CString::new(login_form._auth_code()).unwrap();
        let app_id = CString::new(login_form._app_id()).unwrap();
        let production_info = CString::new(login_form._production_info()).unwrap();
        unsafe {
            CThostFtdcMdApi_ReqUserLogin(
                self.market_api,
                &mut CThostFtdcReqUserLoginField::default(),
                self.request_id.clone(),
            )
        };
    }

    /// 注册前置地址
    fn register_fronted_address(&mut self, register_addr: CString) {
        let front_socket_address_ptr = register_addr.into_raw();
        unsafe { CThostFtdcMdApi_RegisterFront(self.market_api, front_socket_address_ptr) };
    }

    /// 注册回调
    fn register_spi(&mut self, login_info: LoginForm, producer: ProducerMdApi) {
        self.login_info = Some(login_info);

        let collector = DataCollector {
            producer,
            login_status: false,
            connect_status: false,
            api: self,
        };
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
        let producer = self.producer.take().unwrap();
        self.register_spi(req.clone(), producer);
        let addr = CString::new(req._md_address()).unwrap();
        self.register_fronted_address(addr);
        self.init();
        println!("初始化成功");
    }

    fn subscribe(&mut self, symbols: &[&str]) {
        symbols.into_iter().enumerate().for_each(|(index, symbol)| {
            let code = CString::new(*symbol).unwrap();
            let mut c = code.into_raw();
            // symbols的index对应策略的分组，应该传给回调
            unsafe { CThostFtdcMdApi_SubscribeMarketData(self.market_api, &mut c, index as i32) };
        });
    }

    fn unsubscribe(&mut self, symbol: String) {
        unimplemented!()
    }

    fn exit(&mut self) {
        unimplemented!()
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
