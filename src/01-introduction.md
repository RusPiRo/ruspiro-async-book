# Introduction

One of the main promises of asynchronous, parallel processing is that it can increase overal performance of your program. Although this does not apply to any generic situation most of us would like to try out whether this would give the desired boost or not. However, designing and writing a program, ready to benefit from asynchronous execution is often challenging.

I'm - at the time of writing - working on a Raspberry Pi OS written as much as possible in Rust. Especialy in the space of micro computer and embedded systems where GPIO's are used to access sensors and the like the OS or program often *idles* waiting for external events but also blocking any further processing while doing so. So the decision was made, that one of the main features of this OS will be, that it utilises the 4 cores of the provided processor of the Raspberry Pi as best as possbile and thus allows asynchronous and parallel processing.  But writing a custom operating system does mean that one need to deal with some restrictions. One with the most impact on the usage of Rust as programming language is, that it is not possible to use the standard library. So everything need to be built with `[no_std]`.

While Rusts still young feature to support asynchronous processing is already available for a while as part of the standard library, some compiler feature did not work for `no_std`. But, recently the continues efforts of the Ruast language group also made the use of the additional needed syntax available to be successfully compiled with stable Rust in a `no_std` environment. Even though the whole *async/await* in Rust was quite a mystery to me I started to design and implement a runtime for my Raspberry Pi OS. This pretty much allowd me to unserstand all the different pieces of this mystery - like `async fn()` and `future.await` thingies - and how everything fits together.

To share my experience and insights that lead to the demystification for me I've written this book to also help others to experience this kind of *a-ha* 💡 effect.

## Prerequisits

The provided code is used in `no_std` environment. However, some specific use cases require heap memory allocations to work. This requires a custom allocator to be present to perform the those allocations used for example by `Arc` and `Box`.
