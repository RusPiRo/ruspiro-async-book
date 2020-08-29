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

use crate::brain::wakeable::Wakeable;
use ruspiro_channel::mpmc;
use ruspiro_lock::DataLock;

// ANCHOR: thought
pub struct Thought {
    /// This is the actual thing the brain should process as part of the Thought
    pub thinkable: DataLock<Pin<Box<dyn Future<Output = ()> + 'static>>>,

    pub sender: mpmc::Sender<Arc<Thought>>,
}
// ANCHOR_END: thought


impl Wakeable for Thought {
    fn wake_by_ref(self: &Arc<Self>) {
        let clone = Arc::clone(self);

        self.sender.send(clone);
    }
}