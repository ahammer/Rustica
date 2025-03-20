# Rustica Event System Guide

## Overview

The `rustica_event` crate provides an event handling system for the Rustica game engine. It enables components to communicate via events without having direct dependencies on each other, promoting a decoupled architecture.

## Core Components

### Events

The core of the event system is the `Events<T>` storage container, which manages collections of events of a specific type. The system uses a double-buffering approach where:

- New events are added to a "current" buffer
- During frame updates, the "current" buffer is swapped with an "available" buffer
- Systems can read from the "available" buffer during the frame

### EventReader and EventWriter

- `EventReader<T>`: Provides a way to read events of type T, keeping track of which events have already been read
- `EventWriter<T>`: Provides a way to send events of type T

### Plugin Integration

The `EventPlugin` provides integration with the Rustica engine's plugin system, registering the necessary resources and systems for event handling.

## Usage Examples

### Defining an Event

```rust
// Define a custom event
struct CollisionEvent {
    entity_a: Entity,
    entity_b: Entity,
    position: Vec3,
}

// No need to implement Event trait - it's automatically implemented
// for all types that are Send + Sync + 'static
```

### Sending Events

```rust
// In a system:
fn collision_detection_system(
    entities: Query<(Entity, &Transform, &Collider)>,
    mut collision_events: EventWriter<CollisionEvent>,
) {
    // Detect collisions...
    if let Some(collision) = detect_collision(entity_a, entity_b) {
        collision_events.send(CollisionEvent {
            entity_a,
            entity_b,
            position: collision.position,
        });
    }
}
```

### Reading Events

```rust
// In a system:
fn collision_response_system(
    mut collision_events: EventReader<CollisionEvent>,
    mut query: Query<&mut Health>,
) {
    for collision in collision_events.read() {
        if let Ok(mut health) = query.get_mut(collision.entity_a) {
            health.value -= 10.0;
        }
        // Handle entity_b...
    }
}
```

## Integration with App

```rust
// Add the event plugin to your app
App::new()
    .add_plugin(EventPlugin::default())
    .register_event::<CollisionEvent>()
    .register_event::<GameOverEvent>()
    // ...more setup
    .run();
```

## Best Practices

1. Keep events small and focused on a single purpose
2. Use events for decoupling systems, not for direct communication between tightly coupled components
3. Consider event lifetimes - some events may need to persist across multiple frames
4. Process events in a deterministic order when sequence matters
