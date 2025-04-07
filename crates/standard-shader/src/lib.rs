//! Standard shader implementation for the Rustica engine
//!
//! This crate provides a reusable standard shader implementation that supports
//! common features like vertex attributes, instancing, lighting, and animations.

use rustica_render::Vertex;
use rustica_render_derive::ShaderProperties;
use cgmath::{Matrix4, Vector3, Point3};
use bytemuck::{Pod, Zeroable};

// Define our standard shader using the ShaderProperties derive macro
#[derive(ShaderProperties)]
#[shader(file = "shaders/standard_shader.wgsl")]
pub struct StandardShader {
    // Vertex attributes
    #[vertex(location = 0)]
    pub position: [f32; 3],
    
    #[vertex(location = 1)]
    pub normal: [f32; 3],
    
    #[vertex(location = 2)]
    pub color: [f32; 3],
    
    #[vertex(location = 3)]
    pub uv: [f32; 2],

    // Instance attributes
    #[instance(location = 4)]
    pub model_matrix: [[f32; 4]; 4],
    
    #[instance(location = 8)]
    pub instance_color: [f32; 3],

    // Uniforms
    #[uniform(binding = 0)]
    pub view: [[f32; 4]; 4],
    
    #[uniform(binding = 1)]
    pub projection: [[f32; 4]; 4],
    
    #[uniform(binding = 2)]
    pub time: f32,
}

/// Create a prelude module to simplify imports
pub mod prelude {
    pub use crate::{
        StandardShader,
        StandardShaderVertex,
        StandardShaderInstances,        
    };
}