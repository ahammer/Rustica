// Prelude module - Re-exports commonly used types from the Render crate

// Re-export core rendering types
pub use crate::{
    Canvas,
    RenderWindow,
    CustomShader,
    ShaderDescriptor,
    UniformParameter,
};

// Re-export geometry traits
pub use rustica_foundation::geometry::{
    Vertex,
    VertexAttributeProvider,
    VertexAttribute,
    Triangle,
    GeometryBuilder,
};

