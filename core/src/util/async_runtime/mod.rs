mod waker;

pub mod park;
pub mod runtime;

use core::future::Future;

use self::{
    park::{Park, Unpark},
    runtime::Runtime,
};

struct Parker(parking::Parker);

struct Unparker(parking::Unparker);

impl Park for Parker {
    type Unparker = Unparker;

    fn unparker(&mut self) -> Self::Unparker {
        Unparker(self.0.unparker())
    }

    #[inline]
    fn park(&self) {
        self.0.park();
    }
}

impl Unpark for Unparker {
    #[inline]
    fn unpark(&self) {
        self.0.unpark();
    }
}

pub struct StdRuntime(Runtime<Parker>);

impl Default for StdRuntime {
    fn default() -> Self {
        Self::new()
    }
}

impl StdRuntime {
    pub fn new() -> Self {
        let parker = Parker(parking::Parker::new());
        let rt = Runtime::new(parker);
        Self(rt)
    }

    #[inline]
    pub fn block_on<Fut: Future>(&mut self, fut: Fut) -> Fut::Output {
        self.0.block_on(fut)
    }
}
