use rustica_render::{RenderWindow, Vertex};
use rustica_render_derive::ShaderProperties;
use rustica_foundation::{geometry::{GeometryBuilder, Triangle as GeometryTriangle}, VertexSemantic};

// Define our shader using the ShaderProperties derive macro
#[derive(ShaderProperties)]
#[shader(file = "./src/shaders/basic_triangle.wgsl")]
struct BasicShader {
    // Vertex attributes
    #[vertex(location = 0, semantic = VertexSemantic::Position)]
    position: [f32; 3],
    
    #[vertex(location = 1, semantic = VertexSemantic::Color)]
    color: [f32; 3],
    
    // Instance attributes
    #[instance(location = 3)]
    model_matrix: [[f32; 4]; 4],
    
    #[instance(location = 7)]
    instance_color: [f32; 3],
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a shader descriptor
    let shader_descriptor = BasicShader::descriptor();
    
    // Create a render window with a frame callback
    let mut window = RenderWindow::new("Basic Triangle (Instanced)", 800, 600);
    
    // Register the shader with the render window
    let shader_id = window.register_shader(shader_descriptor);
    
    window.with_frame_callback(move |canvas| {
        // Get the vertex factory
        let vertex_factory = BasicShader::vertex_factory();
        
        // Define the triangle vertices using the vertex factory
        let vertices = [
            vertex_factory.create_vertex(
                [0.0, 0.5, 0.0],     // Position: Top
                [1.0, 0.0, 0.0]      // Color: Red
            ),
            vertex_factory.create_vertex(
                [-0.5, -0.5, 0.0],   // Position: Bottom left
                [0.0, 1.0, 0.0]      // Color: Green
            ),
            vertex_factory.create_vertex(
                [0.5, -0.5, 0.0],    // Position: Bottom right
                [0.0, 0.0, 1.0]      // Color: Blue
            ),
        ];
                
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
        instances.push(BasicShaderInstances {
            model_matrix: identity,
            instance_color: [1.0, 1.0, 1.0], // White tint (preserves original vertex colors)
        });
        
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
            instances.push(BasicShaderInstances {
                model_matrix: model,
                instance_color: [0.7, 0.7, 0.7], // Slightly dimmer
            });
        }
        
        // Draw all triangles using instanced rendering
        let mut builder = BasicShader::geometry_builder();
        builder.triangle_strip(&vertices);
        let geometry = builder.build();
        canvas.draw_with_instances(shader_id)
              .pump_geometry(&geometry, &instances);
    })
    .run()?;
    
    Ok(())
}
