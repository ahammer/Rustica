use rustica_render::RenderWindow;
use rustica_standard_shader::{StandardShader, StandardShaderInstances, StandardShaderVertexFactory};
use rustica_graphics::primitives::camera::Camera;
use cgmath::{Matrix4, Point3, SquareMatrix, Vector3};
use std::time::Instant;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a shader descriptor
    let shader_descriptor = StandardShader::descriptor();
    
    // Create a render window with a frame callback
    let mut window = RenderWindow::new("Basic Triangle (Instanced)", 800, 600);
    
    // Register the shader with the render window
    let shader_id = window.register_shader(shader_descriptor);
    
    // Create a camera for proper view and projection matrices
    let mut camera = Camera::perspective(800.0 / 600.0);
    
    // Position camera to view the scene (back from origin to see triangles)
    camera.look_at_from(
        Point3::new(0.0, 0.0, 3.0),  // Position camera at z=3
        Point3::new(0.0, 0.0, 0.0),  // Looking at origin
    );
    
    // Track time for animation effects
    let start_time = Instant::now();
    
    window.with_frame_callback(move |canvas| {
        // Get the vertex factory
        let vertex_factory = StandardShader::vertex_factory();
        
        // Define the triangle vertices using the vertex factory, with proper normals
        let vertices = [
            StandardShaderVertexFactory::create_vertex(
                [0.0, 0.5, 0.0],             // Position
                [0.0, 0.0, 1.0],             // Normal pointing out of screen
                [1.0, 0.0, 0.0],             // Red
                [0.5, 0.0],                  // UV coordinates
            ),
            StandardShaderVertexFactory::create_vertex(
                [-0.5, -0.5, 0.0],           // Position
                [0.0, 0.0, 1.0],             // Normal pointing out of screen
                [0.0, 1.0, 0.0],             // Green
                [0.0, 1.0],                  // UV coordinates
            ),
            StandardShaderVertexFactory::create_vertex(
                [0.5, -0.5, 0.0],            // Position
                [0.0, 0.0, 1.0],             // Normal pointing out of screen
                [0.0, 0.0, 1.0],             // Blue
                [1.0, 1.0],                  // UV coordinates
            ),
        ];
                
        // Create instance data for multiple triangles
        let mut instances = Vec::new();
        
        // Center triangle (identity matrix)
        let identity = Matrix4::identity().into();
        
        // Add center triangle with full color
        instances.push(StandardShaderInstances {
            model_matrix: identity,
            instance_color: [1.0, 1.0, 1.0], // White tint
        });
        
        // Add smaller triangles arranged in a pattern
        for i in 0..3 {
            // Calculate angle based on position (120 degrees apart)
            let angle = i as f32 * std::f32::consts::PI * 2.0 / 3.0;
            
            // Position offset
            let offset_x = angle.cos() * 0.7;
            let offset_y = angle.sin() * 0.7;
            
            // Create transformation matrix with cgmath
            let scale = 0.5; // Half size
            let model = Matrix4::from_scale(scale) * 
                        Matrix4::from_translation(Vector3::new(offset_x, offset_y, 0.0));
            
            // Create instance with position offset and slightly dimmer color
            instances.push(StandardShaderInstances {
                model_matrix: model.into(),
                instance_color: [0.7, 0.7, 0.7], // Slightly dimmer
            });
        }
        
        // Get camera matrices
        let matrices = camera.get_render_matrices();
        
        // Calculate current time for shader animation
        let time = start_time.elapsed().as_secs_f32();
        
        // Draw all triangles using instanced rendering
        let mut builder = StandardShader::geometry_builder();        
        builder.triangle_strip(&vertices);  // Using triangle_list for proper winding
        let geometry = builder.build();

        // Chain all uniform values in the draw call
        canvas.draw_with_instances(shader_id)
              .uniform("view", matrices.view)
              .uniform("projection", matrices.projection)
              .uniform("time", time)
              .pump_geometry(&geometry, &instances);
    })
    .run()?;
    
    Ok(())
}
