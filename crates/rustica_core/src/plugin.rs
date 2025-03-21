//! # Plugin System
//! 
//! This module defines the Plugin trait, which is the primary extension
//! mechanism for the Rustica engine.
//!
//! ## AGENT DIRECTIVES
//! 
//! - DOC_ROOT: /docs/
//! - ARCH_DOC: /docs/architecture.md#PluginSystem
//! - API_RULES: /docs/api_conventions.md#PluginAPI
//! 
//! ## Critical Rules
//! 
//! 1. Plugins must be composable and have clear responsibilities
//! 2. Plugins should declare their dependencies explicitly
//! 3. Plugin initialization must be idempotent when possible

// === REGION: IMPORTS ===
use crate::App;
use rustica_common::PluginMetadata;

// === REGION: PLUGIN TRAIT ===

/// The Plugin trait defines the interface for extending the Rustica engine.
///
/// Plugins are used to modularize functionality and extend the engine
/// in a maintainable way. Each plugin can register resources,
/// systems, and other components with the application.
pub trait Plugin: PluginMetadata {
    /// Configures the application with this plugin's functionality.
    ///
    /// This is called when the plugin is added to the application,
    /// allowing it to register resources, systems, and other components.
    fn build(&self, app: &mut App);
}

// === REGION: TESTS ===
#[cfg(test)]
mod tests {
    use super::*;
    use crate::App;
    
    #[test]
    fn test_plugin_name() {
        struct TestPlugin;
        
        impl PluginMetadata for TestPlugin {}
        
        impl Plugin for TestPlugin {
            fn build(&self, _app: &mut App) {}
        }
        
        let plugin = TestPlugin;
        assert_eq!(plugin.name(), "rustica_core::plugin::tests::test_plugin_name::TestPlugin");
    }
    
    #[test]
    fn test_plugin_dependencies() {
        struct TestPlugin;
        
        impl PluginMetadata for TestPlugin {
            fn dependencies(&self) -> Vec<&str> {
                vec!["CorePlugin", "RenderPlugin"]
            }
        }
        
        impl Plugin for TestPlugin {
            fn build(&self, _app: &mut App) {}
        }
        
        let plugin = TestPlugin;
        let deps = plugin.dependencies();
        assert_eq!(deps.len(), 2);
        assert_eq!(deps[0], "CorePlugin");
        assert_eq!(deps[1], "RenderPlugin");
    }
}
