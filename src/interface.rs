#![allow(dead_code, unused_variables)]

use crate::structs::{CancelRequest, LoginForm, OrderRequest};

/// 用户登录接口,包含用户的\
pub trait Interface {
    /// 发单
    fn send_order(&mut self, order: OrderRequest) -> String;
    /// 撤单
    fn cancel_order(&mut self, req: CancelRequest);
    /// 登录接口
    fn connect(&mut self, req: &LoginForm);
    /// 订阅行情
    fn subscribe(&mut self, symbols: Vec<&'static str>);
    /// 取消订阅行情
    fn unsubscribe(&mut self, symbol: String);
    /// 释放退出接口
    fn exit(&mut self);
}
