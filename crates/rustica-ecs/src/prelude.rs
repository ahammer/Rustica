// Prelude module - Re-exports commonly used types from the ECS crate

// Core ECS types
pub use crate::component::Component;
pub use crate::entity::{Entity, EntityBuilder};
pub use crate::system::System;
pub use crate::world::World;

// Re-export common external types used throughout the codebase
pub use rustica_foundation::prelude::*;
