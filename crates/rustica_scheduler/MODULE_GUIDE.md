# Rustica Scheduler Module Guide

This guide provides information about the `rustica_scheduler` module, which implements system scheduling for the Rustica game engine.

## Purpose

The scheduler module provides functionality for organizing and executing systems:

- System registration and execution
- Dependency tracking between systems
- Execution stages for organizing system execution
- Integration with the ECS and core engine

## Code Organization

The `rustica_scheduler` module is organized as follows:

```
rustica_scheduler/
├── src/
│   ├── lib.rs           # Main entry point, re-exports
│   ├── error.rs         # Error types
│   ├── system.rs        # System trait and implementation
│   ├── schedule.rs      # Schedule for organizing and executing systems
│   ├── stage.rs         # Execution stages for systems
│   └── plugin.rs        # Plugin implementation for engine integration
```

## Key Types and Functionality

### System

- `System` trait: The core trait for systems that can be executed by the scheduler
- `SystemFn<F>`: An implementation of the `System` trait for functions

Systems are the core unit of logic in the engine, operating on the World resource to modify entities and components.

### Schedule

The `Schedule` type manages a collection of systems, organizing them by stage and dependencies, and ensuring they are executed in the correct order.

Key functionality:
- Adding systems to the schedule
- Setting up dependencies between systems
- Running systems in the correct order

### Stage

The `Stage` enum defines the stages of the game loop in which systems can be executed:

1. `Start`: Initial setup
2. `EarlyUpdate`: Pre-update operations
3. `Update`: Main game logic
4. `LateUpdate`: Post-update operations
5. `PreRender`: Prepare for rendering
6. `Render`: Rendering operations
7. `PostRender`: Post-rendering operations
8. `End`: Cleanup operations

## Usage

```rust
use rustica::prelude::*;
use rustica_ecs::World;
use rustica_scheduler::{Schedule, Stage, System};

// Define a system function
fn update_positions(world: &mut World) {
    // Update position components based on velocity components
    for (position, velocity) in world.query::<(&mut Position, &Velocity)>() {
        position.x += velocity.x;
        position.y += velocity.y;
        position.z += velocity.z;
    }
}

// Add the system to a schedule
let mut schedule = Schedule::default();
schedule.add_system(update_positions, "update_positions", Stage::Update)
    .expect("Failed to add system");

// Run the schedule
let mut world = World::new();
schedule.run(&mut world);
```

## Plugin System Integration

The `SchedulerPlugin` allows integration with the Rustica engine's plugin system. It adds a default `Schedule` resource to the app:

```rust
use rustica::prelude::*;

fn main() {
    let mut app = App::new();
    app.add_plugin(SchedulerPlugin::default());
    // ...
}
```

## Development Guidelines

1. **System Organization**: Organize systems into appropriate stages based on their purpose.
2. **Dependencies**: Use the dependency system to ensure systems are executed in the correct order.
3. **Testing**: Test systems independently with mocked worlds to ensure they behave correctly.
4. **Documentation**: Document all systems with their purpose, inputs, and outputs.

## Future Improvements

- Parallel execution of systems (multithreading)
- System groups for more fine-grained control
- Conditional execution of systems
- System parameters to avoid having to pass the entire World
