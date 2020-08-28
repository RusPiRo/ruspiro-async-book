## The simple Brain

Now we have everything in place to implement a first functional simple `Brain`.

```rust ,ignore,noplayground
{{#include ../listings/03-simple-brain/src/main.rs:brain}}
```

The usage of this simple brain is pretty much similar to the first attempt. We implement an example `Future` and a simple `async` function that can be *spawned* to the `Brain` and is `await`ing the example `Future`.

```rust ,ignore,noplayground
{{#include ../listings/03-simple-brain/src/main.rs:brain_usage}}
```

And finally the actual `main` function utilizing the `Brain` to process the `Future`.

```rust ,ignore,noplayground
{{#rustdoc_include ../listings/03-simple-brain/src/main.rs:brain_usage_main}}
```

Running this will yield the result

```text
Hello, world!
polled 1 time(s)
polled 2 time(s)
polled 3 time(s)
polled 4 time(s)
polled 5 time(s)
polled 6 time(s)
polled 7 time(s)
polled 8 time(s)
polled 9 time(s)
polled 10 time(s)
waited for 20
```
