//! # Naive Async Processing
//!
//! Implementing a simple *Brain*

extern crate alloc;

use core::{
    future::Future,
    task::{Poll, Context},
    pin::Pin,
};

use alloc::sync::Arc;

mod brain;

// ANCHOR: brain_usage
struct GiveNumberFuture {
    number_to_give: u32,
    give_after_tries: u32,
    current_tries: u32,
}

impl Future for GiveNumberFuture {
    type Output = u32;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.get_mut();
        println!("polled {} time(s) - now on {:?}",
            this.current_tries + 1,
            std::thread::current().id()
        );
        if this.give_after_tries > this.current_tries + 1 {
            this.current_tries += 1;
            cx.waker().wake_by_ref();
            Poll::Pending
        } else {
            Poll::Ready(this.number_to_give)
        }
    }
}

async fn main_thought(number: u32, tries: u32) {
    let future = GiveNumberFuture {
        number_to_give: number,
        give_after_tries: tries,
        current_tries: 0,
    };

    let number = future.await;
    println!("waited for {}", number);
}
// ANCHOR_END: brain_usage

// ANCHOR: brain_usage_main
fn main() {
    println!("Hello, world!");

    let brain = Arc::new(
        brain::Brain::default()
    );

    // assume we have 4 cores on the target system -> spawn 3 threads
    // the current thread is the 4th core though
    for _ in 0..3 {
        let cloned_brain = Arc::clone(&brain);
        std::thread::spawn( move || {
            loop {
                cloned_brain.do_thinking();
            }
        });
    }

    // just spawn 4 async executions ...
    brain.think_on(main_thought(10, 10));
    brain.think_on(main_thought(20, 5));
    brain.think_on(main_thought(30, 6));
    brain.think_on(main_thought(40, 3));

    // use the current thread as the 4th core
    loop {
        brain.do_thinking();

        // we could also spawn new  [Thought]s here or from within an
        // async fn to keep the Brain busy ..
    }
}
// ANCHOR_END: brain_usage_main