// Geometry module - Contains basic geometric primitives for Rustica engine


pub mod traits;
pub mod geometry_builder;

// Re-export common types
pub use traits::{Vertex, VertexAttributeProvider, VertexAttribute, Triangle};
pub use geometry_builder::{Geometry, GeometryBuilder};
