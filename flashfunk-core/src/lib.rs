#![allow(clippy::mutex_atomic)]
#![allow(clippy::type_complexity)]


use std::env::var;
use std::path::PathBuf;

mod app;
mod builder;
mod worker;
mod timer;
mod data_collect;
mod mock;
pub mod account;

pub use data_collect::get_ticks;
pub use mock::MockMdApi;
pub use mock::MockTdApi;

pub mod prelude {
    pub use flashfunk_level::interface::{Ac};
    pub use crate::app::Flash;
    pub use flashfunk_level::context::{Context, ContextTrait};
    pub use flashfunk_level::types::message::StrategyMessage;
}

// pub use util::mock::{MockMdApi, MockTdApi};
pub use flashfunk_level::util::channel::{GroupSender};

