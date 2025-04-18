// Rustica ECS - Entity Component System implementation for the Rustica engine
//
// This crate provides a simple and efficient Entity Component System (ECS)
// that can be used to organize game logic in a data-oriented way.

// Export modules
pub mod component;
pub mod entity;
pub mod system;
pub mod world;
pub mod prelude;

// Re-export prelude for convenience
pub use prelude::*;
