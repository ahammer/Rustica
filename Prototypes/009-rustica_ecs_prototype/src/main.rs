use std::time::Duration;
use std::thread;
use std::collections::HashMap;

// Import the ECS from the new crate
use rustica_ecs::prelude::*;

// ============ Conway's Game of Life Implementation ============

// Position component
#[derive(Debug, Clone)]
struct Position {
    x: usize,
    y: usize,
}

impl Component for Position {}

// Cell state component
#[derive(Debug, Clone)]
struct CellState {
    alive: bool,
}

impl Component for CellState {}

// System to apply Conway's Game of Life rules
struct LifeSystem {
    grid_width: usize,
    grid_height: usize,
}

impl System for LifeSystem {
    fn run(&self, world: &mut World) {
        // First, collect all cell states
        let mut grid = vec![vec![false; self.grid_width]; self.grid_height];
        let cells = world.query_two::<Position, CellState>();
        
        // Populate the grid with current cell states
        for (_, (pos, state)) in &cells {
            grid[pos.y][pos.x] = state.alive;
        }
        
        // Calculate next state for each cell
        let mut next_states = HashMap::new();
        
        for (entity, (pos, _)) in &cells {
            let x = pos.x;
            let y = pos.y;
            
            let mut live_neighbors = 0;
            
            // Check all 8 neighbors
            for dy in -1..=1 {
                for dx in -1..=1 {
                    // Skip the cell itself
                    if dx == 0 && dy == 0 {
                        continue;
                    }
                    
                    let nx = (x as isize + dx) as usize;
                    let ny = (y as isize + dy) as usize;
                    
                    // Check bounds
                    if nx < self.grid_width && ny < self.grid_height {
                        if grid[ny][nx] {
                            live_neighbors += 1;
                        }
                    }
                }
            }
            
            // Get current state
            let current_alive = grid[y][x];
            
            // Apply Conway's rules:
            let next_alive = match (current_alive, live_neighbors) {
                // Rule 1: Any live cell with fewer than two live neighbors dies (underpopulation)
                (true, 0..=1) => false,
                
                // Rule 2: Any live cell with two or three live neighbors lives on
                (true, 2..=3) => true,
                
                // Rule 3: Any live cell with more than three live neighbors dies (overpopulation)
                (true, _) => false,
                
                // Rule 4: Any dead cell with exactly three live neighbors becomes a live cell (reproduction)
                (false, 3) => true,
                
                // Otherwise remain in current state
                (state, _) => state,
            };
            
            next_states.insert(*entity, next_alive);
        }
        
        // Update all cell states
        for (entity, next_alive) in next_states {
            if let Some(cell) = world.get_component_mut::<CellState>(entity) {
                cell.alive = next_alive;
            }
        }
    }
}

// System to render the grid
struct RenderSystem {
    grid_width: usize,
    grid_height: usize,
}

impl System for RenderSystem {
    fn run(&self, world: &mut World) {
        // Create an empty grid
        let mut grid = vec![vec![' '; self.grid_width]; self.grid_height];
        
        // Fill in live cells
        for (_, (pos, state)) in world.query_two::<Position, CellState>() {
            grid[pos.y][pos.x] = if state.alive { '■' } else { '□' };
        }
        
        // Clear screen (in a simple way)
        print!("\x1B[2J\x1B[1;1H");
        
        // Print the grid
        for row in grid {
            for cell in row {
                print!("{}", cell);
            }
            println!();
        }
    }
}

fn main() {
    // Create a new world
    let mut world = World::new();
    
    // Register components
    world.register::<Position>();
    world.register::<CellState>();
    
    // Set up grid dimensions
    let grid_width = 40;
    let grid_height = 20;
    
    // Create entities for each cell in the grid
    for y in 0..grid_height {
        for x in 0..grid_width {
            world.create_entity()
                .with(Position { x, y })
                .with(CellState { alive: false })
                .build();
        }
    }
    
    // Set up a glider pattern
    let glider_positions = [
        (1, 0), (2, 1), (0, 2), (1, 2), (2, 2)
    ];
    
    // We can't modify components directly from the query results
    // So we'll gather the entities that need to be modified
    let mut entities_to_update = Vec::new();
    
    for (entity, (pos, _)) in world.query_two::<Position, CellState>() {
        for &(gx, gy) in &glider_positions {
            if pos.x == gx && pos.y == gy {
                entities_to_update.push(entity);
            }
        }
    }
    
    // Now update the entities
    for entity in entities_to_update {
        if let Some(state) = world.get_component_mut::<CellState>(entity) {
            state.alive = true;
        }
    }
    
    // Add systems
    world.add_system(RenderSystem {
        grid_width,
        grid_height,
    });
    
    world.add_system(LifeSystem {
        grid_width,
        grid_height,
    });
    
    // Run simulation
    println!("Conway's Game of Life - ECS Implementation");
    println!("Press Ctrl+C to exit");
    
    loop {
        world.run_systems();
        thread::sleep(Duration::from_millis(200));
    }
}
