use cgmath::{Point3, Vector3};
use rustica_render::{RenderWindow, ShaderType};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a render window with a frame callback
    RenderWindow::new("Basic Triangle", 800, 600)
        .with_frame_callback(|canvas| {
            // Define the triangle vertices (static positions)
            let points = [
                Point3::new(0.0, 0.5, 0.0),    // Top
                Point3::new(-0.5, -0.5, 0.0),  // Bottom left
                Point3::new(0.5, -0.5, 0.0),   // Bottom right
            ];
            
            // Define the triangle colors (RGB for each vertex)
            let colors = [
                Vector3::new(1.0, 0.0, 0.0),  // Red (top)
                Vector3::new(0.0, 1.0, 0.0),  // Green (bottom left)
                Vector3::new(0.0, 0.0, 1.0),  // Blue (bottom right)
            ];
            
            // Draw the triangle using the debug color shader
            canvas.draw_triangle(points, colors, ShaderType::DebugColor);
        })
        .run()?;
    
    Ok(())
}
