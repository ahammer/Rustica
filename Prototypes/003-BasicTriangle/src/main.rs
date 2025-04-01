use cgmath::{Matrix4, Point3, Vector3};
use rustica_render::{RenderWindow, Triangle, Vertex, ShaderDescriptor};
use rustica_foundation::geometry::Triangle as GeometryTriangle;

// Define a custom vertex type with the Vertex trait
#[repr(C)]
#[derive(Copy, Clone, Debug, Vertex)]
struct BasicVertex {
    position: [f32; 3], // location = 0
    color: [f32; 3],    // location = 1
}

// Define an instance struct for instanced rendering
#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct TriangleInstance {
    model_matrix: [[f32; 4]; 4], // locations 3,4,5,6 (4 rows)
    color: [f32; 3],             // location 7
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
#[shader(source = "./src/shaders/basic_triangle.wgsl")]
struct BasicShaderDescriptor {
    #[vertex_type]
    vertex: BasicVertex,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a shader descriptor
    let shader_descriptor = BasicShaderDescriptor::descriptor();
    
    // Create a render window with a frame callback
    let mut window = RenderWindow::new("Basic Triangle (Instanced)", 800, 600);
    
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
        
        // Create instance data for multiple triangles
        let mut instances = Vec::new();
        
        // Center triangle (identity matrix)
        let identity = [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ];
        
        // Add center triangle with full color
        instances.push(TriangleInstance::new(
            identity,
            [1.0, 1.0, 1.0] // White tint (preserves original vertex colors)
        ));
        
        // Add smaller triangles arranged in a pattern
        for i in 0..3 {
            // Calculate angle based on position (120 degrees apart)
            let angle = i as f32 * std::f32::consts::PI * 2.0 / 3.0;
            
            // Position offset
            let offset_x = angle.cos() * 0.7;
            let offset_y = angle.sin() * 0.7;
            
            // Scale and translate matrix
            let scale = 0.5; // Half size
            let model = [
                [scale, 0.0, 0.0, 0.0],
                [0.0, scale, 0.0, 0.0],
                [0.0, 0.0, scale, 0.0],
                [offset_x, offset_y, 0.0, 1.0],
            ];
            
            // Create instance with position offset and slightly dimmer color
            instances.push(TriangleInstance::new(
                model,
                [0.7, 0.7, 0.7] // Slightly dimmer
            ));
        }
        
        // Draw all triangles using instanced rendering
        canvas.draw_with_instances(shader_id)
              .colored_instanced_triangles(&[triangle], &instances);
    })
    .run()?;
    
    Ok(())
}
