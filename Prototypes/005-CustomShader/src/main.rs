use rustica_render::{RenderWindow, ShaderProperties};
use rustica_foundation::geometry::Triangle as GeometryTriangle;

// Define our shader using the ShaderProperties derive macro
#[derive(ShaderProperties)]
#[shader(file = "./src/shaders/custom_shader.wgsl")]
struct PlasmaShader {
    // Vertex attributes
    #[allow(dead_code)]
    #[vertex(location = 0)]
    position: [f32; 3],
    
    #[allow(dead_code)]
    #[vertex(location = 1)]
    color: [f32; 3],
    
    #[allow(dead_code)]
    #[vertex(location = 2)]
    uv: [f32; 2],
    
    // Instance attributes
    #[allow(dead_code)]
    #[instance(location = 3)]
    model_matrix: [[f32; 4]; 4],
    
    #[allow(dead_code)]
    #[instance(location = 7)]
    instance_color: [f32; 3],
    
    // Uniform for time animation
    #[allow(dead_code)]
    #[uniform(binding = 0)]
    time: f32,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a shader descriptor
    let shader_descriptor = PlasmaShader::descriptor();
    
    // Create a render window and register the shader
    let mut window = RenderWindow::new("Full-Screen Plasma Effect", 800, 600);
    let shader_id = window.register_shader(shader_descriptor);
    
    window.with_frame_callback(move |canvas| {
        let time_value = canvas.time().as_secs_f32();
        
        // Define vertices for a full-screen quad with proper UV coordinates
        let vertices = [
            PlasmaShaderVertex {
                position: [-1.0, 1.0, 0.0],    // Top-left
                color: [1.0, 1.0, 1.0],        // White
                uv: [0.0, 0.0],                // Top-left UV
            },
            PlasmaShaderVertex {
                position: [-1.0, -1.0, 0.0],   // Bottom-left
                color: [1.0, 1.0, 1.0],        // White
                uv: [0.0, 1.0],                // Bottom-left UV
            },
            PlasmaShaderVertex {
                position: [1.0, -1.0, 0.0],    // Bottom-right
                color: [1.0, 1.0, 1.0],        // White
                uv: [1.0, 1.0],                // Bottom-right UV
            },
        ];
        
        // Create first triangle for the quad
        let triangle1 = GeometryTriangle {
            vertices: [vertices[0], vertices[1], vertices[2]],
        };
        
        // Create second triangle to complete the quad
        let triangle2 = GeometryTriangle {
            vertices: [
                vertices[0],                   // Top-left
                vertices[2],                   // Bottom-right
                PlasmaShaderVertex {
                    position: [1.0, 1.0, 0.0], // Top-right
                    color: [1.0, 1.0, 1.0],    // White
                    uv: [1.0, 0.0],            // Top-right UV
                },
            ],
        };

        // Identity matrix for the instance (no transformation)
        let identity = [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ];

        // Create a single instance with white color
        let instance = PlasmaShaderInstances {
            model_matrix: identity,
            instance_color: [1.0, 1.0, 1.0],  // White (allows plasma colors to shine through)
        };

        // Draw the triangles using instanced rendering
        canvas.draw_with_instances(shader_id)
              .uniform("time", time_value)
              .colored_instanced_triangles(&[triangle1, triangle2], &[instance]);
    })
    .run()?;

    Ok(())
}
