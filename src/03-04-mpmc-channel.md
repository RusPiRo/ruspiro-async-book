## Multi Producer Multi Consumer (MPMC) Channel

The MPMC channel allows adding new entries from any core or thread (multi producer), picking the next entry by any core or thread (multi consumer) and works like a FIFO queue. There are for sure multiple approaches possible to implement such a channel in a non-blocking way. I will present quite a simple one here that has proven to work at least for all my current use cases in a bare metal ``no_std`` environment. The source code can be found [here](https://github.com/RusPiRo/ruspiro-channel).


### Revisit the Brain

With the *channel* in place we can adopt the `Brain` to keep the `Sender` and the `Receiver` side of it. The `Sender` will be shared with the `Waker` while the `Receiver` is used by the `Brain` only.

```rust ,ignore,noplayground
{{#include ../listings/03-simple-brain/src/main.rs:brain_struct}}
```

>![Note](./images/note.png) Well you might wonder why there is a *Multi-Sender-Multi-Consumer* channel if the consuming part is used by the `Brain` only? The reason is, that we will use the `Brain` to run it's code on different cores and therefor indeed a *multi-consumer* is required.


### Revisit the Wakeable

In addition to updating the `Brain` storing now the two sides of the channel we also can now store the `Sender` of the channel with each `Thought` which allows the same to push itself back to the queue of things the `Brain` should process.

The updated `Thought` will look like this:

```rust ,ignore,noplayground
{{#include ../listings/03-simple-brain/src/thought.rs:thought}}
```

The implementation of the `Wakeable` trait to add the wake behavior to it can now finally be implemented:

```rust ,ignore,noplayground
{{#include ../listings/03-simple-brain/src/thought.rs:thought_waking}}
```

The final bit now is to always pass the `Sender` to the `Thought` once the `Brain` is requested to think on one:

```rust ,ignore,noplayground
fn think_on(&self, thinkable: impl Future<Output = ()> + 'static) {
    // ensure the given Future is getting a fixed position on the HEAP
    let thinkable = Box::pin(thinkable);
    // create the Thought
    let thought = Arc::new(
        Thought {
                thinkable: DataLock::new(thinkable),
                sender: self.sender.clone(),
            }
        );
    // push the Thought to the list of thoughts to think on
    self.sender.send(thought);
}
```
