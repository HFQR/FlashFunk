#[cfg(feature = "async")]
pub mod async_impl;
mod no_op;

pub trait OwnedLog: Send + Sync + 'static {
    fn log(&self, value: Box<dyn Value>);
}

pub trait Value: Send + 'static {
    fn display(&mut self);
}

#[macro_export]
macro_rules! log {
    ($value: expr) => {
        ::owned_log::__private::OWNED_LOGGER_THREAD_LOCAL
            .with(|logger| logger.log(Box::new($value) as _));
    };
}

#[doc(hidden)]
pub mod __private {
    use super::*;

    use std::sync::Arc;

    use once_cell::sync::OnceCell;

    pub static OWNED_LOGGER: OnceCell<Arc<dyn OwnedLog>> = OnceCell::new();

    thread_local! {
        pub static OWNED_LOGGER_THREAD_LOCAL: Arc<dyn OwnedLog> = {
            OWNED_LOGGER.get_or_init(|| Arc::new(NoOpLogger)).clone()
        };
    }

    pub use crate::no_op::NoOpLogger;
}
