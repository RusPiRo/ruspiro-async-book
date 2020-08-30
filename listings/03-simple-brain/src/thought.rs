//! # A Thought
//!
//! A Thought is an entity that wraps a `Future` the *Brain* could *think* on.
//!

use core::{
    future::Future,
    pin::Pin,
};

use alloc::{
    boxed::Box,
    sync::Arc,
};

use crate::wakeable::Wakeable;
use ruspiro_channel::mpmc;
use ruspiro_lock::DataLock;

// ANCHOR: thought
pub struct Thought {
    /// This is the actual thing the brain should process as part of the Thought
    pub thinkable: DataLock<Pin<Box<dyn Future<Output = ()> + 'static>>>,
    /// The sender side of the queue of the `Brain` to push myself for
    /// re-processing
    pub sender: mpmc::Sender<Arc<Thought>>,
}
// ANCHOR_END: thought

// ANCHOR: thought_waking
impl Wakeable for Thought {
    fn wake_by_ref(self: &Arc<Self>) {
        let clone = Arc::clone(self);

        self.sender.send(clone);
    }
}
// ANCHOR_END: thought_waking