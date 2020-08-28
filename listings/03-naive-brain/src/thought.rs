//! # A Thought
//!
//! A Thought is an entity that wraps a `Future` the *Brain* could *think* on.
//!

extern crate alloc;

use core::{
    future::Future,
    pin::Pin,
};

use alloc::boxed::Box;

// ANCHOR: thought
pub struct Thought {
    /// This is the actual thing the brain should process as part of the Thought
    pub thinkable: Pin<Box<dyn Future<Output = ()> + 'static>>,
}
// ANCHOR_END: thought

