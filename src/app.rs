#![allow(dead_code, unused_variables)]

use libc;

extern {
    fn double_input(input: libc::c_int) -> libc::c_int;
}