use rustica_graphics::Camera;
use rustica_render::{RenderWindow, Canvas};
use cgmath::{Matrix4, Vector3, Point3, Rad}; 
use rustica_standard_geometry::GeometryFactory;
use rustica_standard_shader::{StandardShader, StandardShaderInstances};
use glam::Vec3 as GlamVec3; // Use glam for color input to factory

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a render window and register the standard shader
    let mut window = RenderWindow::new("007 - UV Sphere (Instanced)", 1024, 768);
    let shader_descriptor = StandardShader::descriptor(); // Use StandardShader
    let shader_id = window.register_shader(shader_descriptor);

    // Create UV sphere geometry using the GeometryFactory
    // We can customize the resolution with sectors (longitude) and stacks (latitude)
    let sphere_geometry = GeometryFactory::uv_sphere(
        1.0,           // radius
        32,            // sectors (longitude segments)
        16,            // stacks (latitude segments)
        GlamVec3::ONE  // white color base
    );
    
    // --- Camera Setup ---
    let initial_width = 1024.0;
    let initial_height = 768.0;
    let aspect_ratio = initial_width / initial_height;
    let camera = Camera::new(
        Point3::new(0.0, 0.0, 15.0), // eye position
        Point3::new(0.0, 0.0, 0.0),   // target
        Vector3::unit_y(),            // up vector
        45.0,                    // fov
        aspect_ratio,
        0.1,                          // near plane
        100.0,                        // far plane
    );

    // Set up the frame callback with instanced rendering
    window.with_frame_callback(move |canvas| {
        let time = canvas.time().as_secs_f32();
        
        // Get view and projection matrices from camera
        let view = camera.view_matrix();
        let projection = camera.projection_matrix();
        
        // Create instance data for multiple spheres
        let mut instances = Vec::new();
        
        // Create a solar system arrangement
        
        // Sun (central sphere)
        let sun_model = Matrix4::from_scale(2.0); // Larger size for the sun
        
        // Add sun with yellow/orange color
        instances.push(StandardShaderInstances {
            model_matrix: sun_model.into(),
            instance_color: [1.0, 0.7, 0.2], // Sun color (yellow/orange)
        });
        
        // Add planets with moons
        let num_planets = 4;
        
        for i in 0..num_planets {
            // Planet orbit parameters
            let orbit_radius = 4.0 + (i as f32 * 1.5); // Increasing radius for each planet
            let orbit_speed = 0.3 / (i as f32 + 1.0);  // Slower speed for outer planets
            let orbit_angle = time * orbit_speed;
            let planet_size = 0.4 + (i as f32 * 0.1);  // Slightly increasing sizes
            
            // Calculate planet position in orbit
            let orbit_x = orbit_radius * orbit_angle.cos();
            let orbit_z = orbit_radius * orbit_angle.sin();
            
            // Create planet rotation and position
            let planet_rotation = Matrix4::from_angle_y(Rad(time * (0.5 + i as f32 * 0.2)));
            let planet_scale = Matrix4::from_scale(planet_size);
            let planet_translation = Matrix4::from_translation(Vector3::new(orbit_x, 0.0, orbit_z));
            
            // Combine transformations
            let planet_model = planet_translation * planet_rotation * planet_scale;
            
            // Create planet color based on position
            let planet_color = [
                0.3 + (i as f32 * 0.15),
                0.5 - (i as f32 * 0.1),
                0.7 + (i as f32 * 0.05),
            ];
            
            // Add planet to instances
            instances.push(StandardShaderInstances {
                model_matrix: planet_model.into(),
                instance_color: planet_color,
            });
            
            // Add moons for each planet (more moons for outer planets)
            let num_moons = i + 1;
            
            for j in 0..num_moons {
                // Moon orbit parameters
                let moon_radius = 0.8 + (j as f32 * 0.3); // Moon orbit radius
                let moon_speed = 2.0 + (j as f32 * 0.5);  // Moon orbit speed
                let moon_angle = time * moon_speed + (j as f32 * std::f32::consts::PI * 0.5);
                let moon_size = 0.15; // Small moons
                
                // Calculate moon position relative to planet
                let moon_x = moon_radius * moon_angle.cos();
                let moon_z = moon_radius * moon_angle.sin();
                
                // Moon transformations (first relative to planet)
                let moon_scale = Matrix4::from_scale(moon_size);
                let moon_translation_local = Matrix4::from_translation(Vector3::new(moon_x, 0.0, moon_z));
                
                // Combine with planet position to get world position
                let moon_model = planet_translation * moon_translation_local * moon_scale;
                
                // Create moon color (gray-ish)
                let moon_color = [0.8, 0.8, 0.85];
                
                // Add moon to instances
                instances.push(StandardShaderInstances {
                    model_matrix: moon_model.into(),
                    instance_color: moon_color,
                });
            }
        }
        
        // Draw all spheres with a single instanced call using StandardShader
        canvas.draw_with_instances(shader_id)
              .uniform("view", view)
              .uniform("projection", projection)
              .uniform("time", time)
              .pump_geometry(&sphere_geometry, &instances);
    }).run()?;
    
    Ok(())
}
