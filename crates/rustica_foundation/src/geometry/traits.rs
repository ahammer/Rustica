// Core geometry traits for Rustica engine

use bytemuck::{Pod, Zeroable};
use wgpu::{VertexBufferLayout, VertexFormat};


// Semantic types for vertex attributes
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum VertexSemantic {
    Position,
    Normal,
    Color,
    TexCoord,
    Tangent,
    Bitangent,
    // Add other semantics as needed
}

/// Trait for vertex data that can be used with shaders
pub trait Vertex: Pod + Zeroable {
    /// Get the vertex buffer layout for this vertex type
    fn layout() -> VertexBufferLayout<'static>;
}

/// Trait for types that can provide vertex attributes
pub trait VertexAttributeProvider {
    /// Get the vertex attributes for this type
    fn attributes() -> Vec<VertexAttribute>;
}

/// Vertex attribute descriptor
pub struct VertexAttribute {
    /// Name of the attribute
    pub name: String,
    /// Shader location
    pub location: u32,
    /// Format of the attribute
    pub format: VertexFormat,
    /// Offset in the vertex buffer
    pub offset: u64,
    /// Semantic meaning of the attribute (optional)
    pub semantic: Option<VertexSemantic>,
}

/// A triangle with generic vertex data
#[derive(Clone)]
pub struct Triangle<V: Vertex> {
    /// Vertices of the triangle
    pub vertices: [V; 3],
}

impl<V: Vertex> Triangle<V> {
    /// Create a new triangle with the given vertices
    pub fn new(vertices: [V; 3]) -> Self {
        Self { vertices }
    }
}
