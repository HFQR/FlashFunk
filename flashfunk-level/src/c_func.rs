/// This module include the c string operation function
use core::fmt;
use std::borrow::Cow;
use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int, c_uchar, c_void};

use encoding::all::GB18030;
use encoding::{DecoderTrap, Encoding};

use crate::get_interface_path;
use chrono::{NaiveDate, NaiveTime, Timelike};

/// change the string to CString and will be panic at a bad string
pub fn to_c_string(string: String) -> CString {
    CString::new(string).expect(format!("转换CString失败").as_str())
}

/// make the string
pub fn to_i8_array(string: &str) -> Vec<i8> {
    string.as_bytes().iter().map(|x| *x as i8).collect()
}

/// make the string to CStr
pub fn to_c_str<'a>(string: String) -> &'a CStr {
    unsafe { CStr::from_ptr(to_c_string(string).as_ptr()) }
}

/// covert the u8 array to Cow
pub fn translate_zh(v: &[u8]) -> Cow<str> {
    let slice = v
        .split(|&c| c == 0u8)
        .next()
        .expect(format!("unicode 切换失败 无法转换中文").as_str());
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

/// covert i8 array to string
pub fn bytes_to_string(v: &[i8]) -> String {
    let r = v
        .split(|x| *x == 0i8)
        .next()
        .unwrap()
        .iter()
        .map(|x| *x as u8)
        .collect::<Vec<u8>>();
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
    // SAFETY:
    //
    // Input pointer must be valid utf8 bytes. With example format like:
    // "20210325" (4 chars for year, 2 for month and 2 for day) for date
    // and "15:00:00"(hour:minute:second) for time.
    unsafe {
        let a = CStr::from_ptr(date).to_bytes();
        let u = CStr::from_ptr(time).to_bytes();
        let sub_t = mill as u32 * 1_000_000;

        let y = parse_c_str(&a[0..4]) as i32;
        let m = parse_c_str(&a[4..6]);
        let d = parse_c_str(&a[6..]);

        let h = parse_c_str(&u[0..2]);
        let m = parse_c_str(&u[3..5]);
        let s = parse_c_str(&u[6..]);

        let date = NaiveDate::from_ymd(y, m, d);

        let time = NaiveTime::from_hms(h, m, s).with_nanosecond(sub_t).unwrap();

        (date, time)
    }
}

fn parse_c_str(slice: &[u8]) -> u32 {
    let mut res = 0;
    slice.iter().for_each(|n| {
        let digit = (*n as char as u32).wrapping_sub('0' as u32);
        res = res * 10 + digit;
    });
    res
}

pub fn translate_zh_to_string(v: &[i8]) -> String {
    let r = v.iter().map(|x| *x as u8).collect::<Vec<u8>>();
    translate_zh(&*r).to_string()
}

const ORDER_ID_LENGTH: usize = 12usize;
