use crate::{OwnedLog, Value};

// default logger that do nothing.
pub struct NoOpLogger;

impl OwnedLog for NoOpLogger {
    fn log(&self, _: Box<dyn Value>) {}
}
