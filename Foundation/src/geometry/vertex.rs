// Standard vertex definition for Rustica engine

use cgmath::{Vector2, Vector3};
use bytemuck::{Pod, Zeroable};
// No need to import VertexBufferLayout since it's used directly in the function
use super::traits::Vertex;

/// Standard vertex format with position, normal, color, and texture coordinates
#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct StandardVertex {
    /// Position of the vertex in 3D space
    pub position: [f32; 3],
    /// Normal vector of the vertex
    pub normal: [f32; 3],
    /// Vertex color
    pub color: [f32; 3],
    /// Texture coordinates
    pub tex_coords: [f32; 2],
}

impl StandardVertex {
    /// Create a new standard vertex
    pub fn new(position: Vector3<f32>, normal: Vector3<f32>, color: Vector3<f32>, tex_coords: Vector2<f32>) -> Self {
        Self {
            position: [position.x, position.y, position.z],
            normal: [normal.x, normal.y, normal.z],
            color: [color.x, color.y, color.z],
            tex_coords: [tex_coords.x, tex_coords.y],
        }
    }

    /// Map this standard vertex to a custom vertex type using a mapping function
    pub fn map<V, F>(&self, mapper: F) -> V
    where
        F: FnOnce(&StandardVertex) -> V,
    {
        mapper(self)
    }
}

impl Vertex for StandardVertex {
    /// Get the vertex buffer layout for this vertex type
    fn layout() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<StandardVertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                // Position
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                // Normal
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x3,
                },
                // Color
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 6]>() as wgpu::BufferAddress,
                    shader_location: 2,
                    format: wgpu::VertexFormat::Float32x3,
                },
                // Texture coordinates
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 9]>() as wgpu::BufferAddress,
                    shader_location: 3,
                    format: wgpu::VertexFormat::Float32x2,
                },
            ],
        }
    }

}
