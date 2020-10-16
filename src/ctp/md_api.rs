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

struct DataCollector {
    sender: GroupSender<MdApiMessage>,
    symbols: Vec<*const i8>,
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
impl QuoteApi for DataCollector {
    fn on_front_connected(&mut self) {
        // 解除login线程的阻塞
        self.blocker.as_ref().unwrap().0.step1.unblock();
    }

    fn on_front_disconnected(&mut self, reason: DisconnectionReason) {}

    fn on_rsp_user_login(
        &mut self,
        pRspUserLogin: *mut CThostFtdcRspUserLoginField,
        pRspInfo: *mut CThostFtdcRspInfoField,
        request_id: TThostFtdcRequestIDType,
        is_last: bool,
    ) {
        self.blocker.take().unwrap().0.step2.unblock();
    }

    fn on_rtn_depth_market_data(&mut self, pDepthMarketData: *mut CThostFtdcDepthMarketDataField) {
        let instant = Instant::now();
        unsafe {
            let depth = *pDepthMarketData;

            let v = depth.InstrumentID.as_ptr();

            let symbol = CStr::from_ptr(v);

            let index = self.symbols.iter().enumerate().find_map(|(i, s)| {
                if symbol == CStr::from_ptr(*s) {
                    Some(i)
                } else {
                    None
                }
            });

            if let Some(i) = index {
                let msg = {
                    let symbol = symbol.to_str().unwrap().to_owned();

                    let a = depth.ActionDay.as_ptr();
                    let u = depth.UpdateTime.as_ptr();

                    // // ToDo: 处理这里的错误，如果api返回结果不可信
                    let a = CStr::from_ptr(a).to_str().unwrap();
                    let u = CStr::from_ptr(u).to_str().unwrap();

                    let sub_t = depth.UpdateMillisec as u32 * 1_000_000;

                    let date = NaiveDate::from_ymd(
                        a[0..4].parse().unwrap(),
                        a[4..6].parse().unwrap(),
                        a[6..].parse().unwrap(),
                    );

                    let time = NaiveTime::from_hms(
                        u[0..2].parse().unwrap(),
                        u[3..5].parse().unwrap(),
                        u[6..].parse().unwrap(),
                    )
                    .with_nanosecond(sub_t)
                    .unwrap();

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
                };

                let msg: &'static TickData = Box::leak(Box::new(msg));

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
            symbols: self
                .symbols
                .iter()
                .map(|r| CString::new(*r).unwrap().into_raw() as *const i8)
                .collect(),
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

    fn new(
        id: impl Into<Vec<u8>>,
        pwd: impl Into<Vec<u8>>,
        path: impl Into<Vec<u8>>,
        symbols: Vec<&'static str>,
    ) -> Self {
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
