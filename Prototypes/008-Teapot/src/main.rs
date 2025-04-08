use glam::{Mat4, Vec3};
use rustica_graphics::Camera;
use rustica_render::RenderWindow;


fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    // Create window
    let mut window = RenderWindow::new("Utah Teapot Demo (Instanced)", 800, 600);
    
    // Register shader
    let shader_id = window.register_shader(shader_descriptor);
    
    // Create teapot mesh
    let teapot_mesh = // Acquire through standard geometry
        
    
      // Create camera
    let mut camera = Camera::perspective(800.0 / 600.0);
    camera.look_at_from(
        Vec3::new(0.0, 3.0, 10.0), 
        Vec3::new(0.0, 0.0, 0.0)
    );
    
    // Set up frame callback
    window.with_frame_callback(move |canvas| {
        let time = canvas.time().as_secs_f32();
        
        // Get view and projection matrices
        let view = camera.view_matrix();
        let projection = camera.projection_matrix();
        
        // Create instances of teapots
        let mut instances = Vec::new();
          // Create a central large teapot
        let base_scale = 1.0;
        let central_model = Mat4::from_scale(Vec3::splat(base_scale));
        let central_rotation = Mat4::from_rotation_y(time * 0.5);
        
        // Convert the model matrix to array format
        let central_model_array = (central_rotation * central_model).to_cols_array_2d();
        
        // Add central teapot
        instances.push(TeapotInstance::new(
            central_model_array,
            [0.8, 0.2, 0.2] // Reddish
        ));
        
        // Add a circle of smaller teapots
        let num_teapots = 5;
        let circle_radius = 4.0;
        
        for i in 0..num_teapots {
            // Calculate position on the circle
            let angle = i as f32 * 2.0 * std::f32::consts::PI / num_teapots as f32;
            let position_x = circle_radius * angle.cos();
            let position_z = circle_radius * angle.sin();
              // Create independent rotation for each teapot
            let spin_speed = 1.0 + (i as f32 * 0.2);
            let local_rotation = Mat4::from_rotation_y(time * spin_speed);
            
            // Create bobbing motion
            let bob_height = (time * 1.5 + angle).sin() * 0.5;
            
            // Create scale (smaller than central teapot)
            let scale = 0.5;
            let scale_matrix = Mat4::from_scale(Vec3::splat(scale));
            
            // Create translation
            let translation = Mat4::from_translation(Vec3::new(position_x, bob_height, position_z));
            
            // Combine transformations
            let model = translation * local_rotation * scale_matrix;
            
            // Convert to array format
            let model_array = model.to_cols_array_2d();
            
            // Create color based on position
            let color = [
                0.3 + 0.5 * ((angle + time * 0.1).cos() * 0.5 + 0.5),
                0.3 + 0.5 * ((angle + time * 0.2).sin() * 0.5 + 0.5),
                0.3 + 0.5 * ((angle + time * 0.3).cos() * 0.5 + 0.5),
            ];
            
            // Add the teapot instance
            instances.push(TeapotInstance::new(model_array, color));
        }
        
        // Draw all teapots with a single instanced draw call
    }).run()?;
    
    Ok(())
}

// Helper function to convert a Matrix4 to a 2D array is no longer needed
// as we now use glam's built-in to_cols_array_2d()
