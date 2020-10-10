use std::borrow::Cow;

use crate::constants::{Direction, Exchange, Offset, Status};
use crate::structs::{AccountData, ContractData, OrderData, Position, PositionData};
use crate::types::message::StrategyMessage;
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
    position_map: HashMap<Cow<'static, str>, Position>,
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
    pub fn position_mut(&mut self, symbol: &str) -> &mut Position {
        self.position_map
            .entry(Cow::Owned(symbol.to_owned()))
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
                    .get_mut(position_data.symbol.as_ref())
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
                            position_data.symbol.clone(),
                            Position::new_with_long(
                                position_data.symbol.as_ref(),
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
                    .get_mut(position_data.symbol.as_ref())
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
                            position_data.symbol.clone(),
                            Position::new_with_short(
                                position_data.symbol.as_ref(),
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
        match order.status {
            Status::NOTTRADED => {}
            Status::ALLTRADED => {
                self.update_trade(order);
            }
            Status::PARTTRADED => {
                // fixme we should get a more useful way to cal pos when get PartTraded
                // self.update_trade(order);
            }
            Status::CANCELLED => {}
            Status::SUBMITTING => {}
            _ => {}
        }
    }

    fn update_trade(&mut self, order: &OrderData) {
        // should check the update logic
        let mut pos = self.position_mut(order.symbol.as_str());
        match order.direction.unwrap() {
            Direction::LONG => {
                match order.offset {
                    Offset::OPEN => {
                        pos.long_price = ((pos.long_price) * pos.long_volume
                            + order.price * order.traded)
                            / (pos.long_volume + order.traded);
                        pos.long_volume += order.traded;
                    }
                    Offset::CLOSETODAY => {
                        if order.traded == pos.short_volume {
                            pos.short_volume = 0.0;
                            pos.short_price = 0.0;
                        } else if order.traded < pos.short_volume {
                            pos.short_price = (pos.short_price * pos.short_volume
                                - order.price * order.volume)
                                / (pos.short_volume - order.traded);
                            pos.short_volume -= order.traded;
                        }
                    }
                    Offset::CLOSE => match order.exchange {
                        Exchange::ACTIVE_TODAY => {
                            pos.short_yd_volume -= order.traded;
                            pos.short_price = (pos.short_price * pos.short_volume
                                - order.traded * order.price)
                                / (pos.short_volume - order.traded);
                            pos.short_volume -= order.traded;
                        }

                        _ => {
                            pos.short_price = (pos.short_price * pos.short_volume
                                - order.traded * order.price)
                                / (pos.short_volume - order.traded);
                            pos.short_volume -= order.traded;
                        }
                    },
                    Offset::CLOSEYESTERDAY => {
                        // 平昨數量剛好等於昨倉數量
                        if order.traded == pos.short_volume {
                            pos.short_yd_volume = 0.0;
                            pos.short_price = 0.0;
                            pos.short_volume = pos.short_volume - order.traded;
                        } else if order.traded < pos.short_volume
                            && order.traded <= pos.short_yd_volume
                        {
                            pos.short_price = (pos.short_price * pos.short_volume
                                - order.traded * order.price)
                                / (pos.short_volume - order.traded);
                            pos.short_yd_volume -= order.traded;
                            pos.short_volume -= order.traded;
                        }
                    }
                    _ => {}
                }
            }
            Direction::SHORT => {
                match order.offset {
                    Offset::OPEN => {
                        pos.short_price = ((pos.short_price) * pos.short_volume
                            + order.price * order.traded)
                            / (pos.short_volume + order.traded);
                        pos.short_volume += order.traded;
                    }
                    Offset::CLOSETODAY => {
                        if order.traded == pos.long_volume {
                            pos.long_volume = 0.0;
                            pos.long_price = 0.0;
                        } else if order.traded < pos.long_volume {
                            pos.long_price = (pos.long_price * pos.long_volume
                                - order.price * order.traded)
                                / (pos.long_volume - order.traded);
                            pos.long_volume -= order.traded;
                        }
                    }
                    Offset::CLOSE => {
                        match order.exchange {
                            Exchange::ACTIVE_TODAY => {
                                //
                                pos.long_yd_volume -= order.traded;
                                pos.long_price = (pos.long_price * pos.short_volume
                                    - order.traded * order.price)
                                    / (pos.long_volume - order.traded);
                                pos.long_volume -= order.traded;
                            }
                            _ => {
                                pos.long_price = (pos.long_price * pos.short_volume
                                    - order.traded * order.price)
                                    / (pos.long_volume - order.traded);
                                pos.long_volume -= order.traded;
                            }
                        }
                    }
                    Offset::CLOSEYESTERDAY => {
                        // 平昨數量剛好等於昨倉數量
                        if order.traded == pos.long_volume {
                            pos.long_yd_volume = 0.0;
                            pos.long_price = 0.0;
                            pos.long_volume -= order.traded;
                        } else if order.traded < pos.long_volume
                            && order.traded <= pos.long_yd_volume
                        {
                            pos.long_price = (pos.long_price * pos.long_volume
                                - order.traded * order.price)
                                / (pos.long_volume - order.traded);
                            pos.long_yd_volume -= order.traded;
                            pos.long_volume -= order.traded;
                        }
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }

    pub fn update_account(&mut self, account: &AccountData) {
        self.account.balance = account.balance;
        self.account.frozen = account.frozen;
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

    fn update_account(&mut self, account: &AccountData);

    fn update_position_by_price(&mut self, price: f64);

    fn update_position_by_pos(&mut self, position_data: &PositionData);
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
        self.1.insert_order(&order);
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

    fn get_position(&mut self, symbol: &str) -> &Position {
        self.1.position_mut(symbol)
    }

    fn get_account(&mut self) -> &AccountData {
        self.1.get_account()
    }

    fn update_account(&mut self, account: &AccountData) {
        self.1.update_account(account);
    }

    fn update_position_by_price(&mut self, price: f64) {
        unimplemented!()
    }

    fn update_position_by_pos(&mut self, position_data: &PositionData) {
        self.1.insert_position(position_data);
    }
}
