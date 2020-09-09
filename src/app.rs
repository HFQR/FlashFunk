#![allow(dead_code, unused_imports, unused_variables)]

use super::interface::Interface;
use crate::ac::Ac;
use crate::account::Account;
use crate::constants::{Direction, OrderType};
use crate::structs::{AccountData, BarData, CancelRequest, ContractData, OrderData, OrderRequest, PositionData, SubscribeRequest, TickData, TradeData, LoginForm};
use actix::prelude::*;
use chrono::Local;
use core::fmt;
use futures::{AsyncReadExt, SinkExt, StreamExt};
use std::cell::RefCell;
use std::collections::{HashMap, VecDeque};
use std::fmt::Debug;
use std::mem::swap;
use std::rc::Rc;
use std::time::{Instant, Duration};

use crossbeam::channel::Sender;
use std::sync::{Arc, Condvar, Mutex};
use std::thread::JoinHandle;

use crossbeam::queue::spsc::{self, Consumer, Producer};
use std::borrow::Cow;
use failure::_core::marker::PhantomData;
use crate::ctp::api::MdApi;
use std::thread;
use std::ffi::CString;

lazy_static! {
    pub static ref TIMER: Mutex<Instant> = Mutex::new(Instant::now());
}

/// ctpbee核心运行器
/// 作为该运行器
pub struct CtpbeeR {
    name: String,
    acc: Account,
    login_info: HashMap<String, String>,
}

pub struct StrategyProducer {
    producers: Vec<Producer<CtpbeeRMessage>>,
}

impl StrategyProducer {
    pub fn send(&self, msg: impl Into<CtpbeeRMessage> + Clone) {
        self.producers.iter().for_each(|p| {
            p.push(msg.clone().into());
        })
    }
}

pub struct OrderConsumer {
    consumers: Vec<Consumer<u8>>,
}

impl OrderConsumer {
    pub fn recv(&self) -> Vec<u8> {
        self.consumers.iter().filter_map(|c| c.pop().ok()).collect()
    }
}

pub struct CtpBuilder<'a, S, T>
where
    T: Into<Cow<'a, str>>  + Default
{
    name: Cow<'a, str>,
    id: Cow<'a, str>,
    pwd: Cow<'a, str>,
    path: Cow<'a, str>,
    strategy: Vec<S>,
    login_form: LoginForm2<'a, T>,
    symbol: Cow<'a, str>,
}

#[derive(Default)]
pub struct LoginForm2<'a, T>
where
    T: Into<Cow<'a, str>> + Default
{
    pub user_id: T,
    pub password: T,
    pub broke_id: T,
    pub app_id: T,
    pub md_address: T,
    pub td_address: T,
    pub auth_code: T,
    pub production_info: T,
    pub _lifetime: PhantomData<&'a ()>
}

impl<'a, T> From<LoginForm2<'a, T>> for LoginForm
    where
        T: Into<Cow<'a, str>> + Default
{
    fn from(form: LoginForm2<'a, T>) -> Self {
        LoginForm {
            user_id: form.user_id.into().into(),
            password: form.password.into().into(),
            broke_id: form.broke_id.into().into(),
            app_id: form.app_id.into().into(),
            md_address: form.md_address.into().into(),
            td_address: form.td_address.into().into(),
            auth_code: form.auth_code.into().into(),
            production_info: form.production_info.into().into(),
        }
    }
}

impl<'a, S, T> CtpBuilder<'a, S, T>
where
    S: Send + Ac + 'static,
    T: Into<Cow<'a, str>>  + Default
{
    pub fn id(mut self, id: impl Into<Cow<'a, str>>) -> Self {
        self.id = id.into();
        self
    }

    pub fn pwd(mut self, pwd: impl Into<Cow<'a, str>>) -> Self {
        self.pwd = pwd.into();
        self
    }

    pub fn path(mut self, path: impl Into<Cow<'a, str>>) -> Self {
        self.path = path.into();
        self
    }

    pub fn strategy(mut self, strategy: Vec<S>) -> Self {
        self.strategy = strategy;
        self
    }

    pub fn login_form(mut self, login_form: LoginForm2<'a, T>) -> Self {
        self.login_form = login_form;
        self
    }

    pub fn subscribe(mut self, symbol: impl Into<Cow<'a, str>>) -> Self {
        self.symbol = symbol.into();
        self
    }

    pub fn start(self) {
        let cap = self.strategy.len();

        let mut producers = Vec::with_capacity(cap);
        let mut consumers = Vec::with_capacity(cap);
        let mut handles = Vec::with_capacity(cap);

        let ctp = CtpbeeR {
            name: self.name.into(),
            acc: Account::new(),
            login_info: HashMap::new(),
        };

        for mut strat in self.strategy.into_iter() {
            // 第一组spsc用来发送信息至策略
            let (tx, rx) = spsc::new(8);

            // 第二组spsc用来发送策略的返回信息
            let (tx2, rx2) = spsc::new::<u8>(8);

            // 每个策略一个线程
            // 第一组spsc的接收端和第二组的发送端移入线程
            let handle = std::thread::spawn({
                let tx2 = tx2;
                move || loop {
                    match rx.pop() {
                        Ok(msg) => match msg {
                            CtpbeeRMessage::TickData(data) => strat.on_tick(&data),
                            CtpbeeRMessage::TradeData(data) => strat.on_trade(&data),
                            CtpbeeRMessage::OrderData(data) => strat.on_order(&data),
                            CtpbeeRMessage::BarData(data) => strat.on_bar(&data),
                            CtpbeeRMessage::AccountData(data) => strat.on_account(&data),
                            CtpbeeRMessage::PositionData(data) => strat.on_position(&data),
                            CtpbeeRMessage::ContractData(data) => strat.on_contract(&data),
                            _ => {}
                        },
                        Err(_) => (),
                    }
                }
            });

            // 收集第一组发送端和第二组接收端。
            producers.push(tx);
            consumers.push(rx2);
            handles.push(handle);
        }

        let producer = StrategyProducer { producers };

        let mut md_api = MdApi::new(
            self.id.into(),
            self.pwd.into(),
            self.path.into(),
            producer,
        );

        md_api.connect(self.login_form.into());

        thread::sleep(Duration::from_secs(1));
        md_api.subscribe(self.symbol.into());

        // 第二组spsc接收端待定

        handles.into_iter().for_each(|handle| handle.join().unwrap());
    }
}

impl CtpbeeR {
    pub fn new<'a, S, T>(name: T) -> CtpBuilder<'a, S, T>
    where
        S: Send,
        T: Into<Cow<'a, str>>  + Default
    {
        CtpBuilder {
            name: name.into(),
            id: Default::default(),
            pwd: Default::default(),
            path: Default::default(),
            strategy: vec![],
            login_form: Default::default(),
            symbol: Default::default()
        }
    }

    pub fn login(&mut self) -> bool {
        true
    }

    /// 从HashMap载入登录信息
    pub fn load_setting(&mut self, map: HashMap<String, String>) {
        let mut dt = HashMap::new();
        for (key, value) in map {
            dt.insert(key, value);
        }
        self.login_info = dt;
    }
}

// 信息类型。使用enum包裹避免堆上分配
pub enum CtpbeeRMessage {
    TradeData(TradeData),
    OrderData(OrderData),
    TickData(TickData),
    BarData(BarData),
    AccountData(AccountData),
    PositionData(PositionData),
    ContractData(ContractData),
    OrderRequest(OrderRequest),
    CancelRequest(CancelRequest),
    SubscribeRequest(SubscribeRequest),
}

impl From<OrderRequest> for CtpbeeRMessage {
    fn from(data: OrderRequest) -> Self {
        Self::OrderRequest(data)
    }
}

impl From<CancelRequest> for CtpbeeRMessage {
    fn from(data: CancelRequest) -> Self {
        Self::CancelRequest(data)
    }
}

impl From<SubscribeRequest> for CtpbeeRMessage {
    fn from(data: SubscribeRequest) -> Self {
        Self::SubscribeRequest(data)
    }
}

impl From<TickData> for CtpbeeRMessage {
    fn from(data: TickData) -> Self {
        Self::TickData(data)
    }
}

impl Debug for CtpbeeR {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Ctpbee >>> : {}", self.name)
    }
}
