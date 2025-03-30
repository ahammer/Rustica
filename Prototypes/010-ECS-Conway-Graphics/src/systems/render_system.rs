use rustica_conway::prelude::*;
use rustica_ecs::prelude::*;
use rustica_render::prelude::*;
use rustica_graphics::primitives::shapes::factory::ShapeFactory;
use rustica_foundation::prelude::*;

use cgmath::{Matrix4, Point3, Vector3};

/// System to render Conway's Game of Life cells as 3D cubes
pub struct ConwayRenderSystem {
    pub grid_width: usize,
    pub grid_height: usize,
    pub cube_size: f32,
    pub spacing: f32,
    render_context: RenderContext,
    cube_mesh: Option<StandardMesh>,
}

impl ConwayRenderSystem {
    /// Create a new Conway render system
    pub fn new(render_context: RenderContext, cube_size: f32, spacing: f32, grid_width: usize, grid_height: usize) -> Self {
        ConwayRenderSystem {
            grid_width,
            grid_height,
            cube_size,
            spacing,
            render_context,
            cube_mesh: None,
        }
    }
    
    /// Initialize rendering resources
    pub fn initialize(&mut self) {
        // Create a cube mesh to reuse for all cells
        let mut shape_factory = ShapeFactory::new();
        self.cube_mesh = Some(shape_factory.create_cube(&self.render_context, self.cube_size));
    }
    
    /// Calculate the transformation matrix for a cell at the specified position
    fn calculate_transform(&self, x: usize, y: usize) -> Matrix4<f32> {
        // Center the grid at (0,0,0)
        let grid_width_f32 = self.grid_width as f32;
        let grid_height_f32 = self.grid_height as f32;
        
        // Total size of a cell including spacing
        let cell_size = self.cube_size + self.spacing;
        
        // Calculate position
        let pos_x = (x as f32 - grid_width_f32 / 2.0) * cell_size;
        let pos_y = 0.0; // Cubes rest on the XZ plane
        let pos_z = (y as f32 - grid_height_f32 / 2.0) * cell_size;
        
        // Create translation matrix
        Matrix4::from_translation(Vector3::new(pos_x, pos_y, pos_z))
    }
}

impl System for ConwayRenderSystem {
    fn run(&self, world: &mut World) {
        // Check if mesh is initialized
        if let Some(cube_mesh) = &self.cube_mesh {
            // Clear previous draw commands
            self.render_context.begin_frame();
            
            // Query for alive cells
            for (_, (pos, state)) in world.query_two::<Position, CellState>() {
                // Only render alive cells
                if state.alive {
                    let transform = self.calculate_transform(pos.x, pos.y);
                    
                    // Add cube draw command
                    self.render_context.add_mesh_draw_command(
                        cube_mesh,
                        transform,
                    );
                }
            }
            
            // Commit the draw commands
            self.render_context.end_frame();
        }
    }
}
