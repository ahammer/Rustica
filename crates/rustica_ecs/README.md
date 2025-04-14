# Rustica ECS

An Entity Component System implementation for the Rustica engine.

## Overview

This crate provides a simple and efficient ECS that can be used to organize game logic in a data-oriented way. The ECS is designed to be intuitive to use while maintaining good performance characteristics.

## Key Components

- **Entity**: Just an ID (usize) that serves as a handle to identify game objects
- **Component**: Data attached to entities (implements the `Component` trait)
- **System**: Logic that processes entities and their components (implements the `System` trait)
- **World**: The main container that manages entities, components, and systems

## Usage Example

```rust
use rustica_ecs::prelude::*;

// Define a component
#[derive(Debug)]
struct Position {
    x: f32,
    y: f32,
}

impl Component for Position {}

// Define a system
struct MovementSystem;

impl System for MovementSystem {
    fn run(&self, world: &mut World) {
        // Process entities with Position components
        for (entity, position) in world.query_one::<Position>() {
            println!("Entity {} is at position ({}, {})", 
                     entity, position.x, position.y);
        }
    }
}

fn main() {
    // Create a new world
    let mut world = World::new();
    
    // Register components (optional)
    world.register::<Position>();
    
    // Create an entity with components
    world.create_entity()
        .with(Position { x: 10.0, y: 20.0 })
        .build();
    
    // Add a system
    world.add_system(MovementSystem);
    
    // Run all systems
    world.run_systems();
}
