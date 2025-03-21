//! # Rustica Common: Shared Types and Interfaces
//! 
//! This crate provides common types and interfaces shared between
//! different components of the Rustica engine.
//!
//! ## Critical Rules
//! 
//! 1. This crate should have minimal dependencies
//! 2. No circular dependencies are allowed
//! 3. All types should be well-documented and stable

// === REGION: MODULE DEFINITIONS ===
pub mod plugin;
pub mod resource;

// === REGION: PUBLIC EXPORTS ===
pub use plugin::PluginMetadata;
pub use resource::Resource;

/// Prelude module containing the most commonly used types.
pub mod prelude {
    pub use crate::PluginMetadata;
    pub use crate::Resource;
}

// === REGION: TESTS ===
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_imports() {
        // Just testing that the types can be imported
        let _: Option<&dyn PluginMetadata> = None;
        // Test passes if no panic occurs
    }
}
