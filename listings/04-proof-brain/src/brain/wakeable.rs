
use alloc::sync::Arc;
use core::task::{Waker, RawWaker, RawWakerVTable};

// ANCHOR: trait_part1
pub trait Wakeable {
    fn wake(self: Arc<Self>) {
        Self::wake_by_ref(&self)
    }

    fn wake_by_ref(self: &Arc<Self>);
// ANCHOR_END: trait_part1
// ANCHOR: trait_part2
    fn into_waker(self: &Arc<Self>) -> Waker
    where
        Self: Sized,
    {
        let wakeable_ptr = Arc::as_ptr(self) as *const ();
        unsafe {
            Waker::from_raw(RawWaker::new(
                wakeable_ptr,
                &RawWakerVTable::new(
                    clone_wakeable_raw::<Self>,
                    wake_wakeable_raw::<Self>,
                    wake_by_ref_wakeable_raw::<Self>,
                    // when the Waker is created from the Wakeable reference
                    // this one is not allowed to be dropped. But any Clone of
                    // it will and has to!
                    noop::<Self>,
                )
            ))
        }
    }
}
// ANCHOR_END: trait_part2

// ANCHOR: wake_functions
/// The first function that shall be able to *wake* the ``Wakeable``. The
/// ``Wakeable`` is consumed by this call.
pub unsafe fn wake_wakeable_raw<T: Wakeable>(wakeable: *const ()) {
    // transfer the raw pointer back into it's type pointer
    let wakeable: Arc<T> = Arc::from_raw(wakeable as *const T);
    // wake the wakeable
    Wakeable::wake(wakeable);
}

/// The second function that shall be able to *wake* the ``Wakeable``. The
/// ``Wakeable`` should be woken by reference and not beeing consumed
pub unsafe fn wake_by_ref_wakeable_raw<T: Wakeable>(wakeable: *const ()) {
    // transfer the raw pointer back into it's type pointer
    let wakeable: Arc<T> = Arc::from_raw(wakeable as *const T);
    // wake the wakeable
    Wakeable::wake_by_ref(&wakeable);

    // don't drop the wakeable as we do not consume it
    core::mem::forget(wakeable);
}

/// The third function is able to clone the current ``Wakeable``
pub unsafe fn clone_wakeable_raw<T: Wakeable>(wakeable: *const ()) -> RawWaker {
    let arc: Arc<T> = Arc::from_raw(wakeable as *const T);
    let cloned = arc.clone();
    // forget both references to keep ref-count up
    core::mem::forget(arc);
    core::mem::forget(cloned);

    RawWaker::new(
        wakeable,
        &RawWakerVTable::new(
            clone_wakeable_raw::<T>,
            wake_wakeable_raw::<T>,
            wake_by_ref_wakeable_raw::<T>,
            // each new clone of the Waker requires to be propperly dropped
            drop_wakeable_raw::<T>,
        )
    )
}

/// finally the function to drop the ``Wakeable`` after it is no longer needed
pub unsafe fn drop_wakeable_raw<T: Wakeable>(wakeable: *const ()) {
    println!("drop wakeable {:?}", wakeable);
    drop(Arc::<T>::from_raw(wakeable as *const T));
}

/// Special case where a function pointer of the Waker should do nothing
pub unsafe fn noop<T: Wakeable>(_: *const ()) {}
// ANCHOR_END: wake_functions
