#[macro_use]
extern crate bitflags;

use std::env::var;
use std::path::PathBuf;

mod app;
mod builder;
mod context;
mod util;
mod worker;

pub mod ac;
pub mod account;
pub mod constants;
pub mod ctp;
pub mod interface;
pub mod structs;
pub mod types;

pub mod prelude {
    pub use crate::ac::{Ac, IntoStrategy};
    pub use crate::app::CtpbeeR;
    pub use crate::context::{Context, ContextTrait};
    pub use crate::types::message::StrategyMessage;
}

pub use util::mock::MockMdApi;

fn get_interface_path(path: &str) -> PathBuf {
    let px = format!("{}/HFQ/{}", var("HOME").unwrap(), path);
    let path_buffer = PathBuf::from(px);
    if !path_buffer.exists() {
        panic!("please mkdier interface dir fisrt");
    }
    path_buffer.join("bindings.rs")
}
