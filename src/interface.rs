#![allow(dead_code, unused_variables)]

use crate::structs::{OrderRequest, CancelRequest, LoginForm};

/// 用户登录接口,包含用户的
pub trait Interface {
    /// 发单
    fn send_order(&mut self, order: OrderRequest) -> String { unimplemented!() }
    /// 撤单
    fn cancel_order(&mut self, req: CancelRequest) -> String { unimplemented!() }

    fn connect(&mut self, req: LoginForm) -> bool { unimplemented!() }
    /// 订阅行情
    fn subscribe(&mut self, symbol: String) -> bool { unimplemented!() }
}

