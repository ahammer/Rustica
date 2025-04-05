// Mesh primitives module - Contains various mesh primitives

pub mod cube;
pub mod teapot;
pub mod factory;
pub mod plane;
pub mod sphere;

// Re-export common types and functions
pub use cube::*;
pub use teapot::*;
pub use plane::*;
pub use sphere::*;
