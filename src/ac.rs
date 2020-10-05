use core::ops::{Deref, DerefMut};

use crate::app::StrategyWorkerContext;
use crate::constants::{Direction, Exchange, Offset, OrderType, Status};
use crate::structs::{AccountData, ContractData, OrderData, PositionData, TickData, TradeData};

pub trait IntoStrategy: Sized + Send + Ac + 'static {
    fn into_str(self) -> __Strategy {
        __Strategy {
            str: Box::new(self),
            name: Self::name(),
            symbols: Self::local_symbol(),
        }
    }

    fn name() -> &'static str;

    fn local_symbol() -> Vec<&'static str>;
}

pub struct __Strategy {
    str: Box<dyn Ac + Send>,
    name: &'static str,
    symbols: Vec<&'static str>,
}

impl __Strategy {
    pub fn symbols(&self) -> &[&'static str] {
        &self.symbols
    }
}

impl Deref for __Strategy {
    type Target = Box<dyn Ac + Send>;

    fn deref(&self) -> &Self::Target {
        &self.str
    }
}

impl DerefMut for __Strategy {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.str
    }
}

#[allow(unused_variables)]
pub trait Ac {
    fn on_tick(&mut self, tick: &TickData, ctx: &mut StrategyWorkerContext);

    fn on_contract(&mut self, contract: &ContractData, ctx: &mut StrategyWorkerContext) {}

    fn on_position(&mut self, position: &PositionData, ctx: &mut StrategyWorkerContext) {}

    fn on_trade(&mut self, trade: &TradeData, ctx: &mut StrategyWorkerContext) {}

    fn on_order(&mut self, order: &OrderData, ctx: &mut StrategyWorkerContext) {}

    fn on_account(&mut self, account: &AccountData, ctx: &mut StrategyWorkerContext) {}

    fn on_realtime(&mut self, ctx: &mut StrategyWorkerContext) {}
}
