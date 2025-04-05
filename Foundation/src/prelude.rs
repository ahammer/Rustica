// Prelude module - Re-exports commonly used types from the Foundation crate

// Re-export geometry types
pub use crate::geometry::{    
    // Traits
    Vertex,
    VertexAttributeProvider,
    VertexAttribute,
    Triangle,    
    // Geometry
    geometry_builder::Geometry,
};

// Re-export common external types used throughout the codebase
pub use cgmath::{Vector2, Vector3, Point3, Matrix4, InnerSpace, EuclideanSpace};
pub use bytemuck::{Pod, Zeroable};
pub use wgpu;
