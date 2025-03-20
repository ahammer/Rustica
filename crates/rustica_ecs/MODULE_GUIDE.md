# Rustica ECS Module Guide

This document contains the specific guidelines and rules for the `rustica_ecs` module, which provides the Entity-Component-System implementation for the Rustica game engine.

## Module Responsibility

The ECS module is responsible for:

1. Entity management (creation, deletion, lifecycle)
2. Component storage and access
3. Query system for efficient component access
4. Component traits and registration

The ECS module explicitly does NOT:

1. Handle rendering or graphics (rustica_render)
2. Manage input (future input module)
3. Control application lifecycle (rustica_core)
4. Implement game-specific logic (user code)

## Public API Guidelines

### World Structure

The `World` struct is the main entry point for ECS operations:

```rust
pub struct World {
    // Implementation details hidden
}

impl World {
    pub fn new() -> Self;
    pub fn spawn(&mut self) -> EntityBuilder;
    pub fn despawn(&mut self, entity: Entity);
    pub fn get<T: Component>(&self, entity: Entity) -> Option<&T>;
    pub fn get_mut<T: Component>(&mut self, entity: Entity) -> Option<&mut T>;
    pub fn query<Q: Query>(&self) -> QueryResult<Q>;
    pub fn query_mut<Q: QueryMut>(&mut self) -> QueryResultMut<Q>;
}
```

### Entity API

Entities are lightweight identifiers:

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Entity(u64);

impl Entity {
    pub fn id(&self) -> u64;
}
```

### Component API

Components are plain data types:

```rust
pub trait Component: 'static + Send + Sync {}

// Auto-implementation for eligible types
impl<T: 'static + Send + Sync> Component for T {}
```

### Query System

The query system enables efficient access to components:

```rust
pub trait Query {
    type Item;
}

impl<T: Component> Query for &T {
    type Item = &T;
}

pub struct QueryResult<Q: Query> {
    // Implementation details hidden
}
```

## Implementation Guidelines

### Performance Considerations

1. Storage layout should be optimized for cache locality
2. Queries should minimize runtime overhead
3. Component access should be inlined in hot paths
4. Entity creation/deletion should be efficient

### Thread Safety

1. Components must be `Send + Sync`
2. The world should support parallel queries where possible
3. Mutable access must be carefully controlled to prevent data races

### Memory Management

1. Use generational indices for entities to prevent use-after-free
2. Recycle entity IDs to avoid exhaustion
3. Consider using custom allocators for component storage
4. Support sparse component storage for memory efficiency

## Testing Guidelines

### Unit Testing

1. Test entity creation/deletion
2. Test component access
3. Test query system
4. Test edge cases like missing components

### Integration Testing

1. Test with multiple component types
2. Test with large numbers of entities
3. Test with complex queries
4. Test parallel access patterns

## Documentation Requirements

Each public item must be documented with:

1. A clear description of its purpose
2. Examples of usage
3. Performance characteristics
4. Thread safety guarantees

## Typical Usage Pattern

```rust
use rustica_ecs::prelude::*;

// Define components
#[derive(Debug)]
struct Position {
    x: f32,
    y: f32,
    z: f32,
}

#[derive(Debug)]
struct Velocity {
    x: f32,
    y: f32,
    z: f32,
}

// Create a world
let mut world = World::new();

// Spawn entities with components
let entity = world.spawn()
    .insert(Position { x: 0.0, y: 0.0, z: 0.0 })
    .insert(Velocity { x: 1.0, y: 0.0, z: 0.0 })
    .id();

// Query components
for (position, velocity) in world.query::<(&mut Position, &Velocity)>() {
    position.x += velocity.x;
    position.y += velocity.y;
    position.z += velocity.z;
}
```
