use crate::plugin::EventPlugin;
use rustica_core::Plugin;
use rustica_core::App;

#[cfg(test)]
mod plugin_tests {
    use super::*;

    #[test]
    fn test_plugin_creation() {
        let plugin = EventPlugin::new();
        assert_eq!(plugin.name(), "EventPlugin", "Plugin should have correct name");
    }

    #[test]
    fn test_plugin_default() {
        let plugin = EventPlugin::default();
        assert_eq!(plugin.name(), "EventPlugin", "Default plugin should have correct name");
    }

    #[test]
    fn test_plugin_dependencies() {
        let plugin = EventPlugin::new();
        assert!(plugin.dependencies().is_empty(), "Plugin should have no dependencies");
    }

    #[test]
    fn test_plugin_build() {
        let plugin = EventPlugin::new();
        let mut app = App::new();
        
        // Test that build doesn't panic
        plugin.build(&mut app);
        
        // Currently the build method is stubbed out, so there's not much to test.
        // Once implemented, we can test that it registers event resources correctly.
    }
}
