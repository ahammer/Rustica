//! Rustica Event Handling System
//!
//! This crate provides functionality for handling and processing events
//! in the Rustica game engine. The event system allows for decoupled
//! communication between different parts of the engine and game code.

mod error;
mod event;
mod event_system;

#[cfg(test)]
mod event_tests;

pub use error::{Error, Result};
pub use event::{Event, EventReader, EventWriter, Events};
pub use event_system::EventSystem;

/// Re-exports of commonly used types
pub mod prelude {
    pub use crate::{Error, Result, Event, EventReader, EventWriter, Events, EventSystem};
}
