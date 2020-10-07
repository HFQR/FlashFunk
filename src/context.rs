use crate::app::StrategyMessage;
use crate::constants::{Exchange, Status, Direction};
use crate::structs::{AccountData, ContractData, OrderData, Position, PositionData};
use crate::util::channel::Sender;
use crate::util::hash::HashMap;
use std::borrow::Borrow;

pub type Context<'a> = (&'a Sender<StrategyMessage>, StrategyWorkerContextInner);

pub fn new_context(sender: &Sender<StrategyMessage>) -> Context {
    let inner = StrategyWorkerContextInner {
        order_map: Default::default(),
        contract_map: Default::default(),
        exchange_map: Default::default(),
        position_map: Default::default(),
        account: AccountData::default(),
    };

    (sender, inner)
}

pub struct StrategyWorkerContextInner {
    order_map: HashMap<String, OrderData>,
    contract_map: HashMap<String, ContractData>,
    exchange_map: HashMap<String, Exchange>,
    position_map: HashMap<String, Position>,
    account: AccountData,
}

impl StrategyWorkerContextInner {
    pub fn add_order(&mut self, order: OrderData) {
        self.order_map.insert(order.orderid.clone().unwrap(), order);
    }

    pub fn get_active_orders(&mut self) -> Vec<&OrderData> {
        self.order_map
            .iter()
            .filter(|(_, v)| Status::ACTIVE_IN.contains(v.status))
            .map(|(_, v)| v)
            .collect()
    }

    pub fn get_order(&mut self, order_id: &str) -> Option<&OrderData> {
        self.order_map.get(order_id)
    }

    pub fn get_active_ids(&mut self) -> Vec<&str> {
        self.order_map
            .iter()
            .filter(|(_, v)| Status::ACTIVE_IN.contains(v.status))
            .map(|(i, _)| i.as_str())
            .collect()
    }

    pub fn get_order_ids(&mut self) -> Vec<&str> {
        self.order_map.iter().map(|x| x.0.as_str()).collect()
    }

    pub fn get_exchange(&mut self, symbol: &str) -> Option<&Exchange> {
        self.exchange_map.get(symbol)
    }

    pub fn get_contract(&mut self, symbol: &str) -> Option<&ContractData> {
        self.contract_map.get(symbol)
    }

    pub fn get_position(&mut self, symbol: &str) -> &Position {
        // fixme: a bad implementation why not have get_or_insert()?
        //  code review need
        self.position_map.get(symbol).unwrap()
    }
    pub fn get_account(&mut self) -> &AccountData {
        self.account.borrow()
    }

    pub fn insert_exchange(&mut self, symbol: &str, exchange: &Exchange) {
        self.exchange_map.insert(symbol.parse().unwrap(), *exchange);
    }

    pub fn insert_position(&mut self, position_data: &PositionData) {
        // should promise the position_data's direction has bee set
        // fixme: should provide a more effective way to insert and update tick price
        match position_data.direction.unwrap() {
            Direction::LONG => {
                let pos = self.position_map.get_mut(position_data.symbol.as_str()).unwrap();
                pos.long_volume = position_data.volume;
                pos.long_price = position_data.price;
                pos.long_available = position_data.available;
                pos.long_frozen = position_data.frozen;
                pos.long_yd_volume = position_data.yd_volume;
                pos.long_pnl = position_data.pnl;
            }
            Direction::SHORT => {
                let pos = self.position_map.get_mut(position_data.symbol.as_str()).unwrap();
                pos.short_volume = position_data.volume;
                pos.short_price = position_data.price;
                pos.short_available = position_data.available;
                pos.short_frozen = position_data.frozen;
                pos.short_yd_volume = position_data.yd_volume;
                pos.short_pnl = position_data.pnl;
            }
            _ => {}
        }
    }

    pub fn insert_order(&mut self, order: &OrderData) {
        // fix an complex calulation
        // if
    }

}

pub trait ContextTrait {
    fn enter<F>(&mut self, f: F)
        where
            F: FnOnce(&Sender<StrategyMessage>, &mut StrategyWorkerContextInner);

    fn send(&self, m: impl Into<StrategyMessage>);

    fn add_order(&mut self, order: OrderData);

    fn get_active_orders(&mut self) -> Vec<&OrderData>;

    fn get_order(&mut self, order_id: &str) -> Option<&OrderData>;

    fn get_active_ids(&mut self) -> Vec<&str>;

    fn get_order_ids(&mut self) -> Vec<&str>;

    fn get_exchange(&mut self, symbol: &str) -> Option<&Exchange>;

    fn get_contract(&mut self, symbol: &str) -> Option<&ContractData>;

    fn get_position(&mut self, symbol: &str) -> &Position;

    fn get_account(&mut self) -> &AccountData;

    fn update_position_by_price(&mut self, price: f64);

    fn update_position_by_pos(&mut self, position_data: &PositionData);

    fn insert_order(&mut self, order: &OrderData);

    fn init_pos(&mut self, symbol: &str);
}

impl ContextTrait for Context<'_> {
    fn enter<F>(&mut self, f: F)
        where
            F: FnOnce(&Sender<StrategyMessage>, &mut StrategyWorkerContextInner),
    {
        let (sender, inner) = self;
        f(*sender, inner);
    }

    fn send(&self, m: impl Into<StrategyMessage>) {
        self.0.send(m.into());
    }

    fn add_order(&mut self, order: OrderData) {
        self.1.add_order(order);
    }

    fn get_active_orders(&mut self) -> Vec<&OrderData> {
        self.1.get_active_orders()
    }

    fn get_order(&mut self, order_id: &str) -> Option<&OrderData> {
        self.1.get_order(order_id)
    }

    fn get_active_ids(&mut self) -> Vec<&str> {
        self.1.get_active_ids()
    }

    fn get_order_ids(&mut self) -> Vec<&str> {
        self.1.get_order_ids()
    }

    fn get_exchange(&mut self, symbol: &str) -> Option<&Exchange> {
        self.1.get_exchange(symbol)
    }

    fn get_contract(&mut self, symbol: &str) -> Option<&ContractData> {
        self.1.get_contract(symbol)
    }

    fn get_position(&mut self, symbol: &str) -> &Position {
        self.1.get_position(symbol)
    }

    fn get_account(&mut self) -> &AccountData {
        self.1.get_account()
    }

    fn update_position_by_price(&mut self, price: f64) {
        unimplemented!()
    }

    fn update_position_by_pos(&mut self, position_data: &PositionData) {
        self.1.insert_position(position_data);
    }

    fn insert_order(&mut self, order: &OrderData) {
        self.1.insert_order(order);
    }

    fn init_pos(&mut self, symbol: &str) {
        self.1.ini
    }
}
