#![allow(dead_code, unused_variables, unused_imports)]

/// Here are the api detail implement

use super::interface::Interface;

/// 行情网关的实现
pub struct MdApi {
    user_id: String,
    password: String,
}

impl Interface for MdApi {}