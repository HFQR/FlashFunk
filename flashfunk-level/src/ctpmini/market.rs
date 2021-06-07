use crate::ctpmini::sys;
use std::ffi::CString;
use crate::data_type::LoginForm;
use crate::util::channel::GroupSender;
use crate::types::message::MdApiMessage;

/// Mini 核心API
struct MiniMdApi {
    user_id: CString,
    password: CString,
    request_id: i32,
    pub flow_path_ptr: *const i8,
    collector: LevelControl,
}

/// 衔接初始数据的衔接层
struct LevelControl {
    sender: GroupSender<MdApiMessage>,
    symbols: Vec<&'static str>,
    blocker: Option<MdApiBlocker>,
    login_form: LoginForm,
    market_pointer: *mut CThostFtdcMdApi,
    request_id: i32,
}

