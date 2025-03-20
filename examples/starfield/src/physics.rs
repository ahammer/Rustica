//! # Physics System for Starfield
//! 
//! This module provides components and systems for physics simulation in the starfield example,
//! including position and velocity components, and systems for updating entities based on
//! these components.

use rustica::prelude::*;
use rustica::scheduler::system::SystemFn;
use std::time::Duration;

/// Position component for entities in 3D space
#[derive(Debug, Clone, Copy)]
pub struct Position {
    /// The position in 3D space
    pub value: Vec3<f32>,
}

impl Default for Position {
    fn default() -> Self {
        Self {
            value: Vec3::new(0.0, 0.0, 0.0),
        }
    }
}

/// Velocity component for moving entities
#[derive(Debug, Clone, Copy)]
pub struct Velocity {
    /// The velocity vector
    pub value: Vec3<f32>,
    /// A damping factor to gradually reduce velocity (0.0 = full damping, 1.0 = no damping)
    pub damping: f32,
}

impl Default for Velocity {
    fn default() -> Self {
        Self {
            value: Vec3::new(0.0, 0.0, 0.0),
            damping: 1.0, // No damping by default
        }
    }
}

/// Acceleration component for forces acting on entities
#[derive(Debug, Clone, Copy)]
pub struct Acceleration {
    /// The acceleration vector
    pub value: Vec3<f32>,
}

impl Default for Acceleration {
    fn default() -> Self {
        Self {
            value: Vec3::new(0.0, 0.0, 0.0),
        }
    }
}

/// Configuration for the physics system
#[derive(Debug, Clone, Copy)]
pub struct PhysicsConfig {
    /// How large the world boundaries are
    pub world_bounds: Vec3<f32>,
    /// Whether to wrap entities around world boundaries
    pub wrap_around_bounds: bool,
}

impl Default for PhysicsConfig {
    fn default() -> Self {
        Self {
            world_bounds: Vec3::new(1000.0, 1000.0, 1000.0),
            wrap_around_bounds: true,
        }
    }
}

/// System for updating positions based on velocities
pub fn position_update_system(_world: &mut World) {
    // In a real implementation, we would query for all entities with Position and Velocity components,
    // and update their positions based on velocity and delta time.
    
    // For now, this is a stub that doesn't do anything since the World implementation is not complete.
    
    // A proper implementation would look like:
    // let delta_time = world.resource::<Time>().delta_seconds();
    // for (mut position, velocity) in world.query::<(&mut Position, &Velocity)>() {
    //     position.value += velocity.value * delta_time;
    // }
}

/// System for updating velocities based on accelerations
pub fn velocity_update_system(_world: &mut World) {
    // Similar to position_update_system, this is a stub for now
    
    // A proper implementation would look like:
    // let delta_time = world.resource::<Time>().delta_seconds();
    // for (mut velocity, acceleration) in world.query::<(&mut Velocity, &Acceleration)>() {
    //     velocity.value += acceleration.value * delta_time;
    //     velocity.value *= velocity.damping.powf(delta_time);
    // }
}

/// System for wrapping entities around world boundaries
pub fn boundary_wrap_system(_world: &mut World) {
    // A stub system that would wrap entities around world boundaries
    
    // A proper implementation would look like:
    // let config = world.resource::<PhysicsConfig>();
    // if config.wrap_around_bounds {
    //     for mut position in world.query::<&mut Position>() {
    //         // Wrap around X axis
    //         if position.value.x > config.world_bounds.x / 2.0 {
    //             position.value.x -= config.world_bounds.x;
    //         } else if position.value.x < -config.world_bounds.x / 2.0 {
    //             position.value.x += config.world_bounds.x;
    //         }
    //         
    //         // Wrap around Y axis
    //         if position.value.y > config.world_bounds.y / 2.0 {
    //             position.value.y -= config.world_bounds.y;
    //         } else if position.value.y < -config.world_bounds.y / 2.0 {
    //             position.value.y += config.world_bounds.y;
    //         }
    //         
    //         // Wrap around Z axis
    //         if position.value.z > config.world_bounds.z / 2.0 {
    //             position.value.z -= config.world_bounds.z;
    //         } else if position.value.z < -config.world_bounds.z / 2.0 {
    //             position.value.z += config.world_bounds.z;
    //         }
    //     }
    // }
}

/// A simple timer resource for tracking delta time
#[derive(Debug, Clone, Copy)]
pub struct Time {
    /// The time elapsed since the last frame
    pub delta: Duration,
}

impl Default for Time {
    fn default() -> Self {
        Self {
            delta: Duration::from_secs_f32(1.0 / 60.0), // Default to 60 FPS
        }
    }
}

impl Time {
    /// Get the delta time in seconds
    pub fn delta_seconds(&self) -> f32 {
        self.delta.as_secs_f32()
    }
    
    /// Update the delta time
    pub fn update(&mut self, delta: Duration) {
        self.delta = delta;
    }
}

/// Create and return a position update system
pub fn create_position_update_system() -> Box<dyn System> {
    Box::new(SystemFn::new(
        position_update_system,
        "position_update"
    ))
}

/// Create and return a velocity update system
pub fn create_velocity_update_system() -> Box<dyn System> {
    Box::new(SystemFn::new(
        velocity_update_system,
        "velocity_update"
    ).with_dependency("position_update")) // Velocity updates should come before position updates
}

/// Create and return a boundary wrap system
pub fn create_boundary_wrap_system() -> Box<dyn System> {
    Box::new(SystemFn::new(
        boundary_wrap_system,
        "boundary_wrap"
    ).with_dependency("position_update")) // Boundary wrap should happen after position update
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_position_default() {
        let pos = Position::default();
        assert_eq!(pos.value.x, 0.0);
        assert_eq!(pos.value.y, 0.0);
        assert_eq!(pos.value.z, 0.0);
    }
    
    #[test]
    fn test_velocity_default() {
        let vel = Velocity::default();
        assert_eq!(vel.value.x, 0.0);
        assert_eq!(vel.value.y, 0.0);
        assert_eq!(vel.value.z, 0.0);
        assert_eq!(vel.damping, 1.0);
    }
    
    #[test]
    fn test_create_systems() {
        let pos_sys = create_position_update_system();
        let vel_sys = create_velocity_update_system();
        let bound_sys = create_boundary_wrap_system();
        
        assert_eq!(pos_sys.name(), "position_update");
        assert_eq!(vel_sys.name(), "velocity_update");
        assert_eq!(bound_sys.name(), "boundary_wrap");
        
        assert!(pos_sys.dependencies().is_empty());
        assert_eq!(vel_sys.dependencies()[0], "position_update");
        assert_eq!(bound_sys.dependencies()[0], "position_update");
    }
    
    #[test]
    fn test_time_delta_seconds() {
        let time = Time {
            delta: Duration::from_millis(16), // ~60 FPS
        };
        assert!((time.delta_seconds() - 0.016).abs() < 0.0001);
    }
}
