## The Thought

The *Thought* is an entity the *Brain* is able to process. It is kind of a wrapper that contains the thing that need to be thought on - the `Future`. This `Future` requires polling until it unveils it's result.

```rust ,no_run,noplayground
{{#include ../listings/03-naive-brain/src/thought.rs:thought}}
```

You might be wondering, why the `Future` that is assigned to the `Thought` has a fixed `Output` type being the unit type `()`. The initial intuition might indicate that this is wrong, how could a `Future` that will be defined somewhere and somewhen along the implementation ever return a value that will be available at a future point in time? And you are doing right questioning this and I also struggled at the first place at this point. So lets try to explain why this is required and why it is correct to define it this way.

**The Requirement:** The associated type definition within the `Future` trait needs to be fully typed as we would like to send the `Thought` through a channel.

**Why is this correct:** In a typical sequential execution model the process flow starts by entering the `main` function and continues until it reaches the end of the main function which typically does not return any value (keep aside any error codes or the like). In this flow, when we'd like to introduce asynchronous processing the sequential execution would require to pause at a certain point and once the required data is available it will continue from this exact position. But finally we will at some point in time also reach the end of the `main` function that does not return any value. So in a way to get this pausing and re-entering the processing we would need some kind of a *state machine* that can be continuesly requested (*polled*) for it's current state and whether it is done (*Ready*) or not (*Pending*). Those terms sound familiar? They do - this is what a `Future` is for - allowing to be polled until the value - in this case the final state with the unit value - has been reached. So as a conclusion: There will always be an outermost `Future` with the unit output type `()` that may or may not contain a state where it need to wait for another embedded `Future` that is actually returning a specific value. So the outermost `Future` represents the asynchronous version of the sequential `main` function.

Let's try to picture this high-level explanation with a code snipped:

```rust ,ignore,codenotcompile
use core::future::*;
use core::task::*;
use core::pin::Pin;
use thought::Thought;

enum MasterFuture {
    State1(/* data associated with this state - quite likely another Future */),
    State2(/* data associated with this state - quite likely another Future */),
    State3(/* data associated with this state - quite likely another Future */),
}

/// Implement the Future trait for the `MasterFuture` to allow it to be *polled*
impl Future for MasterFuture {
    // The outermost Future does not return a value
    type Output = ();
    // Polling the outermost future moves its inner state until it is ready
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.get_mut();
        match this {
            Self::State1(/* data omitted */) => {
                // processing of this state happens here. If the processing has
                // been finished - continue with the next state if this future
                // gets polled again.
                *this = Self::State2();
                Poll::Pending
            },
            Self::State2(/* data omitted */) => {
                // processing of this state happens here. If the processing has
                // been finished - continue with the next state if this future
                // gets polled again.
                *this = Self::State2();
                Poll::Pending
            },
            Self::State3(/* data omitted */) => {
                // processing of this state happens here. If the processing has
                // been finished - the future is ready and does not need to be
                // polled again.
                Poll::Ready(())
            }
        }
    }
}

fn main() {
    // create the outermost Future
    let master = Box::pin(MasterFuture::State1(/* the initial state data */));

    // and the corresponding Thought
    let thought = Thought {
        thinkable: DataLock::new(Some(master)),
    };

    // now process this Thought until it is finished
    loop {
        // lock the inner future of this thought to be processed
        let mut maybe_future = thought.thinkable.lock();
        // check if - even we could get the lock there is a future assigned
        // and take it
        if let Some(future) = maybe_future.take() {
            // poll the future - the context (cx) will be discussed later
            match master.as_mut().poll(cx) {
                Poll::Pending => (),
                Poll::Ready(_) => break,
            }

            // put the future back into the option for the next round
            *maybe_future = Some(future);
        }
    }
}
```

>![Note](./images/note.png) The above code snipped does only illustrate the principle of the outermost ``Thought`` and ``Future``. It does not provide any real asynchronous processing but is a core building block that is required to be understood from my point of view.

The part in the previous code snipped that happens inside the loop is the part that the *Brain* should be responsible for - picking up a ``Thought`` and *think* on it :). So lets have a look at a first variant of the *Brain* in the next chapter.
