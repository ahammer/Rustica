use std::sync::Arc;
use std::time::{Duration, Instant};

use rustica_conway::prelude::*;
use rustica_render::{
    RenderWindow, ShaderDescriptor, Vertex
};
use rustica_graphics::Camera;

use glam::{Mat4, Vec3};
use rustica_standard_shader::{StandardShader, StandardShaderInstances};
use rustica_standard_geometry::GeometryFactory;

use crate::systems::{VisualAnimationSystem, CameraAnimationSystem, CellSpawnerSystem}; // Added CellSpawnerSystem
use crate::components::{CellVisual, CameraState, CellInstance};

pub mod components;
mod systems;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Set up Conway grid dimensions
    let grid_width = 30;
    let grid_height = 30;
    let cube_size = 0.5;
    let spacing = 0.1;
    
    // Create a new ECS world
    let mut world = World::new();
    
    // Add some interesting pattern
    // Use a larger pattern for better visual effect
    setup_conway_grid(&mut world, grid_width, grid_height, &patterns::PULSAR);
    
    // Create render window
    let mut window = RenderWindow::new("Conway's Game of Life - Infinity Board", 800, 600);
    
    // Register the shader
    let shader_id = window.register_shader(StandardShader::descriptor());
      // Create a sphere mesh to reuse for all cells
    let sphere_mesh = GeometryFactory::uv_sphere(
        0.5,           // radius
        32,            // sectors (longitude segments)
        16,            // stacks (latitude segments)
        Vec3::ONE      // white color base (will be tinted by instance color)
    );
    
    
    // Add the life system to the world with wraparound enabled
    world.add_system(LifeSystem {
        grid_width,
        grid_height,
        wraparound: true, // Enable the wraparound for infinite board behavior
    });
    
    // Set up simulation timing
    let simulation_interval = Duration::from_millis(200);
    let mut last_update = Instant::now();
    
    // Set up animation parameters
    let animation_duration = 0.5; // Half-second transitions
    let mut last_frame = Instant::now();

    // Initialize the spawner system
    let mut spawner_system = CellSpawnerSystem::new(
        Duration::from_secs(2), // Spawn every 2 seconds
        grid_width,
        grid_height,
    );

    // Set up rendering with frame callback
    window.with_frame_callback(move |canvas| {
        // Get time since start
        let now = Instant::now();
        let delta_time = (now - last_frame).as_secs_f32();
        last_frame = now;
        
        // Update Conway simulation at fixed intervals
        if now - last_update >= simulation_interval {
            // Run the simulation step (only LifeSystem)
            world.run_systems();
            last_update = now;
        }

        // Run spawner system first
        spawner_system.update(&mut world, Duration::from_secs_f32(delta_time));

        // Run animation systems every frame

        // 1. Cell visual animations
        let visual_system = VisualAnimationSystem {
            transition_duration: animation_duration,
            delta_time,
        };
        visual_system.run(&mut world);
        
        // 2. Camera animation
        let camera_system = CameraAnimationSystem {
            delta_time,
            grid_width,
            grid_height,
            cube_size,
            spacing,
        };
        
        camera_system.run(&mut world);
        
        // Get current camera state
        let default_camera = CameraState::default();
        
        // Find the camera entity and get its state
        let mut camera_state_ref = &default_camera;
        {
            let camera_query = world.query_one::<CameraState>();
            for (_, state) in camera_query {
                camera_state_ref = state;
                break;
            }
        }
        
        // Update camera from animation state
        let mut camera = Camera::perspective(800.0 / 600.0);
        camera.fov = 30.0;
        camera.look_at_from(
            camera_state_ref.position,
            camera_state_ref.target,
        );
        
        // Get camera matrices
        let camera_matrices = camera.get_render_matrices();
        let view = camera_matrices.view;
        let projection = camera_matrices.projection;
        
        
        // Collect instance data for all cells with visuals
        let mut instances = Vec::new();
        
        // Query for all cells with positions and visuals
        for (_, (pos, visual)) in world.query_two::<Position, CellVisual>() {
            // Only render if scale is not too small
            if visual.scale > 0.01 {                        // Create scale matrix
                        let scale = Mat4::from_scale(Vec3::splat(visual.scale));
                        
                        // Render the cell in a 3x3 grid to create the infinity board effect
                        // This repeats the board in all 8 surrounding directions
                        for offset_x in -1..=1 {
                            for offset_z in -1..=1 {
                                // Calculate model matrix with appropriate offset
                                let model = calculate_infinity_cell_transform(
                                    pos.x, pos.y, grid_width, grid_height, 
                                    cube_size, spacing, offset_x, offset_z
                                );
                                
                                // Combine matrices
                                let combined = model * scale;
                                
                                // Convert to shader-compatible format
                                let model_array = combined.to_cols_array_2d();
                                
                                // Create instance data with color
                                // Slightly fade the color for cells in the surrounding grids
                                let color = if offset_x == 0 && offset_z == 0 {
                                    visual.color // Main grid has original color
                                } else {                                    // Surrounding grids have slightly dimmer color
                                    [
                                        visual.color[0] * 0.85,
                                        visual.color[1] * 0.85,
                                        visual.color[2] * 0.85,
                                    ]
                                };
                        
                        // Use StandardShaderInstances for compatibility with standard shader
                        let instance = StandardShaderInstances {
                            model_matrix: model_array,
                            instance_color: color,
                        };
                        instances.push(instance);
                    }
                }
            }
        }
        
        // Draw all cells with a single instanced draw call if any cells are ready to render
        if !instances.is_empty() {
            // Get the time value before starting the method chain
            let time = canvas.time().as_secs_f32();
            // Draw the geometry with instances using the standard shader pattern
            canvas.draw_with_instances(shader_id)
                  .uniform("view", view)
                  .uniform("projection", projection)
                  .uniform("time", time)
                  .pump_geometry(&sphere_mesh, &instances);
        }
    }).run()?;
    
    Ok(())
}

// Calculate transformation matrix for a cell
fn calculate_cell_transform(
    x: usize, 
    y: usize, 
    grid_width: usize, 
    grid_height: usize, 
    cube_size: f32, 
    spacing: f32
) -> Mat4 {
    // Center the grid at (0,0,0)
    let grid_width_f32 = grid_width as f32;
    let grid_height_f32 = grid_height as f32;
    
    // Total size of a cell including spacing
    let cell_size = cube_size + spacing;
    
    // Calculate position
    let pos_x = (x as f32 - grid_width_f32 / 2.0) * cell_size;
    let pos_y = 0.0; // Cubes rest on the XZ plane
    let pos_z = (y as f32 - grid_height_f32 / 2.0) * cell_size;
    
    // Create translation matrix
    Mat4::from_translation(Vec3::new(pos_x, pos_y, pos_z))
}

// Calculate transformation matrix for a cell in the infinity board (with offset)
fn calculate_infinity_cell_transform(
    x: usize, 
    y: usize, 
    grid_width: usize, 
    grid_height: usize, 
    cube_size: f32, 
    spacing: f32,
    grid_offset_x: i32,
    grid_offset_z: i32
) -> Mat4 {
    // Center the grid at (0,0,0)
    let grid_width_f32 = grid_width as f32;
    let grid_height_f32 = grid_height as f32;
    
    // Total size of a cell including spacing
    let cell_size = cube_size + spacing;
    
    // Calculate the total width and height of one grid
    let grid_total_width = grid_width_f32 * cell_size;
    let grid_total_height = grid_height_f32 * cell_size;
    
    // Calculate position within original grid
    let local_pos_x = (x as f32 - grid_width_f32 / 2.0) * cell_size;
    let local_pos_y = 0.0; // Cubes rest on the XZ plane
    let local_pos_z = (y as f32 - grid_height_f32 / 2.0) * cell_size;
    
    // Add grid offset
    let pos_x = local_pos_x + (grid_offset_x as f32 * grid_total_width);
    let pos_z = local_pos_z + (grid_offset_z as f32 * grid_total_height);
    
    // Create translation matrix
    Mat4::from_translation(Vec3::new(pos_x, local_pos_y, pos_z))
}
