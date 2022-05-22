//! A strict spin lock. No yield, no spin loop hint.

use core::{
    cell::UnsafeCell,
    fmt,
    ops::{Deref, DerefMut},
    sync::atomic::{AtomicBool, Ordering},
};

pub struct SpinLock<T: ?Sized> {
    locked: AtomicBool,
    value: UnsafeCell<T>,
}

impl<T: fmt::Debug> fmt::Debug for SpinLock<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.try_lock() {
            Some(guard) => write!(f, "SpinLock {{ value: ")
                .and_then(|()| (&*guard).fmt(f))
                .and_then(|()| write!(f, "}}")),
            None => write!(f, "SpinLock {{ <locked> }}"),
        }
    }
}

// SAFETY: As long as T is Send type the lock is Send is Sync.
unsafe impl<T: ?Sized + Send> Send for SpinLock<T> {}
unsafe impl<T: ?Sized + Send> Sync for SpinLock<T> {}

impl<T> SpinLock<T> {
    pub const fn new(value: T) -> Self {
        Self {
            locked: AtomicBool::new(false),
            value: UnsafeCell::new(value),
        }
    }

    #[inline]
    pub fn lock(&self) -> SpinGuard<'_, T> {
        while self
            .locked
            .compare_exchange_weak(false, true, Ordering::Acquire, Ordering::Relaxed)
            .is_err()
        {
            #[allow(clippy::missing_spin_loop)]
            while self.locked.load(Ordering::Relaxed) {}
        }

        SpinGuard(self)
    }

    #[inline]
    pub fn into_inner(self) -> T {
        self.value.into_inner()
    }

    pub fn try_lock(&self) -> Option<SpinGuard<'_, T>> {
        if self.locked.swap(true, Ordering::Acquire) {
            None
        } else {
            Some(SpinGuard(self))
        }
    }

    #[inline(always)]
    fn release(&self) {
        self.locked.store(false, Ordering::Release)
    }
}

pub struct SpinGuard<'a, T>(&'a SpinLock<T>);

impl<T: fmt::Debug> fmt::Debug for SpinGuard<'_, T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(&self.0, f)
    }
}

impl<T> Deref for SpinGuard<'_, T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        // SAFETY:
        // safe as spin guard is referencing the lock and have exclusive to value.
        unsafe { &*self.0.value.get() }
    }
}

impl<T> DerefMut for SpinGuard<'_, T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        // SAFETY:
        // same reason as Deref is safe.
        unsafe { &mut *self.0.value.get() }
    }
}

impl<T> Drop for SpinGuard<'_, T> {
    #[inline]
    fn drop(&mut self) {
        self.0.release();
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use core::ptr;

    use alloc::sync::Arc;

    use std::time::Duration;

    #[test]
    fn lock() {
        let spin = Arc::new(SpinLock::new(0usize));

        let spin2 = spin.clone();
        let handle = std::thread::spawn(move || {
            for _ in 0..99 {
                {
                    let mut guard = spin.lock();
                    *guard += 1;
                }
                std::thread::sleep(Duration::from_nanos(1));
            }
        });

        let handle2 = std::thread::spawn(move || {
            while *spin2.lock() != 99 {
                std::thread::sleep(Duration::from_nanos(1));
            }
        });

        handle.join().unwrap();
        handle2.join().unwrap();
    }

    #[test]
    fn try_lock() {
        let spin = Arc::new(SpinLock::new(0usize));

        let spin2 = spin.clone();
        let handle = std::thread::spawn(move || {
            for _ in 0..99 {
                {
                    let mut guard = spin.lock();
                    *guard += 1;
                }
                std::thread::sleep(Duration::from_nanos(1));
            }
        });

        let handle2 = std::thread::spawn(move || loop {
            if let Some(guard) = spin2.try_lock() {
                if *guard == 99 {
                    return;
                } else {
                    drop(guard);
                    std::thread::sleep(Duration::from_nanos(1));
                }
            }
        });

        handle.join().unwrap();
        handle2.join().unwrap();
    }

    #[test]
    fn send_non_sync() {
        #[allow(dead_code)]
        struct Foo {
            v: *const (),
        }

        unsafe impl Send for Foo {}

        let spin = Arc::new(SpinLock::new(Foo { v: ptr::null() }));

        let spin2 = spin.clone();
        let handle = std::thread::spawn(move || {
            let _guard = spin.lock();
        });

        let handle2 = std::thread::spawn(move || {
            let _ = spin2.lock();
        });

        handle.join().unwrap();
        handle2.join().unwrap();
    }
}
