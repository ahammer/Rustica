# Rustica Core Module Guide

This document contains the specific guidelines and rules for the `rustica_core` module, which serves as the minimal orchestration layer for the Rustica game engine.

## Module Responsibility

The core module has a very specific and limited responsibility:

1. Provide the Plugin trait interface for extending engine functionality
2. Manage the App lifecycle and plugin registration
3. Act as the central orchestration point for the engine
4. Provide minimal dependency resolution between plugins

The core module explicitly does NOT:

1. Implement ECS functionality (handled by rustica_ecs)
2. Manage rendering (handled by rustica_render)
3. Handle event dispatching (handled by rustica_event)
4. Implement any game-specific logic

## Public API Guidelines

### App Structure

The `App` struct is the central structure in the core module:

```rust
pub struct App {
    // Contains registries for plugins and other engine resources
    // but minimal implementation details
}

impl App {
    pub fn new() -> Self;
    pub fn add_plugin<P: Plugin>(&mut self, plugin: P) -> &mut Self;
    pub fn run(&mut self);
    pub fn update(&mut self);
}
```

### Plugin Interface

The `Plugin` trait defines the interface for extending the engine:

```rust
pub trait Plugin {
    fn build(&self, app: &mut App);
    
    // Optional methods with default implementations
    fn name(&self) -> &str {
        std::any::type_name::<Self>()
    }
    
    fn dependencies(&self) -> Vec<&'static str> {
        Vec::new()
    }
}
```

## Implementation Guidelines

### Minimal Dependencies

The core module should have minimal external dependencies. It should only depend on:

1. Standard Rust libraries
2. Essential utilities (e.g., log, thiserror)
3. NO graphics libraries
4. NO physics libraries
5. NO specific ECS implementations

### Error Handling

Core errors should be well-defined and comprehensive:

```rust
#[derive(Debug, thiserror::Error)]
pub enum CoreError {
    #[error("Plugin error: {0}")]
    PluginError(String),
    
    #[error("Duplicate plugin: {0}")]
    DuplicatePlugin(String),
    
    #[error("Missing plugin dependency: {0} requires {1}")]
    MissingDependency(String, String),
}
```

### Plugin Registration

Plugin registration should check for dependencies and prevent duplicates:

```rust
fn register_plugin<P: Plugin>(&mut self, plugin: P) -> Result<(), CoreError> {
    let name = plugin.name();
    
    // Check if plugin already registered
    if self.plugins.contains_key(name) {
        return Err(CoreError::DuplicatePlugin(name.to_string()));
    }
    
    // Check dependencies
    for dep in plugin.dependencies() {
        if !self.plugins.contains_key(dep) {
            return Err(CoreError::MissingDependency(name.to_string(), dep.to_string()));
        }
    }
    
    // Register plugin
    plugin.build(self);
    self.plugins.insert(name.to_string(), Box::new(plugin));
    
    Ok(())
}
```

## Testing Guidelines

### Unit Testing

Unit tests should verify:

1. App creation and initialization
2. Plugin registration
3. Plugin dependency resolution
4. App lifecycle methods

### Integration Testing

Integration tests should verify:

1. Multiple plugin interactions
2. Complete app lifecycle
3. Error handling in edge cases

## Performance Considerations

The core module should be lightweight and efficient:

1. Plugin registration is a one-time cost during initialization
2. App update loop should defer to subsystems for actual processing
3. Minimize allocations in hot paths

## Documentation Requirements

Each public item must be documented with:

1. A clear description of its purpose
2. Examples of usage
3. Parameter and return value explanations
4. Any constraints or invariants

## Typical Usage Pattern

```rust
use rustica_core::prelude::*;

struct MyPlugin;

impl Plugin for MyPlugin {
    fn build(&self, app: &mut App) {
        // Configure the app
    }
}

fn main() {
    let mut app = App::new();
    app.add_plugin(MyPlugin)
       .run();
}
```
