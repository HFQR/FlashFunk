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
        ::owned_log::__private::OWNED_LOGGER
            .get_or_init(|| ::std::sync::Arc::new(::owned_log::__private::NoOpLogger))
            .log(Box::new($value) as _);
    };
}

#[doc(hidden)]
pub mod __private {
    use super::*;

    use std::sync::Arc;

    use once_cell::sync::OnceCell;

    pub static OWNED_LOGGER: OnceCell<Arc<dyn OwnedLog>> = OnceCell::new();

    pub use crate::no_op::NoOpLogger;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        struct MyLogger;

        impl MyLogger {
            fn init() {
                OWNED_LOGGER.set(Arc::new(MyLogger)).ok().unwrap();
            }
        }

        impl OwnedLog for MyLogger {
            fn log(&self, mut value: Box<dyn Value>) {
                value.display();
            }
        }

        MyLogger::init();

        struct MyValue(Option<usize>);

        impl Value for MyValue {
            fn display(&mut self) {
                assert_eq!(self.0.take().unwrap(), 996);
            }
        }

        log!(MyValue(Some(996)))
    }
}
