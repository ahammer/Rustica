// Prelude module - Re-exports commonly used types from the Conway crate

// Re-export components
pub use crate::components::{Position, CellState};

// Re-export systems
pub use crate::systems::{LifeSystem, TextRenderSystem};

// Re-export utility functions
pub use crate::setup_conway_grid;

// Re-export patterns
pub use crate::patterns;

// Re-export ECS types
pub use rustica_ecs::prelude::*;
