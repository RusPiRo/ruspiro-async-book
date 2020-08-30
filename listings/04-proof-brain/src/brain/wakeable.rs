
use alloc::sync::Arc;
use core::task::{Waker, RawWaker, RawWakerVTable};
use core::mem::ManuallyDrop;

// ANCHOR: wakeable
pub trait Wakeable: Sized {
    fn wake(self: Arc<Self>) {
        Self::wake_by_ref(&self)
    }

    fn wake_by_ref(self: &'_ Arc<Self>);

    fn into_waker(self: &Arc<Self>) -> Waker {
        unsafe {
            Waker::from_raw(
                Self::into_raw_waker(Arc::clone(self))
            )
        }
    }
}
// ANCHOR_END: wakeable

/// Every type beeing a [Wakeable] can also be represented as
/// [WakeableTraitObject]
impl<T: Wakeable> WakeableTraitObject for T {}
trait WakeableTraitObject: Wakeable + Sized {
    /// build the RawWaker from the Wakeable consuming the [Arc] of it
    fn into_raw_waker(self: Arc<Self>) -> RawWaker {
        let raw_wakeable: *const () = Arc::into_raw(self).cast();
        let raw_wakeabe_vtable = &Self::WAKER_VTABLE;

        RawWaker::new(
            raw_wakeable,
            raw_wakeabe_vtable,
        )
    }

    /// specifiying the VTable for this Wakeable
    const WAKER_VTABLE: RawWakerVTable = RawWakerVTable::new(
        {
            unsafe fn clone<T: Wakeable>(wakeable: *const ()) -> RawWaker {
                let wakeable: *const T = wakeable.cast();
                let wakeable_ref: &Arc<T> = &*ManuallyDrop::new(
                    Arc::from_raw(wakeable)
                );
                
                Arc::clone(wakeable_ref).into_raw_waker()
            }
            clone::<Self>
        },
        {
            unsafe fn wake<T: Wakeable>(wakeable: *const ()) {
                // transfer the raw pointer back into it's type pointer
                let wakeable: *const T = wakeable.cast();
                let wakeable: Arc<T> = Arc::from_raw(wakeable);
                // wake the wakeable
                Wakeable::wake(wakeable);
            }
            wake::<Self>
        },
        {
            unsafe fn wake_by_ref<T: Wakeable>(wakeable: *const ()) {
                // transfer the raw pointer back into it's type pointer
                let wakeable: *const T = wakeable.cast();
                let wakeable_ref = &*ManuallyDrop::new(Arc::from_raw(wakeable));
                Wakeable::wake_by_ref(wakeable_ref);
            }
            wake_by_ref::<Self>
        },
        {
            unsafe fn drop<T: Wakeable>(wakeable: *const ()) {
                // transfer the raw pointer back into it's type pointer
                let wakeable: *const T = wakeable.cast();
                core::mem::drop(Arc::from_raw(wakeable));
            }
            drop::<Self>
        }
    );
}
// ANCHOR: wake_functions
