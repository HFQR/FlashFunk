use core::fmt::{Debug, Formatter, Result as FmtResult};
use core::marker::PhantomData;

use std::borrow::Cow;

use crate::account::Account;
use crate::builder::Builder;
use crate::util::hash::HashMap;

/// ctpbee核心运行器
/// 作为该运行器
pub struct Flash {
    name: String,
    acc: Account,
    login_info: HashMap<String, String>,
}

impl Flash {
    // Interface类型需要作为类型提示传入. N类型可以为自动推导
    pub fn builder<'a, I, I2, N>(name: N) -> Builder<'a, I, I2>
    where
        N: Into<Cow<'a, str>>,
    {
        Builder {
            name: name.into(),
            id: Default::default(),
            pwd: Default::default(),
            path: Default::default(),
            strategy: vec![],
            login_form: Default::default(),
            _i: PhantomData,
            _i2: PhantomData,
        }
    }

    pub fn login(&mut self) -> bool {
        true
    }

    /// 从HashMap载入登录信息
    pub fn load_setting(&mut self, map: HashMap<String, String>) {
        let mut dt = HashMap::new();
        for (key, value) in map {
            dt.insert(key, value);
        }
        self.login_info = dt;
    }
}

impl Debug for Flash {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "Flash >>> : {}", self.name)
    }
}
