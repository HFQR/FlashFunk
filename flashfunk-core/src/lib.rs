#![allow(clippy::mutex_atomic)]
#![allow(clippy::type_complexity)]

pub mod account;
mod app;
mod builder;
mod data_collect;
mod mock;
mod timer;
mod worker;

pub use data_collect::get_ticks;
pub use mock::MockMdApi;
pub use mock::MockTdApi;

pub mod prelude {
    pub use crate::app::Flash;
    pub use flashfunk_level::constant::*;
    pub use flashfunk_level::context::{Context, ContextTrait};
    pub use flashfunk_level::data_type::*;
    pub use flashfunk_level::interface::Ac;
}

// pub use util::mock::{MockMdApi, MockTdApi};
pub use flashfunk_level::util::channel::GroupSender;
