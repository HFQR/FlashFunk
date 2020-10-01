use crate::get_interface_path;
use core::fmt;
use encoding::all::GB18030;
use encoding::{DecoderTrap, Encoding};
use std::borrow::Cow;
use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int, c_uchar, c_void};

include!(concat!(env!("HOME"), "/.HFQ/ctp/bindings.rs"));

pub fn to_c_ptr(string: String) -> *const i8 {
    CString::new(string).unwrap().as_ptr()
}

pub fn to_c_string(string: String) -> CString {
    CString::new(string).unwrap()
}

pub fn to_i8_arrary(string: String) -> Vec<i8> {
    CString::new(string)
        .unwrap()
        .as_bytes()
        .into_iter()
        .map(|x| *x as i8)
        .collect()
}

pub fn to_c_str<'a>(string: String) -> &'a CStr {
    unsafe { CStr::from_ptr(to_c_string(string).as_ptr()) }
}

pub trait ToCSlice<T> {
    fn to_c_slice(&self) -> T;
}

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
                msg: slice_to_string(msg),
            }),
        },
        None => Ok(()),
    }
}

pub fn zh_cstr_to_str(v: &[u8]) -> Cow<str> {
    let slice = v.split(|&c| c == 0u8).next().unwrap();
    if slice.is_ascii() {
        unsafe {
            return Cow::Borrowed::<str>(std::str::from_utf8_unchecked(slice));
        }
    }
    match GB18030.decode(slice, DecoderTrap::Replace) {
        Ok(s) => Cow::Owned(s),
        Err(e) => e,
    }
}

/// todo: 下面有问题描述
pub fn slice_to_string(v: &[i8]) -> String {
    let x: Vec<u8> = v.to_vec().iter().map(|x| *x as u8).collect();
    zh_cstr_to_str(x.as_slice()).to_string()
}

impl ToCSlice<[i8; 2561]> for String {
    fn to_c_slice(&self) -> [i8; 2561] {
        let mut array = [0i8; 2561];
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

impl ToCSlice<[i8; 24]> for String {
    fn to_c_slice(&self) -> [i8; 24] {
        let mut array = [0i8; 24];
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

impl ToCSlice<[i8; 61]> for String {
    fn to_c_slice(&self) -> [i8; 61] {
        let mut array = [0i8; 61];
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

impl ToCSlice<[i8; 22]> for String {
    fn to_c_slice(&self) -> [i8; 22] {
        let mut array = [0i8; 22];
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

impl ToCSlice<[i8; 41]> for String {
    fn to_c_slice(&self) -> [i8; 41] {
        let mut array = [0i8; 41];
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

impl ToCSlice<[i8; 7]> for String {
    fn to_c_slice(&self) -> [i8; 7] {
        let mut array = [0i8; 7];
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

impl ToCSlice<[i8; 365]> for String {
    fn to_c_slice(&self) -> [i8; 365] {
        let mut array = [0i8; 365];
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

impl ToCSlice<[i8; 15]> for String {
    fn to_c_slice(&self) -> [i8; 15] {
        let mut array = [0i8; 15];
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

impl ToCSlice<[i8; 161]> for String {
    fn to_c_slice(&self) -> [i8; 161] {
        let mut array = [0i8; 161];
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

impl ToCSlice<[i8; 11]> for String {
    fn to_c_slice(&self) -> [i8; 11] {
        let mut array = [0i8; 11];
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

impl ToCSlice<[i8; 1001]> for String {
    fn to_c_slice(&self) -> [i8; 1001] {
        let mut array = [0i8; 1001];
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

impl ToCSlice<[i8; 31]> for String {
    fn to_c_slice(&self) -> [i8; 31] {
        let mut array = [0i8; 31];
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

impl ToCSlice<[i8; 301]> for String {
    fn to_c_slice(&self) -> [i8; 301] {
        let mut array = [0i8; 301];
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

impl ToCSlice<[i8; 21]> for String {
    fn to_c_slice(&self) -> [i8; 21] {
        let mut array = [0i8; 21];
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

impl ToCSlice<[i8; 4]> for String {
    fn to_c_slice(&self) -> [i8; 4] {
        let mut array = [0i8; 4];
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

impl ToCSlice<[i8; 256]> for String {
    fn to_c_slice(&self) -> [i8; 256] {
        let mut array = [0i8; 256];
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

impl ToCSlice<[i8; 349]> for String {
    fn to_c_slice(&self) -> [i8; 349] {
        let mut array = [0i8; 349];
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

impl ToCSlice<[i8; 71]> for String {
    fn to_c_slice(&self) -> [i8; 71] {
        let mut array = [0i8; 71];
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

impl ToCSlice<[i8; 23]> for String {
    fn to_c_slice(&self) -> [i8; 23] {
        let mut array = [0i8; 23];
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

impl ToCSlice<[i8; 17]> for String {
    fn to_c_slice(&self) -> [i8; 17] {
        let mut array = [0i8; 17];
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

impl ToCSlice<[i8; 261]> for String {
    fn to_c_slice(&self) -> [i8; 261] {
        let mut array = [0i8; 261];
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

impl ToCSlice<[i8; 401]> for String {
    fn to_c_slice(&self) -> [i8; 401] {
        let mut array = [0i8; 401];
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

impl ToCSlice<[i8; 51]> for String {
    fn to_c_slice(&self) -> [i8; 51] {
        let mut array = [0i8; 51];
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

impl ToCSlice<[i8; 201]> for String {
    fn to_c_slice(&self) -> [i8; 201] {
        let mut array = [0i8; 201];
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

impl ToCSlice<[i8; 25]> for String {
    fn to_c_slice(&self) -> [i8; 25] {
        let mut array = [0i8; 25];
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

impl ToCSlice<[i8; 9]> for String {
    fn to_c_slice(&self) -> [i8; 9] {
        let mut array = [0i8; 9];
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

impl ToCSlice<[i8; 20]> for String {
    fn to_c_slice(&self) -> [i8; 20] {
        let mut array = [0i8; 20];
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

impl ToCSlice<[i8; 12]> for String {
    fn to_c_slice(&self) -> [i8; 12] {
        let mut array = [0i8; 12];
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

impl ToCSlice<[i8; 5]> for String {
    fn to_c_slice(&self) -> [i8; 5] {
        let mut array = [0i8; 5];
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

impl ToCSlice<[i8; 501]> for String {
    fn to_c_slice(&self) -> [i8; 501] {
        let mut array = [0i8; 501];
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

impl ToCSlice<[i8; 81]> for String {
    fn to_c_slice(&self) -> [i8; 81] {
        let mut array = [0i8; 81];
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

impl ToCSlice<[i8; 257]> for String {
    fn to_c_slice(&self) -> [i8; 257] {
        let mut array = [0i8; 257];
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

impl ToCSlice<[i8; 65]> for String {
    fn to_c_slice(&self) -> [i8; 65] {
        let mut array = [0i8; 65];
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

impl ToCSlice<[i8; 10]> for String {
    fn to_c_slice(&self) -> [i8; 10] {
        let mut array = [0i8; 10];
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

impl ToCSlice<[i8; 101]> for String {
    fn to_c_slice(&self) -> [i8; 101] {
        let mut array = [0i8; 101];
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

impl ToCSlice<[i8; 129]> for String {
    fn to_c_slice(&self) -> [i8; 129] {
        let mut array = [0i8; 129];
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

impl ToCSlice<[i8; 36]> for String {
    fn to_c_slice(&self) -> [i8; 36] {
        let mut array = [0i8; 36];
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

impl ToCSlice<[i8; 513]> for String {
    fn to_c_slice(&self) -> [i8; 513] {
        let mut array = [0i8; 513];
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

impl ToCSlice<[i8; 2]> for String {
    fn to_c_slice(&self) -> [i8; 2] {
        let mut array = [0i8; 2];
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

impl ToCSlice<[i8; 6]> for String {
    fn to_c_slice(&self) -> [i8; 6] {
        let mut array = [0i8; 6];
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

impl ToCSlice<[i8; 100]> for String {
    fn to_c_slice(&self) -> [i8; 100] {
        let mut array = [0i8; 100];
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

impl ToCSlice<[i8; 2049]> for String {
    fn to_c_slice(&self) -> [i8; 2049] {
        let mut array = [0i8; 2049];
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

impl ToCSlice<[i8; 13]> for String {
    fn to_c_slice(&self) -> [i8; 13] {
        let mut array = [0i8; 13];
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

impl ToCSlice<[i8; 273]> for String {
    fn to_c_slice(&self) -> [i8; 273] {
        let mut array = [0i8; 273];
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

impl ToCSlice<[i8; 33]> for String {
    fn to_c_slice(&self) -> [i8; 33] {
        let mut array = [0i8; 33];
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

impl ToCSlice<[i8; 3]> for String {
    fn to_c_slice(&self) -> [i8; 3] {
        let mut array = [0i8; 3];
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

impl ToCSlice<[i8; 16]> for String {
    fn to_c_slice(&self) -> [i8; 16] {
        let mut array = [0i8; 16];
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

impl ToCSlice<[i8; 1025]> for String {
    fn to_c_slice(&self) -> [i8; 1025] {
        let mut array = [0i8; 1025];
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
