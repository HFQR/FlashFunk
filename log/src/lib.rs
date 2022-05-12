mod no_op;
mod value;

use std::sync::Arc;

use once_cell::sync::OnceCell;

pub use crate::value::Value;

/// Trait for customizable logger that take in ownership of the loggable value.
pub trait OwnedLog: Send + Sync + 'static {
    fn log(&self, value: Box<dyn Value>);
}

/// Global static instance of logger object.
pub static OWNED_LOGGER: OnceCell<Arc<dyn OwnedLog>> = OnceCell::new();

#[macro_export]
macro_rules! log {
    ($value: expr) => {
        ::owned_log::__private::OWNED_LOGGER_LOCAL.with(|logger| logger.log($value));
    };
}

// private public module to hide api call from exported macro.
#[doc(hidden)]
pub mod __private {
    use super::*;

    thread_local! {
        /// A thread local cache of global static [OWNED_LOGGER].
        pub static OWNED_LOGGER_LOCAL: Arc<dyn OwnedLog> = {
            OWNED_LOGGER.get_or_init(|| Arc::new(crate::no_op::NoOpLogger)).clone()
        };
    }
}
