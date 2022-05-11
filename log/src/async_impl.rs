use std::{
    io,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
};

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

    pub fn bind_to_cpus<const N: usize>(&mut self, cpus: &[usize; N]) -> &mut Self {
        use core_affinity::CoreId;

        struct CpuAff {
            cores: Box<[CoreId]>,
            len: usize,
            next: AtomicUsize,
        }

        impl CpuAff {
            fn next(&self) -> CoreId {
                let n = self.next.fetch_add(1, Ordering::Relaxed);
                self.cores[n % self.len]
            }
        }

        let aff = Arc::new(CpuAff {
            cores: cpus
                .into_iter()
                .map(|id| CoreId { id: *id })
                .collect::<Vec<_>>()
                .into_boxed_slice(),
            len: N,
            next: AtomicUsize::new(0),
        });

        self.0.on_thread_start(move || {
            let aff = aff.clone();
            let id = aff.next();
            core_affinity::set_for_current(id);
        });

        self
    }

    pub fn build(&mut self) -> io::Result<()> {
        let rt = self.0.build()?;

        crate::__private::OWNED_LOGGER
            .set(Arc::new(TokioLogger { rt }) as _)
            .map_err(|_| io::ErrorKind::Other.into())
    }
}
