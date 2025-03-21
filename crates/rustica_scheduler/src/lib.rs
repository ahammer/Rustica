//! # Rustica Scheduler
//! 
//! The scheduler module provides system execution scheduling for the Rustica game engine.
//! It allows for organizing and executing systems in a deterministic order, with dependency
//! handling and execution stages.
//!
//! ## Features
//!
//! - System registration and execution
//! - Dependency tracking between systems
//! - Execution stages for organizing system execution
//! - Integration with the ECS and core engine
//!


pub mod error;
pub mod system;
pub mod schedule;
pub mod stage;
pub mod plugin;

#[cfg(test)]
mod system_tests;
#[cfg(test)]
mod schedule_tests;
#[cfg(test)]
mod stage_tests;
#[cfg(test)]
mod plugin_tests;

pub use error::SchedulerError;
pub use system::System;
pub use schedule::Schedule;
pub use stage::Stage;
pub use plugin::{SchedulerPlugin, InsertResource};
