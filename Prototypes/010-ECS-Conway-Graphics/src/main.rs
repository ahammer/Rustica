use std::sync::Arc;
use std::time::{Duration, Instant};

use rustica_conway::prelude::*;
use rustica_render::{
    RenderWindow, ShaderDescriptor, Vertex, StandardMeshAdapter
};
use rustica_graphics::{Camera, primitives::shapes::cube::create_cube};

use cgmath::{Matrix4, Point3, Vector3, Deg};

// Define a custom vertex type with derive macro
#[repr(C)]
#[derive(Copy, Clone, Debug, Vertex)]
struct CellVertex {
    position: [f32; 3], // location = 0, format = Float32x3
    color: [f32; 3],    // location = 1, format = Float32x3
    normal: [f32; 3],   // location = 2, format = Float32x3
}

// Define a shader descriptor using the derive macro
#[derive(ShaderDescriptor)]
#[shader(source = "./src/shaders/conway_shader.wgsl")]
struct ConwayShaderDescriptor {    
    #[vertex_type]
    vertex: CellVertex,
    
    #[uniform(binding = 0)]
    model: Matrix4<f32>,
    
    #[uniform(binding = 1)]
    view: Matrix4<f32>,
    
    #[uniform(binding = 2)]
    projection: Matrix4<f32>,
}

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
    
    // Calculate total grid size for camera positioning
    let total_width = (grid_width as f32) * (cube_size + spacing);
    let total_height = (grid_height as f32) * (cube_size + spacing);
    
    // Create render window
    let mut window = RenderWindow::new("Conway's Game of Life - ECS Graphics", 800, 600);
    
    // Register the shader
    let shader_id = window.register_shader(ConwayShaderDescriptor::descriptor());
    
    // Create a cube mesh to reuse for all cells
    let cube_mesh = Arc::new(create_cube(cube_size));
    
    // Create mesh adapter with vertex mapper
    let mesh_adapter = StandardMeshAdapter::new(cube_mesh, |standard_vertex| {
        CellVertex {
            position: standard_vertex.position,
            color: [0.2, 0.8, 0.3], // Green color for cells
            normal: standard_vertex.normal,
        }
    });
    
    // Add the life system to the world
    world.add_system(LifeSystem {
        grid_width,
        grid_height,
    });
    
    // Set up simulation timing
    let simulation_interval = Duration::from_millis(200);
    let mut last_update = Instant::now();
    
    // Set up rendering with frame callback
    window.with_frame_callback(move |canvas| {
        // Get time since start
        let now = Instant::now();
        
        // Update Conway simulation at fixed intervals
        if now - last_update >= simulation_interval {
            // Run the simulation step (only LifeSystem)
            world.run_systems();
            last_update = now;
        }
        
        // Set up camera
        let camera_height = total_height.max(total_width) * 1.2;
        
        // Create perspective camera looking down at the grid
        let mut camera = Camera::perspective(800.0 / 600.0);
        camera.look_at_from(
            Point3::new(0.0, camera_height, camera_height * 0.75), // Position above and slightly back
            Point3::new(0.0, 0.0, 0.0),                           // Look at center            
        );
        
        // Get camera matrices
        let camera_matrices = camera.get_render_matrices();
        let view = camera_matrices.view;
        let projection = camera_matrices.projection;
        
        // Get triangles from mesh adapter (we'll reuse these for each cell)
        let triangles = mesh_adapter.to_triangles();
        
        // Query for alive cells and render each one
        for (_, (pos, state)) in world.query_two::<Position, CellState>() {
            // Only render alive cells
            if state.alive {
                // Calculate model matrix for this cell
                let model = calculate_cell_transform(pos.x, pos.y, grid_width, grid_height, cube_size, spacing);
                
                // Draw the cell with the shader
                canvas.draw_with_shader(shader_id)
                      .uniform("model", model)
                      .uniform("view", view)
                      .uniform("projection", projection)
                      .triangles(&triangles);
            }
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
) -> Matrix4<f32> {
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
    Matrix4::from_translation(Vector3::new(pos_x, pos_y, pos_z))
}
