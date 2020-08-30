## The Context and the Waker

The first issue of our [naive Brain](./03-02-first-brain.md) is, that it does not compile for an obvious reason: The signature of the ``poll`` function requires to pass a ``Context`` and we where not able to provide one for now, but - what is this `Context` about ?

### The Context

In the current version of Rust, the ``Context`` that will be handed over to the ``Future`` while polling the same only contains a reference to a ``Waker``. This ``Waker`` can be used to *wake* the processing of a ``Future`` at a later point in time, when the result of ths ``Future`` is likely to be present.

Our initial implementation of the `Brain` in the last chapter took a ``Future`` that returned a constant value after it has been polled 10 times. Everytime this polling returned ``Poll::Pending`` the ``Brain`` ensured that the next *processing cycle* will again invoke the ``poll`` function of this ``Future``. This is quite inefficient as the contineusly polling of the ``Future`` will likely waste resources and processing capabilities of the `Brain`. In a real world scenario it will more likely be an event - a timer, an I/O event, an extern GPIO interrupt - that will indicate that the requested result of a ``Future`` is available.

But how could the ``Brain`` know, that the event for a specific ``Future`` has been raised and therefore the wrapping ``Thought`` need re-processing? This is, where the `Context` and it's containing ``Waker`` comes into play. The ``Brain`` will create a ``Waker`` for each ``Thought`` that is about to be processed and pass this as part of the ``Context`` to the ``Future`` that is polled. It is now the responsibility of the ``Future`` to store this `Waker` and use it to signal to the ``Brain`` that it need to re-process the current ``Thought`` this ``Future`` is wrapped into. The most common use-case is to register this ``Waker`` with a system I/O event handler or an interrupt handler. So if processing the ``Thought`` returns ``Poll::Pending`` the ``Brain`` can *park* this one until it got *woken* by the ``Waker``.

>![Note](./images/note.png) It's also possible that multiple `Thought`'s might share the same `Waker` if their re-processing is likely to depend on the same event, but for the sake of simplicity we stick to the creation of individual `Waker` for each `Thought` the `Brain` will process.

### The Waker

The high level concept of a *Waker* is kind of straight forward. A *Waker* defines the behavior of a thing (in our case the `Thought`) that provides the methods to get woken. This behavior is defined with the `Wakeable` trait. The high-level intuiton of a `Waker` will look like this:

```rust ,ignore
type Waker = Arc<dyn Wakeable>;

pub trait Wakeable {
    /// Wake a Wakeable and consuming the [Arc]
    fn wake(self: Arc<Self>) {
        Self::wake_by_ref(&self)
    }

    /// Wake the Wakeable keeping the Arc ref count intact
    fn wake_by_ref(self: &'_ Arc<Self>);
}
```
<br>
<details>
<summary>There are several challenges that need to be solved for a <code class="hljs">Waker</code> to allow its usage in a general way, as the fact that it is <i>tied</i> to a <code class="hljs">Thought</code> is kind of arbitrary from the <code class="hljs">Waker</code>'s point of view. So if you are curious about the low-level details feel free to continue reading by expanding this block.</summary>
<br>

When `poll`ing a generic `Future` we've already seen, that the `Waker` will be passed as part of the `Context`. In the same way the `Future` as part of the `Thought` can't contain any generic *output type* because we would like to store it in some sort of a list in the *brain* - the `Waker` can't hold any generic type information either.

The solution to this problem is to use trsit objects like `Arc<dyn Wakeable>`. But as this still covers the type information we would need the *raw* version of the trait object to be stored. How can this be achieved? Well, the answer to this lies in the definition of a trait object. On raw/memory level its nothing more than a pointer to the actual data of the trait object together with a [V(irtual Function)Table](https://en.wikipedia.org/wiki/Virtual_method_table). The VTable as such is a list of function pointers where the very first parameter passed is the pointer to the actual data of the object this function belongs to. This type erased representation of a `Waker` is provided within the rust core library as [RawWaker](https://doc.rust-lang.org/core/task/struct.RawWaker.html).

For reference the definitions from the core library below:

```rust ,ignore
pub struct RawWaker {
    data: *const (),
    vtable: &'static RawWakerVTable,
}

pub struct RawWakerVTable {
    clone: unsafe fn(*const ()) -> RawWaker,
    wake: unsafe fn(*const ()),
    wake_by_ref: unsafe fn(*const ()),
    drop: unsafe fn(*const ()),
}
```

### Being Wakeable

So the first thing to get the `Thought` being wakeable is to create the functions that will make up the VTable for it. All those functions has one thing in common: They get the pointer to the current `Wakeable` as a type erased raw pointer. This need to be cast back into a typed raw pointer and from there to its `Arc` representation. This is actually safe as the only way this raw pointer could have been created is from the corresponding `Arc::into_raw` as shown later.

#### Function to Clone

The first function required will clone the `RawWaker` from the `Wakeable`. Being able to create clones of the `Waker` enables them to be stored as part of interrupt handler or I/O event handlers to allow waking the `Thought`'s from within them.

```rust ,ignore
unsafe fn clone<T: Wakeable>(wakeable: *const ()) -> RawWaker {
    let wakeable: *const T = wakeable.cast();
    let wakeable_ref: &Arc<T> = &*ManuallyDrop::new(
        Arc::from_raw(wakeable)
    );

    Arc::clone(wakeable_ref).into_raw_waker()
}
```

#### Function to Wake

The second function required will call the `wake` function of the `Wakeable` trait that actually will be implemented in the `Thought`. This function will consume the `Waker` (it's `Arc`) when called. This is most likely being called on cloned `Waker` for example inside an interrupt handler.

```rust ,ignore
unsafe fn wake<T: Wakeable>(wakeable: *const ()) {
    // transfer the raw pointer back into it's type pointer
    let wakeable: *const T = wakeable.cast();
    let wakeable: Arc<T> = Arc::from_raw(wakeable);
    // wake the wakeable
    Wakeable::wake(wakeable);
}
```

There is also a non-consuming version of the `wake` function that should be used when the current `Wakeable` should not be consumed (as it is the only existing reference for example - like the one directly stored within the context)

```rust ,ignore
unsafe fn wake_by_ref<T: Wakeable>(wakeable: *const ()) {
    // transfer the raw pointer back into it's type pointer
    let wakeable: *const T = wakeable.cast();
    let wakeable_ref = &*ManuallyDrop::new(Arc::from_raw(wakeable));
    Wakeable::wake_by_ref(wakeable_ref);
}
```

#### Function to Drop

When handing out clones of the `Wakeable` it was necessary to ensure those will be dropped manually (by using `ManuallyDrop`). For this very reason we also require to implement the drop function for those clones. So just safely drop the `Arc` we build from the raw pointer.

```rust ,ignore
unsafe fn drop<T: Wakeable>(wakeable: *const ()) {
    // transfer the raw pointer back into it's type pointer
    let wakeable: *const T = wakeable.cast();
    core::mem::drop(Arc::from_raw(wakeable));
}
```

With the functions in place building up the VTable its now possible to create a `RawWaker` that is a type erased version of a trait object representing the `Wakeable`.

```rust ,ignore
fn into_raw_waker(self: Arc<Self>) -> RawWaker {
    let raw_wakeable: *const () = Arc::into_raw(self).cast();
    let raw_wakeabe_vtable = &Self::WAKER_VTABLE;

    RawWaker::new(
        raw_wakeable,
        raw_wakeabe_vtable,
    )
}
```

This function will be the only way to construct the `RawWaker` from a `Wakeable`. So it uses the `Arc::into_raw` to convert the `Arc` into a raw pointer which makes it totally safe to convert the raw pointers passed to the VTable functions back into an `Arc` using the `Arc::from_raw` function. To keep things tied to gether that belongs together we define a private trait the covers the VTable as well as the `into_raw_waker` function. In the following listing the details of the VTable functions are omitted for brevity.

```rust ,ignored
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
                /* details omitted */
            }
            clone::<Self>
        },
        {
            unsafe fn wake<T: Wakeable>(wakeable: *const ()) {
                /* details omitted */
            }
            wake::<Self>
        },
        {
            unsafe fn wake_by_ref<T: Wakeable>(wakeable: *const ()) {
                /* details omitted */
            }
            wake_by_ref::<Self>
        },
        {
            unsafe fn drop<T: Wakeable>(wakeable: *const ()) {
                /* details omitted */
            }
            drop::<Self>
        }
    );
}
```

Finally we provide an auto trait implementation for all types that implement the `Wakeable` trait to also implement the `WakeableTraitObject` trait.

```rust ,ignore
impl<T: Wakeable> WakeableTraitObject for T {}
```

With this in place we can now provide a function as part of the `Wakeable` trait that allows direct conversion from a `Wakeable` into a `Waker`.

```rust ,ignore
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
```

</details>

### Waking the Wakeable

With the ``Wakeable`` trait we can now define our ``Thought`` to be able to get woken, right?

```rust ,ignore,noplayground
impl Wakeable for Thought {
    fn wake_by_ref(self: &Arc<Self>) {
        // this Thought shall be able to get woken. This would require the Brain
        // to re-process the same. How to achive this? How to push to the Brain?
        // Should we pass a borrow of the Thoughtlist of the Brain to the
        // Thought?
    }
}
```

Even though we made the ``Thought`` wakeable and we could implement the waking functionality we are struggling here at the next issue with our initial [naive Brain implementation](./03-02-first-brain.md)

The reason is, that we store the `Thought`'s that require processing in a ``Vec`` within the ``Brain``. To add entries to this list we would require mutual exclusive access to the same and we would require to share the access accross different ``Thought``s. One possible way to address this is to use an ``Arc`` and a *Mutex-Like-Lock* around this ``Vec``. But this will also not really solve the problem as the ``Brain`` always acquires this lock while processing the list of `Thought`'s. Therefore it is very unlikely that the ``Waker`` will ever get a chance to acquire the lock for adding it's related ``Thought`` back to the things the ``Brain`` need to process. Kind of a dilemma, right?

But - there is a solution to this. If we carefully check what the requirements of the list of the `Thought`'s within the ``Brain`` are we see that it is more acting like a *queue*. The ``Brain`` is picking up the things that need processing from the front of the queue and the ``Wakeable`` will push itself to the end of the queue once woken.

So what we will need is the implementation of a **queue**!

The queue does have 2 sides, one that allows *popping* of ``Thought``s and one that allows *pushing* of ``Thought``'s. And both ends shall be able to be shared independendtly - for example to the `Thought`'s that requires to push themself to the queue again once they are woken. The perfect candidate here is a *channel*. There are several kinds and implementations of channels available in the open source community. Based on our specific use case, where we might run the ``Brain`` on bare metal in ``no_std`` environment likely on different cores and the need to push ``Thought``'s to this channel from different ``Waker``, we require a **Multi Producer Multi Consumer** channel, that preferrably is implemented in a lock-free fashion.

Check out the next chapter for the details.
