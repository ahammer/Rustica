use std::f32::consts::PI;

use rustica_render::RenderWindow;
use rustica_render_derive::ShaderProperties;
use rustica_standard_shader::{StandardShader, StandardShaderInstances};
use rustica_graphics::primitives::camera::Camera;
use cgmath::{Matrix4, Point3, SquareMatrix, Vector3};
use std::time::Instant;
use rustica_render::Vertex;

use bytemuck::{Pod, Zeroable};

// Define our shader using the ShaderProperties derive macro
#[derive(ShaderProperties)]
#[shader(file = "shaders/animated_triangle.wgsl")]
struct AnimatedShader {
    // Vertex attributes
    #[vertex(location = 0)]
    position: [f32; 3],
    
    #[vertex(location = 1)]
    color: [f32; 3],
    
    // Instance attributes
    #[instance(location = 3)]
    model_matrix: [[f32; 4]; 4],
    
    #[instance(location = 7)]
    instance_color: [f32; 3],
    
    // Uniform
    #[uniform(binding = 0)]
    time: f32,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a shader descriptor
    let shader_descriptor = AnimatedShader::descriptor();
    
    // Create a render window and register the shader
    let mut window = RenderWindow::new("Spinning/Scaling Triangle (Instanced)", 800, 600);
    let shader_id = window.register_shader(shader_descriptor);
    
    window.with_frame_callback(move |canvas| {
        let time = canvas.time();
        let seconds = time.as_secs_f32();
        let vertex_factory = AnimatedShader::vertex_factory();
        
        // Define the triangle vertices using the vertex factory
        let vertices = [
            vertex_factory.create_vertex(
                [0.0, 0.5, 0.0],    // Top
                [1.0, 0.0, 0.0]     // Red
            ),
            vertex_factory.create_vertex(
                [-0.5, -0.5, 0.0],  // Bottom left
                [0.0, 1.0, 0.0]     // Green
            ),
            vertex_factory.create_vertex(
                [0.5, -0.5, 0.0],   // Bottom right
                [0.0, 0.0, 1.0]     // Blue
            ),
        ];
        
        // Create instance data for multiple triangles
        let mut instances = Vec::new();
        
        // Create a central spinning triangle
        
        // Rotation: full rotation every 4 seconds
        let angle = seconds * PI / 2.0;
        
        // Scaling: oscillates between 0.5 and 1.5
        let scale = 0.5 * (seconds * 2.0).sin() + 1.0;
        
        // Create rotation matrix
        let rot_matrix = [
            [angle.cos() * scale, -angle.sin() * scale, 0.0, 0.0],
            [angle.sin() * scale, angle.cos() * scale, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ];
        
        // Add central rotating triangle with pulsing colors
        let r = (seconds * 2.0).sin() * 0.5 + 0.5;
        let g = (seconds * 2.0 + PI / 3.0).sin() * 0.5 + 0.5;
        let b = (seconds * 2.0 + 2.0 * PI / 3.0).sin() * 0.5 + 0.5;
        
        instances.push(AnimatedShaderInstances {
            model_matrix: rot_matrix,
            instance_color: [r, g, b],
        });
        
        // Add orbiting triangles
        let num_orbits = 3;
        for i in 0..num_orbits {
            // Different orbit radius for each triangle
            let orbit_radius = 0.6 + (i as f32 * 0.2);
            
            // Different rotation speeds
            let orbit_speed = 1.0 + (i as f32 * 0.5);
            let orbit_angle = seconds * orbit_speed;
            
            // Position on orbit
            let orbit_x = orbit_radius * orbit_angle.cos();
            let orbit_y = orbit_radius * orbit_angle.sin();
            
            // Individual rotation
            let local_angle = seconds * (i as f32 + 1.0) * 1.5;
            let local_scale = 0.3; // Smaller triangles
            
            // Create transformation matrix
            let model_matrix = [
                [local_angle.cos() * local_scale, -local_angle.sin() * local_scale, 0.0, 0.0],
                [local_angle.sin() * local_scale, local_angle.cos() * local_scale, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [orbit_x, orbit_y, 0.0, 1.0],
            ];
            
            // Create instance with unique color
            let phase = (i as f32 / num_orbits as f32) * 2.0 * PI;
            let r = (seconds * 1.5 + phase).sin() * 0.5 + 0.5;
            let g = (seconds * 1.5 + phase + 2.0 * PI / 3.0).sin() * 0.5 + 0.5;
            let b = (seconds * 1.5 + phase + 4.0 * PI / 3.0).sin() * 0.5 + 0.5;
            
            instances.push(AnimatedShaderInstances {
                model_matrix: model_matrix,
                instance_color: [r, g, b],
            });
        }
        
        // Build geometry from vertices
        let mut builder = AnimatedShader::geometry_builder();
        builder.triangle_strip(&vertices);
        let geometry = builder.build();
        
        // Draw all triangles using instanced rendering
        canvas.draw_with_instances(shader_id)
              .uniform("time", seconds)
              .pump_geometry(&geometry, &instances);
    })
    .run()?;
    
    Ok(())
}
