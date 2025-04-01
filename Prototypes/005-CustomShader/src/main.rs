use std::sync::Arc;
use rustica_render::{
    RenderWindow, ShaderDescriptor, Vertex, StandardMeshAdapter
};

use rustica_foundation::prelude::*;
use rustica_graphics::prelude::*;

// Define a custom vertex type with derive macro
// The Vertex derive macro will automatically:
// 1. Assign shader locations (0, 1, 2) in order of field declaration
// 2. Detect the correct format based on field type ([f32; 3] -> Float32x3)
// 3. Implement Pod and Zeroable traits for GPU data transfer
#[repr(C)]
#[derive(Copy, Clone, Debug, Vertex)]
struct CustomVertex {
    position: [f32; 3], // location = 0, format = Float32x3
    color: [f32; 3],    // location = 1, format = Float32x3
    uv: [f32; 2],       // location = 2, format = Float32x2
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
    
    let mut window = RenderWindow::new("Custom Shader Demo", 800, 600);
    
    // Register the shader with the render context
    let shader_id = window.register_shader(shader_descriptor);
    
    // Create a plane mesh using Graphics factory function
    // Adjust to make it a full-screen quad (-1 to 1)
    let plane_mesh = Arc::new(create_plane(2.0, 2.0));
    
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
        
        // Use the new API to draw triangles with uniforms
        canvas.draw_with_shader(shader_id)
              .uniform("time", time_value)
              .triangles(&triangles);
    }).run()?;
    
    Ok(())
}
