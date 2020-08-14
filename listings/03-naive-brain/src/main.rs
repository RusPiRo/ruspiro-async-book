//! # Naive Async Processing
//!
//! Implementing a simple *Brain*

extern crate alloc;

use core::{
    future::Future,
    task::{Poll, Context},
    pin::Pin,
};

use alloc::{
    boxed::Box,
    vec::Vec,
};

mod thought;
use thought::*;

// ANCHOR: brain
/// A quite naive Brain that should process the `Future`s wrapped in a `Thought`
// ANCHOR: brain_struct
struct Brain {
    /// the list of `Thoughts`s that require processing
    thoughts: Vec<Option<Thought>>,
}
// ANCHOR_END: brain_struct
// ANCHOR: brain_think_on
impl Brain {
    /// Add a new `Future` to the [Brain], so it can be processed
    fn think_on(&mut self, thinkable: impl Future<Output = ()> + 'static) {
        // ensure the given Future is getting a fixed position on the HEAP
        let thinkable = Box::pin(thinkable);
        // create the Thought
        let thought = Thought {
            thinkable,
        };
        // push the Thought to the list of thoughts to think on
        self.thoughts.push(Some(thought));
    }
}
// ANCHOR_END: brain_think_on
// ANCHOR: brain_do_thinking
impl Brain {
    /// Do the actual thinking - check for Thoughts that waits for processing
    /// and drive them to completion
    fn do_thinking(&mut self) {
        // run through the list of Thoughts that require thinking
        for maybe_thought in self.thoughts.iter_mut() {
            if let Some(thought) = maybe_thought.take() {
                // polling the Future requires some kind of Context, we will
                // discuss this in the next chapter
                if let Poll::Pending = thought.thinkable.as_mut().poll(cx) {
                    // as long as the state is Poll::Pending we put the
                    // the Thought back in place for the next round
                    *maybe_thought = Some(thought);
                }
            }
        }
    }
}
// ANCHOR_END: brain_do_thinking
// ANCHOR_END: brain

// ANCHOR: brain_usage_1
struct GiveNumberFuture {
    give_after_tries: u32,
    current_tries: u32,
}

impl Future for GiveNumberFuture {
    type Output = u32;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.get_mut();
        println!("polled {} time(s)", this.current_tries + 1);
        if this.give_after_tries > this.current_tries + 1 {
            this.current_tries += 1;
            Poll::Pending
        } else {
            Poll::Ready(20)
        }
    }
}
// ANCHOR_END: brain_usage_1
// ANCHOR: brain_usage_2
enum MasterFuture {
    State1(Pin<Box<dyn Future<Output = u32>>>),
}

impl Future for MasterFuture {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.get_mut();
        match this {
            Self::State1(wait_for) => {
                if let Poll::Ready(number) = wait_for.as_mut().poll(cx) {
                    println!("waited for {}", number);
                    Poll::Ready(())
                } else {
                    Poll::Pending
                }
            }
        }
    }
}
// ANCHOR_END: brain_usage_2

// ANCHOR: brain_usage_main
fn main() {
    println!("Hello, world!");

    let mut brain = Brain {
        thoughts: Vec::new(),
    };

    let future = GiveNumberFuture {
        give_after_tries: 10,
        current_tries: 0,
    };
    brain.think_on(MasterFuture::State1(
        Box::pin(future)
    ));

    loop {
        brain.do_thinking();
    }
}
// ANCHOR_END: brain_usage_main