#![allow(dead_code, unused_variables, unused_imports)]


use super::interface::Interface;

/// the implement of market api
pub struct MdApi {
    user_id: String,
    password: String,
}

/// the implement of trader api
pub struct TraderApi {
    user_id: String,
    password: String,
}

impl Interface for TraderApi {
    // 开肝了少年 !
}

impl Interface for MdApi {}