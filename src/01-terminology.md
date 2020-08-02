# Terminology

Before we start implementing our embedded async/await runtime we will first cover some terms and explain them. This will ensure a common understanding througout this book.

## Future

A `Future` represents the processing whose result may be available at a future point in time. The actual procesing result need to be actively requested. This is called *polling* and is implemented as the `poll` function on the `Future` trait. The result of the `poll` function represents the the state of the `Future`. It could either be `Ready` yielding the actual result of the processing, or `Pending` indicating that actual result is still not available.

## Executor

The executor is the heart of the asynchronous processing. It provides the runtime that is able to continuesely request the result of the `Future`s until they return the `Ready` state. Registering a `Future` at the executer to allow the same to run it to completion is called *spawning*

## Async


## Await
