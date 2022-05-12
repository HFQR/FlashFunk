mod no_op;

use std::{any::TypeId, sync::Arc};

use once_cell::sync::OnceCell;

pub trait OwnedLog: Send + Sync + 'static {
    fn log(&self, value: Box<dyn Value>);
}

pub trait Value: Send + 'static {
    fn display(&mut self);

    #[doc(hidden)]
    #[allow(dead_code)]
    fn __private_get_type_id__(&self, _: PrivateHelper) -> (TypeId, PrivateHelper)
    where
        Self: 'static,
    {
        (TypeId::of::<Self>(), PrivateHelper(()))
    }
}

#[doc(hidden)]
#[allow(dead_code)]
pub struct PrivateHelper(());

impl dyn Value {
    #[allow(dead_code)]
    pub fn downcast_ref<T: Value>(&self) -> Option<&T> {
        if self.__private_get_type_id__(PrivateHelper(())).0 == TypeId::of::<T>() {
            // SAFETY: external crates cannot override the default
            // implementation of `__private_get_type_id__`, since
            // it requires returning a private type. We can therefore
            // rely on the returned `TypeId`, which ensures that this
            // case is correct.
            unsafe { Some(&*(self as *const dyn Value as *const T)) }
        } else {
            None
        }
    }

    #[allow(dead_code)]
    pub fn downcast_mut<T: Value>(&mut self) -> Option<&mut T> {
        if self.__private_get_type_id__(PrivateHelper(())).0 == TypeId::of::<T>() {
            // SAFETY: external crates cannot override the default
            // implementation of `__private_get_type_id__`, since
            // it requires returning a private type. We can therefore
            // rely on the returned `TypeId`, which ensures that this
            // case is correct.
            unsafe { Some(&mut *(self as *mut dyn Value as *mut T)) }
        } else {
            None
        }
    }
}

pub static OWNED_LOGGER: OnceCell<Arc<dyn OwnedLog>> = OnceCell::new();

#[macro_export]
macro_rules! log {
    ($value: expr) => {
        ::owned_log::__private::OWNED_LOGGER_LOCAL.with(|logger| logger.log($value));
    };
}

#[doc(hidden)]
pub mod __private {
    use super::*;

    thread_local! {
        pub static OWNED_LOGGER_LOCAL: Arc<dyn OwnedLog> = {
            OWNED_LOGGER.get_or_init(|| Arc::new(crate::no_op::NoOpLogger)).clone()
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_any_casting() {
        struct MyValue;

        impl Value for MyValue {
            fn display(&mut self) {
                todo!()
            }
        }

        let mut value = Box::new(MyValue) as Box<dyn Value>;

        value.downcast_mut::<MyValue>().unwrap();
    }
}
