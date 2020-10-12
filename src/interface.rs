#![allow(dead_code, unused_variables)]

use crate::structs::{CancelRequest, LoginForm, OrderRequest};
use crate::util::channel::GroupSender;

/// 用户登录接口,包含用户的
pub trait Interface {
    type Message;

    fn new(id: String, pwd: String, path: String, symbols: Vec<&'static str>) -> Self;

    /// 发单
    fn send_order(&mut self, idx: usize, order: OrderRequest) {}

    /// 撤单
    fn cancel_order(&mut self, req: CancelRequest) {}

    /// 登录接口
    fn connect(&mut self, req: &LoginForm, sender: GroupSender<Self::Message>) {}

    /// 订阅行情
    fn subscribe(&mut self) {}

    /// 取消订阅行情
    fn unsubscribe(&mut self, symbol: String) {}

    /// 释放退出接口
    fn exit(&mut self) {}

    fn req_account(&mut self) {}

    fn req_position(&mut self) {}
}
