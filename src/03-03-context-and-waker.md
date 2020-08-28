## The Context and the Waker

The first issue of our [naive Brain](./03-02-first-brain.md) is, that it does not compile for an obvious reason: The signature of the ``poll`` function requires to pass a ``Context`` and we where not able to provide one for now, but - what is this `Context` about ?

### The Context

In the current version of Rust, the ``Context`` that will be handed over to the ``Future`` while polling the same only contains a reference to a ``Waker``. This ``Waker`` can be used to *wake* the processing of a ``Future`` at a later point in time, when the result of ths ``Future`` is likely to be present.

Our initial implementation of the `Brain` in the last chapter took a ``Future`` that returned a constant value after it has been polled 10 times. Everytime this polling returned ``Poll::Pending`` the ``Brain`` ensured that the next *processing cycle* will again invoke the ``poll`` function of this ``Future``. This is quite inefficient as the contineusly polling of the ``Future`` will likely waste resources and processing capabilities of the `Brain`. In a real world scenario it will more likely be an event - a timer, an I/O event, an extern GPIO interrupt - that will indicate that the requested result of a ``Future`` is available.

But how could the ``Brain`` know, that the event for a specific ``Future`` has been raised and therefore the wrapping ``Thought`` need re-processing? This is, where the `Context` and it's containing ``Waker`` comes into play. The ``Brain`` will create a ``Waker`` for each ``Thought`` that is about to be processed and pass this as part of the ``Context`` to the ``Future`` that is polled. It is now the responsibility of the ``Future`` to store this `Waker` and use it to signal to the ``Brain`` that it need to re-process the current ``Thought`` this ``Future`` is wrapped into. The most common use-case is to register this ``Waker`` with a system I/O event handler or an interrupt handler. So if processing the ``Thought`` returns ``Poll::Pending`` the ``Brain`` can *park* this one until it got *woken* by the ``Waker``.

>![Note](./images/note.png) It's also possible that multiple `Thought`'s might share the same `Waker` if their re-processing is likely to depend on the same event, but for the sake of simplicity we stick to the creation of individual `Waker` for each `Thought` the `Brain` will process.

### The Waker

The actual implementation details of a ``Waker`` might feel a bit overwhelming at first look, as it is really low level and does use some *tricks* . Nevertheless, let's try to walk through the different pieces and how it can be utilized in the context of our ``Brain``.

The ``Waker`` is used to notifiy the ``Brain`` that a ``Thought`` need to be processed (again). But that we deal with a ``Brain`` and a ``Thought`` in this book is kind of arbitratry to the definition of the ``Waker`` in the Rust core library. So this one requires to be agnostic to any *Brain* and any *Thought* one could come up with. For this reason the ``Waker`` requires to be as *generic* as possible - yet not using any *Generics* as such. Thus the ``Waker`` is some kind of a container that stores some raw pointers. How to convert from and to those raw pointers is left to the implementer of the *Brains* and *Thoughts* - or in other words the *Executors* and *Tasks* out there. The following raw pointer are required by a ``Waker``:

- Raw pointer to the actual *Thing* that shall be woken - the *Wakeable*
- Raw pointer to a ``wake`` function to notify the *Executor* to process this *Thing*
- Raw pointer to a ``clone`` function to allow cloning of the ``Waker`` as it may be shared accross several event handlers
- Raw pointer to a ``drop`` function to drop the actual *Thing* that never requires to be woken again

The raw pointer to the respective functions are stored in a type called ``RawWakerVTable``. Each of those functions require only one parameter beeing the raw pointer to the thing that requires waking - the ``Thought`` in our case.

The raw pointer to the actual thing to be woken is stored together with the ``RawWakerVTable`` in a type called ``RawWaker`` that is finally wrapped by ``Waker`` struct. See the [rust documentation](https://doc.rust-lang.org/core/task/struct.Waker.html).

### Beeing Wakeable

While the Rust core library provides the format and structure how a ``Waker`` should look like, the instantiation of one and the provisioning of the proper raw pointers to be used is a *Executor* specific implementation. How can we apply this to our ``Brain``?

Let's start buttom up.

The lowest thing we need are those functions that can accept a raw pointer that represents the ``Wakeable`` thing. As all of those functions are dealing with raw pointers they are *unsafe* by nature, however, if we adhere to some boundary conditions they are actually safe to use. One of those conditions is, that the raw pointer to the ``Wakeable`` is always representing an ``Arc``. Another boundary condition would be, that the type contained in the ``Arc`` is actually something we could wake. To adhere to this condition we define a Trait repesenting a ``Wakeable`` with the required functions that will be called to actualy wake it.

```rust ,ignore,noplayground
{{#include ../listings/03-simple-brain/src/wakeable.rs:trait_part1}}
```

With the ``Wakeable`` trait in place we can now define the different functions. It's fine to use raw pointers to those functions as they are built into the binary at a fixed position and therefore are immovable with a static lifetime.

```rust ,ignore, noplayground
{{#include ../listings/03-simple-brain/src/wakeable.rs:wake_functions}}
```

The final bit that is missing is that we would require to be able to convert a ``Wakebale`` into an actual ``Waker``. For doing so, we provide a corresponding function with a default implementation at the ``Wakeable`` trait.

```rust ,ignore, noplayground
{{#include ../listings/03-simple-brain/src/wakeable.rs:trait_part2}}
```

>![Note](./images/note.png) The initial created ``Waker`` is not allowed to provide a ``drop`` function as it stores the raw pointer of the initial ``Arc<Thought>``. This could be *solved* if we would already clone this first ``Arc`` and pass this clone to the ``Waker``, but this would lead to an unnecceary additional heap allocation assuming that it is rather unlikely that the ``Waker`` will be cloned at all. So this is a kind of an optimization for setting up the first ``Waker`` of a ``Wakeable``.

### Waking the Wakeable

With the ``Wakeable`` trait we can now define our ``Thought`` to be able to get woken, right?

```rust ,ignore,noplayground
impl Wakeable for Thought {
    fn wake_by_ref(self: &Arc<Self>) {
        let clone = Arc::clone(self);
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
