use crate::plugin::SchedulerPlugin;
use rustica_core::{App, Plugin};

#[cfg(test)]
mod plugin_tests {
    use super::*;

    #[test]
    fn test_plugin_name() {
        let plugin = SchedulerPlugin::default();
        assert_eq!(plugin.name(), "SchedulerPlugin", "Plugin should have correct name");
    }

    #[test]
    fn test_plugin_debug() {
        let plugin = SchedulerPlugin::default();
        let debug_str = format!("{:?}", plugin);
        assert!(debug_str.contains("SchedulerPlugin"), "Debug output should contain plugin name");
    }

    #[test]
    fn test_plugin_build() {
        let plugin = SchedulerPlugin::default();
        let mut app = App::new();
        
        // Just test that build doesn't panic
        plugin.build(&mut app);
        
        // Once build is fully implemented, we can add more assertions here
        // to test that it correctly registers the schedule resource
    }

    #[test]
    fn test_plugin_impl_send_sync() {
        // Test that the plugin implements Send and Sync
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<SchedulerPlugin>();
    }
}
