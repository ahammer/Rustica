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

// Re-export math functionality
pub use rustica_math as math;

// Re-export scheduler functionality
pub use rustica_scheduler as scheduler;

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
    
    // Re-export from math
    pub use rustica_math::{Vec2, Vec3, Mat4, Quat, vec2, vec3, mat4_identity, quat_identity};
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
