//! A strict spin lock. No yield, no spin loop hint.

use core::{
    cell::UnsafeCell,
    ops::{Deref, DerefMut},
    sync::atomic::{AtomicBool, Ordering},
};

pub struct SpinLock<T> {
    locked: AtomicBool,
    value: UnsafeCell<T>,
}

// SAFETY: As long as T is Send type the lock is Send is Sync.
unsafe impl<T: Send> Send for SpinLock<T> {}
unsafe impl<T: Send> Sync for SpinLock<T> {}

impl<T> SpinLock<T> {
    pub const fn new(value: T) -> Self {
        Self {
            locked: AtomicBool::new(false),
            value: UnsafeCell::new(value),
        }
    }

    #[inline]
    pub fn lock(&self) -> SpinGuard<'_, T> {
        loop {
            if let Some(guard) = self.try_lock() {
                return guard;
            }
        }
    }

    #[inline]
    pub fn into_inner(self) -> T {
        self.value.into_inner()
    }

    fn try_lock(&self) -> Option<SpinGuard<'_, T>> {
        if self.locked.swap(true, Ordering::Release) {
            None
        } else {
            Some(SpinGuard(self))
        }
    }

    #[inline(always)]
    fn release(&self) {
        self.locked.store(false, Ordering::Relaxed)
    }
}

pub struct SpinGuard<'a, T>(&'a SpinLock<T>);

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
}
