// Canvas for drawing operations


use std::time::Duration;
use std::collections::HashMap;
use glam::Mat4;
use crate::draw_commands::{DrawCommand, UniformValue};
use crate::custom_shader::CustomShader;
use rustica_foundation::prelude::*;
use wgpu::Queue;

/// Canvas for drawing operations
pub struct Canvas<'a> {
    pub(crate) commands: Vec<DrawCommand>,
    time: Duration,
    render_context: Option<&'a mut crate::render_context::RenderContext>,
    pub(crate) queue: Option<&'a Queue>,
}

impl<'a> Canvas<'a> {
    pub(crate) fn new(time: Duration) -> Self {
        Self {
            commands: Vec::new(),
            time,
            render_context: None,
            queue: None,
        }
    }    
    
    /// Get a mutable reference to a custom shader by ID
    pub fn get_shader_mut(&mut self, id: usize) -> Option<&mut CustomShader> {
        if let Some(context) = &mut self.render_context {
            context.get_shader_mut(id)
        } else {
            None
        }
    }
    
    /// Get the queue for submitting GPU commands
    pub fn queue(&self) -> Option<&Queue> {
        self.queue
    }
    
    /// Set the queue for submitting GPU commands
    pub fn set_queue(&mut self, queue: &'a Queue) {
        self.queue = Some(queue);
    }    
    
    /// Create a shader draw builder for instanced drawing
    pub fn draw_with_instances(&mut self, shader_id: usize) -> InstancedShaderDrawBuilder<'_, 'a> {
        InstancedShaderDrawBuilder::new(self, shader_id)
    }
    
    /// Get the elapsed time since the application started
    pub fn time(&self) -> Duration {
        self.time
    }
}


/// Builder for instanced shader draw operations
pub struct InstancedShaderDrawBuilder<'b, 'a> {
    canvas: &'b mut Canvas<'a>,
    shader_id: usize,
    uniforms: HashMap<String, UniformValue>,
}

impl<'b, 'a> InstancedShaderDrawBuilder<'b, 'a> {
    /// Create a new instanced shader draw builder
    fn new(canvas: &'b mut Canvas<'a>, shader_id: usize) -> Self {
        Self {
            canvas,
            shader_id,
            uniforms: HashMap::new(),
        }
    }
    
    /// Add a uniform value
    pub fn uniform<S: Into<String>, V: Into<UniformValue>>(mut self, name: S, value: V) -> Self {
        self.uniforms.insert(name.into(), value.into());
        self
    }
    
    /// Draw geometry with instances using the configured shader and uniforms
    pub fn pump_geometry<V: Vertex, I: bytemuck::Pod>(
        self,
        geometry: &Geometry<V>,
        instances: &[I]
    ) {
        let vertices_bytes = bytemuck::cast_slice(&geometry.vertices).to_vec();
        let indices_bytes = bytemuck::cast_slice(&geometry.indices).to_vec();
        let instances_bytes = bytemuck::cast_slice(instances).to_vec();
        
        self.canvas.commands.push(DrawCommand::GeometryWithInstances {
            shader_id: self.shader_id,
            vertices: vertices_bytes,
            indices: indices_bytes,
            instances: instances_bytes,
            vertex_count: geometry.vertices.len() as u32,
            index_count: geometry.indices.len() as u32,
            instance_count: instances.len() as u32,
            uniforms: self.uniforms,
        });
    }
}

// Implement From traits for UniformValue to make the API more ergonomic
impl From<f32> for UniformValue {
    fn from(value: f32) -> Self {
        UniformValue::Float(value)
    }
}

impl From<[f32; 2]> for UniformValue {
    fn from(value: [f32; 2]) -> Self {
        UniformValue::Vec2(value)
    }
}

impl From<[f32; 3]> for UniformValue {
    fn from(value: [f32; 3]) -> Self {
        UniformValue::Vec3(value)
    }
}

impl From<[f32; 4]> for UniformValue {
    fn from(value: [f32; 4]) -> Self {
        UniformValue::Vec4(value)
    }
}

impl From<i32> for UniformValue {
    fn from(value: i32) -> Self {
        UniformValue::Int(value)
    }
}

impl From<u32> for UniformValue {
    fn from(value: u32) -> Self {
        UniformValue::UInt(value)
    }
}

impl From<Mat4> for UniformValue {
    fn from(matrix: Mat4) -> Self {
        // Convert Mat4 to array format for the shader
        let array = matrix.to_cols_array_2d();
        UniformValue::Mat4(array)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;
    use rustica_foundation::geometry::Triangle;

    #[test]
    fn test_canvas_new() {
        let time = Duration::from_secs(5);
        let canvas = Canvas::new(time);
        assert_eq!(canvas.time(), time);
        assert!(canvas.commands.is_empty());
    }

}
