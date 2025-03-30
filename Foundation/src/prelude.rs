// Prelude module - Re-exports commonly used types from the Foundation crate

// Re-export geometry types
pub use crate::geometry::{
    // Core types
    StandardVertex,
    Face,
    Mesh,
    
    // Traits
    Vertex,
    VertexAttributeProvider,
    VertexAttribute,
    Triangle,
    
    // Bezier types
    BezierCurve,
    BezierPatch,
    CubicBezier2D,
    CubicBezier3D,
};

// Re-export common external types used throughout the codebase
pub use cgmath::{Vector2, Vector3, Point3, Matrix4, InnerSpace, EuclideanSpace};
pub use bytemuck::{Pod, Zeroable};
pub use wgpu;
