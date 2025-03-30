use cgmath::{Point3, Vector3, Rad};
use rustica_render::{RenderWindow, ShaderType};
use std::f32::consts::PI;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    RenderWindow::new("Spinning/Scaling Triangle", 800, 600)
        .with_frame_callback(|canvas| {
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
            
            // Draw the transformed triangle.
            canvas.draw_triangle(
                [transformed_points[0], transformed_points[1], transformed_points[2]],
                colors,
                ShaderType::DebugColor,
            );
        })
        .run()?;
    
    Ok(())
}
