# Rustica Core

The Core crate is the central engine component for the Rustica engine.

## Overview

Rustica Core is currently a placeholder for the future core engine implementation. It will eventually serve as the central component that ties together all other parts of the engine, providing a unified API for game and application development.

## Current Status

This crate is in early development and currently serves as a placeholder. Future versions will include:

- **Scene Management**: Hierarchical scene graph for organizing game objects
- **Entity Component System**: Flexible ECS architecture for game logic
- **Asset Management**: Loading and managing game assets
- **Physics Integration**: Interface for physics simulations
- **Audio System**: Sound playback and spatial audio

## Planned Usage

```rust
// This is a preview of the planned API (not yet implemented)
use rustica_core::{Engine, Scene, Entity};
use rustica_graphics::primitives::sphere::create_default_sphere;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the engine
    let mut engine = Engine::new()?;
    
    // Create a scene
    let mut scene = Scene::new();
    
    // Create an entity with a mesh component
    let sphere_entity = Entity::new()
        .with_mesh(create_default_sphere())
        .with_transform(Transform::from_position([0.0, 0.0, 0.0]));
    
    // Add the entity to the scene
    scene.add_entity(sphere_entity);
    
    // Run the engine with the scene
    engine.run(scene)?;
    
    Ok(())
}
```

## Integration

The Core crate will integrate with:
- **Foundation Crate**: For basic data structures and utilities
- **Render Crate**: For rendering capabilities
- **Graphics Crate**: For high-level graphics primitives
- **Window Crate**: For window management
- **Extensions Crate**: For optional engine extensions
