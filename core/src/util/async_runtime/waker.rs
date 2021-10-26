use core::{
    mem::{self, ManuallyDrop},
    task::{RawWaker, RawWakerVTable, Waker},
};

use super::park::Unpark;

pub(crate) fn waker<U: Unpark>(f: U) -> Waker {
    let raw = &f as *const U as *const ();
    mem::forget(f);
    let vtable = &Helper::<U>::VTABLE;
    unsafe { Waker::from_raw(RawWaker::new(raw, vtable)) }
}

struct Helper<F>(F);

impl<U: Unpark> Helper<U> {
    const VTABLE: RawWakerVTable = RawWakerVTable::new(
        Self::clone_waker,
        Self::wake,
        Self::wake_by_ref,
        Self::drop_waker,
    );

    unsafe fn clone_waker(ptr: *const ()) -> RawWaker {
        let u = ManuallyDrop::new(ptr as *const U);
        mem::forget(u.clone());
        RawWaker::new(ptr, &Self::VTABLE)
    }

    unsafe fn wake(ptr: *const ()) {
        let u = ptr as *const U;
        (&*u).unpark();
    }

    unsafe fn wake_by_ref(ptr: *const ()) {
        let u = ManuallyDrop::new(ptr as *const U);
        (&**u).unpark();
    }

    unsafe fn drop_waker(ptr: *const ()) {
        drop(ptr as *const U);
    }
}
