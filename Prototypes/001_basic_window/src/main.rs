use rustica_window::WindowApp;


fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a window application
    let app = WindowApp::new("Basic Window", 800, 600);
    app.run()?;    
    Ok(())
}
