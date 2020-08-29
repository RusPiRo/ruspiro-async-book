# Proof the Runtime

After we have implemented our own simple runtime / executor to support the async implementation paradigm it's time to proof that it is really working. As we might do this without requiring to deploy this part to any actual embedded hardware we can execute this test within a normal operating system environment.

To do so we use a small binary crate utilizing our `Brain` implementation. The presence of different cores can be simulated by running different threads.

So the `main` function in this validation will look like this:

```rust ,noplayground
{{#include ../listings/04-proof-brain/src/main.rs:brain_usage_main}}
```

To be able to *look behind the scenes* a bit we adjust the implementation of our `Future` that will hand out a number after a certain amount of `poll`'s to see which *core*/*thread* the current `poll` is executed.

```rust ,noplayground
{{#include ../listings/04-proof-brain/src/main.rs:brain_usage}}
```

Running now this example will yield the follwoing output

```text
Hello, world!
polled 1 time(s) - now on ThreadId(1)
polled 1 time(s) - now on ThreadId(1)
polled 2 time(s) - now on ThreadId(1)
polled 3 time(s) - now on ThreadId(1)
waited for 40
polled 1 time(s) - now on ThreadId(3)
polled 2 time(s) - now on ThreadId(1)
polled 3 time(s) - now on ThreadId(3)
polled 4 time(s) - now on ThreadId(1)
polled 5 time(s) - now on ThreadId(3)
polled 1 time(s) - now on ThreadId(2)
polled 2 time(s) - now on ThreadId(3)
polled 3 time(s) - now on ThreadId(2)
polled 4 time(s) - now on ThreadId(3)
polled 5 time(s) - now on ThreadId(2)
waited for 20
polled 6 time(s) - now on ThreadId(1)
waited for 30
polled 2 time(s) - now on ThreadId(4)
polled 3 time(s) - now on ThreadId(3)
polled 4 time(s) - now on ThreadId(2)
polled 5 time(s) - now on ThreadId(1)
polled 6 time(s) - now on ThreadId(3)
polled 7 time(s) - now on ThreadId(2)
polled 8 time(s) - now on ThreadId(1)
polled 9 time(s) - now on ThreadId(2)
polled 10 time(s) - now on ThreadId(3)
waited for 10
```

You can clearely see how the different *cores*/*threads* are picking up the work to drive a waiting `Future` to completion.
