## The Thought

The *Thought* is an entity the *Brain* is able to process. It is kind of a wrapper that contains the thing that need to be thought on - the `Future`. This `Future` requires polling until it unveils it's result. As the `Future` contained in the `Thought` will be shared accross *threads* or *cores* in my Raspberry Pi it need to be stored on heap memory and pinned at it location to prevent it from moving around in memory.

```rust ,no_run,noplayground
{{#rustdoc_include ../listings/03-naive-brain/src/thought.rs:thought}}
```

You might be wondering, why the `Future` that is assigned to the `Thought` has a fixed `Output` type being the unit type `()`?!

The initial intuition might indicate that this is wrong! How could a `Future` with the unit return type ever yield an actual value our code might `await` at some point? And you are doing right questioning this and I also struggled at this point in the first place. So lets try to explain why this actually is correct!

**The Requirement:** At some point the *Brain* is required to maintain a list of `Thought`s that require processing. As the `Future` beeing part of the `Thought` ultimately will be part of the list as well, it's associated type required to be fully specified to allow it to participate in the list.

**Why is this correct:** In a typical sequential execution model the process flow starts by entering the `main` function and continues until it reaches the end of the main function which typically does not return any value (keep aside any error codes or the like for the time beeing). However, within the `main` function you are free to call functions that returns values, work with those values and do further processing. But finally the program does not yield a value at all. From this we can draw an intuition to the asyncronous world. The `Thought` (and it's `Future` is kind of the async representation of the synchronous *main* function. Within the `Thought`'s `Future` we can embed other `Future`s that yields values, wait on them, process those values etc. But ultimately at the end the `Thought` itself does not return anything. However, the advantage of the `Thought`'s in async programming model is, that we can *throw* as many of them as we like onto the *Brain*. And the *Brain* can *decide* which `Thought` to process next and which need to be *parked* as it still waits for a value inside it's processing to be available. *Throwing* new `Thought`'s onto the *Brain*is also called *spawning*.

So the conslusion is: It's totally fine and absolutely correct that the `Thought` stores a `Future` that does not yield any result.

>![Note](./images/note.png) If a `Future` *embedding* another `Future` and awaiting it's result before processing can continue it is also called *chaining* of `Future`'s. *Chaining* of `Future`'s in the async world is compareable to function calls in the syncronous world, where a function can only continue if the called function returns. The *chaining* of `Future`'s is unlikely to be implemented *manually* as this is done by the compiler when de-sugaring the `await` points within an `async` function.

With the `Thought` defined let's try to implement ur first version of a *Brain* in the next chapter.
