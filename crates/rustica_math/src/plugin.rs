//! Plugin implementation for integrating math functionality with the engine.

use rustica_core::{App, Plugin};

/// Plugin that provides math functionality to the Rustica engine.
///
/// This plugin registers math resources and systems with the app.
#[derive(Debug, Default)]
pub struct MathPlugin;

impl Plugin for MathPlugin {
    fn build(&self, app: &mut App) {
        // Currently, the math module only provides types and functions,
        // without needing to register any resources or systems.
        // This plugin exists to provide a consistent interface with the
        // engine's plugin system.
    }

    fn name(&self) -> &str {
        "MathPlugin"
    }
}
