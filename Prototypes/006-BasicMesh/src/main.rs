use rustica_render::RenderWindow;


fn main() -> Result<(), Box<dyn std::error::Error>> {    
    RenderWindow::
        new("Cube Demo", 800, 600)
        .with_frame_callback(move | canvas| {

    }).run()?;
    
    Ok(())
}
