//! # Rustica Core: Minimal Engine Orchestration Layer
//! 
//! This crate provides the central orchestration for the Rustica game engine.
//! It defines the plugin system, application lifecycle, and core abstractions.
//! 
//! ## AGENT DIRECTIVES
//! 
//! - DOC_ROOT: /docs/
//! - ARCH_DOC: /docs/architecture.md#RusticaCore
//! - API_RULES: /docs/api_conventions.md#CoreAPI
//! - TEST_RULES: /docs/testing_standards.md#CoreTesting
//! 
//! ## Critical Rules
//! 
//! 1. Core must remain minimal with no dependencies on specific implementations
//! 2. Public API must be stable and well-documented
//! 3. All functionality besides basic orchestration should be in plugins
//! 
//! ## Usage Example
//! 
//! ```rust
//! use rustica_core::prelude::*;
//! 
//! // Define a custom plugin
//! struct MyGamePlugin;
//! 
//! impl Plugin for MyGamePlugin {
//!     fn build(&self, app: &mut App) {
//!         // Configure the app with game-specific functionality
//!     }
//! }
//! 
//! fn main() {
//!     // Create the application
//!     let mut app = App::new();
//!     
//!     // Add plugins
//!     app.add_plugin(MyGamePlugin);
//!     
//!     // Run the application
//!     app.run();
//! }
//! ```

// === REGION: MODULE DEFINITIONS ===
mod app;
mod plugin;
mod error;

// === REGION: PUBLIC EXPORTS ===
pub use app::App;
pub use plugin::Plugin;
pub use error::CoreError;

/// Prelude module containing the most commonly used types.
pub mod prelude {
    pub use crate::App;
    pub use crate::Plugin;
    pub use crate::CoreError;
}

// === REGION: TESTS ===
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_creation() {
        let app = App::new();
        // Simply testing that the app can be created without errors
    }

    #[test]
    fn test_plugin_registration() {
        struct TestPlugin;
        
        impl Plugin for TestPlugin {
            fn build(&self, _app: &mut App) {
                // Empty implementation for testing
            }
        }
        
        let mut app = App::new();
        app.add_plugin(TestPlugin);
        // Test passes if no panic occurs
    }
}
