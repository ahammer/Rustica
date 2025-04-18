use rustica_ecs::prelude::*;
use rustica_conway::prelude::*; // For Position and CellState
use rand::Rng; // For random placement
use std::time::Duration;

// Define the Glider pattern relative coordinates
const GLIDER_PATTERN: [(usize, usize); 5] = [
    (1, 0), (2, 1), (0, 2), (1, 2), (2, 2)
];
const GLIDER_WIDTH: usize = 3;
const GLIDER_HEIGHT: usize = 3;

/// System to periodically spawn patterns (like Gliders) onto the Conway grid.
pub struct CellSpawnerSystem {
    pub interval: Duration, // How often to spawn (e.g., 2 seconds)
    timer: Duration,        // Time remaining until next spawn
    pub grid_width: usize,
    pub grid_height: usize,
}

impl CellSpawnerSystem {
    pub fn new(interval: Duration, grid_width: usize, grid_height: usize) -> Self {
        Self {
            interval,
            timer: interval, // Start ready to spawn
            grid_width,
            grid_height,
        }
    }

    /// Update the spawner state, checking if it's time to spawn a pattern.
    /// This method requires mutable access to self to update the timer.
    pub fn update(&mut self, world: &mut World, delta_time: Duration) {
        self.timer = self.timer.saturating_sub(delta_time);

        if self.timer == Duration::ZERO {
            // Reset timer for the next interval
            self.timer = self.interval;

            // Ensure grid is large enough for the pattern
            if self.grid_width < GLIDER_WIDTH || self.grid_height < GLIDER_HEIGHT {
                eprintln!("Grid too small to spawn Glider pattern.");
                return;
            }

            // Choose a random top-left starting position for the pattern
            let mut rng = rand::rng();
            let max_start_x = self.grid_width - GLIDER_WIDTH;
            let max_start_y = self.grid_height - GLIDER_HEIGHT;
            let start_x = rng.random_range(0..=max_start_x);
            let start_y = rng.random_range(0..=max_start_y);

            // Collect entities and their positions first to avoid borrowing issues
            let mut entities_to_update = Vec::new();
            {
                let query = world.query_two::<Position, CellState>();
                for (entity, (pos, _state)) in query {
                    // Check if this entity's position falls within the pattern's target area
                    for (dx, dy) in GLIDER_PATTERN.iter() {
                        let target_x = start_x + dx;
                        let target_y = start_y + dy;
                        if pos.x == target_x && pos.y == target_y {
                            entities_to_update.push(entity);
                            break; // Move to the next entity once matched
                        }
                    }
                }
            } // Query borrow ends here

            // Now, update the state of the collected entities
            for entity in entities_to_update {
                if let Some(state) = world.get_component_mut::<CellState>(entity) {
                    state.alive = true;
                    // Optional: Reset visual transition if CellVisual exists
                    // if let Some(visual) = world.get_component_mut::<CellVisual>(entity) {
                    //     visual.is_transitioning = false; // Force immediate visual update? Or let animation handle it?
                    // }
                }
            }

            // println!("Spawned Glider at ({}, {})", start_x, start_y); // Debugging
        }
    }
}

// Note: This system does not implement the `System` trait because it needs mutable access
// to its internal timer state (`&mut self`). It should be called directly in the main loop
// like `spawner_system.update(&mut world, delta_time);`
