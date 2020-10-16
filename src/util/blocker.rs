use std::sync::{Arc, Condvar, Mutex};

#[allow(clippy::mutex_atomic)]
// 简易线程阻塞器
pub(crate) struct Blocker {
    inner: Arc<WaiterInner>,
}

impl Clone for Blocker {
    fn clone(&self) -> Self {
        Self {
            inner: Arc::clone(&self.inner),
        }
    }
}

impl Default for Blocker {
    fn default() -> Self {
        Self {
            inner: Arc::new(WaiterInner {
                cond: Condvar::new(),
                lock: Mutex::new(false),
            }),
        }
    }
}

impl Blocker {
    // 阻塞当前线程
    pub(crate) fn block(&self) {
        let mut should_unblock = self.inner.lock.lock().unwrap();
        while !*should_unblock {
            should_unblock = self.inner.cond.wait(should_unblock).unwrap();
        }
    }

    // 解阻所有被此blocker阻塞的线程
    pub(crate) fn unblock(&self) {
        let mut should_unblock = self.inner.lock.lock().unwrap();
        *should_unblock = true;
        self.inner.cond.notify_all();
    }
}

struct WaiterInner {
    cond: Condvar,
    lock: Mutex<bool>,
}
