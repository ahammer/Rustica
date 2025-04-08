
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Minimal demo setup
    let shader_descriptor = SphereShaderDescriptor::descriptor();
    let mut window = RenderWindow::new("UV Sphere Demo (Instanced)", 800, 600);
    let shader_id = window.register_shader(shader_descriptor);
    
    // Create a UV sphere mesh using the graphics crate implementation
    let sphere_mesh = // create the sphere like 006 cube;
    
    
    let mut camera = Camera::perspective(800.0 / 600.0);
    camera.look_at_from(
        Point3::new(0.0, 0.0, 15.0), 
        Point3::new(0.0, 0.0, 0.0)
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
        let sun_model = Matrix4::from_scale(2.0); // Larger size
        let sun_model_array = [
            [sun_model.x.x, sun_model.x.y, sun_model.x.z, sun_model.x.w],
            [sun_model.y.x, sun_model.y.y, sun_model.y.z, sun_model.y.w],
            [sun_model.z.x, sun_model.z.y, sun_model.z.z, sun_model.z.w],
            [sun_model.w.x, sun_model.w.y, sun_model.w.z, sun_model.w.w],
        ];
        
        // Add sun with yellow/orange color
        instances.push(SphereInstance::new(
            sun_model_array,
            [1.0, 0.7, 0.2] // Sun color
        ));
        
        // Add planets with moons
        let num_planets = 4;
        
        for i in 0..num_planets {
            // Planet orbit parameters
            let orbit_radius = 4.0 + (i as f32 * 1.5); // Increasing radius for each planet
            let orbit_speed = 0.3 / (i as f32 + 1.0); // Slower speed for outer planets
            let orbit_angle = time * orbit_speed;
            let planet_size = 0.4 + (i as f32 * 0.1); // Slightly increasing sizes
            
            // Calculate planet position in orbit
            let orbit_x = orbit_radius * orbit_angle.cos();
            let orbit_z = orbit_radius * orbit_angle.sin();
            
            // Create planet rotation and position
            let planet_rotation = Matrix4::from_angle_y(Rad(time * (0.5 + i as f32 * 0.2)));
            let planet_scale = Matrix4::from_scale(planet_size);
            let planet_translation = Matrix4::from_translation(Vector3::new(orbit_x, 0.0, orbit_z));
            
            // Combine transformations
            let planet_model = planet_translation * planet_rotation * planet_scale;
            
            // Convert to array format
            let planet_model_array = [
                [planet_model.x.x, planet_model.x.y, planet_model.x.z, planet_model.x.w],
                [planet_model.y.x, planet_model.y.y, planet_model.y.z, planet_model.y.w],
                [planet_model.z.x, planet_model.z.y, planet_model.z.z, planet_model.z.w],
                [planet_model.w.x, planet_model.w.y, planet_model.w.z, planet_model.w.w],
            ];
            
            // Create planet color based on position
            let planet_color = [
                0.3 + (i as f32 * 0.15),
                0.5 - (i as f32 * 0.1),
                0.7 + (i as f32 * 0.05),
            ];
            
            // Add planet to instances
            instances.push(SphereInstance::new(
                planet_model_array,
                planet_color
            ));
            
            // Add moons for each planet (more moons for outer planets)
            let num_moons = i + 1;
            
            for j in 0..num_moons {
                // Moon orbit parameters
                let moon_radius = 0.8 + (j as f32 * 0.3); // Moon orbit radius
                let moon_speed = 2.0 + (j as f32 * 0.5); // Moon orbit speed
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
                
                // Convert to array format
                let moon_model_array = [
                    [moon_model.x.x, moon_model.x.y, moon_model.x.z, moon_model.x.w],
                    [moon_model.y.x, moon_model.y.y, moon_model.y.z, moon_model.y.w],
                    [moon_model.z.x, moon_model.z.y, moon_model.z.z, moon_model.z.w],
                    [moon_model.w.x, moon_model.w.y, moon_model.w.z, moon_model.w.w],
                ];
                
                // Create moon color (gray-ish)
                let moon_color = [0.8, 0.8, 0.85];
                
                // Add moon to instances
                instances.push(SphereInstance::new(
                    moon_model_array,
                    moon_color
                ));
            }
        }
        
        // Draw all spheres with a single instanced call
        let geometry = GeometryBuilder::new().with_triangles(&mesh_adapter.to_triangles()).build();
        canvas.draw_with_instances(shader_id)
              .uniform("view", view)
              .uniform("projection", projection)
              .uniform("time", time)
              .pump_geometry(&geometry, &instances);
    }).run()?;
    
    Ok(())
}
