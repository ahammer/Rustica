use cgmath::{Point3, Vector3, Rad};
use rustica_render::{RenderWindow, Triangle};
use rustica_render_derive::{ShaderDescriptor, Vertex};
use rustica_foundation::geometry::Triangle as GeometryTriangle;
use std::f32::consts::PI;

// Define a custom vertex type with the Vertex trait
#[repr(C)]
#[derive(Copy, Clone, Debug, Vertex)]
struct BasicVertex {
    position: [f32; 3], // location = 0
    color: [f32; 3],    // location = 1
}

// Define a shader descriptor using the derive macro
#[derive(ShaderDescriptor)]
#[shader(source = "./src/shaders/animated_triangle.wgsl")]
struct AnimatedShaderDescriptor {
    #[vertex_type]
    vertex: BasicVertex,
    
    #[uniform(binding = 0)]
    time: f32,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a shader descriptor
    let shader_descriptor = AnimatedShaderDescriptor::descriptor();
    
    // Create a render window and register the shader
    let mut window = RenderWindow::new("Spinning/Scaling Triangle", 800, 600);
    let shader_id = window.register_shader(shader_descriptor);
    
    window.with_frame_callback(move |canvas| {
        let time = canvas.time();
        let seconds = time.as_secs_f32();
        
        // Rotation: full rotation every 4 seconds.
        let angle = Rad(seconds * PI / 2.0);
        
        // Scaling: oscillates between 0.5 and 1.5.
        let scale = 0.5 * (seconds * 2.0).sin() + 1.0;
        
        // Original triangle vertices (not centered)
        let vertices = [
            Point3::new(0.0, 0.5, 0.0),
            Point3::new(-0.5, -0.5, 0.0),
            Point3::new(0.5, -0.5, 0.0),
        ];
        
        // Compute the centroid of the triangle.
        let center = Point3::new(
            (vertices[0].x + vertices[1].x + vertices[2].x) / 3.0,
            (vertices[0].y + vertices[1].y + vertices[2].y) / 3.0,
            0.0,
        );
        
        // Apply rotation and scaling around the centroid.
        let transformed_points: Vec<Point3<f32>> = vertices.iter().map(|p| {
            // Translate vertex so that the centroid is at the origin.
            let dx = p.x - center.x;
            let dy = p.y - center.y;
            
            // Rotate the point.
            let rotated_x = dx * angle.0.cos() - dy * angle.0.sin();
            let rotated_y = dx * angle.0.sin() + dy * angle.0.cos();
            
            // Scale and translate back.
            Point3::new(
                rotated_x * scale + center.x,
                rotated_y * scale + center.y,
                p.z,
            )
        }).collect();
        
        // Define colors that still pulse over time.
        let colors = [
            Vector3::new((seconds * 2.0).sin() * 0.5 + 0.5, 0.0, 0.0),
            Vector3::new(0.0, (seconds * 2.0 + PI / 3.0).sin() * 0.5 + 0.5, 0.0),
            Vector3::new(0.0, 0.0, (seconds * 2.0 + 2.0 * PI / 3.0).sin() * 0.5 + 0.5),
        ];
        
        // Create basic vertices with transformed positions and colors
        let basic_vertices = [
            BasicVertex {
                position: [transformed_points[0].x, transformed_points[0].y, transformed_points[0].z],
                color: [colors[0].x, colors[0].y, colors[0].z],
            },
            BasicVertex {
                position: [transformed_points[1].x, transformed_points[1].y, transformed_points[1].z],
                color: [colors[1].x, colors[1].y, colors[1].z],
            },
            BasicVertex {
                position: [transformed_points[2].x, transformed_points[2].y, transformed_points[2].z],
                color: [colors[2].x, colors[2].y, colors[2].z],
            },
        ];
        
        // Create a triangle from vertices
        let triangle = GeometryTriangle { vertices: basic_vertices };
        
        // Draw the transformed triangle using the modern shader API
        canvas.draw_with_shader(shader_id)
              .uniform("time", seconds)
              .triangles(&[triangle]);
    })
    .run()?;
    
    Ok(())
}
