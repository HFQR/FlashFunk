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
fn os_path() -> String {
    var("HOME").unwrap()
}

#[cfg(target_os = "windows")]
fn os_path() -> PathBuf {
    PathBuf::from(format!("{}{}", var("HOMEDRIVE").unwrap(), var("HOMEPATH").unwrap()))
}


#[cfg(not(target_os = "windows"))]
fn os_path() -> PathBuf {
    PathBuf::from(var("HOME").unwrap())
}

#[cfg(target_os = "windows")]
fn get_interface_path(interface: &str) -> PathBuf {
    let home = os_path();
    home.join(interface).join("bindings.rs")
}
