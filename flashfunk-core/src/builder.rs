use core::marker::PhantomData;

use std::borrow::Cow;

use crate::ac::{IntoStrategy, __Strategy};
use crate::interface::Interface;
use crate::structs::LoginForm;
use crate::types::message::{MdApiMessage, TdApiMessage};
use crate::worker::start_workers;

pub struct CtpBuilder<'a, Interface, Interface2> {
    pub(crate) name: Cow<'a, str>,
    pub(crate) id: Cow<'a, str>,
    pub(crate) pwd: Cow<'a, str>,
    pub(crate) path: Cow<'a, str>,
    pub(crate) strategy: Vec<__Strategy>,
    pub(crate) login_form: LoginForm,
    pub(crate) _i: PhantomData<Interface>,
    pub(crate) _i2: PhantomData<Interface2>,
}

impl<'a, I, I2> CtpBuilder<'a, I, I2>
where
    I: Interface<Message = MdApiMessage>,
    I2: Interface<Message = TdApiMessage>,
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

pub struct CtpBuilder2<'a, I>
    where
        I: Interface2
{
    pub(crate) name: Cow<'a, str>,
    pub(crate) id: Cow<'a, str>,
    pub(crate) pwd: Cow<'a, str>,
    pub(crate) path: Cow<'a, str>,
    pub(crate) strategy: Vec<__Strategy>,
    pub(crate) login_form: LoginForm,
    pub(crate) interface: I,
}

impl<'a, I> CtpBuilder2<'a, I>
    where
        I: Interface2,
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

    // 可以重复调用加载界面。被加载的界面需要实现Transform<I>。
    pub fn interface<I2: Transform<I>>(mut self, interface: I2) -> CtpBuilder2<'a, I2> {
        CtpBuilder2 {
            name: self.name,
            id: self.id,
            pwd: self.pwd,
            path: self.path,
            strategy: self.strategy,
            login_form: self.login_form,
            interface: interface.transform(self.interface),
        }
    }

    pub fn start(self) {}
}

pub trait Interface2 {
    // 该界面的构造工厂
    type Factory;
    // 该界面派发给策略工人的消息类型
    type Message;
    // 改界面接收主工人的请求类型
    type Request;

    fn ready(factory: Self::Factory) -> Self;

    fn call(&mut self, req: Self::Request);
}

// 将I类型转换为自身，I类型和自身都实现Interface2
pub trait Transform<I>
    where
        I: Interface2,
        Self: Interface2
{
    fn transform(self, interface: I) -> Self;
}
