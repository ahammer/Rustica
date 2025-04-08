use std::f32::consts::PI;
use rustica_render::RenderWindow;
use rustica_standard_shader::{StandardShader, StandardShaderInstances, StandardShaderVertexFactory};
use rustica_graphics::primitives::camera::Camera;
use glam::{Mat4, Vec3, Quat};
use std::time::Instant;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a shader descriptor
    let shader_descriptor = StandardShader::descriptor();
    
    // Create a render window and register the shader
    let mut window = RenderWindow::new("Spinning/Scaling Triangle (Instanced)", 800, 600);
    let shader_id = window.register_shader(shader_descriptor);
      // Create and position camera
    let mut camera = Camera::perspective(800.0 / 600.0);
    camera.look_at_from(
        Vec3::new(0.0, 0.0, 3.0),
        Vec3::new(0.0, 0.0, 0.0),
    );
    
    let start_time = Instant::now();
    
    window.with_frame_callback(move |canvas| {        let time = start_time.elapsed().as_secs_f32();
        
        // Define the triangle vertices
        let vertices = [
            StandardShaderVertexFactory::create_vertex(
                [0.0, 0.5, 0.0],     // Top
                [0.0, 0.0, 1.0],     // Normal
                [1.0, 0.0, 0.0],     // Red
                [0.5, 0.0],          // UV
            ),
            StandardShaderVertexFactory::create_vertex(
                [-0.5, -0.5, 0.0],   // Bottom left
                [0.0, 0.0, 1.0],     // Normal
                [0.0, 1.0, 0.0],     // Green
                [0.0, 1.0],          // UV
            ),
            StandardShaderVertexFactory::create_vertex(
                [0.5, -0.5, 0.0],    // Bottom right
                [0.0, 0.0, 1.0],     // Normal
                [0.0, 0.0, 1.0],     // Blue
                [1.0, 1.0],          // UV
            ),
        ];
        
        // Create instance data for multiple triangles
        let mut instances = Vec::new();
          // Create a central spinning triangle
        let angle = time * PI / 2.0;  // full rotation every 4 seconds
        let scale = 0.5 * (time * 2.0).sin() + 1.0;  // oscillates between 0.5 and 1.5
        
        // Create transformation matrix using glam
        let rotation = Mat4::from_rotation_z(angle);
        let scaling = Mat4::from_scale(Vec3::splat(scale));
        let central_transform = rotation * scaling;
        
        // Add central rotating triangle with pulsing colors
        let r = (time * 2.0).sin() * 0.5 + 0.5;
        let g = (time * 2.0 + PI / 3.0).sin() * 0.5 + 0.5;
        let b = (time * 2.0 + 2.0 * PI / 3.0).sin() * 0.5 + 0.5;
        
        instances.push(StandardShaderInstances {
            model_matrix: central_transform.to_cols_array_2d(),
            instance_color: [r, g, b],
        });
        
        // Add orbiting triangles
        let num_orbits = 3;
        for i in 0..num_orbits {
            let orbit_radius = 0.6 + (i as f32 * 0.2);
            let orbit_speed = 1.0 + (i as f32 * 0.5);
            let orbit_angle = time * orbit_speed;
            
            let orbit_x = orbit_radius * orbit_angle.cos();
            let orbit_y = orbit_radius * orbit_angle.sin();
            
            let local_angle = time * (i as f32 + 1.0) * 1.5;
            let local_scale = 0.3;
              // Create transformation matrices using glam
            let rotation = Mat4::from_rotation_z(local_angle);
            let scaling = Mat4::from_scale(Vec3::splat(local_scale));
            let translation = Mat4::from_translation(Vec3::new(orbit_x, orbit_y, 0.0));
            let transform = translation * rotation * scaling;
            
            let phase = (i as f32 / num_orbits as f32) * 2.0 * PI;
            let r = (time * 1.5 + phase).sin() * 0.5 + 0.5;
            let g = (time * 1.5 + phase + 2.0 * PI / 3.0).sin() * 0.5 + 0.5;
            let b = (time * 1.5 + phase + 4.0 * PI / 3.0).sin() * 0.5 + 0.5;
            
            instances.push(StandardShaderInstances {
                model_matrix: transform.to_cols_array_2d(),
                instance_color: [r, g, b],
            });
        }
        
        // Get camera matrices
        let matrices = camera.get_render_matrices();
        
        // Build and render geometry
        let mut builder = StandardShader::geometry_builder();
        builder.triangle_strip(&vertices);
        let geometry = builder.build();
        
        canvas.draw_with_instances(shader_id)
              .uniform("view", matrices.view)
              .uniform("projection", matrices.projection)
              .uniform("time", time)
              .pump_geometry(&geometry, &instances);
    })
    .run()?;
    
    Ok(())
}
