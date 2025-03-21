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
use rustica_common::PluginMetadata;
use crate::World;
use crate::Time;

// === REGION: PLUGIN IMPLEMENTATION ===

/// Plugin for integrating the ECS system with the core engine.
///
/// The EcsPlugin registers the World resource and any core ECS systems
/// needed for the engine to function.
///
/// # Examples
///
/// ```
/// use rustica_ecs::prelude::*;
///
/// // In a typical application context:
/// // let mut app = App::new();
/// // app.build_ecs_plugin(&EcsPlugin::default());
/// ```
pub struct EcsPlugin {
    // Configuration options could go here
}

impl Default for EcsPlugin {
    fn default() -> Self {
        EcsPlugin {}
    }
}

impl PluginMetadata for EcsPlugin {
    fn name(&self) -> &str {
        "EcsPlugin"
    }
    
    // No dependencies by default
}

// Extension trait for building the ECS plugin
pub trait EcsPluginExt {
    fn build_ecs_plugin(&mut self, plugin: &EcsPlugin);
}

impl<T> EcsPluginExt for T 
where 
    T: InsertResource,
{
    fn build_ecs_plugin(&mut self, _plugin: &EcsPlugin) {
        // Add the World resource
        self.insert_resource(World::new());
        
        // Add the Time resource
        self.insert_resource(Time::default());
        
        // todo: implement ECS systems registration
        // For now, this is just a stub
    }
}

/// Resource insertion trait
pub trait InsertResource {
    fn insert_resource<R: 'static>(&mut self, resource: R);
}

// === REGION: TESTS ===
#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use std::any::{Any, TypeId};
    
    // A simple App implementation for testing
    struct TestApp {
        resources: HashMap<TypeId, Box<dyn Any>>,
    }
    
    impl TestApp {
        fn new() -> Self {
            TestApp {
                resources: HashMap::new(),
            }
        }
    }
    
    impl InsertResource for TestApp {
        fn insert_resource<R: 'static>(&mut self, resource: R) {
            self.resources.insert(TypeId::of::<R>(), Box::new(resource));
        }
    }
    
    // Helper method for testing
    impl TestApp {
        fn get_resource<R: 'static>(&self) -> Option<&R> {
            self.resources.get(&TypeId::of::<R>())
                .and_then(|resource| resource.downcast_ref::<R>())
        }
    }
    
    #[test]
    fn test_plugin_creation() {
        let plugin = EcsPlugin::default();
        assert_eq!(plugin.name(), "EcsPlugin");
    }
    
    #[test]
    fn test_plugin_build() {
        let mut app = TestApp::new();
        app.build_ecs_plugin(&EcsPlugin::default());
        
        // Verify the World resource was added
        assert!(app.get_resource::<World>().is_some());
        
        // Verify the Time resource was added
        assert!(app.get_resource::<Time>().is_some());
    }
}
