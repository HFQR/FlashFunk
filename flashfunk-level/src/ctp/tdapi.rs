use core::ffi::c_void;
use core::ptr::slice_from_raw_parts;
use core::sync::atomic::{AtomicI32, Ordering};

use std::borrow::Cow;
use std::cmp::max;
use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int};
use std::sync::Arc;

use chrono::{Date, NaiveDateTime, Timelike, Utc};

use crate::constant::{Direction, Exchange, Offset, OrderType, Status, LogLevel};
use crate::c_func::parse_datetime_from_str;
use crate::ctp::sys::*;
use crate::data_type::{AccountData, CancelRequest, ContractData, ContractVec, ExtraOrder, ExtraTrade, LoginForm, OrderData, OrderRequest, PositionData, TradeData, ContractStatus, Log};
use crate::interface::Interface;
use crate::types::message::TdApiMessage;
use crate::util::blocker::Blocker;
use crate::util::channel::GroupSender;
use crate::util::hash::HashMap;
use crate::{get_interface_path, os_path};
use bitflags::_core::ops::Deref;
use std::fs::create_dir;
use std::path::PathBuf;
use std::process::id;
use crate::ctp::CtpTd::CtpTdCApi;

const POS_LONG: u8 = THOST_FTDC_PD_Long as u8;
const POS_SHORT: u8 = THOST_FTDC_PD_Short as u8;

/// 交易API
pub struct CtpTdApi {
    level: TraderLevel,
    request_id: i32,
    front_id: i32,
    session_id: i32,
}

impl CtpTdApi {
    pub(crate) fn session_id(&self) -> i32 {
        self.session_id
    }

    pub(crate) fn front_id(&self) -> i32 {
        self.front_id
    }
}

struct TdApiBlockerInner {
    front_id: AtomicI32,
    session_id: AtomicI32,
    step1: Blocker,
    step2: Blocker,
    step3: Blocker,
    step4: Blocker,
    step5: Blocker,
}

struct TdApiBlocker(Arc<TdApiBlockerInner>);

impl Clone for TdApiBlocker {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl TdApiBlocker {
    fn new() -> Self {
        Self(Arc::new(TdApiBlockerInner {
            front_id: AtomicI32::new(0),
            session_id: AtomicI32::new(0),
            step1: Default::default(),
            step2: Default::default(),
            step3: Default::default(),
            step4: Default::default(),
            step5: Default::default(),
        }))
    }
}

pub struct TraderLevel {
    session_id: i32,
    login_status: bool,
    connect_status: bool,
    sender: GroupSender<TdApiMessage>,
    blocker: Option<TdApiBlocker>,
    pos: HashMap<Cow<'static, str>, PositionData>,
    size_map: HashMap<Cow<'static, str>, f64>,
    exchange_map: HashMap<Cow<'static, str>, Exchange>,
    order_ref: Arc<AtomicI32>,
    trade_pointer: *mut CThostFtdcTraderApi,
    login_form: LoginForm,
    symbols: Vec<&'static str>,
    pub request_id: i32,
    pub front_id: i32,
    contracts: Vec<ContractData>,
}

impl CtpTdCApi for TraderLevel {
    fn on_front_connected(&mut self) {
        let log = Log::new(LogLevel::INFO, "Front Connected");
        self.sender.send_to(log, 0);
        self.blocker.as_ref().unwrap().0.step1.unblock();
    }

    fn on_front_disconnected(&mut self, nReason: c_int) {
        // todo: 打印连接失败的原因
        self.blocker = Option::from(TdApiBlocker::new());
        println!("function callback: OnFrontDisconnected");
    }

    fn on_rtn_instrument_status(
        &mut self,
        pInstrumentStatus: *mut CThostFtdcInstrumentStatusField,
    ) {
        unsafe {
            let status = *pInstrumentStatus;
            let symbol = CStr::from_ptr(status.InstrumentID.as_ptr());
            let status_code = status.InstrumentStatus.clone();
            let status = ContractStatus {
                code: symbol.to_str().unwrap().to_string(),
                status: status_code as i32,
            };
            self.sender.send_all(status)
        }
    }


    fn on_rsp_authenticate(
        &mut self,
        pRspAuthenticateField: *mut CThostFtdcRspAuthenticateField,
        pRspInfo: *mut CThostFtdcRspInfoField,
        nRequestID: c_int,
        bIsLast: bool,
    ) {
        match get_rsp_info(pRspInfo) {
            Ok(t) => {
                let log = Log::new(LogLevel::INFO, "Td Auth Successful");
                self.sender.send_to(log, 0);
                self.blocker.as_ref().unwrap().0.step2.unblock();
            }
            Err(e) => {
                let log = Log::new(LogLevel::ERROR, &*format!("Td Auth Failed, id: {} msg: {}", e.id, e.msg));
                self.sender.send_to(log, 0);
            }
        }
    }

    fn on_rsp_user_login(
        &mut self,
        pRspUserLogin: *mut CThostFtdcRspUserLoginField,
        pRspInfo: *mut CThostFtdcRspInfoField,
        nRequestID: c_int,
        bIsLast: bool,
    ) {
        match get_rsp_info(pRspInfo) {
            Ok(t) => unsafe {
                let login = *pRspUserLogin;
                let log = Log::new(LogLevel::INFO, "Td Login successful");
                self.sender.send_to(log, 0);
                let blocker = self.blocker.as_ref().unwrap();

                blocker.0.front_id.store(login.FrontID, Ordering::SeqCst);
                blocker
                    .0
                    .session_id
                    .store(login.SessionID, Ordering::SeqCst);

                blocker.0.step3.unblock();
            }
            Err(e) => {
                let log = Log::new(LogLevel::ERROR, &*format!("Td Login Failed, id: {} msg: {}", e.id, e.msg));
                self.sender.send_to(log, 0);
            }
        }
    }

    fn on_rsp_order_insert(
        &mut self,
        pInputOrder: *mut CThostFtdcInputOrderField,
        pRspInfo: *mut CThostFtdcRspInfoField,
        nRequestID: c_int,
        bIsLast: bool,
    ) {}

    fn on_rsp_order_action(
        &mut self,
        pInputOrderAction: *mut CThostFtdcInputOrderActionField,
        pRspInfo: *mut CThostFtdcRspInfoField,
        nRequestID: c_int,
        bIsLast: bool,
    ) {
        match get_rsp_info(pRspInfo) {
            Ok(t) => {}
            Err(e) => {
                let log = Log::new(LogLevel::ERROR, &*format!("Order Action Error, id: {} msg: {}", e.id, e.msg));
                self.sender.send_to(log, 0);
            }
        }
    }

    fn on_rsp_settlement_info_confirm(
        &mut self,
        pSettlementInfoConfirm: *mut CThostFtdcSettlementInfoConfirmField,
        pRspInfo: *mut CThostFtdcRspInfoField,
        nRequestID: c_int,
        bIsLast: bool,
    ) {
        match get_rsp_info(pRspInfo) {
            Ok(t) => {
                let log = Log::new(LogLevel::INFO, "Td Confirmed Successful");
                self.sender.send_to(log, 0);
            }
            Err(e) => {
                let log = Log::new(LogLevel::ERROR, &*format!("Td Confirmed failed, id: {} msg: {}", e.id, e.msg));
                self.sender.send_to(log, 0);
                // println!(">>> Td Confirmed failed, id: {} msg: {}", e.id, e.msg);
            }
        }
    }

    fn on_rsp_qry_investor_position(
        &mut self,
        pInvestorPosition: *mut CThostFtdcInvestorPositionField,
        pRspInfo: *mut CThostFtdcRspInfoField,
        nRequestID: c_int,
        bIsLast: bool,
    ) {
        if pInvestorPosition.is_null() {} else {
            unsafe {
                let position = *pInvestorPosition;
                let symbol = slice_to_string(&position.InstrumentID);
                let open_cost = position.OpenCost;
                let direction = Direction::from(position.PosiDirection);
                let exchange = *self.exchange_map.get(symbol.as_str()).unwrap();
                let td_pos = position.TodayPosition as f64;
                let volume = position.Position as f64;
                let yd_pos = position.YdPosition as f64;
                let profit = position.PositionProfit;
                let frozen = position.ShortFrozen + position.LongFrozen;
                let key = format!("{}_{}", symbol, position.PosiDirection);

                let pos = self
                    .pos
                    .entry(Cow::from(key))
                    .or_insert_with(|| match direction {
                        Direction::SHORT => PositionData::new_with_short(&symbol),
                        Direction::LONG => PositionData::new_with_long(&symbol),
                        _ => panic!("bad direction"),
                    });
                // according to the exchange  to setup the yd position
                match exchange {
                    Exchange::SHFE => {
                        if yd_pos != 0.0 && td_pos == 0.0 {
                            pos.yd_volume = volume;
                        }
                    }
                    _ => {
                        pos.yd_volume = volume - td_pos;
                    }
                }
                let size = self.size_map.get(symbol.as_str()).unwrap();
                // pos.exchange = Some(*exchange);
                pos.price = (pos.price * pos.volume + open_cost / size) / (pos.volume + volume);
                pos.volume += volume;
                pos.pnl += profit;
                // if is the last data that been pushed,  take them and sent it the core
                if bIsLast {
                    self.pos
                        .iter()
                        .for_each(|(k, v)| self.sender.send_all(v.to_owned()));
                    self.pos.clear();
                }
            }
        }
    }

    fn on_rsp_qry_trading_account(
        &mut self,
        pTradingAccount: *mut CThostFtdcTradingAccountField,
        pRspInfo: *mut CThostFtdcRspInfoField,
        nRequestID: c_int,
        bIsLast: bool,
    ) {
        if !pTradingAccount.is_null() {
            unsafe {
                let acc = *pTradingAccount;

                let account_data = AccountData {
                    accountid: slice_to_string(&acc.AccountID),
                    balance: acc.Balance,
                    frozen: acc.FrozenMargin + acc.FrozenCash + acc.FrozenCommission,
                    date: Utc::today(),
                };

                self.sender.send_all(account_data);
            }
        } else {}

        if let Some(block) = self.blocker.take() {
            block.0.step5.unblock();
        }
    }

    fn on_rsp_qry_instrument(
        &mut self,
        pInstrument: *mut CThostFtdcInstrumentField,
        pRspInfo: *mut CThostFtdcRspInfoField,
        nRequestID: c_int,
        bIsLast: bool,
    ) {
        // todo: add Contract data in here and send it
        let instrument = unsafe { *pInstrument };

        let contract = ContractData {
            symbol: slice_to_string(&instrument.InstrumentID),
            exchange: Some(Exchange::from(instrument.ExchangeID)),
            name: None,
            product: None,
            size: instrument.VolumeMultiple as f64,
            pricetick: instrument.PriceTick as f64,
            min_volume: 0.0,
            stop_supported: false,
            net_position: false,
            history_data: false,
            option_strike: 0.0,
            option_underlying: None,
            option_type: None,
            option_expiry: None,
            option_portfolio: None,
            option_index: None,
        };

        self.size_map
            .insert(Cow::Owned(contract.symbol.clone()), contract.size);
        self.exchange_map.insert(
            Cow::Owned(contract.symbol.clone()),
            contract.exchange.unwrap(),
        );
        self.contracts.push(contract);
        if bIsLast {
            let con = std::mem::take(&mut self.contracts);
            self.sender
                .try_send_to(ContractVec::from(con), 0)
                .unwrap_or(());
            let log = Log::new(LogLevel::INFO, "Contract query finished");
            self.sender.send_to(log, 0);
            self.blocker.as_ref().unwrap().0.step4.unblock();
        }
    }

    fn on_rtn_order(&mut self, pOrder: *mut CThostFtdcOrderField) {
        let (mut order, idx) = {
            let order = unsafe { *pOrder };
            let order_ref = slice_to_string(&order.OrderRef);
            let id = format!("{}_{}_{}", order.SessionID, order.FrontID, order_ref);
            let (idx, refs) = split_into_vec(order_ref.as_str());

            self.order_ref.fetch_max(refs, Ordering::SeqCst);
            let (date, time) =
                parse_datetime_from_str(order.InsertDate.as_ptr(), order.InsertTime.as_ptr(), 0);
            let exchange = Exchange::from(order.ExchangeID);
            (
                OrderData {
                    symbol: slice_to_string(&order.InstrumentID),
                    exchange,
                    datetime: NaiveDateTime::new(date, time),
                    orderid: Option::from(id),
                    order_type: OrderType::from(order.OrderPriceType),
                    direction: Some(Direction::from(order.Direction)),
                    offset: Offset::from(order.CombOffsetFlag),
                    price: order.LimitPrice as f64,
                    volume: order.VolumeTotalOriginal as f64,
                    traded: order.VolumeTraded as f64,
                    status: Status::from(order.OrderStatus),
                    is_local: false,
                },
                idx,
            )
        };
        // 这里控制接收order data的策略index
        match idx {
            10000000usize => {
                // let ex = ExtraOrder::from(order);
                self.sender.try_send_to(order, 0).unwrap_or(());
            }
            _ => {
                // todo: why order sender error
                order.is_local = true;
                self.sender.try_send_to(order, idx).unwrap_or(());
            }
        }
    }

    fn on_rtn_trade(&mut self, pTrade: *mut CThostFtdcTradeField) {
        let (mut trade, idx) = {
            let trade = unsafe { *pTrade };
            let (date, time) =
                parse_datetime_from_str(trade.TradeDate.as_ptr(), trade.TradeTime.as_ptr(), 0);
            let order_ref = slice_to_string(&trade.OrderRef);
            let (idx, refs) = split_into_vec(order_ref.as_str());
            (
                TradeData {
                    symbol: Cow::from(slice_to_string(&trade.InstrumentID)),
                    exchange: Some(Exchange::from(trade.ExchangeID)),
                    datetime: NaiveDateTime::new(date, time),
                    orderid: Option::from(order_ref),
                    direction: Some(Direction::from(trade.Direction)),
                    offset: Some(Offset::from(trade.OffsetFlag)),
                    price: trade.Price,
                    volume: trade.Volume,
                    tradeid: Some(slice_to_string(&trade.TradeID)),
                    is_local: false,
                },
                idx,
            )
        };
        // 这里控制接收order data的策略index
        match idx {
            10000000usize => {
                self.sender.try_send_to(trade, 0).unwrap_or(());
            }
            _ => {
                trade.is_local = true;
                self.sender.try_send_to(trade, idx).unwrap_or(());
            }
        }
    }

    fn on_err_rtn_order_insert(
        &mut self,
        pInputOrder: *mut CThostFtdcInputOrderField,
        pRspInfo: *mut CThostFtdcRspInfoField,
    ) {
        let order = unsafe { *pInputOrder };
        let order_id = slice_to_string(&order.OrderRef);
        let (idx, refs) = split_into_vec(order_id.as_str());
        match get_rsp_info(pRspInfo) {
            Ok(t) => {}
            Err(e) => {
                let log = Log::new(LogLevel::ERROR, &*format!("Order Insert Failed, id: {} msg: {} OrderID:{}", e.id, e.msg, order_id));
                self.sender.send_to(log, 0);
            }
        }
    }


    fn on_err_rtn_order_action(
        &mut self,
        pOrderAction: *mut CThostFtdcOrderActionField,
        pRspInfo: *mut CThostFtdcRspInfoField,
    ) {
        match get_rsp_info(pRspInfo) {
            Ok(t) => {}
            Err(e) => {
                let log = Log::new(LogLevel::ERROR, &*format!("Order Cancel Err, id: {} msg: {}", e.id, e.msg));
                self.sender.send_to(log, 0);
            }
        };
    }
}


fn get_order_type(order: OrderType) -> c_char {
    match order {
        OrderType::LIMIT => THOST_FTDC_OPT_LimitPrice as i8,
        OrderType::MARKET => THOST_FTDC_OPT_AnyPrice as i8,
        OrderType::FOK => THOST_FTDC_OPT_LimitPrice as i8,
        OrderType::FAK => THOST_FTDC_OPT_LimitPrice as i8,
        _ => panic!("This Interface do not support this order direction"),
    }
}

fn get_order_offset(offset: Offset) -> u8 {
    match offset {
        Offset::CLOSE => THOST_FTDC_OF_Close,
        Offset::CLOSETODAY => THOST_FTDC_OF_CloseToday,
        Offset::CLOSEYESTERDAY => THOST_FTDC_OF_CloseYesterday,
        Offset::OPEN => THOST_FTDC_OF_Open,
        _ => panic!("This Interface do not support this order direction"),
    }
}

fn get_order_exchange(exchange: Exchange) -> &'static str {
    match exchange {
        Exchange::SHFE => "SHFE",
        Exchange::DCE => "DCE",
        Exchange::CZCE => "CZCE",
        Exchange::INE => "INE",
        Exchange::CFFEX => "CFFEX",
        _ => unreachable!("This Interface do not support this exchange"),
    }
}

fn get_direction(direction: Direction) -> c_char {
    match direction {
        Direction::LONG => THOST_FTDC_D_Buy as i8,
        Direction::SHORT => THOST_FTDC_D_Sell as i8,
        _ => panic!("This Interface do not support this order direction"),
    }
}

impl From<[i8; 9]> for Exchange {
    fn from(i: [i8; 9]) -> Self {
        let exchange = slice_to_string(&i);
        match exchange.as_ref() {
            "SHFE" => Self::SHFE,
            "INE" => Self::INE,
            "CFFEX" => Self::CFFEX,
            "CZCE" => Self::CZCE,
            "DCE" => Self::DCE,
            _ => panic!("ctp do not support this exchange"),
        }
    }
}

impl From<[i8; 5]> for Offset {
    fn from(offset: [i8; 5]) -> Self {
        let x = *offset.first().unwrap() as u8;
        match x {
            THOST_FTDC_OF_Close => Self::CLOSE,
            THOST_FTDC_OF_CloseToday => Self::CLOSETODAY,
            THOST_FTDC_OF_CloseYesterday => Self::CLOSEYESTERDAY,
            THOST_FTDC_OF_Open => Self::OPEN,
            _ => panic!("ctp do not support this"),
        }
    }
}

impl From<i8> for Offset {
    fn from(offset: i8) -> Self {
        let x = offset as u8;
        match x {
            THOST_FTDC_OF_Close => Self::CLOSE,
            THOST_FTDC_OF_CloseToday => Self::CLOSETODAY,
            THOST_FTDC_OF_CloseYesterday => Self::CLOSEYESTERDAY,
            THOST_FTDC_OF_Open => Self::OPEN,
            _ => panic!("ctp do not support this"),
        }
    }
}

impl From<i8> for OrderType {
    fn from(i: i8) -> Self {
        match i as u8 {
            THOST_FTDC_OPT_LimitPrice => Self::LIMIT,
            THOST_FTDC_OPT_AnyPrice => Self::MARKET,
            _ => panic!("ctp do not support this ordertype"),
        }
    }
}

impl From<i8> for Status {
    fn from(i: i8) -> Self {
        match i as u8 {
            THOST_FTDC_OAS_Submitted => Status::SUBMITTING,
            THOST_FTDC_OAS_Accepted => Status::SUBMITTING,
            THOST_FTDC_OAS_Rejected => Status::REJECTED,
            THOST_FTDC_OST_NoTradeQueueing => Status::NOTTRADED,
            THOST_FTDC_OST_PartTradedQueueing => Status::PARTTRADED,
            THOST_FTDC_OST_AllTraded => Status::ALLTRADED,
            THOST_FTDC_OST_Canceled => Status::CANCELLED,
            _ => panic!("ctp do not support this status"),
        }
    }
}

impl From<i8> for Direction {
    fn from(i: i8) -> Self {
        match i as u8 {
            THOST_FTDC_D_Buy => Self::LONG,
            THOST_FTDC_D_Sell => Self::SHORT,
            POS_LONG => Self::LONG,
            POS_SHORT => Self::SHORT,
            _ => panic!("ctp do not support other direction {}", i),
        }
    }
}

impl TraderLevel {
    pub fn auth(&mut self) {
        self.request_id += 1;
        let form = &self.login_form;
        let req = CThostFtdcReqAuthenticateField {
            UserID: form._user_id().to_c_slice(),
            BrokerID: form._broke_id().to_c_slice(),
            AuthCode: form._auth_code().to_c_slice(),
            AppID: form._app_id().to_c_slice(),
            UserProductInfo: form._production_info().to_c_slice(),
        };
        unsafe {
            CThostFtdcTraderApiReqAuthenticate(
                self.trade_pointer,
                Box::into_raw(Box::new(req)) as *mut CThostFtdcReqAuthenticateField,
                self.request_id,
            )
        };
    }

    pub fn login(&mut self) {
        self.request_id += 1;
        let form = &(self.login_form);

        let login_req = CThostFtdcReqUserLoginField {
            BrokerID: form._broke_id().to_c_slice(),
            UserID: form._user_id().to_c_slice(),
            Password: form._password().to_c_slice(),
            UserProductInfo: form._production_info().to_c_slice(),
            ..CThostFtdcReqUserLoginField::default()
        };
        unsafe {
            CThostFtdcTraderApiReqUserLogin(
                self.trade_pointer,
                Box::into_raw(Box::new(login_req)),
                self.request_id,
            )
        };
    }

    pub fn req_instrument(&mut self) {
        self.request_id += 1;
        unsafe {
            CThostFtdcTraderApiReqQryInstrument(
                self.trade_pointer,
                Box::into_raw(Box::new(CThostFtdcQryInstrumentField::default())),
                self.request_id,
            );
        }
    }

    /// 注册交易前值
    fn register_fronted(&mut self, register_addr: CString) {
        let front_socket_address_ptr = register_addr.into_raw();
        unsafe { CThostFtdcTraderApiRegisterFront(self.trade_pointer, front_socket_address_ptr) }
    }

    pub fn init(&mut self) -> bool {
        unsafe { CThostFtdcTraderApiInit(self.trade_pointer) };
        true
    }

    fn register_spi(&mut self) {
        let trait_object_box: Box<Box<&mut CtpTdCApi>> = Box::new(Box::new(self));
        let trait_object_pointer =
            Box::into_raw(trait_object_box) as *mut Box<&mut dyn CtpTdCApi> as *mut c_void;
        // 把 rust对象 传给回调SPI
        let trader_spi = unsafe { CtpTdSpi::new(trait_object_pointer) };
        let ptr = Box::into_raw(Box::new(trader_spi));
        unsafe {
            CThostFtdcTraderApiSubscribePrivateTopic(self.trade_pointer, 0);
            CThostFtdcTraderApiSubscribePublicTopic(self.trade_pointer, 0);
            CThostFtdcTraderApiRegisterSpi(self.trade_pointer, ptr as *mut CThostFtdcTraderSpi)
        };
    }

    fn req_settle(&mut self) {
        self.request_id += 1;
        let form = &(self.login_form);
        let req = CThostFtdcSettlementInfoConfirmField {
            BrokerID: form._broke_id().to_c_slice(),
            InvestorID: form._user_id().to_c_slice(),
            ..CThostFtdcSettlementInfoConfirmField::default()
        };
        unsafe {
            CThostFtdcTraderApiReqSettlementInfoConfirm(
                self.trade_pointer,
                Box::into_raw(Box::new(req)),
                self.request_id,
            );
        }
    }

    pub fn new(
        req: &LoginForm,
        symbols: Vec<&'static str>,
        sender: GroupSender<TdApiMessage>,
        trader_pointer: *mut CThostFtdcTraderApi,
    ) -> Self {
        let blocker = TdApiBlocker::new();
        TraderLevel {
            session_id: 0,
            front_id: 0,
            request_id: 0,
            login_status: false,
            connect_status: false,
            sender: sender,
            blocker: Option::from(blocker),
            pos: Default::default(),
            size_map: Default::default(),
            exchange_map: Default::default(),
            order_ref: Arc::new(Default::default()),
            trade_pointer: trader_pointer,
            login_form: req.clone(),
            symbols,
            contracts: vec![],
        }
    }

    pub fn send_order_local(&mut self, idx: usize, order: OrderRequest) {
        self.request_id += 1;
        self.order_ref.fetch_add(1, Ordering::SeqCst);

        let form = &(self.login_form);

        let (time_condition, volume_condition) = match order.order_type {
            OrderType::FOK => (THOST_FTDC_TC_IOC as i8, THOST_FTDC_VC_CV as i8),
            OrderType::FAK => (THOST_FTDC_TC_IOC as i8, THOST_FTDC_VC_AV as i8),
            _ => (THOST_FTDC_TC_GFD as i8, THOST_FTDC_VC_AV as i8),
        };

        let order_ref = if let Some(refs) = order.reference {
            format!("{:0>12}", refs)
        } else {
            format!("{:0>9}{:0>3}", self.order_ref.load(Ordering::SeqCst), idx)
        };

        let req = CThostFtdcInputOrderField {
            InstrumentID: order.symbol.as_str().to_c_slice(),
            LimitPrice: order.price,
            VolumeTotalOriginal: order.volume as c_int,
            OrderPriceType: get_order_type(order.order_type),
            Direction: get_direction(order.direction),
            UserID: form._user_id().to_c_slice(),
            InvestorID: form._user_id().to_c_slice(),
            BrokerID: form._broke_id().to_c_slice(),
            CombOffsetFlag: String::from_utf8(Vec::from([get_order_offset(order.offset)]))
                .unwrap()
                .to_c_slice(),
            OrderRef: order_ref.to_c_slice(),
            CombHedgeFlag: String::from_utf8(Vec::from([THOST_FTDC_HF_Speculation]))
                .unwrap()
                .to_c_slice(),
            ContingentCondition: THOST_FTDC_CC_Immediately as i8,
            ForceCloseReason: THOST_FTDC_FCC_NotForceClose as i8,
            IsAutoSuspend: 0 as c_int,
            TimeCondition: time_condition,
            VolumeCondition: volume_condition,
            MinVolume: 1 as c_int,
            ExchangeID: get_order_exchange(order.exchange).to_c_slice(),
            ..CThostFtdcInputOrderField::default()
        };
        unsafe {
            CThostFtdcTraderApiReqOrderInsert(
                self.trade_pointer,
                Box::into_raw(Box::new(req)),
                self.request_id,
            )
        };
    }

    fn connect(&mut self) {
        // 阻塞器交给data collector
        self.register_spi();

        let addr = CString::new(self.login_form._td_address()).unwrap();

        self.register_fronted(addr);

        self.init();

        // 阻塞等待on_front_connected
        self.blocker.as_mut().unwrap().0.step1.block();

        self.auth();

        // 阻塞等待on_rsp_authenticate
        self.blocker.as_mut().unwrap().0.step2.block();
        self.login();

        // 阻塞等待on_rsp_user_login
        self.blocker.as_mut().unwrap().0.step3.block();

        // on_rsp_user_login 完成时会写入atomic i32至blocker，我们读取并赋予TdApi.
        self.session_id = self
            .blocker
            .as_mut()
            .unwrap()
            .0
            .session_id
            .load(Ordering::SeqCst);
        self.front_id = self
            .blocker
            .as_mut()
            .unwrap()
            .0
            .front_id
            .load(Ordering::SeqCst);

        self.req_settle();
        self.req_instrument();
        self.blocker.as_mut().unwrap().0.step4.block();
        self.req_position();
        self.req_account();
        // 阻塞等待賬戶查詢完畢
        // self.blocker.as_mut().unwrap().0.step5.block();
    }

    fn cancel(&mut self, req: CancelRequest) {
        self.request_id += 1;
        let exchange = req.exchange;
        let data = req
            .order_id
            .split('_')
            .into_iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>();
        let form = &(self.login_form);
        let action = CThostFtdcInputOrderActionField {
            InstrumentID: req.symbol.to_c_slice(),
            OrderRef: data[2].to_c_slice(),
            FrontID: data[1].parse::<i32>().unwrap() as c_int,
            SessionID: data[0].parse::<i32>().unwrap() as c_int,
            ActionFlag: THOST_FTDC_AF_Delete as i8,
            BrokerID: form._broke_id().to_c_slice(),
            InvestorID: form._user_id().to_c_slice(),
            ExchangeID: get_order_exchange(exchange).to_c_slice(),
            ..CThostFtdcInputOrderActionField::default()
        };
        unsafe {
            CThostFtdcTraderApiReqOrderAction(
                self.trade_pointer,
                Box::into_raw(Box::new(action)),
                self.request_id,
            );
        }
    }

    fn req_account(&mut self) {
        self.request_id += 1;
        unsafe {
            CThostFtdcTraderApiReqQryTradingAccount(
                self.trade_pointer,
                Box::into_raw(Box::new(CThostFtdcQryTradingAccountField::default())),
                self.request_id,
            );
        }
    }

    fn req_position(&mut self) {
        self.request_id += 1;
        let login_info = &(self.login_form);
        unsafe {
            let req = CThostFtdcQryInvestorPositionField {
                BrokerID: login_info._broke_id().to_c_slice(),
                InvestorID: login_info._user_id().to_c_slice(),
                ..CThostFtdcQryInvestorPositionField::default()
            };
            CThostFtdcTraderApiReqQryInvestorPosition(
                self.trade_pointer,
                Box::into_raw(Box::new(req)),
                self.request_id,
            );
        }
    }
}

impl Drop for CtpTdApi {
    fn drop(&mut self) {}
}

impl Interface for CtpTdApi {
    type Message = TdApiMessage;

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
        let flow_path_ptr = p.as_ptr();
        let api = unsafe { CThostFtdcTraderApi::CreateFtdcTraderApi(flow_path_ptr) };
        CtpTdApi {
            level: TraderLevel::new(req, symbols, sender, api),
            request_id: 0,
            front_id: 0,
            session_id: 0,
        }
    }

    fn send_order(&mut self, idx: usize, order: OrderRequest) {
        self.level.send_order_local(idx, order)
    }

    fn cancel_order(&mut self, req: CancelRequest) {
        self.level.cancel(req)
    }

    fn connect(&mut self) {
        // 建立线程阻塞器
        self.level.connect()
    }

    fn req_account(&mut self) {
        self.level.req_account();
    }

    fn req_position(&mut self) {
        self.level.req_position();
    }
}
