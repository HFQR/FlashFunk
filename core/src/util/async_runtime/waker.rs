use core::mem::{self, ManuallyDrop};
use core::task::{RawWaker, RawWakerVTable, Waker};

use alloc::sync::Arc;

use super::park::Unpark;

pub(crate) fn waker_fn<U: Unpark>(u: U) -> Waker {
    let raw = Arc::into_raw(Arc::new(u)) as *const ();
    let vtable = &Helper::<U>::VTABLE;
    unsafe { Waker::from_raw(RawWaker::new(raw, vtable)) }
}

struct Helper<U>(U);

impl<U: Unpark> Helper<U> {
    const VTABLE: RawWakerVTable = RawWakerVTable::new(
        Self::clone_waker,
        Self::wake,
        Self::wake_by_ref,
        Self::drop_waker,
    );

    unsafe fn clone_waker(ptr: *const ()) -> RawWaker {
        let arc = ManuallyDrop::new(Arc::from_raw(ptr as *const U));
        mem::forget(arc.clone());
        RawWaker::new(ptr, &Self::VTABLE)
    }

    unsafe fn wake(ptr: *const ()) {
        let arc = Arc::from_raw(ptr as *const U);
        arc.unpark();
    }

    unsafe fn wake_by_ref(ptr: *const ()) {
        let arc = ManuallyDrop::new(Arc::from_raw(ptr as *const U));
        arc.unpark();
    }

    unsafe fn drop_waker(ptr: *const ()) {
        drop(Arc::from_raw(ptr as *const U));
    }
}
