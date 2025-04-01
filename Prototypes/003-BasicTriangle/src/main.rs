use cgmath::{Point3, Vector3};
use rustica_render::{RenderWindow, Triangle};
use rustica_render_derive::{ShaderDescriptor, Vertex};
use rustica_foundation::geometry::Triangle as GeometryTriangle;

// Define a custom vertex type with the Vertex trait
#[repr(C)]
#[derive(Copy, Clone, Debug, Vertex)]
struct BasicVertex {
    position: [f32; 3], // location = 0
    color: [f32; 3],    // location = 1
}

// Define a shader descriptor using the derive macro
#[derive(ShaderDescriptor)]
#[shader(source = "./src/shaders/basic_triangle.wgsl")]
struct BasicShaderDescriptor {
    #[vertex_type]
    vertex: BasicVertex,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a shader descriptor
    let shader_descriptor = BasicShaderDescriptor::descriptor();
    
    // Create a render window with a frame callback
    let mut window = RenderWindow::new("Basic Triangle", 800, 600);
    
    // Register the shader with the render window
    let shader_id = window.register_shader(shader_descriptor);
    
    window.with_frame_callback(move |canvas| {
        // Define the triangle vertices (static positions)
        let vertices = [
            BasicVertex {
                position: [0.0, 0.5, 0.0],    // Top
                color: [1.0, 0.0, 0.0],       // Red
            },
            BasicVertex {
                position: [-0.5, -0.5, 0.0],  // Bottom left
                color: [0.0, 1.0, 0.0],       // Green
            },
            BasicVertex {
                position: [0.5, -0.5, 0.0],   // Bottom right
                color: [0.0, 0.0, 1.0],       // Blue
            },
        ];
        
        // Create a triangle from vertices
        let triangle = GeometryTriangle { vertices };
        
        // Draw the triangle using the modern shader API
        canvas.draw_with_shader(shader_id)
              .triangles(&[triangle]);
    })
    .run()?;
    
    Ok(())
}
