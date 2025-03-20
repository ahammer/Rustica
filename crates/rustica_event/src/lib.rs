//! Rustica Event Handling System
//!
//! This crate provides functionality for handling and processing events
//! in the Rustica game engine.

mod error;
mod plugin;
mod event;

#[cfg(test)]
mod event_tests;
#[cfg(test)]
mod plugin_tests;

pub use error::{Error, Result};
pub use plugin::EventPlugin;
pub use event::{Event, EventReader, EventWriter, Events};

/// Re-exports of commonly used types
pub mod prelude {
    pub use crate::{Error, Result, EventPlugin, Event, EventReader, EventWriter, Events};
}
