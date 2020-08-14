# Terminology

Before we start implementing our embedded async/await runtime we will first cover some terms to ensure a common understanding througout this book.

## Future

A `Future` represents the processing whose result may be available at a future point in time. The actual procesing result need to be actively requested. This is called *polling* and is implemented as the `poll` function on the `Future` trait. The result of the `poll` function represents the the state of the `Future`. It could either be `Ready` yielding the actual result of the processing, or `Pending` indicating that actual result is still not available.

## Executor

The executor is the brain of the asynchronous processing. It provides the runtime that is able to continuesely request the result of the `Future`s until they return the `Ready` state. Registering a `Future` at the executer to allow the same to run it to completion is called *spawning*. As part of this the `Future` is wrapped into a structure that allows proper handling of processing and re-spawning of `Pending` futures. In the Rust libraries this wrapper is called *Task*. To stick with the *Brain* analogy for the executore, however, we will call this wrapper `Thought`. This also allows to spot easily the custom types that are required to be implemented in `no_std` environments.

## Async

## Await
