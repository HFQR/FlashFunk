/// This module include the c string operation function
use core::fmt;
use std::borrow::Cow;
use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int, c_uchar, c_void};

use encoding::all::GB18030;
use encoding::{DecoderTrap, Encoding};

use crate::get_interface_path;
use chrono::{NaiveDate, NaiveTime, Timelike};


pub fn to_c_string(string: String) -> CString {
    CString::new(string).unwrap()
}

pub fn to_i8_arrary(string: String) -> Vec<i8> {
    string.into_bytes().iter().map(|x| *x as i8).collect()
}

pub fn to_c_str<'a>(string: String) -> &'a CStr {
    unsafe { CStr::from_bytes_with_nul_unchecked(string.as_bytes()) }
}

/// 翻译中文字符串
pub fn translate_zh(v: &[u8]) -> Cow<str> {
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

/// i8字节流转换String
pub fn bytes_to_string(v: &[i8]) -> String {
    let r = v.iter().map(|x| *x as u8).collect::<Vec<u8>>();
    unsafe { String::from_utf8_unchecked(r) }
}

/// 提供高性能的时间解析 从微妙级别   从i8到NaiveDate, NaiveTime 对象  ( NaiveDate, and NaiveTime)
/// ```
/// use std::ffi::{CStr, CString};
/// let date = "20210325";
/// let time = "15:00:00";
/// // todo: 添加测试用例
/// ```
pub fn parse_datetime_from_str(
    date: *const i8,
    time: *const i8,
    mill: c_int,
) -> (NaiveDate, NaiveTime) {
    unsafe {
        let a = CStr::from_ptr(date).to_str().unwrap();
        let u = CStr::from_ptr(time).to_str().unwrap();
        let sub_t = mill as u32 * 1_000_000;
        let date = NaiveDate::from_ymd(
            a[0..4].parse().unwrap(),
            a[4..6].parse().unwrap(),
            a[6..].parse().unwrap(),
        );

        let time = NaiveTime::from_hms(
            u[0..2].parse().unwrap(),
            u[3..5].parse().unwrap(),
            u[6..].parse().unwrap(),
        )
            .with_nanosecond(sub_t)
            .unwrap();

        (date, time)
    }
}


pub fn check_slice_to_string(v: &[i8]) -> String {
    let r = v
        .split(|x| *x == 0i8)
        .next()
        .unwrap()
        .iter()
        .map(|x| *x as u8)
        .collect::<Vec<u8>>();
    unsafe { String::from_utf8_unchecked(r) }
}

pub fn slice_to_string(v: &[i8]) -> String {
    let r = v.iter().map(|x| *x as u8).collect::<Vec<u8>>();
    zh_cstr_to_str(&*r).to_string()
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
    use crate::c_fun::{split_into_vec, ToCSlice};

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
