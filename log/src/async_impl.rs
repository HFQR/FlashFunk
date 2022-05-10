use std::{io, sync::Arc};

use tokio::runtime::{Builder, Runtime};

use crate::{OwnedLog, Value};

/// A multi thread tokio runtime that would run [Value::display] method in spawned async task.
#[derive(Debug)]
pub struct TokioLogger {
    rt: Runtime,
}

impl OwnedLog for TokioLogger {
    fn log(&self, mut value: Box<dyn Value>) {
        self.rt.spawn(async move {
            value.display();
        });
    }
}

impl TokioLogger {
    pub fn builder() -> TokioLoggerBuilder {
        TokioLoggerBuilder::new_multi_thread()
    }
}

pub struct TokioLoggerBuilder(Builder);

impl TokioLoggerBuilder {
    pub fn new_multi_thread() -> Self {
        Self(Builder::new_multi_thread())
    }

    pub fn worker_threads(&mut self, val: usize) -> &mut Self {
        self.0.worker_threads(val);
        self
    }

    pub fn build(&mut self) -> io::Result<()> {
        let rt = self.0.build()?;

        crate::__private::OWNED_LOGGER
            .set(Arc::new(TokioLogger { rt }) as _)
            .map_err(|_| io::ErrorKind::Other.into())
    }
}
