mod no_op;

pub trait OwnedLog: 'static {
    fn log(&self, value: Box<dyn Value>);
}

pub trait Value: Send + 'static {
    fn display(&mut self);
}

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
