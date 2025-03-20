//! Plugin implementation for integrating scheduler functionality with the engine.

use rustica_core::{App, Plugin};
use crate::Schedule;

/// Plugin that provides scheduler functionality to the Rustica engine.
///
/// This plugin registers a schedule resource with the app, which is used
/// to manage and execute systems.
#[derive(Debug, Default)]
pub struct SchedulerPlugin;

impl Plugin for SchedulerPlugin {
    fn build(&self, app: &mut App) {
        // Add the schedule resource to the app
        app.add_resource(Schedule::default());
    }

    fn name(&self) -> &str {
        "SchedulerPlugin"
    }
}
