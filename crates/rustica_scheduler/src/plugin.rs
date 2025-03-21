//! Plugin implementation for integrating scheduler functionality with the engine.

use log::info;
use rustica_common::PluginMetadata;
use crate::schedule::Schedule;

/// Plugin that provides scheduler functionality to the Rustica engine.
///
/// This plugin registers a schedule resource with the app, which is used
/// to manage and execute systems.
#[derive(Debug, Default)]
pub struct SchedulerPlugin;

impl PluginMetadata for SchedulerPlugin {
    fn name(&self) -> &str {
        "SchedulerPlugin"
    }
    
    fn dependencies(&self) -> Vec<&str> {
        // The scheduler depends on having an ECS world
        vec!["EcsPlugin"]
    }
}

/// Scheduler plugin implementation.
/// 
/// This is a temporary implementation until we have a proper app trait.
pub trait SchedulerPluginExt {
    fn build_scheduler_plugin(&mut self, plugin: &SchedulerPlugin);
}

impl<T> SchedulerPluginExt for T 
where 
    T: InsertResource,
{
    fn build_scheduler_plugin(&mut self, _plugin: &SchedulerPlugin) {
        info!("Building SchedulerPlugin");
        
        // Create a new schedule
        let schedule = Schedule::new();
        
        // Add the schedule as a resource
        self.insert_resource(schedule);
        
        info!("SchedulerPlugin: Schedule created and added as resource");
    }
}

/// Resource insertion trait
pub trait InsertResource {
    fn insert_resource<R: 'static>(&mut self, resource: R);
}
