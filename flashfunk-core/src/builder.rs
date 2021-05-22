use core::marker::PhantomData;

use std::borrow::Cow;
use flashfunk_level::interface::{Ac, Interface};
use flashfunk_level::data_type::LoginForm;
use flashfunk_level::types::message::{MdApiMessage, TdApiMessage};
use crate::worker::start_workers;

pub struct Builder<'a, MdApi, TdApi> {
    pub(crate) name: Cow<'a, str>,
    pub(crate) id: Cow<'a, str>,
    pub(crate) pwd: Cow<'a, str>,
    pub(crate) path: Cow<'a, str>,
    pub(crate) strategy: Vec<Box<dyn Ac + Send>>,
    pub(crate) login_form: LoginForm,
    pub(crate) _md: PhantomData<MdApi>,
    pub(crate) _td: PhantomData<TdApi>,
}

impl<'a, I, I2> Builder<'a, I, I2>
    where
        I: Interface<Message=MdApiMessage>,
        I2: Interface<Message=TdApiMessage>,
{
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

    pub fn strategy(mut self, strategy: Box<dyn Ac + Send>) -> Self {
        self.strategy.push(strategy);
        self
    }

    pub fn strategies(mut self, strategy: Vec<Box<dyn Ac + Send>>) -> Self {
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
