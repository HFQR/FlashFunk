use core::ffi::c_void;
use core::fmt;

use std::borrow::Cow;
use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int, c_uchar};
use std::process::id;
use std::sync::Arc;
use std::time::{Duration, Instant};

use crate::c_func::parse_datetime_from_str;
use crate::constant::LogLevel;
use crate::ctp::sys::*;
use crate::ctp::CtpMd::CtpMdCApi;
use crate::data_type::{CancelRequest, Log, LoginForm, OrderRequest, TickData};
use crate::interface::Interface;
use crate::types::message::MdApiMessage;
use crate::util::blocker::Blocker;
use crate::util::channel::GroupSender;
use crate::{get_interface_path, os_path};
use chrono::{DateTime, Local, NaiveDate, NaiveDateTime, NaiveTime, Timelike};
use std::fs::create_dir;
use std::path::PathBuf;
use std::thread::sleep;

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
    level: Level,
}

struct Level {
    sender: GroupSender<MdApiMessage>,
    symbols: Vec<&'static str>,
    blocker: Option<MdApiBlocker>,
    login_form: LoginForm,
    market_pointer: *mut CThostFtdcMdApi,
    request_id: i32,
}

/// 此处我们实现种种方法来构建ctp的登录流程
impl CtpMdCApi for Level {
    fn on_front_connected(&mut self) {
        // 当前置连上后 开始进行登录
        let log = Log::new(LogLevel::INFO, "Md Front Connect");
        self.sender.send_to(log, 0);
        self.login();
    }

    fn on_front_disconnected(&mut self, reason: i32) {
        let log = Log::new(
            LogLevel::ERROR,
            "CtpMdApi Front Disconnected, Please Check Your Network",
        );
        self.sender.send_to(log, 0);
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
        unsafe {
            let depth = *pDepthMarketData;

            let v = depth.InstrumentID.as_ptr();

            let symbol = CStr::from_ptr(v).to_str().unwrap();

            let index = self.symbols.iter().enumerate().find(|(i, s)| **s == symbol);

            if let Some((i, _)) = index {
                let msg = {
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
                        ..TickData::default()
                    }
                };

                let msg = Arc::new(msg);

                // 如果需要错误处理请在这里取回消息

                let _ = self.sender.try_send_group(msg, i);
            }
        }
    }
}

unsafe impl Send for CtpMdApi {}

/// Now we get a very useful spi, and we get use the most important things to let everything works well
impl Level {
    /// 在构建API的时候把相关信息都传入进来 ， 包含登录信息以及订阅列表 和发送器和 行情指针
    /// 注意 MdApi只负责构建回调对象
    pub fn new(
        req: &LoginForm,
        symbols: Vec<&'static str>,
        sender: GroupSender<MdApiMessage>,
        market_pointer: *mut CThostFtdcMdApi,
    ) -> Self {
        let blocker = MdApiBlocker::new();
        Level {
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
        let boxed: Box<Box<&mut dyn CtpMdCApi>> = Box::new(Box::new(self));
        let pointer = Box::into_raw(boxed) as *mut Box<&mut dyn CtpMdCApi> as *mut c_void;
        // 把rust对象 传给回调SPI
        let callback = unsafe { CtpMdSpi::new(pointer) };
        let ptr = Box::into_raw(Box::new(callback));
        unsafe { CThostFtdcMdApiRegisterSpi(self.market_pointer, ptr as *mut CThostFtdcMdSpi) };
        let addr = CString::new(self.login_form._md_address()).unwrap();
        let addr_i = addr.into_raw();
        unsafe {
            CThostFtdcMdApiRegisterFront(self.market_pointer, addr_i);
        };
        unsafe {
            CThostFtdcMdApiInit(self.market_pointer);
        };
        // 等待 login完成后才发送订阅
        self.blocker.as_mut().unwrap().0.step2.block();
        let log = Log::new(LogLevel::INFO, "Md Api Init Successful");
        self.sender.send_to(log, 0);
    }

    pub fn subscribe(&mut self) {
        self.request_id += 1;
        self.symbols.iter().for_each(|symbol| {
            let code = CString::new(*symbol).unwrap();
            let mut c = code.into_raw();
            unsafe {
                CThostFtdcMdApiSubscribeMarketData(self.market_pointer, &mut c, self.request_id)
            };
        });
    }

    /// 获取交易日
    pub fn get_trading_day<'a>(&mut self) -> &'a str {
        let trading_day_cstr = unsafe { CThostFtdcMdApiGetTradingDay(self.market_pointer) };
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
            CThostFtdcMdApiReqUserLogin(
                self.market_pointer,
                &mut CThostFtdcReqUserLoginField::default(),
                self.request_id,
            )
        };
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
            level: Level::new(req, symbols.clone(), sender, market_pointer),
        }
    }

    fn connect(&mut self) {
        self.level.connect();
    }

    fn subscribe(&mut self) {
        self.level.subscribe();
    }
}
