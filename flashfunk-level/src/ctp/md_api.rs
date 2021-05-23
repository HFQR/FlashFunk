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
    another_slice_to_string, check_slice_to_string, parse_datetime_from_str,
    parse_datetime_from_str_with_mill, slice_to_string, CThostFtdcDepthMarketDataField,
    CThostFtdcFensUserInfoField, CThostFtdcForQuoteRspField, CThostFtdcMdApi,
    CThostFtdcMdApi_GetTradingDay, CThostFtdcMdApi_Init, CThostFtdcMdApi_RegisterFront,
    CThostFtdcMdApi_RegisterSpi, CThostFtdcMdApi_ReqUserLogin, CThostFtdcMdApi_SubscribeMarketData,
    CThostFtdcMdSpi, CThostFtdcReqUserLoginField, CThostFtdcRspInfoField,
    CThostFtdcRspUserLoginField, CThostFtdcSpecificInstrumentField, CThostFtdcTraderApi,
    CThostFtdcUserLogoutField, DisconnectionReason, QuoteSpi, QuoteSpi_Destructor,
    TThostFtdcErrorIDType, TThostFtdcRequestIDType, ToCSlice,
};
use crate::data_type::{CancelRequest, LoginForm, OrderRequest, TickData};
use crate::interface::Interface;
use crate::types::message::MdApiMessage;
use crate::util::blocker::Blocker;
use crate::util::channel::GroupSender;
use crate::{get_interface_path, os_path};
use std::fs::create_dir;
use std::path::PathBuf;

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

pub struct CtpMdApi {
    user_id: CString,
    password: CString,
    request_id: i32,
    pub flow_path_ptr: *const i8,
    collector: DataCollector,
}

struct DataCollector {
    sender: GroupSender<MdApiMessage>,
    symbols: Vec<&'static str>,
    blocker: Option<MdApiBlocker>,
    login_form: LoginForm,
    market_pointer: *mut CThostFtdcMdApi,
    request_id: i32,
}

/// 此处我们实现种种方法来构建ctp的登录流程
impl QuoteApi for DataCollector {
    fn on_front_connected(&mut self) {
        // 当前置连上后 开始进行登录
        self.login();
    }

    fn on_front_disconnected(&mut self, reason: DisconnectionReason) {
        println!(">>> CtpMdApi front disconnected, please check your network");
        self.blocker = Option::from(MdApiBlocker::new());
    }

    fn on_rsp_user_login(
        &mut self,
        pRspUserLogin: *mut CThostFtdcRspUserLoginField,
        pRspInfo: *mut CThostFtdcRspInfoField,
        request_id: TThostFtdcRequestIDType,
        is_last: bool,
    ) {
        self.blocker.take().unwrap().0.step2.unblock();
        self.subscribe();
    }

    fn on_rtn_depth_market_data(&mut self, pDepthMarketData: *mut CThostFtdcDepthMarketDataField) {
        let instant = Instant::now();
        unsafe {
            let depth = *pDepthMarketData;

            let v = depth.InstrumentID.as_ptr();

            let symbol = CStr::from_ptr(v);

            let index = self.symbols.iter().enumerate().find_map(|(i, s)| {
                let r = CString::new(s.clone()).unwrap();
                if symbol == CStr::from_ptr(r.as_ptr()) {
                    Some(i)
                } else {
                    None
                }
            });

            if let Some(i) = index {
                let msg = {
                    let symbol = symbol.to_str().unwrap().to_owned();
                    let (date, time) = parse_datetime_from_str_with_mill(
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
                };

                let msg: &'static TickData = Box::leak(Box::new(msg));

                // 如果需要错误处理请在这里取回消息

                let _ = self.sender.try_send_group(msg, i);
            }
        }
    }
}

unsafe impl Send for CtpMdApi {}

/// Now we get a very useful spi, and we get use the most important things to let everything works well
impl DataCollector {
    /// 在构建API的时候把相关信息都传入进来 ， 包含登录信息以及订阅列表 和发送器和 行情指针
    /// 注意 MdApi只负责构建回调对象
    pub fn new(
        req: &LoginForm,
        symbols: Vec<&'static str>,
        sender: GroupSender<MdApiMessage>,
        market_pointer: *mut CThostFtdcMdApi,
    ) -> Self {
        let blocker = MdApiBlocker::new();
        DataCollector {
            sender,
            symbols,
            blocker: Some(blocker),
            login_form: req.clone(),
            market_pointer,
            request_id: 0,
        }
    }
    // 底层发起主动连接逻辑
    pub fn connect(&mut self) {
        // 阻塞器交给data collector
        self.register_spi();

        let addr = CString::new(self.login_form._md_address()).unwrap();
        self.register_fronted_address(addr);

        self.init();

        // 等待 login完成后才发送订阅
        self.blocker.as_mut().unwrap().0.step2.block();

        println!(">>> CtpMdApi init successful");
    }

    pub fn subscribe(&mut self) {
        self.request_id += 1;
        self.symbols.iter().for_each(|symbol| {
            let code = CString::new(*symbol).unwrap();
            let mut c = code.into_raw();
            unsafe {
                CThostFtdcMdApi_SubscribeMarketData(self.market_pointer, &mut c, self.request_id)
            };
        });
    }

    /// 初始化调用
    pub fn init(&mut self) -> bool {
        unsafe { CThostFtdcMdApi_Init(self.market_pointer) };
        true
    }
    /// 获取交易日
    pub fn get_trading_day<'a>(&mut self) -> &'a str {
        let trading_day_cstr = unsafe { CThostFtdcMdApi_GetTradingDay(self.market_pointer) };
        unsafe {
            CStr::from_ptr(trading_day_cstr as *const i8)
                .to_str()
                .unwrap()
        }
    }

    fn login(&mut self) {
        let login_form = &(self.login_form);
        let user_id = CString::new(login_form._user_id()).unwrap();
        let pwd = CString::new(login_form._password()).unwrap();
        let broker_id = CString::new(login_form._broke_id()).unwrap();
        let auth_code = CString::new(login_form._auth_code()).unwrap();
        let app_id = CString::new(login_form._app_id()).unwrap();
        let production_info = CString::new(login_form._production_info()).unwrap();
        unsafe {
            CThostFtdcMdApi_ReqUserLogin(
                self.market_pointer,
                &mut CThostFtdcReqUserLoginField::default(),
                self.request_id,
            )
        };
    }

    /// 注册前置地址
    fn register_fronted_address(&mut self, register_addr: CString) {
        println!(
            ">>> MdAddress: {:?}",
            register_addr.clone().into_string().unwrap()
        );
        let front_socket_address_ptr = register_addr.into_raw();
        unsafe { CThostFtdcMdApi_RegisterFront(self.market_pointer, front_socket_address_ptr) };
    }

    /// 注册回调
    fn register_spi(&mut self) {
        let trait_object_box: Box<Box<&mut dyn QuoteApi>> = Box::new(Box::new(self));
        let trait_object_pointer =
            Box::into_raw(trait_object_box) as *mut Box<&mut dyn QuoteApi> as *mut c_void;
        // 把 rust对象 传给回调SPI
        let quote_spi = unsafe { QuoteSpi::new(trait_object_pointer) };
        let ptr = Box::into_raw(Box::new(quote_spi));
        unsafe { CThostFtdcMdApi_RegisterSpi(self.market_pointer, ptr as *mut CThostFtdcMdSpi) };
    }

    fn release(&mut self) {
        println!("Release Quote Interface");
        // unsafe { QuoteSpi_Destructor(self as QuoteSpi) };
    }
}

impl Interface for CtpMdApi {
    type Message = MdApiMessage;

    fn new(
        id: impl Into<Vec<u8>>,
        pwd: impl Into<Vec<u8>>,
        symbols: Vec<&'static str>,
        req: &LoginForm,
        sender: GroupSender<Self::Message>,
    ) -> Self {
        let home_path = os_path("ctp");
        let string = String::from_utf8(id.into()).unwrap();
        let path = home_path.join(string).to_string_lossy().to_string() + "//";
        if !PathBuf::from(path.clone()).exists() {
            create_dir(path.clone()).expect("create dir failed ");
        }
        let p = CString::new(path).unwrap();
        let pwd = CString::new(pwd).unwrap();
        let flow_path_ptr = p.as_ptr();
        let market_pointer = unsafe { CThostFtdcMdApi::CreateFtdcMdApi(flow_path_ptr, true, true) };
        // 创建了行情对象
        CtpMdApi {
            user_id: p.clone(),
            flow_path_ptr,
            password: pwd,
            request_id: 0,
            collector: DataCollector::new(req, symbols.clone(), sender, market_pointer),
        }
    }

    fn connect(&mut self) {
        self.collector.connect();
    }

    fn subscribe(&mut self) {
        self.collector.subscribe();
    }
}

impl Drop for CtpMdApi {
    fn drop(&mut self) {
        self.collector.release();
    }
}
