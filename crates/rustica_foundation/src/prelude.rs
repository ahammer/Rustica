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

pub use crate::geometry::traits::VertexSemantic;
// Re-export common external types used throughout the codebase
pub use glam::{Vec2, Vec3, Mat4};
pub use bytemuck::{Pod, Zeroable};
pub use wgpu;
