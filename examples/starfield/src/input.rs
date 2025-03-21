//! # Input handling for Starfield
//! 
//! This module provides input handling functionality for the Starfield example,
//! including keyboard controls for star movement and camera control.

use rustica::prelude::*;
use crate::physics::PhysicsConfig;

/// A system that handles keyboard input for star movement
pub fn star_movement_system(world: &mut World) {
    // For now, just print a message as a placeholder
    println!("Star movement system running");

    // Get physics config for bounds
    let config = match world.get_resource::<PhysicsConfig>() {
        Some(config) => *config,
        None => PhysicsConfig::default(),
    };

    // This is a simplified placeholder until the input system is fully integrated
    println!("Physics bounds: {:?}", config.world_bounds);
}

/// Create and return a star movement input system
pub fn create_star_movement_system() -> Box<dyn System> {
    Box::new(SystemFn::new(
        star_movement_system,
        "star_movement"
    ))
}

/// Handle keyboard input for toggling star properties
pub fn star_property_system(world: &mut World) {
    // For now, just print a message as a placeholder
    println!("Star property system running");
    
    // In a real implementation, we would handle key presses for:
    // - Space key to pause/resume stars
    // - R key to reset star positions
}

/// Create and return a star property system
pub fn create_star_property_system() -> Box<dyn System> {
    Box::new(SystemFn::new(
        star_property_system,
        "star_property"
    ))
}
