use std::os::raw::{c_char, c_int, c_uchar, c_void};


#[cfg(not(target_os = "windows"))]
include!(concat!(env!("HOME"), "/.HFQ/ctpmini/bindings.rs"));

#[cfg(target_os = "windows")]
include!(concat!(
env!("HOMEDRIVE"),
env!("HOMEPATH"),
"/.HFQ/ctpmini/bindings.rs"
));


impl_to_c_slice!(
    301, 11, 2561, 349, 100, 501, 129, 9, 31, 12, 17, 13, 51, 71, 25, 10, 513, 21, 256, 365, 36,
    15, 16, 20, 23, 5, 41, 257, 3, 2, 33, 1001, 201, 273, 65, 401, 261, 24, 61, 4, 81, 161, 2049,
    6, 22, 1025, 7, 101
);
