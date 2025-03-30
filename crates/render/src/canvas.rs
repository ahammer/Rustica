// Canvas for drawing operations

use std::time::Duration;
use std::collections::HashMap;
use cgmath::{Point3, Vector3, Matrix4};
use crate::shader_types::ShaderType;
use crate::draw_commands::{DrawCommand, UniformValue};
use crate::custom_shader::CustomShader;
use rustica_foundation::geometry::{Triangle, Vertex};
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
    
    /// Draw a triangle with the given points and shader
    pub fn draw_triangle(&mut self, points: [Point3<f32>; 3], colors: [Vector3<f32>; 3], shader: ShaderType) {
        self.commands.push(DrawCommand::Triangle {
            points,
            colors,
            shader,
        });
    }
    
    /// Draw triangles using a custom shader
    pub fn draw_triangles<V: Vertex>(&mut self, triangles: &[Triangle<V>], shader_id: usize) {
        self.draw_triangles_with_uniforms(triangles, shader_id, HashMap::new());
    }
    
    /// Draw triangles using a custom shader with uniform values
    pub fn draw_triangles_with_uniforms<V: Vertex>(
        &mut self, 
        triangles: &[Triangle<V>], 
        shader_id: usize,
        uniforms: HashMap<String, UniformValue>
    ) {
        // Flatten the triangles into a single vertex buffer
        let vertices: Vec<V> = triangles.iter()
            .flat_map(|t| t.vertices)
            .collect();
        
        // Convert to raw bytes
        let vertices_bytes = bytemuck::cast_slice(&vertices).to_vec();
        
        self.commands.push(DrawCommand::CustomTriangles {
            shader_id,
            vertices: vertices_bytes,
            vertex_count: (triangles.len() * 3) as u32,
            uniforms,
        });
    }
    
    /// Create a shader draw builder for more fluent API
    pub fn draw_with_shader(&mut self, shader_id: usize) -> ShaderDrawBuilder<'_, 'a> {
        ShaderDrawBuilder::new(self, shader_id)
    }
    
    /// Get the elapsed time since the application started
    pub fn time(&self) -> Duration {
        self.time
    }
}

/// Builder for shader draw operations
pub struct ShaderDrawBuilder<'b, 'a> {
    canvas: &'b mut Canvas<'a>,
    shader_id: usize,
    uniforms: HashMap<String, UniformValue>,
}

impl<'b, 'a> ShaderDrawBuilder<'b, 'a> {
    /// Create a new shader draw builder
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
    
    /// Draw triangles with the configured shader and uniforms
    pub fn triangles<V: Vertex>(self, triangles: &[Triangle<V>]) {
        self.canvas.draw_triangles_with_uniforms(triangles, self.shader_id, self.uniforms);
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

impl From<Matrix4<f32>> for UniformValue {
    fn from(matrix: Matrix4<f32>) -> Self {
        // Convert Matrix4 to array format for the shader
        let array = [
            [matrix.x.x, matrix.x.y, matrix.x.z, matrix.x.w],
            [matrix.y.x, matrix.y.y, matrix.y.z, matrix.y.w],
            [matrix.z.x, matrix.z.y, matrix.z.z, matrix.z.w],
            [matrix.w.x, matrix.w.y, matrix.w.z, matrix.w.w],
        ];
        UniformValue::Mat4(array)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn test_canvas_new() {
        let time = Duration::from_secs(5);
        let canvas = Canvas::new(time);
        assert_eq!(canvas.time(), time);
        assert!(canvas.commands.is_empty());
    }

    #[test]
    fn test_canvas_draw_triangle() {
        let time = Duration::from_secs(0);
        let mut canvas = Canvas::new(time);
        
        let points = [
            Point3::new(0.0, 0.5, 0.0),
            Point3::new(-0.5, -0.5, 0.0),
            Point3::new(0.5, -0.5, 0.0),
        ];
        
        let colors = [
            Vector3::new(1.0, 0.0, 0.0),
            Vector3::new(0.0, 1.0, 0.0),
            Vector3::new(0.0, 0.0, 1.0),
        ];
        
        canvas.draw_triangle(points, colors, ShaderType::DebugColor);
        
        assert_eq!(canvas.commands.len(), 1);
        
        if let DrawCommand::Triangle { points: p, colors: c, shader } = &canvas.commands[0] {
            assert_eq!(p, &points);
            assert_eq!(c, &colors);
            assert_eq!(shader, &ShaderType::DebugColor);
        } else {
            panic!("Expected Triangle draw command");
        }
    }
}
