use std::any::TypeId;

/// Trait of objectify loggable value.
pub trait Value: Send + sealed::GetTypeId + 'static {
    fn display(&mut self);
}

#[doc(hidden)]
mod sealed {
    use super::*;

    pub trait GetTypeId: 'static {
        fn get_type_id(&self) -> TypeId {
            TypeId::of::<Self>()
        }
    }

    impl<T: 'static> GetTypeId for T {}
}

impl dyn Value {
    /// Downcast a dyn Value trait object to immutable reference of concrete type.
    pub fn downcast_ref<T: Value>(&self) -> Option<&T> {
        if sealed::GetTypeId::get_type_id(self) == TypeId::of::<T>() {
            // SAFETY:
            // This is safe as cast only happen when TypeId is the same.
            unsafe { Some(&*(self as *const dyn Value as *const T)) }
        } else {
            None
        }
    }

    /// Downcast a dyn Value trait object to mutable reference of concrete type.
    pub fn downcast_mut<T: Value>(&mut self) -> Option<&mut T> {
        if sealed::GetTypeId::get_type_id(self) == TypeId::of::<T>() {
            // SAFETY:
            // This is safe as cast only happen when TypeId is the same.
            unsafe { Some(&mut *(self as *mut dyn Value as *mut T)) }
        } else {
            None
        }
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
