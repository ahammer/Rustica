use rustica_core::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a render window
    let mut render_window = RenderWindow::new("Basic WGPU", 800, 600);
    
    // Set the clear color (optional)
    render_window.set_clear_color(0.0, 1.0, 0.0, 1.0);
    
    // Run the render window
    render_window.run()?;
    
    Ok(())
}
