use core::fmt;
use std::borrow::Cow;
use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int, c_uchar, c_void};

use encoding::all::GB18030;
use encoding::{DecoderTrap, Encoding};

use crate::{get_interface_path, os_path};
use chrono::{NaiveDate, NaiveTime, Timelike};
use crate::c_func::translate_zh_to_string;

#[cfg(not(target_os = "windows"))]
include!(concat!(env!("HOME"), "/.HFQ/ctp/bindings.rs"));

#[cfg(target_os = "windows")]
include!(concat!(
env!("HOMEDRIVE"),
env!("HOMEPATH"),
"/.HFQ/ctp/bindings.rs"
));

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DisconnectionReason {
    ReadError = 0x1001,
    WriteError = 0x1002,
    HeartbeatTimeout = 0x2001,
    HeartbeatSendError = 0x2002,
    ErrorMessageReceived = 0x2003,
    Unknown = 0x0000,
}

impl std::convert::From<c_int> for DisconnectionReason {
    fn from(reason: c_int) -> DisconnectionReason {
        match reason {
            0x1001 => DisconnectionReason::ReadError,
            0x1002 => DisconnectionReason::WriteError,
            0x2001 => DisconnectionReason::HeartbeatTimeout,
            0x2002 => DisconnectionReason::HeartbeatSendError,
            0x2003 => DisconnectionReason::ErrorMessageReceived,
            _ => DisconnectionReason::Unknown,
        }
    }
}

#[must_use]
pub type RspResult = Result<(), RspError>;

#[derive(Clone, Debug, PartialEq)]
pub struct RspError {
    pub id: TThostFtdcErrorIDType,
    pub msg: String,
}

impl std::error::Error for RspError {}

impl fmt::Display for RspError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.id, self.msg)
    }
}

pub fn result_to_string(rsp_result: &RspResult) -> String {
    match rsp_result {
        Ok(()) => "Ok(())".to_string(),
        Err(err) => format!("Err(RspError{{ id: {}, msg: {} }})", err.id, err.msg),
    }
}

pub fn get_rsp_info(rsp_info: *const CThostFtdcRspInfoField) -> RspResult {
    #[allow(unused_unsafe)] // for future "unsafe blocks in unsafe fn" feature
    match unsafe { rsp_info.as_ref() } {
        Some(info) => match info {
            CThostFtdcRspInfoField { ErrorID: 0, .. } => Ok(()),
            CThostFtdcRspInfoField {
                ErrorID: id,
                ErrorMsg: msg,
            } => Err(RspError {
                id: *id,
                msg: translate_zh_to_string(msg),
            }),
        },
        None => Ok(()),
    }
}


pub trait ToCSlice<T> {
    fn to_c_slice(&self) -> T;
}

macro_rules! impl_to_c_slice {
    ($len: expr) => (
        impl ToCSlice<[i8; $len]> for &str {
            fn to_c_slice(&self) -> [i8; $len] {
                let mut array = [0i8; $len];
                let mut i = 0;
                let size = array.len();
                for x in self.chars() {
                    if i >= size {
                        break;
                    }
                    array[i] = x as i8;
                    i += 1;
                }
                array
            }
        }
        impl ToCSlice<[i8; $len]> for String {
            fn to_c_slice(&self) -> [i8; $len] {
                let mut array = [0i8; $len];
                let mut i = 0;
                let size = array.len();
                for x in self.chars() {
                    if i >= size {
                        break;
                    }
                    array[i] = x as i8;
                    i += 1;
                }
                array
            }
        }
    );
    ($len:expr, $($len2:expr),+) => (
        impl_to_c_slice!($len);
        impl_to_c_slice!($($len2),+);
    )
}

impl_to_c_slice!(
    301, 11, 2561, 349, 100, 501, 129, 9, 31, 12, 17, 13, 51, 71, 25, 10, 513, 21, 256, 365, 36,
    15, 16, 20, 23, 5, 41, 257, 3, 2, 33, 1001, 201, 273, 65, 401, 261, 24, 61, 4, 81, 161, 2049,
    6, 22, 1025, 7, 101
);

const ORDER_ID_LENGTH: usize = 12usize;

/// 拆分訂單號爲 index和id
/// todo: 減少二重判斷
pub fn split_into_vec(order_id: &str) -> (usize, i32) {
    if order_id.len().eq(&ORDER_ID_LENGTH) {
        (
            order_id[9..12].parse::<usize>().unwrap_or(10000000),
            order_id[0..9].parse::<i32>().unwrap_or(10000000),
        )
    } else {
        (10000000 as usize, order_id.parse::<i32>().unwrap())
    }
}

#[cfg(test)]
mod test {
    use crate::ctp::sys::{split_into_vec, ToCSlice};

    #[test]
    fn test_split_into_vec() {
        let data = "000000000000";
        assert_eq!(data.len(), 12usize);
        let (index, id) = split_into_vec(data);
        assert_eq!(index, 0usize);
        assert_eq!(id, 0i32);

        let data2 = "450201";
        let (id, index) = split_into_vec(data2);
        assert_eq!(id, 10000000usize);
        assert_eq!(index, 450201i32);
    }

    #[test]
    fn test_macro_use() {
        let a = "hello";
        let x: [i8; 5] = a.to_c_slice();
        assert_eq!(x, [104, 101, 108, 108, 111]);
    }
}
