// Core geometry traits for Rustica engine

use bytemuck::{Pod, Zeroable};
use wgpu::{VertexBufferLayout, VertexFormat};

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

/// Implement VertexAttributeProvider for any type that implements Vertex
impl<T: Vertex> VertexAttributeProvider for T {
    fn attributes() -> Vec<VertexAttribute> {
        let layout = T::layout();
        let mut attributes = Vec::new();
        
        for attr in layout.attributes {
            attributes.push(VertexAttribute {
                name: format!("attribute_{}", attr.shader_location), // Default name
                location: attr.shader_location,
                format: attr.format,
                offset: attr.offset,
            });
        }
        
        attributes
    }
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
