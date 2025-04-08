use rustica_graphics::Camera;
use rustica_render::{RenderWindow, Canvas};
use cgmath::{Matrix4, Vector3, Point3}; 
use rustica_standard_geometry::GeometryFactory;
use rustica_standard_shader::{StandardShader, StandardShaderInstances};
use glam::Vec3 as GlamVec3; // Use glam for color input to factory



fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a render window and register the standard shader
    let mut window = RenderWindow::new("006 - Basic Mesh (Instanced Cubes)", 1024, 768);
    let shader_descriptor = StandardShader::descriptor(); // Use StandardShader
    let shader_id = window.register_shader(shader_descriptor);

    // Create cube geometry using the GeometryFactory
    // Use StandardShaderVertex (re-exported in standard_geometry::prelude)
    let cube_geometry = GeometryFactory::cube(1.0, GlamVec3::ONE); // Create a 1x1x1 white cube

    // --- Camera Setup ---
    let initial_width = 1024.0;
    let initial_height = 768.0;
    let aspect_ratio = initial_width / initial_height;
    // Camera setup remains the same for now, using rustica_graphics::Camera
    let camera = Camera::new(
        Point3::new(10.0, 10.0, 15.0), // eye position
        Point3::new(0.0, 0.0, 0.0),   // target
        Vector3::unit_y(),            // up vector
        45.0,                    // fov (using cgmath::Deg)
        aspect_ratio,
        0.1,                          // near plane
        100.0,                        // far plane
    );


    // --- Instancing Setup ---
    const GRID_SIZE: i32 = 15; // e.g., 15x15 grid
    const SPACING: f32 = 1.5;
    const FREQUENCY: f32 = 0.5;
    const AMPLITUDE: f32 = 1.5;

    // Removed comment about view_proj_matrix calculation location

    window.with_frame_callback(move |canvas: &mut Canvas| {
        let time = canvas.time().as_secs_f32();

  

        // --- Instance Generation ---
        let mut instances = Vec::new();
        for x in -GRID_SIZE / 2..GRID_SIZE / 2 {
            for z in -GRID_SIZE / 2..GRID_SIZE / 2 {
                let base_pos = Vector3::new(x as f32 * SPACING, 0.0, z as f32 * SPACING);

                // Simple wave calculation
                let dist_from_center = (base_pos.x.powi(2) + base_pos.z.powi(2)).sqrt();
                let y_offset = (dist_from_center * FREQUENCY - time * 2.0).sin() * AMPLITUDE;

                let instance_pos = base_pos + Vector3::new(0.0, y_offset, 0.0);
                let model_matrix = Matrix4::from_translation(instance_pos);

                // Color based on position/wave height
                let normalized_y = (y_offset / AMPLITUDE + 1.0) / 2.0; // Normalize y_offset to 0..1
                let instance_color = [normalized_y, 0.5, 1.0 - normalized_y]; // Example: Blue to Red gradient

                // Create the instance data struct expected by StandardShader (plural)
                // Convert cgmath types to raw arrays
                let instance_data = StandardShaderInstances { // Use StandardShaderInstances
                    model_matrix: model_matrix.into(), // Convert Matrix4 to [[f32; 4]; 4]
                    instance_color,
                };
                instances.push(instance_data);
            }
        }

        // --- Drawing ---
        // Get view and projection matrices from the camera
        let view_matrix = camera.view_matrix();
        let projection_matrix = camera.projection_matrix(); // Assuming Camera provides separate matrices

        // Use draw_with_instances and pump_geometry with StandardShader uniforms
        canvas.draw_with_instances(shader_id)
              .uniform("view", view_matrix) // Pass view matrix
              .uniform("projection", projection_matrix) // Pass projection matrix
              .uniform("time", time)
              .pump_geometry(&cube_geometry, &instances);
    })
    .run()?;

    Ok(())
}
