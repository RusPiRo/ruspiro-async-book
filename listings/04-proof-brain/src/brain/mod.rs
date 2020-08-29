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
pub struct Brain {
    /// the sender side of the mpmc channel to pass the ``Thought``s that
    /// require processing
    sender: mpmc::Sender<Arc<Thought>>,
    /// the receiver side of the mpmc channel the ``Brain`` picks ``Thought``s
    /// from to process them
    receiver: mpmc::Receiver<Arc<Thought>>,
}
// ANCHOR_END: brain_struct

// ANCHOR: brain_impl
impl Brain {
    pub fn default() -> Self {
        let (sender, receiver) = mpmc::channel();

        Self {
            sender,
            receiver,
        }
    }

    /// Add a new `Future` to the [Brain], so it can be processed
    pub fn think_on(&self, thinkable: impl Future<Output = ()> + 'static) {
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

    /// Do the actual thinking - check for Thoughts that waits for processing
    /// and drive them to completion
    pub fn do_thinking(&self) {
        // check if there is a new Thought available in the channel
        while let Ok(thought) = self.receiver.recv() {
            // create the Waker from the current Thought
            let waker = Wakeable::into_waker(&thought);
            //println!("{:?}", waker);
            // create the Context from the given Waker
            let mut context = Context::from_waker(&waker);
            // lock the Future contained in the Thought and poll it
            let mut thinkable = thought.thinkable.lock();
            if let Poll::Pending = thinkable.as_mut().poll(&mut context) {
                // if the state is Poll::Pending we just unlock the Future
                drop(thinkable);
                // in case it will be still valid and required to re-process this
                // Thought the Waker will re-send it through the channel
            }
        }
    }
}
// ANCHOR_END: brain_impl
// ANCHOR_END: brain