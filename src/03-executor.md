# The Executor

The *Executor* is the brain of the runtime that allows asynchronous processing - so let's call it **Brain** from now on. The *Brain* maintains a queue of tasks (*Thoughts*) that require processing and also processes (*thinks*) them. But, before we actually design and implement our *Brain* lets sketch some requirements and context around the same:

- The *Brain* should be able to utilise all 4 cores of the RPi
- The *Thoughts* require safe shareabilty accross those cores
- Picking a new *Thought* to think on it should not block adding new *Thoughts* to the queue

Let's work through the list bottom up. The last point could be achieved with the implementation of a special kind of a channel.





## The Brain

With the MPMC channel in place we could define the *Brain*.

```rust ,ignore
use mpmc::{Sender, Receiver, channel};

struct Brain {
    sender: Sender<Arc<Though>>,
    receiver: Receiver<Arc<Though>>,
}
```