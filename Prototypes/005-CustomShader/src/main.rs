use rustica_render::{RenderWindow, ShaderProperties};

// Define our shader using the ShaderProperties derive macro
#[derive(ShaderProperties)]
#[shader(file = "./src/shaders/custom_shader.wgsl")]
struct PlasmaShader {
    // Vertex attributes
    #[vertex(location = 0)]
    position: [f32; 3],
    
    #[vertex(location = 1)]
    color: [f32; 3],
    
    #[vertex(location = 2)]
    uv: [f32; 2],
    
    // Instance attributes
    #[instance(location = 3)]
    model_matrix: [[f32; 4]; 4],
    
    #[instance(location = 7)]
    instance_color: [f32; 3],
    
    // Uniform for time animation    
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
        
        // Create a full-screen quad using triangle strip (more efficient)
        let mut builder = PlasmaShader::geometry_builder();
        
        // Add vertices for a triangle strip (quad)
        builder.triangle_strip(&[
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
                position: [1.0, 1.0, 0.0],     // Top-right
                color: [1.0, 1.0, 1.0],        // White
                uv: [1.0, 0.0],                // Top-right UV
            },
            PlasmaShaderVertex {
                position: [1.0, -1.0, 0.0],    // Bottom-right
                color: [1.0, 1.0, 1.0],        // White
                uv: [1.0, 1.0],                // Bottom-right UV
            }
        ]);
        
        // Build the final geometry
        let geometry = builder.build();

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

        // Draw using our new pump_geometry method
        canvas.draw_with_instances(shader_id)
              .uniform("time", time_value)
              .pump_geometry(&geometry, &[instance]);
    })
    .run()?;

    Ok(())
}
