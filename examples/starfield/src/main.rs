//! # Starfield Example
//! 
//! A simple starfield simulation using the Rustica game engine.
//! This example demonstrates the basic usage of the engine,
//! including entity creation, component definition, and plugins.
//!
//! The starfield consists of stars that move across the screen.
//! This serves as a minimal "hello world" demonstrating the engine's
//! architecture and API.

use rustica::prelude::*;

// === REGION: COMPONENT DEFINITIONS ===

/// A position component for entities in 3D space.
#[derive(Debug)]
struct Position {
    x: f32,
    y: f32,
    z: f32,
}

/// A velocity component for moving entities.
#[derive(Debug)]
struct Velocity {
    x: f32,
    y: f32,
    z: f32,
}

/// A star component that represents a star in the starfield.
#[derive(Debug)]
struct Star {
    brightness: f32,
    size: f32,
}

// === REGION: PLUGIN DEFINITION ===

/// The main plugin for the Starfield example.
struct StarfieldPlugin;

impl Plugin for StarfieldPlugin {
    fn build(&self, app: &mut App) {
        // Register systems
        // In a real implementation, we would add systems here
        // For now, just a stub
        
        // In a real implementation, this would spawn multiple stars
        // For now, just spawn a few stars as a demonstration
        let world = app.get_resource_mut::<World>().unwrap();
        
        // Spawn 5 stars with random positions and velocities
        for i in 0..5 {
            let x = (i as f32 - 2.5) * 100.0;
            let y = (i as f32 - 2.5) * 80.0;
            
            world.spawn()
                .insert(Position { x, y, z: 100.0 + i as f32 * 50.0 })
                .insert(Velocity { x: 0.0, y: 0.0, z: -i as f32 - 1.0 })
                .insert(Star { brightness: 0.5 + i as f32 * 0.1, size: 1.0 + i as f32 * 0.5 });
        }
    }
    
    fn name(&self) -> &str {
        "StarfieldPlugin"
    }
}

// === REGION: SYSTEMS ===

// In a real implementation, we would define systems here
// For example:

/*
fn update_positions(world: &mut World) {
    for (position, velocity) in world.query::<(&mut Position, &Velocity)>() {
        position.x += velocity.x;
        position.y += velocity.y;
        position.z += velocity.z;
        
        // Wrap stars that go off screen
        if position.z < 0.0 {
            position.z = 1000.0;
        }
    }
}
*/

// === REGION: MAIN FUNCTION ===

fn main() {
    // Create a new application
    let mut app = App::new();
    
    // Add the ECS plugin for entity management
    app.add_plugin(EcsPlugin::default());
    
    // Add the starfield plugin
    app.add_plugin(StarfieldPlugin);
    
    // Run the application
    println!("Starting Starfield example - Hello Rustica World!");
    
    // In a real application, we would call app.run() here
    // For now, just print what's in the world
    
    if let Some(world) = app.get_resource::<World>() {
        // In a real implementation, this would query and print star info
        println!("Starfield initialized with 5 stars");
        println!("This is a minimal example showing the engine structure");
        println!("In a full implementation, this would display moving stars");
    }
}
