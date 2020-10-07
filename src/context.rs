use crate::app::StrategyMessage;
use crate::constants::{Direction, Exchange, Status};
use crate::structs::{AccountData, ContractData, OrderData, Position, PositionData};
use crate::util::channel::Sender;
use crate::util::hash::HashMap;

pub type Context<'a> = (&'a Sender<StrategyMessage>, ContextInner);

pub fn new_context(sender: &Sender<StrategyMessage>) -> Context {
    let inner = ContextInner {
        order_map: Default::default(),
        contract_map: Default::default(),
        exchange_map: Default::default(),
        position_map: Default::default(),
        account: AccountData::default(),
    };

    (sender, inner)
}

pub struct ContextInner {
    order_map: HashMap<String, OrderData>,
    contract_map: HashMap<String, ContractData>,
    exchange_map: HashMap<String, Exchange>,
    position_map: HashMap<&'static str, Position>,
    account: AccountData,
}

impl ContextInner {
    pub fn add_order(&mut self, order: OrderData) {
        self.order_map.insert(order.orderid.clone().unwrap(), order);
    }

    pub fn get_active_orders(&mut self) -> impl Iterator<Item=&OrderData> {
        self.order_map
            .iter()
            .filter(|(_, v)| Status::ACTIVE_IN.contains(v.status))
            .map(|(_, v)| v)
    }

    pub fn get_order(&mut self, order_id: &str) -> Option<&OrderData> {
        self.order_map.get(order_id)
    }

    pub fn get_active_ids(&mut self) -> impl Iterator<Item=&str> {
        self.order_map
            .iter()
            .filter(|(_, v)| Status::ACTIVE_IN.contains(v.status))
            .map(|(i, _)| i.as_str())
    }

    pub fn get_order_ids(&mut self) -> impl Iterator<Item=&str> {
        self.order_map.iter().map(|(i, _)| i.as_str())
    }

    pub fn get_exchange(&mut self, symbol: &str) -> Option<&Exchange> {
        self.exchange_map.get(symbol)
    }

    pub fn get_contract(&mut self, symbol: &str) -> Option<&ContractData> {
        self.contract_map.get(symbol)
    }

    pub fn position_mut(&mut self, symbol: &'static str) -> &mut Position {
        self.position_map
            .entry(symbol)
            .or_insert_with(|| Position::new_with_symbol(symbol))
    }

    pub fn get_account(&mut self) -> &AccountData {
        &self.account
    }

    pub fn insert_exchange(&mut self, symbol: &str, exchange: Exchange) {
        self.exchange_map.insert(symbol.parse().unwrap(), exchange);
    }

    pub fn insert_position(&mut self, position_data: &PositionData) {
        match position_data.direction.unwrap() {
            Direction::LONG => {
                self.position_map
                    .get_mut(position_data.symbol)
                    .map(|p| {
                        p.long_volume = position_data.volume;
                        p.long_price = position_data.price;
                        p.long_available = position_data.available;
                        p.long_frozen = position_data.frozen;
                        p.long_yd_volume = position_data.yd_volume;
                        p.long_pnl = position_data.pnl;
                    })
                    .unwrap_or_else(|| {
                        self.position_map.insert(
                            position_data.symbol,
                            Position::new_with_long(
                                position_data.symbol,
                                position_data.volume,
                                position_data.price,
                                position_data.available,
                                position_data.frozen,
                                position_data.yd_volume,
                                position_data.pnl,
                            ),
                        );
                    });
            }
            Direction::SHORT => {
                self.position_map
                    .get_mut(position_data.symbol)
                    .map(|p| {
                        p.short_volume = position_data.volume;
                        p.short_price = position_data.price;
                        p.short_available = position_data.available;
                        p.short_frozen = position_data.frozen;
                        p.short_yd_volume = position_data.yd_volume;
                        p.short_pnl = position_data.pnl;
                    })
                    .unwrap_or_else(|| {
                        self.position_map.insert(
                            position_data.symbol,
                            Position::new_with_short(
                                position_data.symbol,
                                position_data.volume,
                                position_data.price,
                                position_data.available,
                                position_data.frozen,
                                position_data.yd_volume,
                                position_data.pnl,
                            ),
                        );
                    });
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
        F: FnOnce(&Sender<StrategyMessage>, &mut ContextInner);

    fn send(&self, m: impl Into<StrategyMessage>);

    fn add_order(&mut self, order: OrderData);

    fn get_active_orders(&mut self) -> Box<dyn Iterator<Item=&OrderData> + '_>;

    fn get_order(&mut self, order_id: &str) -> Option<&OrderData>;

    fn get_active_ids(&mut self) -> Box<dyn Iterator<Item=&str> + '_>;

    fn get_order_ids(&mut self) -> Box<dyn Iterator<Item=&str> + '_>;

    fn get_exchange(&mut self, symbol: &str) -> Option<&Exchange>;

    fn get_contract(&mut self, symbol: &str) -> Option<&ContractData>;

    fn get_position(&mut self, symbol: &'static str) -> &Position;

    fn get_account(&mut self) -> &AccountData;

    fn update_position_by_price(&mut self, price: f64);

    fn insert_position(&mut self, position_data: &PositionData);

    fn insert_order(&mut self, order: &OrderData);
}

impl ContextTrait for Context<'_> {
    fn enter<F>(&mut self, f: F)
    where
        F: FnOnce(&Sender<StrategyMessage>, &mut ContextInner),
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

    fn get_active_orders(&mut self) -> Box<dyn Iterator<Item=&OrderData> + '_> {
        Box::new(self.1.get_active_orders())
    }

    fn get_order(&mut self, order_id: &str) -> Option<&OrderData> {
        self.1.get_order(order_id)
    }

    fn get_active_ids(&mut self) -> Box<dyn Iterator<Item=&str> + '_> {
        Box::new(self.1.get_active_ids())
    }

    fn get_order_ids(&mut self) -> Box<dyn Iterator<Item=&str> + '_> {
        Box::new(self.1.get_order_ids())
    }

    fn get_exchange(&mut self, symbol: &str) -> Option<&Exchange> {
        self.1.get_exchange(symbol)
    }

    fn get_contract(&mut self, symbol: &str) -> Option<&ContractData> {
        self.1.get_contract(symbol)
    }

    fn get_position(&mut self, symbol: &'static str) -> &Position {
        self.1.position_mut(symbol)
    }

    fn get_account(&mut self) -> &AccountData {
        self.1.get_account()
    }

    fn update_position_by_price(&mut self, price: f64) {
        unimplemented!()
    }

    fn insert_position(&mut self, position_data: &PositionData) {
        self.1.insert_position(position_data);
    }

    fn insert_order(&mut self, order: &OrderData) {
        self.1.insert_order(order);
    }
}
