// Rustica Conway - Conway's Game of Life implementation for the Rustica engine
//
// This crate provides components and systems for Conway's Game of Life
// built on top of the rustica_ecs Entity Component System.

// Export modules
pub mod components;
pub mod systems;
pub mod prelude;

// Re-export prelude for convenience
pub use prelude::*;

/// Utility function to setup a grid of cells for Conway's Game of Life
pub fn setup_conway_grid(
    world: &mut rustica_ecs::world::World,
    width: usize,
    height: usize,
    initial_pattern: &[(usize, usize)],
) {
    use components::{Position, CellState};
    
    // Register components
    world.register::<Position>();
    world.register::<CellState>();
    
    // Calculate the bounding box of the pattern to center it
    let mut min_x = usize::MAX;
    let mut max_x = 0;
    let mut min_y = usize::MAX;
    let mut max_y = 0;
    
    for &(x, y) in initial_pattern {
        min_x = min_x.min(x);
        max_x = max_x.max(x);
        min_y = min_y.min(y);
        max_y = max_y.max(y);
    }
    
    // Calculate pattern dimensions
    let pattern_width = if initial_pattern.is_empty() { 0 } else { max_x - min_x + 1 };
    let pattern_height = if initial_pattern.is_empty() { 0 } else { max_y - min_y + 1 };
    
    // Calculate offsets to center the pattern
    let offset_x = (width.saturating_sub(pattern_width)) / 2;
    let offset_y = (height.saturating_sub(pattern_height)) / 2;
    
    // Create entities for each cell in the grid
    for y in 0..height {
        for x in 0..width {
            // Check if this position corresponds to a live cell in the centered pattern
            let pattern_x = x.checked_sub(offset_x);
            let pattern_y = y.checked_sub(offset_y);
            
            let is_alive = match (pattern_x, pattern_y) {
                (Some(px), Some(py)) => {
                    px <= max_x - min_x && 
                    py <= max_y - min_y && 
                    initial_pattern.contains(&(px + min_x, py + min_y))
                },
                _ => false
            };
            
            world.create_entity()
                .with(Position { x, y })
                .with(CellState { alive: is_alive })
                .build();
        }
    }
}

/// Standard patterns for Conway's Game of Life
pub mod patterns {
    /// Glider pattern
    pub const GLIDER: [(usize, usize); 5] = [
        (1, 0), (2, 1), (0, 2), (1, 2), (2, 2)
    ];
    
    /// Blinker pattern (oscillates with period 2)
    pub const BLINKER: [(usize, usize); 3] = [
        (1, 0), (1, 1), (1, 2)
    ];
    
    /// Toad pattern (oscillates with period 2)
    pub const TOAD: [(usize, usize); 6] = [
        (2, 1), (3, 1), (4, 1), (1, 2), (2, 2), (3, 2)
    ];
    
    /// Beacon pattern (oscillates with period 2)
    pub const BEACON: [(usize, usize); 6] = [
        (1, 1), (2, 1), (1, 2), (4, 3), (3, 4), (4, 4)
    ];
    
    /// Pulsar pattern (oscillates with period 3)
    pub const PULSAR: [(usize, usize); 48] = [
        (2, 0), (3, 0), (4, 0), (8, 0), (9, 0), (10, 0),
        (0, 2), (5, 2), (7, 2), (12, 2),
        (0, 3), (5, 3), (7, 3), (12, 3),
        (0, 4), (5, 4), (7, 4), (12, 4),
        (2, 5), (3, 5), (4, 5), (8, 5), (9, 5), (10, 5),
        (2, 7), (3, 7), (4, 7), (8, 7), (9, 7), (10, 7),
        (0, 8), (5, 8), (7, 8), (12, 8),
        (0, 9), (5, 9), (7, 9), (12, 9),
        (0, 10), (5, 10), (7, 10), (12, 10),
        (2, 12), (3, 12), (4, 12), (8, 12), (9, 12), (10, 12)
    ];
}
