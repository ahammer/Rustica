use std::sync::Arc;
use cgmath::{Matrix4, Point3, Vector3, Deg};
use rustica_render::{
    RenderWindow, ShaderDescriptor, Vertex, StandardMeshAdapter
};

use rustica_foundation::prelude::*;
use rustica_graphics::prelude::*;

// Define a custom vertex type with derive macro
#[repr(C)]
#[derive(Copy, Clone, Debug, Vertex)]
struct CustomVertex {
    position: [f32; 3], // location = 0, format = Float32x3
    color: [f32; 3],    // location = 1, format = Float32x3
    uv: [f32; 2],       // location = 2, format = Float32x2
}

// Define an instance struct for instanced rendering
#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct PlaneInstance {
    model_matrix: [[f32; 4]; 4], // locations 3,4,5,6 (4 rows)
    color: [f32; 3],             // location 7
    _padding: u32,               // For memory alignment
}

impl PlaneInstance {
    pub fn new(model_matrix: [[f32; 4]; 4], color: [f32; 3]) -> Self {
        Self {
            model_matrix,
            color,
            _padding: 0,
        }
    }
}

// Define a shader descriptor using the derive macro
#[derive(ShaderDescriptor)]
#[shader(source = "./src/shaders/custom_shader.wgsl")]
struct CustomShaderDescriptor {
    #[vertex_type]
    vertex: CustomVertex,
    
    #[uniform(binding = 0)]
    time: f32,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a shader descriptor using the derive macro
    let shader_descriptor = CustomShaderDescriptor::descriptor();
    
    let mut window = RenderWindow::new("Custom Shader Demo (Instanced)", 800, 600);
    
    // Register the shader with the render context
    let shader_id = window.register_shader(shader_descriptor);
    
    // Create a plane mesh - smaller size for multiple instances
    let plane_mesh = Arc::new(create_plane(0.8, 0.8));
    
    // Create a mesh adapter with a vertex mapper function
    let mesh_adapter = StandardMeshAdapter::new(plane_mesh, |standard_vertex| {
        CustomVertex {
            position: standard_vertex.position,
            color: standard_vertex.color,
            uv: standard_vertex.tex_coords,
        }
    });
    
    // Set up the frame callback
    window.with_frame_callback(move |canvas| {                
        let time_value = canvas.time().as_secs_f32();
        
        // Get triangles from the mesh adapter
        let triangles = mesh_adapter.to_triangles();
        
        // Create instance data for multiple quads
        let mut instances = Vec::new();
        
        // Add a grid of instances
        let grid_size = 3;
        let spacing = 0.7;
        
        for i in 0..grid_size {
            for j in 0..grid_size {
                // Calculate position offset
                let offset_x = (i as f32 - (grid_size as f32 - 1.0) / 2.0) * spacing;
                let offset_y = (j as f32 - (grid_size as f32 - 1.0) / 2.0) * spacing;
                
                // Create unique time offset for varied animation
                let time_offset = (i as f32 * 0.5) + (j as f32 * 0.3);
                
                // Rotate around Z axis with time and position-dependent speed
                let angle = time_value * (0.5 + (i as f32 * 0.2) + (j as f32 * 0.1));
                let cos_angle = angle.cos();
                let sin_angle = angle.sin();
                
                // Scale factor with time-based oscillation
                let scale_factor = 0.3 + 0.15 * ((time_value + time_offset) * 1.5).sin();
                
                // Build transformation matrix: scale, rotate, translate
                let model = [
                    [cos_angle * scale_factor, -sin_angle * scale_factor, 0.0, 0.0],
                    [sin_angle * scale_factor, cos_angle * scale_factor, 0.0, 0.0],
                    [0.0, 0.0, scale_factor, 0.0],
                    [offset_x, offset_y, 0.0, 1.0],
                ];
                
                // Create color based on position and time
                let r = 0.5 + 0.5 * ((time_value + time_offset) * 0.7).sin();
                let g = 0.5 + 0.5 * ((time_value + time_offset + 2.0) * 0.7).sin();
                let b = 0.5 + 0.5 * ((time_value + time_offset + 4.0) * 0.7).sin();
                
                instances.push(PlaneInstance::new(model, [r, g, b]));
            }
        }
        
        // Draw all quads using instanced rendering
        canvas.draw_with_instances(shader_id)
              .uniform("time", time_value)
              .colored_instanced_triangles(&triangles, &instances);
    }).run()?;
    
    Ok(())
}
