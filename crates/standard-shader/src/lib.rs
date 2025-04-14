//! Standard shader implementation for the Rustica engine
//!
//! This crate provides a reusable standard shader implementation that supports
//! common features like vertex attributes, instancing, lighting, and animations.

// Include the bindings generated by build.rs
// This brings structs like `standard_shader::VertexInput`, `standard_shader::InstanceInput`,
// `standard_shader::Uniforms`, and layout constants into scope.
// include!(concat!(env!("OUT_DIR"), "/standard_shader_bindings.rs"));

use rustica_render::{CustomShader, ShaderDescriptor, UniformParameter};
use rustica_foundation::geometry::{GeometryBuilder, VertexAttribute, VertexAttributeProvider}; // Assuming GeometryBuilder is here
use glam::{Mat4, Vec3}; // Keep glam for potential use in factory or examples
use wgpu::include_wgsl; // To include the WGSL source directly

// --- Public Aliases for Generated Types ---

/// Public alias for the generated vertex input struct.
pub type StandardShaderVertex = standard_shader::VertexInput;
/// Public alias for the generated instance input struct.
pub type StandardShaderInstances = standard_shader::InstanceInput;
/// Public alias for the generated uniform struct.
pub type StandardShaderUniforms = standard_shader::Uniforms;

// --- StandardShader Definition ---

/// Represents the Standard Shader pipeline.
/// This struct primarily acts as a handle to get the shader descriptor
/// and potentially helper methods.
#[derive(Debug, Clone, Default)]
pub struct StandardShader;

impl StandardShader {
    /// Returns the `ShaderDescriptor` for the Standard Shader.
    /// This descriptor tells the renderer how to configure the pipeline.
    pub fn descriptor() -> standard_shader::ShaderDescriptor;

    /// Provides a simple factory for creating `StandardShaderVertex` instances.
    pub fn vertex_factory() -> StandardShaderVertexFactory {
        StandardShaderVertexFactory
    }

    /// Provides a geometry builder compatible with `StandardShaderVertex`.
    /// Note: This assumes `GeometryBuilder` is generic or adaptable.
    /// If `GeometryBuilder` is tied to a specific vertex type, this might need adjustment
    /// or the `render` crate needs a way to handle different vertex types.
    pub fn geometry_builder() -> GeometryBuilder<StandardShaderVertex> {
         GeometryBuilder::<StandardShaderVertex>::new()
    }
}

// --- Vertex Factory ---

/// A helper struct to create `StandardShaderVertex` instances.
#[derive(Debug, Copy, Clone)]
pub struct StandardShaderVertexFactory;

impl StandardShaderVertexFactory {
    /// Creates a new vertex with the specified attributes.
    pub fn create_vertex(
        position: [f32; 3],
        normal: [f32; 3],
        color: [f32; 3],
        uv: [f32; 2],
    ) -> StandardShaderVertex {
        StandardShaderVertex {
            position,
            normal,
            color,
            uv,
        }
    }
}

/// Create a prelude module to simplify imports
pub mod prelude {
    pub use crate::{
        StandardShader,
        StandardShaderVertex,      // Use the alias
        StandardShaderInstances,   // Use the alias
        StandardShaderUniforms,    // Use the alias
        StandardShaderVertexFactory,
    };
}
