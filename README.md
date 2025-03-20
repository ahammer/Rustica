# Rustica Game Engine

Rustica is a modular, testable game engine written in Rust. It provides a clean, flexible architecture for building games with a strong focus on testability and abstraction.

## Project Structure

```
rustica/ (workspace)
├── Cargo.toml                   // Workspace manifest
├── README.md                    // This file
├── AGENT_GUIDE.md               // Guide for AI agents working with the codebase
├── crates/
│   ├── rustica/                 // Main library crate (re-exports)
│   ├── rustica_core/            // Minimal orchestration layer
│   ├── rustica_ecs/             // ECS implementation
│   │   └── ...                  // (Future: event, render, scheduler, math, etc.)
├── examples/
│   └── starfield/               // Hello World starfield example
└── docs/
    ├── architecture.md          // Architecture overview
    ├── api_conventions.md       // API design rules
    ├── testing_standards.md     // Testing requirements
    └── implementation_rules.md  // Implementation guidelines
```

## Design Philosophy

Rustica is built on these core principles:

1. **Minimal Core**: The core module provides only orchestration and plugin management
2. **Modularity**: All functionality is implemented in independent subsystems
3. **Plugin-Based**: New functionality is added through the plugin system
4. **Data-Oriented**: Focus on data layouts and transformations over object hierarchies
5. **Testability**: All components are designed for testing without dependencies

## Getting Started

### Using the Engine

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

### Running the Example

```bash
# Run the starfield example
cargo run --bin starfield
```

## Documentation

Comprehensive documentation is available in the `docs/` directory:

- [Architecture Overview](docs/architecture.md): System design and component relationships
- [API Conventions](docs/api_conventions.md): Rules for API design
- [Testing Standards](docs/testing_standards.md): Testing requirements and patterns
- [Implementation Rules](docs/implementation_rules.md): Guidelines for implementation

Each subsystem (crate) also has its own `MODULE_GUIDE.md` file with specific guidelines:

- [Core Module Guide](crates/rustica_core/MODULE_GUIDE.md)
- [ECS Module Guide](crates/rustica_ecs/MODULE_GUIDE.md)
- [Main Module Guide](crates/rustica/MODULE_GUIDE.md)

## Status

This project is in the early scaffolding stage. The current implementation provides:

- Core orchestration layer (plugin system, application lifecycle)
- ECS subsystem interfaces
- Main library module that re-exports functionality
- Starfield example demonstrating the API

Future work will implement the subsystem functionality and add additional features.

## License

MIT License
