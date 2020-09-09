#![allow(dead_code, unused_variables)]

use crate::app::{CtpbeeR, CtpbeeRMessage};
use crate::structs::{CancelRequest, LoginForm, OrderRequest};
use crossbeam::channel::Sender;

/// 用户登录接口,包含用户的\
pub trait Interface {
    /// 发单
    fn send_order(&mut self, order: OrderRequest) -> String;
    /// 撤单
    fn cancel_order(&mut self, req: CancelRequest);
    /// 登录接口
    fn connect(&mut self, req: LoginForm);
    /// 订阅行情
    fn subscribe(&mut self, symbol: String);
    /// 取消订阅行情
    fn unsubscribe(&mut self, symbol: String);
    /// 释放退出接口
    fn exit(&mut self);
    // /// 获取到App
    // fn get_app(&mut self) -> &Sender<CtpbeeRMessage>;
}
