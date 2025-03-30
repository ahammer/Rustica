// Prelude module - Re-exports commonly used types from the Graphics crate

// Re-export core types
pub use crate::primitives::camera::{Camera, CameraMatrices};
pub use crate::primitives::shapes::ShapeFactory;

// Re-export mesh creation functions
pub use crate::primitives::shapes::cube::create_cube;
pub use crate::primitives::shapes::plane::create_plane;
pub use crate::primitives::shapes::teapot::{create_teapot, create_default_teapot};
pub use crate::primitives::shapes::sphere::{create_uv_sphere, create_solid_sphere, create_default_sphere};

// Re-export Foundation types that are commonly used with Graphics
pub use rustica_foundation::prelude::*;
