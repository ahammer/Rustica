//! # ECS Plugin
//! 
//! This module defines the EcsPlugin for integrating the ECS system
//! with the core engine.
//!
//! ## AGENT DIRECTIVES
//! 
//! - DOC_ROOT: /docs/
//! - ARCH_DOC: /docs/architecture.md#ECSPlugin
//! - API_RULES: /docs/api_conventions.md#PluginAPI
//! 
//! ## Critical Rules
//! 
//! 1. Plugin should provide minimal footprint in the core
//! 2. Configuration should be flexible and customizable
//! 3. Plugin API should be stable

// === REGION: IMPORTS ===
use rustica_core::prelude::*;
use crate::World;

// === REGION: PLUGIN IMPLEMENTATION ===

/// Plugin for integrating the ECS system with the core engine.
///
/// The EcsPlugin registers the World resource and any core ECS systems
/// needed for the engine to function.
///
/// # Examples
///
/// ```
/// use rustica_core::prelude::*;
/// use rustica_ecs::prelude::*;
///
/// let mut app = App::new();
/// app.add_plugin(EcsPlugin::default());
/// ```
pub struct EcsPlugin {
    // Configuration options could go here
}

impl Default for EcsPlugin {
    fn default() -> Self {
        EcsPlugin {}
    }
}

impl Plugin for EcsPlugin {
    fn build(&self, app: &mut App) {
        // Add the World resource
        app.insert_resource(World::new());
        
        // In a real implementation, we would register ECS systems here
        // For now, this is just a stub
    }
    
    fn name(&self) -> &str {
        "EcsPlugin"
    }
    
    // No dependencies by default, but could be overridden if needed
}

// === REGION: TESTS ===
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_plugin_creation() {
        let plugin = EcsPlugin::default();
        assert_eq!(plugin.name(), "EcsPlugin");
    }
    
    #[test]
    fn test_plugin_build() {
        let mut app = App::new();
        let plugin = EcsPlugin::default();
        plugin.build(&mut app);
        
        // Verify the World resource was added
        assert!(app.get_resource::<World>().is_some());
    }
}
