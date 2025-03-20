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
//! ## Example
//!
//! ```rust
//! use rustica_scheduler::{Schedule, System, Stage};
//! use rustica_ecs::World;
//!
//! // A simple system that updates positions based on velocities
//! fn update_positions(world: &mut World) {
//!     // System implementation
//! }
//!
//! // Creating a schedule and adding systems
//! let mut schedule = Schedule::default();
//! schedule.add_system(update_positions, "update_positions", Stage::Update);
//! 
//! // Running the schedule with a world
//! let mut world = World::new();
//! schedule.run(&mut world);
//! ```

pub mod error;
pub mod system;
mod schedule;
mod stage;
mod plugin;

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
pub use plugin::SchedulerPlugin;
