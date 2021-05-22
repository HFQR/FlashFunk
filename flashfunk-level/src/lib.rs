#![allow(clippy::mutex_atomic)]
#![allow(clippy::type_complexity)]

/// In this crate, it provides the data_type and constants.
/// Also, provide the most useful interface like ctp or ctp_mini
///
#[macro_use]
extern crate bitflags;

use std::path::PathBuf;
use std::env::var;

pub mod data_type;
pub mod constant;
pub mod context;
pub mod interface;

#[cfg(feature = "ctp")]
mod ctp;

#[cfg(feature = "ctp")]
pub use ctp::md_api::CtpMdApi;
#[cfg(feature = "ctp")]
pub use ctp::td_api::CtpTdApi;


pub mod util;
pub mod types;


#[cfg(not(target_os = "windows"))]
fn os_path(target: &str) -> String {
    let path = var("HOME").unwrap();
    path.join(".HFQ").join(target)
}

#[cfg(target_os = "windows")]
fn os_path(target: &str) -> PathBuf {
    let path = PathBuf::from(format!("{}{}", var("HOMEDRIVE").unwrap(), var("HOMEPATH").unwrap()));
    path.join(".HFQ").join(target)
}


fn get_interface_path(interface: &str) -> PathBuf {
    let home = os_path(interface);
    home.join("bindings.rs")
}
