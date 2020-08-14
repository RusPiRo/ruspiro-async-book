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
    sync::Arc,
};

use ruspiro_channel::mpmc;
use ruspiro_lock::DataLock;

mod thought;
use thought::*;

mod wakeable;
use wakeable::Wakeable;

// ANCHOR: brain
/// A quite naive Brain that should process the `Future`s wrapped in a `Thought`
// ANCHOR: brain_struct
struct Brain {
    /// the sender side of the mpmc channel to pass the ``Thought``s that
    /// require processing
    sender: mpmc::Sender<Arc<Thought>>,
    /// the receiver side of the mpmc channel the ``Brain`` picks ``Thought``s
    /// from to process them
    receiver: mpmc::Receiver<Arc<Thought>>,
}
// ANCHOR_END: brain_struct
// ANCHOR: brain_think_on
impl Brain {
    fn default() -> Self {
        let (sender, receiver) = mpmc::channel();

        Self {
            sender,
            receiver,
        }
    }

    /// Add a new `Future` to the [Brain], so it can be processed
    fn think_on(&self, thinkable: impl Future<Output = ()> + 'static) {
        // ensure the given Future is getting a fixed position on the HEAP
        let thinkable = Box::pin(thinkable);
        // create the Thought
        let thought = Arc::new(
            Thought {
                    thinkable: DataLock::new(thinkable),
                    sender: self.sender.clone(),
                }
            );
        // push the Thought to the list of thoughts to think on
        self.sender.send(thought);
    }
}
// ANCHOR_END: brain_think_on
// ANCHOR: brain_do_thinking
impl Brain {
    /// Do the actual thinking - check for Thoughts that waits for processing
    /// and drive them to completion
    fn do_thinking(&self) {
        // check if there is a new Thought available in the channel
        while let Ok(thought) = self.receiver.recv() {
            // create the Waker from the current Thought
            let waker = Wakeable::into_waker(&thought);
            // create the Context from the given Waker
            let mut context = Context::from_waker(&waker);
            // lock the Future contained in the Thought and poll it
            let mut thinkable = thought.thinkable.lock();
            if let Poll::Pending = thinkable.as_mut().poll(&mut context) {
                // if the state is Poll::Pending we just unlock the Future
                drop(thinkable);
                // in case it will be still valid and required to re-process this
                // Thought the Waker will resend it through the channel
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
            cx.waker().wake_by_ref();
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

    let brain = Brain::default();

    let future = GiveNumberFuture {
        give_after_tries: 10,
        current_tries: 0,
    };
    brain.think_on(MasterFuture::State1(
        Box::pin(future)
    ));

    brain.do_thinking();
}
// ANCHOR_END: brain_usage_main