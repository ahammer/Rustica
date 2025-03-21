//! # Rustica ECS: Entity-Component-System Implementation
//! 
//! This crate provides the ECS (Entity-Component-System) functionality
//! for the Rustica game engine. It implements a data-oriented design pattern
//! that separates data (components) from behavior (systems).
//! 
//! ## AGENT DIRECTIVES
//! 
//! - DOC_ROOT: /docs/
//! - ARCH_DOC: /docs/architecture.md#ECSSubsystem
//! - API_RULES: /docs/api_conventions.md#ECSAPI
//! - TEST_RULES: /docs/testing_standards.md#ECSTesting
//! 
//! ## Critical Rules
//! 
//! 1. Components must be plain data without behavior
//! 2. Systems should operate on components through queries
//! 3. All component access should be type-safe
//! 4. Performance is critical - optimize for cache efficiency
//! 
//! ## Usage Example
//! 
//! ```rust
//! use rustica_ecs::prelude::*;
//! 
//! // Define components
//! #[derive(Debug)]
//! struct Position {
//!     x: f32,
//!     y: f32,
//!     z: f32,
//! }
//! 
//! #[derive(Debug)]
//! struct Velocity {
//!     x: f32,
//!     y: f32,
//!     z: f32,
//! }
//! 
//! // Create a world
//! let mut world = World::new();
//! 
//! // Spawn an entity with components
//! let entity = world.spawn()
//!     .insert(Position { x: 0.0, y: 0.0, z: 0.0 })
//!     .insert(Velocity { x: 1.0, y: 0.0, z: 0.0 })
//!     .id();
//! 
//! // Access components
//! let position = world.get::<Position>(entity).unwrap();
//! assert_eq!(position.x, 0.0);
//! ```

// === REGION: MODULE DEFINITIONS ===
pub mod entity;
pub mod component;
pub mod world;
pub mod query;
pub mod error;
pub mod time;

// === REGION: PUBLIC EXPORTS ===
pub use entity::{Entity, EntityBuilder};
pub use component::Component;
pub use world::World;
pub use query::{Query, QueryResult};
pub use error::EcsError;
pub use time::Time;

// Re-export rustica_common components
pub use rustica_common;

/// Prelude module containing the most commonly used types.
pub mod prelude {
    pub use crate::Entity;
    pub use crate::EntityBuilder;
    pub use crate::Component;
    pub use crate::World;
    pub use crate::Query;
    pub use crate::QueryResult;
    pub use crate::EcsError;
    pub use crate::Time;
    pub use rustica_common::prelude::*;
}

// === REGION: TESTS ===
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_world_creation() {
        let world = World::new();
        // Simply testing that the world can be created without errors
    }

    #[test]
    fn test_entity_creation() {
        let mut world = World::new();
        let entity = world.spawn().id();
        // Test passes if no panic occurs
    }
}
