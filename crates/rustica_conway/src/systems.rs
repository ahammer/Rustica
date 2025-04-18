// Conway Game of Life systems

use std::collections::HashMap;
use rustica_ecs::prelude::*;
use crate::components::{Position, CellState};

/// System to apply Conway's Game of Life rules
pub struct LifeSystem {
    pub grid_width: usize,
    pub grid_height: usize,
    pub wraparound: bool,
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
                    
                    // Handle wraparound when enabled
                    if self.wraparound {
                        // Use modulo arithmetic to wrap around the grid
                        let nx = (((x as isize + dx) % self.grid_width as isize) + self.grid_width as isize) % self.grid_width as isize;
                        let ny = (((y as isize + dy) % self.grid_height as isize) + self.grid_height as isize) % self.grid_height as isize;
                        
                        if grid[ny as usize][nx as usize] {
                            live_neighbors += 1;
                        }
                    } else {
                        // Original non-wraparound behavior
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

/// Text-based renderer for the Conway grid (for debugging)
pub struct TextRenderSystem {
    pub grid_width: usize,
    pub grid_height: usize,
}

impl System for TextRenderSystem {
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
