use std::borrow::Cow;

use crate::ac::{IntoStrategy, __Strategy};
use crate::structs::LoginForm;
use crate::worker::start_workers;

pub struct CtpBuilder<'a> {
    pub(crate) name: Cow<'a, str>,
    pub(crate) id: Cow<'a, str>,
    pub(crate) pwd: Cow<'a, str>,
    pub(crate) path: Cow<'a, str>,
    pub(crate) strategy: Vec<__Strategy>,
    pub(crate) login_form: LoginForm,
}

impl<'a> CtpBuilder<'a> {
    pub fn id(mut self, id: impl Into<Cow<'a, str>>) -> Self {
        self.id = id.into();
        self
    }

    pub fn pwd(mut self, pwd: impl Into<Cow<'a, str>>) -> Self {
        self.pwd = pwd.into();
        self
    }

    pub fn path(mut self, path: impl Into<Cow<'a, str>>) -> Self {
        self.path = path.into();
        self
    }

    pub fn strategy(mut self, strategy: impl IntoStrategy) -> Self {
        self.strategy.push(strategy.into_str());
        self
    }

    pub fn strategies(mut self, strategy: Vec<__Strategy>) -> Self {
        self.strategy = strategy;
        self
    }

    pub fn login_form(mut self, login_form: LoginForm) -> Self {
        self.login_form = login_form;
        self
    }

    pub fn start(self) {
        start_workers(self);
    }
}
