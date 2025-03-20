# Rustica Main Module Guide

This document contains the specific guidelines and rules for the main `rustica` crate, which serves as the entry point to the Rustica game engine.

## Module Responsibility

The main Rustica crate is responsible for:

1. Re-exporting functionality from all subsystem crates
2. Providing a clean, unified API surface
3. Managing dependencies between subsystems
4. Offering a convenient prelude import

The main crate explicitly does NOT:

1. Implement any engine functionality directly
2. Include game-specific logic
3. Have complex initialization code

## Public API Guidelines

### Prelude Design

The prelude module should include only the most commonly used types:

```rust
use rustica::prelude::*;

// This should bring in exactly what's needed for typical usage:
// - Core App and Plugin types
// - ECS fundamentals
// - Common math types
// - Other frequently used types
```

### Import Patterns

Users should have multiple ways to import functionality:

```rust
// 1. Use the prelude for the most common types
use rustica::prelude::*;

// 2. Import specific subsystems
use rustica::core::{App, Plugin};
use rustica::ecs::{World, Entity};

// 3. Qualify imports to avoid name conflicts
use rustica::math::Vec3;
// vs.
use other_lib::Vec3 as OtherVec3;
```

## Feature Flag Guidelines

1. All subsystems besides core and ecs should be optional
2. The default configuration should be minimal
3. A "full" feature should enable all subsystems
4. Each subsystem should have its own feature flag

## Testing Guidelines

1. Test that imports work correctly
2. Test that feature flags correctly enable/disable subsystems
3. Test integration between subsystems
4. Ensure examples compile and run with different feature sets

## Documentation Requirements

1. Document the purpose of each subsystem
2. Provide examples of common use patterns
3. Clearly indicate which features enable which functionality
4. Include guidance for optimal usage

## Typical Usage Pattern

```rust
use rustica::prelude::*;

// Create a simple game
fn main() {
    // Create the application
    let mut app = App::new();
    
    // Add core plugins
    app.add_plugin(EcsPlugin::default());
    
    // Add your game plugin
    app.add_plugin(MyGamePlugin);
    
    // Run the application
    app.run();
}

// Define a game plugin
struct MyGamePlugin;

impl Plugin for MyGamePlugin {
    fn build(&self, app: &mut App) {
        // Setup your game here
    }
}
```
