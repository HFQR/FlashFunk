use crate::context::Context;
use crate::data_type::{ContractData, PositionData, TradeData, OrderData, AccountData, CancelRequest, LoginForm, OrderRequest, TickData};
use crate::util::channel::GroupSender;

/// Core trait
/// In this project it provide the foreign interface to solve data
#[allow(unused_variables)]
pub trait Ac {
    fn on_tick(&mut self, tick: &TickData, ctx: &mut Context);

    fn on_contract(&mut self, contract: &ContractData, ctx: &mut Context) {}

    fn on_position(&mut self, position: &PositionData, ctx: &mut Context) {}

    fn on_trade(&mut self, trade: &TradeData, ctx: &mut Context) {}

    fn on_order(&mut self, order: &OrderData, ctx: &mut Context) {}

    fn on_account(&mut self, account: &AccountData, ctx: &mut Context) {}

    fn on_realtime(&mut self, ctx: &mut Context) {}

    fn on_l2_order(&mut self, order: &OrderData, ctx: &mut Context) {}

    fn on_l2_trade(&mut self, order: &TradeData, ctx: &mut Context) {}

    fn local_symbols<'a>(&mut self) -> Vec<&'a str>;

    fn name<'a>(&mut self) -> &'a str;
}


/// 用户登录接口,包含用户的
pub trait Interface {
    type Message;

    fn new(
        id: impl Into<Vec<u8>>,
        pwd: impl Into<Vec<u8>>,
        symbols: Vec<&'static str>,
        req: &LoginForm,
        sender: GroupSender<Self::Message>,
    ) -> Self;

    /// 发单
    fn send_order(&mut self, idx: usize, order: OrderRequest) {}

    /// 撤单
    fn cancel_order(&mut self, req: CancelRequest) {}

    /// 登录接口
    fn connect(&mut self) {}

    /// 订阅行情
    fn subscribe(&mut self) {}

    /// 取消订阅行情
    fn unsubscribe(&mut self, symbol: String) {}

    /// 释放退出接口
    fn exit(&mut self) {}

    /// 查詢賬戶
    fn req_account(&mut self) {}

    /// 查詢持倉
    fn req_position(&mut self) {}

    /// 更新交易接口的訂單
    fn update_quote(&mut self, quote: &TickData) {}
}
