// Drawing commands for the rendering system

use std::collections::HashMap;

/// Uniform value that can be passed to a shader
#[derive(Debug, Clone)]
pub enum UniformValue {
    /// Float value
    Float(f32),
    /// Vec2 value
    Vec2([f32; 2]),
    /// Vec3 value
    Vec3([f32; 3]),
    /// Vec4 value
    Vec4([f32; 4]),
    /// Mat4 value
    Mat4([[f32; 4]; 4]),
    /// Integer value
    Int(i32),
    /// Unsigned integer value
    UInt(u32),
}

/// A draw command for the rendering system
#[derive(Debug)]
pub enum DrawCommand {
    /// Draw triangles with a custom shader (deprecated)
    #[allow(deprecated)]
    CustomTriangles {
        shader_id: usize,  // Reference to the shader in the registry
        vertices: Vec<u8>, // Raw vertex data
        vertex_count: u32,
        uniforms: HashMap<String, UniformValue>, // Uniform values to set before drawing
    },
    /// Draw instanced triangles with a custom shader (deprecated)
    #[allow(deprecated)]
    CustomInstancedTriangles {
        shader_id: usize,          // Reference to the shader in the registry
        vertices: Vec<u8>,         // Raw vertex data
        instances: Vec<u8>,        // Raw instance data (array of model matrices)
        vertex_count: u32,
        instance_count: u32,
        uniforms: HashMap<String, UniformValue>, // Global uniforms
    },
    /// Draw geometry with instances (preferred approach)
    GeometryWithInstances {
        shader_id: usize,          // Reference to the shader in the registry
        vertices: Vec<u8>,         // Raw vertex data
        indices: Vec<u8>,          // Index data for indexed rendering
        instances: Vec<u8>,        // Raw instance data
        vertex_count: u32,
        index_count: u32,
        instance_count: u32,
        uniforms: HashMap<String, UniformValue>, // Global uniforms
    },
}
