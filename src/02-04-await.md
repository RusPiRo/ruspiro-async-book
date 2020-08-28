# Await

The term `await` is another keyword in the Rust programming language. This is used to tell the compiler that the current sequential processing shall be paused until the value of an asynchronous processing is available. Once the value is availble the processing continues. Like `async` the `await` keyword is also syntactic sugar for the developer writing code to be run asynchronously. It is used on variables that hold a `Future` to hint the compiler to generate code that allows to `poll` the actual state of the `Future` and only continue in case the value is ready. As *waiting* for the actual result of a `Future` also requires the capability of asynchronous processing this keyword can only be used within an `async fn`.

Let's illustrate the usage:

```rust , ignore
async fn give_number() -> u32 {
    100
}

async fn wait_for_number() {
    let number = give_number().await;
    println!("Number: {}", number);
}
```

So the `wait_for_number` function requires to be `async` as well to be able to contain `await` points. While the `await`-ing of the presence of the value pauses the execution of this actual code the curret *thread* ore *processor core* is free to pick up other things to do until the *executor* decides to re-visit this `await` point to check if progress can be made.
