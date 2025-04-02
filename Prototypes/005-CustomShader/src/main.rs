use cgmath::{Matrix4, Point3, Vector3, Rad};
use rustica_render::{RenderWindow, Triangle, Vertex, ShaderDescriptor};
use rustica_foundation::geometry::Triangle as GeometryTriangle;
use std::f32::consts::PI;

// Define a custom vertex type with the Vertex trait - now with UV coordinates
#[repr(C)]
#[derive(Copy, Clone, Debug, Vertex)]
struct CustomVertex {
    position: [f32; 3], // location = 0
    color: [f32; 3],    // location = 1
    uv: [f32; 2],       // location = 2 - added to match shader expectations
}

// Define an instance struct for instanced rendering
#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct TriangleInstance {
    model_matrix: [[f32; 4]; 4], // locations for model matrix rows
    color: [f32; 3],             // location for color
    _padding: u32,               // For memory alignment
}

impl TriangleInstance {
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
    // Create a shader descriptor
    let shader_descriptor = CustomShaderDescriptor::descriptor();
    
    // Create a render window and register the shader
    let mut window = RenderWindow::new("Two Full-Screen Triangles", 800, 600);
    let shader_id = window.register_shader(shader_descriptor);

    window.with_frame_callback(move |canvas| {
        let time_value = canvas.time().as_secs_f32();
        
        // Define vertices with proper UV coordinates
        let vertices = [
            CustomVertex {
                position: [-1.0, 1.0, 0.0],   // Top-left
                color: [1.0, 0.0, 0.0],       // Red
                uv: [0.0, 0.0],               // Top-left UV
            },
            CustomVertex {
                position: [-1.0, -1.0, 0.0],  // Bottom-left
                color: [0.0, 1.0, 0.0],       // Green
                uv: [0.0, 1.0],               // Bottom-left UV
            },
            CustomVertex {
                position: [1.0, -1.0, 0.0],   // Bottom-right
                color: [0.0, 0.0, 1.0],       // Blue
                uv: [1.0, 1.0],               // Bottom-right UV
            },
            CustomVertex {
                position: [1.0, 1.0, 0.0],    // Top-right
                color: [1.0, 1.0, 0.0],       // Yellow
                uv: [1.0, 0.0],               // Top-right UV
            },
        ];

        let triangle1 = GeometryTriangle {
            vertices: [vertices[0], vertices[1], vertices[2]],
        };

        let triangle2 = GeometryTriangle {
            vertices: [vertices[0], vertices[2], vertices[3]],
        };

        // Identity matrix for the instance (no transformation)
        let identity = [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ];

        // Create a single instance with white color
        let instance = TriangleInstance::new(
            identity,
            [1.0, 1.0, 1.0] // White color
        );

        // Draw the triangles using instanced rendering
        canvas.draw_with_instances(shader_id)
              .uniform("time", time_value)
              .colored_instanced_triangles(&[triangle1, triangle2], &[instance]);
    })
    .run()?;

    Ok(())
}
