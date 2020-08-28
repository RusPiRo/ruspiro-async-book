## Multi Producer Multi Consumer (MPMC) Channel

The MPMC channel allows adding new entries from any core or thread (multi producer), picking the next entry by any core or thread (multi consumer) and works like a FIFO queue. There are for sure multiple approaches possible to implement such a channel in a non-blocking way. I will present quite a simple one here that has proven to work at least for all my current use cases in a bare metal ``no_std`` environment. The source code can be found [here](https://github.com/RusPiRo/ruspiro-channel).

With the *channel* in place we can adopt the `Brain` to keep the `Sender` and the `Receiver` side of it. The `Sender` will be shared with the `Waker` while the `Receiver` is used by the `Brain` only.

```rust ,ignore,noplayground
{{#include ../listings/03-simple-brain/src/main.rs:brain_struct}}
```

>![Note](./images/note.png) Well you might wonder why there is a *Multi-Sender-Multi-Consumer* channel if the consuming part is used by the `Brain` only? The reason is, that we will use the `Brain` to run it's code on different cores and therefor indeed a *multi-consumer* is required.
