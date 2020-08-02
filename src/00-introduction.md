# Introduction

Asynchronous programming promisses parallel processing that can speed up your program execution, while beeing sometimes challanging.

My current projects target the Raspberry Pi. Although the Raspberry Pi with its CPU power and large memory might not count as embedded system, I'm running it bare metal (without any OS), so from my point of view this pretty much also falls into this category then. I quite often stumbled over the requirement to be able to continue processing some tasks while waiting for an external event - e.g. data arriving, signal on a GPIO pin. This would be a perfect match to async processing. However, working on a project targeting embedded systems `[no_std]` is crucial. This was for a certain amount of time hindering using the already stabalized *async/await* features of Rust.


But luckily at the time of writing this book, the *async/await* feature of Rust was already made available for `[no_std]` projects in the stable release. Now was the time to adopt this for my projects as well.

To be able to learn and understand how the whole thing is working under the hood I decided to write a lightweight executor that is able to run in `[no_std]` and allows to use `async fn` and `await`. To share my experience and insights that demystified the async/await for me I've written this book to also help others to experience this kind of *a-ha*  💡 effect.
