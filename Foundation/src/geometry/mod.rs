// Geometry module - Contains basic geometric primitives for Rustica engine

pub mod vertex;
pub mod mesh;
pub mod face;
pub mod traits;
pub mod bezier;
pub mod geometry_builder;

// Re-export common types
pub use vertex::StandardVertex;
pub use mesh::Mesh;
pub use face::Face;
pub use traits::{Vertex, VertexAttributeProvider, VertexAttribute, Triangle};
pub use bezier::{BezierCurve, BezierPatch, CubicBezier2D, CubicBezier3D};
pub use geometry_builder::{Geometry, GeometryBuilder, PrimitiveType};
