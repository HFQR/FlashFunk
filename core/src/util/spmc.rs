use core::{
    fmt, mem,
    sync::atomic::{AtomicUsize, Ordering},
};

use alloc::{sync::Arc, vec::Vec};

use super::cache_padded::CachePadded;

struct Inner<T> {
    tail: CachePadded<AtomicUsize>,
    cap: usize,
    buffer: *mut T,
}

/// construct append only single producer and multi consumer channel.
/// the channel can only hold given size of items and item can not be removed from channel.
pub fn new<T>(cap: usize) -> (Producer<T>, Consumer<T>) {
    assert!(cap > 0, "capacity must be non-zero");

    let buffer = {
        let mut v = Vec::<T>::with_capacity(cap);
        let ptr = v.as_mut_ptr();
        mem::forget(v);
        ptr
    };

    let inner = Arc::new(Inner {
        tail: CachePadded::new(AtomicUsize::new(0)),
        cap,
        buffer,
    });

    let p = Producer {
        tail: 0,
        inner: inner.clone(),
    };

    let c = Consumer {
        inner,
        next: 0,
        tail: 0,
    };

    (p, c)
}

impl<T> Drop for Inner<T> {
    fn drop(&mut self) {
        let tail = self.tail.load(Ordering::Acquire);
        // SAFETY:
        // pointer and capacity is constructed through Vec<T> and upholds the safety.
        // tail is incremented every time a new T added to the vec and upholds the safety.
        let _vec = unsafe { Vec::<T>::from_raw_parts(self.buffer, tail, self.cap) };
    }
}

pub struct Producer<T> {
    tail: usize,
    inner: Arc<Inner<T>>,
}

unsafe impl<T: Send> Send for Producer<T> {}

impl<T> Producer<T> {
    pub fn push(&mut self, val: T) -> Result<(), PushError<T>> {
        if self.tail == self.inner.cap {
            return Err(PushError(val));
        }

        // SAFETY:
        // just checked for out of bound.
        unsafe { self.inner.buffer.add(self.tail).write(val) };

        self.tail += 1;
        self.inner.tail.fetch_add(1, Ordering::Release);

        Ok(())
    }
}

pub struct Consumer<T> {
    inner: Arc<Inner<T>>,
    next: usize,
    tail: usize,
}

impl<T> Clone for Consumer<T> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
            next: 0,
            tail: 0,
        }
    }
}

unsafe impl<T: Send> Send for Consumer<T> {}

impl<T> Consumer<T> {
    pub fn pop(&mut self) -> Option<&T> {
        if self.next == self.tail {
            self.tail = self.inner.tail.load(Ordering::Relaxed);
        }

        if self.next == self.tail {
            return None;
        }

        if self.next == self.inner.cap {
            return None;
        }

        // SAFETY:
        // just checked for out of bound.
        let next = unsafe { &*self.inner.buffer.add(self.next) };
        self.next += 1;

        Some(next)
    }
}

/// Error which occurs when pushing into a full queue.
#[derive(Clone, Copy, Eq, PartialEq)]
pub struct PushError<T>(pub T);

impl<T> fmt::Debug for PushError<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("PushError(..)")
    }
}

impl<T> fmt::Display for PushError<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("pushing into a full channel")
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn push_pop() {
        let (mut p, c) = new::<usize>(2);

        p.push(996).unwrap();
        p.push(251).unwrap();
        p.push(666).unwrap_err();

        let mut c1 = c.clone();
        let mut c2 = c.clone();

        assert_eq!(c1.pop().unwrap(), &996);
        assert_eq!(c2.pop().unwrap(), &996);

        assert_eq!(c1.pop().unwrap(), &251);
        assert_eq!(c2.pop().unwrap(), &251);

        assert!(c1.pop().is_none());
        assert!(c2.pop().is_none());
    }

    #[test]
    fn drop_test() {
        let (mut p, c) = new::<Arc<()>>(2);

        let item = Arc::new(());

        p.push(item.clone()).unwrap();

        assert_eq!(Arc::strong_count(&item), 2);

        drop(p);
        assert_eq!(Arc::strong_count(&item), 2);

        drop(c);
        assert_eq!(Arc::strong_count(&item), 1);
    }
}
