## The First Naive Brain

With the first building blocks in place we might be able to sketch our first version of a *Brain*.

The first part we need to define the struct that will contain the list of ``Thoughts`` that are about to be processed. We wrap them into an ``Option`` to indeciate those that do not need further processing.

```rust ,no_run,noplayground
{{#include ../listings/03-naive-brain/src/main.rs:brain_struct}}
```

Now we can implement a function that is able to take a ``Future``, wrap into a ``Thought`` and push it to the list.

```rust ,no_run,noplayground
{{#include ../listings/03-naive-brain/src/main.rs:brain_think_on}}
```

Finally the *Brain* requires a function that allows processing of the list of ``Thought``'s. This function will iterate over the items of ``Brain::thoughts`` and will call the ``poll`` function for each ``Future`` contain in the ``Thought``. If this polling yields a ``Poll::Pending`` state the ``Thought`` will be kept in place of the list and is polled again at the next *cycle*.

```rust ,no_run,noplayground,codenotcompile
{{#include ../listings/03-naive-brain/src/main.rs:brain_do_thinking}}
```

The actual first sketch of the ``Brain`` has several flaws. One of them is the `poll` function of the ``Future`` requiring a ``Context`` to be passed. Without having this in place the code will actually not compile. However, before dealing with the different challenges of the above coude let's have a look how we could make use of the ``Brain``.

As a first step we will define a ``Future`` that returns a constant value after it has been polled for a fixed number of tries. Nothing really asynchronous here, you are totally right, but let's start simple.

```rust ,no_run,noplayground
{{#include ../listings/03-naive-brain/src/main.rs:brain_usage_1}}
```

This ``Future`` does return a value and will reflect the first state in the already discussed *outermost future* like so:

```rust ,no_run,noplayground
{{#include ../listings/03-naive-brain/src/main.rs:brain_usage_2}}
```

Within the ``main`` function we can now create our ``Brain``, tell it to think on the *master thought* which will ultimately wait for the first state to be ready.

```rust ,no_run,noplayground,codenotcompile
{{#include ../listings/03-naive-brain/src/main.rs:brain_usage_main}}
```

In case this would already compile and run it would create the following output.

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

### The Issues of the Naive Brain

1. The missing ``Context`` hindering to get the implementation compiled - and what is it used for by the way?

2. The usage of a ``Vec`` to store the ``Thought``'s may grow endlessly without further handling.

3. The ``Brain`` requires mutable access to allow adding of new ``Thought``'s and processing them.

4. Not completely related to the *Brain* itself, but writing the outermost ``Future`` by hand is odd, there must be something better! And there is ...

Let's tackle them one by one in the next chapters.
