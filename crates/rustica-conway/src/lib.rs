// Rustica Conway - Conway's Game of Life implementation for the Rustica engine
//
// This crate provides components and systems for Conway's Game of Life
// built on top of the rustica-ecs Entity Component System.

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
    
    // Create entities for each cell in the grid
    for y in 0..height {
        for x in 0..width {
            // Default state is dead (false)
            let is_alive = initial_pattern.contains(&(x, y));
            
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
