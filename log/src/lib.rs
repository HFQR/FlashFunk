#![feature(once_cell)]

mod no_op;

pub trait OwnedLog: 'static {
    fn log(&self, value: Box<dyn Value>);
}

pub trait Value {
    fn display(&mut self);
}

impl Value for ValueType {
    fn display(&mut self) {
        todo!()
    }
}

#[derive(Default)]
pub struct ValueType(
    u32,
    u32,
    u128,
    u8,
    u8,
    u64,
    u32,
    i32,
    i32,
    i32,
    i32,
    usize,
    [u8; 12],
);

use once_cell::unsync::OnceCell;

thread_local! {
    pub static OWNED_LOGGER: OnceCell<Box<dyn OwnedLog>> = OnceCell::new();
}

#[macro_export]
macro_rules! log {
    ($value: expr) => {
        ::owned_log::OWNED_LOGGER.with(|logger| {
            logger
                .get_or_init(|| Box::new(::owned_log::__private::NoOpLogger))
                .log($value)
        });
    };
}

#[doc(hidden)]
pub mod __private {
    pub use crate::no_op::NoOpLogger;
}
