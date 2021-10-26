mod waker;

pub mod park;
pub mod runtime;

#[cfg(feature = "async")]
mod r#async {
    use super::{
        park::{Park, Unpark},
        runtime::Runtime,
    };

    use core::future::Future;

    struct Parker(parking::Parker);

    struct Unparker(parking::Unparker);

    impl Park for Parker {
        type Unparker = Unparker;

        fn unparker(&mut self) -> Self::Unparker {
            let unparker = self.0.unparker();
            Unparker(unparker)
        }

        fn park(&self) {
            self.0.park();
        }
    }

    impl Unpark for Unparker {
        fn unpark(&self) {
            self.0.unpark();
        }
    }

    pub(crate) struct StdRuntime(Runtime<Parker>);

    impl StdRuntime {
        pub(crate) fn new() -> Self {
            let parker = Parker(parking::Parker::new());
            let rt = Runtime::new(parker);
            Self(rt)
        }

        pub(crate) fn block_on<Fut: Future>(&mut self, fut: Fut) -> Fut::Output {
            self.0.block_on(fut)
        }
    }
}

#[cfg(feature = "async")]
pub(crate) use r#async::*;
