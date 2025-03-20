//! # Rustica Game Engine
//! 
//! Rustica is a modular, testable game engine written in Rust.
//! 
//! This crate serves as the main entry point for using the Rustica engine,
//! re-exporting functionality from the various subsystem crates.
//!
//! ## AGENT DIRECTIVES
//! 
//! - DOC_ROOT: /docs/
//! - ARCH_DOC: /docs/architecture.md
//! - API_RULES: /docs/api_conventions.md
//! 
//! ## Critical Rules
//! 
//! 1. The main crate should only re-export functionality
//! 2. Each subsystem should be properly documented
//! 3. Dependencies between subsystems should be clear
//! 
//! ## Usage Example
//! 
//! ```rust
//! use rustica::prelude::*;
//! 
//! // Create a simple game
//! fn main() {
//!     // Create the application
//!     let mut app = App::new();
//!     
//!     // Add core plugins
//!     app.add_plugin(EcsPlugin::default());
//!     // Add your game plugin
//!     app.add_plugin(MyGamePlugin);
//!     
//!     // Run the application
//!     app.run();
//! }
//! 
//! // Define a game plugin
//! struct MyGamePlugin;
//! 
//! impl Plugin for MyGamePlugin {
//!     fn build(&self, app: &mut App) {
//!         // Setup your game here
//!     }
//! }
//! ```

// === REGION: IMPORTS & RE-EXPORTS ===

// Re-export core functionality
pub use rustica_core as core;

// Re-export ECS functionality
pub use rustica_ecs as ecs;

// Re-export math functionality from cgmath
pub use cgmath as math;

// Re-export scheduler functionality
pub use rustica_scheduler as scheduler;

// Re-export render functionality when the render feature is enabled
#[cfg(feature = "render")]
pub use rustica_render as render;

// === REGION: PRELUDE ===

/// Prelude module containing the most commonly used types.
///
/// Importing this module with `use rustica::prelude::*` will bring
/// the most commonly used types into scope.
pub mod prelude {
    // Re-export from core
    pub use rustica_core::App;
    pub use rustica_core::Plugin;
    
    // Re-export from ECS
    pub use rustica_ecs::World;
    pub use rustica_ecs::Entity;
    pub use rustica_ecs::Component;
    pub use rustica_ecs::EcsPlugin;
    
    // Re-export from scheduler
    pub use rustica_scheduler::{Schedule, System, Stage, SchedulerPlugin};
    pub use rustica_scheduler::system::SystemFn;
    
    // Re-export from cgmath
    pub use cgmath::{Vector2 as Vec2, Vector3 as Vec3, Matrix4 as Mat4, Quaternion as Quat};
    // Re-export functions as equivalent operations
    pub use cgmath::vec2;
    pub use cgmath::vec3;
    
    // Re-export render types when the render feature is enabled
    #[cfg(feature = "render")]
    pub use crate::render::prelude::*;
    
    /// Creates an identity 4x4 matrix
    pub fn mat4_identity() -> Mat4<f32> {
        // Create a matrix with diagonals set to 1.0
        Mat4::new(
            1.0, 0.0, 0.0, 0.0,
            0.0, 1.0, 0.0, 0.0,
            0.0, 0.0, 1.0, 0.0,
            0.0, 0.0, 0.0, 1.0,
        )
    }
    
    /// Creates an identity quaternion
    pub fn quat_identity() -> Quat<f32> {
        Quat::new(1.0, 0.0, 0.0, 0.0)
    }
}

// === REGION: TESTS ===
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_imports() {
        // Just verify that imports work
        let _app = core::App::new();
        let _world = ecs::World::new();
    }
}
