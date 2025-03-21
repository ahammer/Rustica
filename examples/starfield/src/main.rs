//! # Starfield Example
//! 
//! A simple starfield simulation using the Rustica game engine.
//! This example demonstrates the basic usage of the engine,
//! including entity creation, component definition, and plugins.
//!
//! The starfield consists of stars that move across the screen.
//! This serves as a minimal "hello world" demonstrating the engine's
//! architecture and API.

mod physics;
mod input;

use rustica::prelude::*;
use physics::{Position, Velocity, Time, PhysicsConfig};
use input::{create_star_movement_system, create_star_property_system};

// === REGION: COMPONENT DEFINITIONS ===

/// A star component that represents a star in the starfield.
#[derive(Debug, Clone)]
pub struct Star {
    /// Brightness of the star (0.0 to 1.0)
    pub brightness: f32,
    /// Size of the star
    pub size: f32,
    /// Color of the star (optional)
    pub color: Option<[f32; 3]>,
}

impl Default for Star {
    fn default() -> Self {
        Self {
            brightness: 0.8,
            size: 1.0,
            color: None,
        }
    }
}

// === REGION: PLUGIN DEFINITION ===

/// The main plugin for the Starfield example.
struct StarfieldPlugin {
    /// Number of stars to create
    star_count: usize,
    /// Physics configuration
    physics_config: PhysicsConfig,
}

impl Default for StarfieldPlugin {
    fn default() -> Self {
        Self {
            star_count: 1000,
            physics_config: PhysicsConfig::default(),
        }
    }
}

impl StarfieldPlugin {
    /// Create a new StarfieldPlugin with custom settings
    pub fn new(star_count: usize, physics_config: PhysicsConfig) -> Self {
        Self {
            star_count,
            physics_config,
        }
    }
    
    /// Spawn stars with random positions and velocities
    fn spawn_stars(&self, world: &mut World) {
        // In a real implementation, we would use random number generation
        // For now, just create stars with deterministic values
        for i in 0..self.star_count.min(5) { // Limit to 5 for now since we're just displaying info
            let position = Position {
                value: Vec3::new(
                    (i as f32 - 2.5) * 100.0,
                    (i as f32 - 2.5) * 80.0,
                    100.0 + i as f32 * 50.0
                ),
            };
            
            let velocity = Velocity {
                value: Vec3::new(0.0, 0.0, -(i as f32) - 1.0),
                damping: 1.0,
            };
            
            let star = Star {
                brightness: 0.5 + i as f32 * 0.1,
                size: 1.0 + i as f32 * 0.5,
                color: None,
            };
            
            world.spawn()
                .insert(position)
                .insert(velocity)
                .insert(star);
        }
    }
}

impl Plugin for StarfieldPlugin {
    fn build(&self, app: &mut App) {
        // Add resources
        app.insert_resource(self.physics_config);
        app.insert_resource(Time::default());
        
        // Register systems
        // In a real implementation with a working scheduler, we would do:
        // app.add_system(create_position_update_system());
        // app.add_system(create_velocity_update_system());
        // app.add_system(create_boundary_wrap_system());
        
        // Register input handling systems
        // app.add_system(create_star_movement_system());
        // app.add_system(create_star_property_system());
        
        // Note: Input handling will be implemented fully in a future update
        // Currently the systems are placeholders
        
        // For now, just spawn stars
        if let Some(world) = app.get_resource_mut::<World>() {
            self.spawn_stars(world);
        }
    }
    
    fn name(&self) -> &str {
        "StarfieldPlugin"
    }
}

// === REGION: SYSTEMS ===

// Our systems are now defined in the physics.rs module

// === REGION: MAIN FUNCTION ===

fn main() {
    // Create a new application
    let mut app = App::new();
    
    // Add the ECS plugin for entity management
    app.add_plugin(EcsPlugin::default());
    
    // Add the RenderPlugin
    app.add_plugin(RenderPlugin::default());
    
    // Configure physics
    let physics_config = PhysicsConfig {
        world_bounds: Vec3::new(1000.0, 800.0, 1000.0),
        wrap_around_bounds: true,
    };
    
    // Add the starfield plugin with 1000 stars
    app.add_plugin(StarfieldPlugin::new(1000, physics_config));
    
    // Print debug information
    println!("Starting Starfield example - Hello Rustica World!");
    
    if let Some(_world) = app.get_resource::<World>() {
        println!("Starfield initialized with Position/Velocity system");
        println!("Physics configuration:");
        if let Some(config) = app.get_resource::<PhysicsConfig>() {
            println!("  World bounds: {:.1} x {:.1} x {:.1}", 
                config.world_bounds.x, 
                config.world_bounds.y, 
                config.world_bounds.z);
            println!("  Wrap around bounds: {}", config.wrap_around_bounds);
        }
        println!("Running application with window...");
    }
    
    // Run the application with the event loop (this is a blocking call)
    app.run();
}
