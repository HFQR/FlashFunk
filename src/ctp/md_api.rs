use core::ffi::c_void;
use core::fmt;

use std::borrow::Cow;
use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int, c_uchar};
use std::process::id;

use chrono::{DateTime, Local, NaiveDateTime};
use encoding::all::GB18030;
use encoding::{DecoderTrap, Encoding};

use crate::ctp::func::QuoteApi;
use crate::ctp::sys::{
    another_slice_to_string, check_slice_to_string, slice_to_string,
    CThostFtdcDepthMarketDataField, CThostFtdcFensUserInfoField, CThostFtdcForQuoteRspField,
    CThostFtdcMdApi, CThostFtdcMdApi_GetTradingDay, CThostFtdcMdApi_Init,
    CThostFtdcMdApi_RegisterFront, CThostFtdcMdApi_RegisterSpi, CThostFtdcMdApi_ReqUserLogin,
    CThostFtdcMdApi_SubscribeMarketData, CThostFtdcMdSpi, CThostFtdcReqUserLoginField,
    CThostFtdcRspInfoField, CThostFtdcRspUserLoginField, CThostFtdcSpecificInstrumentField,
    CThostFtdcTraderApi, CThostFtdcUserLogoutField, DisconnectionReason, QuoteSpi,
    QuoteSpi_Destructor, TThostFtdcErrorIDType, TThostFtdcRequestIDType,
};
use crate::interface::Interface;
use crate::structs::{CancelRequest, LoginForm, OrderRequest, TickData};
use crate::types::message::MdApiMessage;
use crate::util::blocker::Blocker;
use crate::util::channel::GroupSender;
use std::sync::Arc;
use std::time::Instant;

#[allow(non_camel_case_types)]
type c_bool = std::os::raw::c_uchar;

pub struct MdApi {
    user_id: CString,
    password: CString,
    path: CString,
    market_api: *mut CThostFtdcMdApi,
    market_spi: Option<*mut QuoteSpi>,
    login_info: Option<LoginForm>,
    request_id: i32,
    symbols: Vec<&'static str>,
}

struct DataCollector<'a> {
    sender: GroupSender<MdApiMessage>,
    symbols: &'a [&'static str],
    login_status: bool,
    connect_status: bool,
    blocker: Option<MdApiBlocker>,
}

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

/// 此处我们实现种种方法来构建ctp的登录流程
impl QuoteApi for DataCollector<'_> {
    fn on_front_connected(&mut self) {
        self.connect_status = true;
        // 解除login线程的阻塞
        self.blocker.as_ref().unwrap().0.step1.unblock();
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
        self.blocker.take().unwrap().0.step2.unblock();
    }

    fn on_rtn_depth_market_data(&mut self, pDepthMarketData: *mut CThostFtdcDepthMarketDataField) {
        unsafe {
            let depth = *pDepthMarketData;

            let v = depth.InstrumentID.as_ptr();
            let symbol = CStr::from_ptr(v).to_string_lossy();

            let index = self.symbols.iter().enumerate().find_map(|(i, s)| {
                if symbol.starts_with(*s) {
                    Some(i)
                } else {
                    None
                }
            });
            if let Some(i) = index {
                let msg = {
                    let a = depth.ActionDay.as_ptr();
                    let u = depth.UpdateTime.as_ptr();

                    // ToDo: 处理这里的错误，如果api返回结果不可信
                    let a = CStr::from_ptr(a).to_str().unwrap().as_bytes();
                    let u = CStr::from_ptr(u).to_str().unwrap().as_bytes();

                    let sub_t = depth.UpdateMillisec.to_string();
                    let sub_t = sub_t.as_bytes();

                    let mut buf = Vec::with_capacity(a.len() + u.len() + sub_t.len());

                    buf.extend_from_slice(a);
                    buf.push(b' ');
                    buf.extend_from_slice(u);
                    buf.push(b'.');
                    buf.extend_from_slice(sub_t);

                    // # Safety: 这里是安全的，因为CStr转换为str时需要处理错误。
                    // 我们额外增加了' '和'.'以及一个String化的i32，这些都是无需检查的。
                    let str = std::str::from_utf8_unchecked(&buf);

                    let naive = NaiveDateTime::parse_from_str(str, "%Y%m%d %H:%M:%S.%f").unwrap();
                    TickData {
                        symbol,
                        exchange: None,
                        datetime: Option::from(naive),
                        name: None,
                        volume: depth.Volume as f64,
                        open_interest: depth.OpenInterest as f64,
                        last_price: depth.LastPrice as f64,
                        last_volume: 0.0,
                        limit_up: depth.UpperLimitPrice as f64,
                        limit_down: depth.LowerLimitPrice as f64,
                        open_price: depth.OpenPrice as f64,
                        high_price: depth.HighestPrice as f64,
                        low_price: depth.LowestPrice as f64,
                        pre_close: depth.PreClosePrice as f64,
                        bid_price: [
                            depth.BidPrice1 as f64,
                            depth.BidPrice2 as f64,
                            depth.BidPrice3 as f64,
                            depth.BidPrice4 as f64,
                            depth.BidPrice5 as f64,
                        ],
                        ask_price: [
                            depth.AskPrice1 as f64,
                            depth.AskPrice2 as f64,
                            depth.AskPrice3 as f64,
                            depth.AskPrice4 as f64,
                            depth.AskPrice5 as f64,
                        ],
                        bid_volume: [
                            depth.BidVolume1 as f64,
                            depth.BidVolume2 as f64,
                            depth.BidVolume3 as f64,
                            depth.BidVolume4 as f64,
                            depth.BidVolume5 as f64,
                        ],
                        ask_volume: [
                            depth.AskVolume1 as f64,
                            depth.AskVolume2 as f64,
                            depth.AskVolume3 as f64,
                            depth.AskVolume4 as f64,
                            depth.AskVolume5 as f64,
                        ],
                        ..TickData::default()
                    }
                };

                // 如果需要错误处理请在这里取回消息

                let _ = self.sender.try_send_group(msg, i);
            }
        }
    }
}

unsafe impl Send for MdApi {}

/// Now we get a very useful spi, and we get use the most important things to let everything works well
/// the code is from ctp-rs
///
/// 实现行情API的一些主动基准调用方法
impl MdApi {
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
        let login_form = self.login_info.as_ref().unwrap();
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
                self.request_id,
            )
        };
    }

    /// 注册前置地址
    fn register_fronted_address(&mut self, register_addr: CString) {
        let front_socket_address_ptr = register_addr.into_raw();
        unsafe { CThostFtdcMdApi_RegisterFront(self.market_api, front_socket_address_ptr) };
    }

    /// 注册回调
    fn register_spi(
        &mut self,
        login_info: LoginForm,
        sender: GroupSender<MdApiMessage>,
        blocker: MdApiBlocker,
    ) {
        self.login_info = Some(login_info);

        let collector = DataCollector {
            sender,
            symbols: &self.symbols,
            login_status: false,
            connect_status: false,
            blocker: Some(blocker),
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
    type Message = MdApiMessage;

    fn new(id: String, pwd: String, path: String, symbols: Vec<&'static str>) -> Self {
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
            login_info: None,
            request_id: 0,
            symbols,
        }
    }

    fn connect(&mut self, req: &LoginForm, sender: GroupSender<Self::Message>) {
        // 建立一个线程阻塞器
        let blocker = MdApiBlocker::new();

        // 阻塞器交给data collector
        self.register_spi(req.clone(), sender, blocker.clone());

        let addr = CString::new(req._md_address()).unwrap();
        self.register_fronted_address(addr);

        self.init();

        // 阻塞线程等待回调
        blocker.0.step1.block();

        // login发生在on_front_connected之后

        self.login();

        // 等待 login完成后才发送订阅
        blocker.0.step2.block();

        println!("初始化成功");
    }

    fn subscribe(&mut self) {
        self.request_id += 1;
        self.symbols.iter().for_each(|symbol| {
            let code = CString::new(*symbol).unwrap();
            let mut c = code.into_raw();
            unsafe {
                CThostFtdcMdApi_SubscribeMarketData(self.market_api, &mut c, self.request_id)
            };
        });
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
