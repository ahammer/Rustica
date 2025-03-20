# Rustica API Conventions

This document outlines the API design rules and conventions for the Rustica game engine. Following these conventions ensures consistency, predictability, and maintainability across the codebase.

## General Principles

### Rule 1: Public API Minimalism
Expose only what's necessary in the public API. Implementation details should remain private.

```rust
// GOOD
// Public interface is clear and minimal
pub struct Transform {
    // Public fields are part of the stable API
    pub position: Vec3,
    pub rotation: Quat,
    pub scale: Vec3,
}

// BAD
// Exposing implementation details
pub struct TransformData {
    pub position: Vec3,
    pub rotation: Quat,
    pub scale: Vec3,
    pub world_matrix: Mat4, // Implementation detail that should be private
    pub local_matrix: Mat4, // Implementation detail that should be private
}
```

### Rule 2: Error Handling Pattern
All functions that can fail must return `Result<T, ErrorType>` and never panic in regular operation.

```rust
// GOOD
pub fn load_texture(path: &str) -> Result<TextureHandle, AssetError> {
    // Implementation with proper error handling
}

// BAD
pub fn load_texture(path: &str) -> TextureHandle {
    // Implementation that might panic
    let data = std::fs::read(path).unwrap(); // Bad: panics on failure
    // ...
}
```

### Rule 3: Consistent Naming Conventions

- Types/traits: `PascalCase` (e.g., `Entity`, `Component`, `RenderPlugin`)
- Functions/methods: `snake_case` (e.g., `add_plugin`, `spawn_entity`)
- Constants: `SCREAMING_SNAKE_CASE` (e.g., `MAX_ENTITIES`)
- Module names: `snake_case` (e.g., `transform`, `rendering`)

```rust
// GOOD
pub struct RenderPlugin;
pub fn add_system(system: impl System);
pub const MAX_ENTITIES: usize = 1000000;

// BAD
pub struct renderPlugin; // Should be PascalCase
pub fn AddSystem(system: impl System); // Should be snake_case
pub const maxEntities: usize = 1000000; // Should be SCREAMING_SNAKE_CASE
```

### Rule 4: Method Chaining
Public builders and configuration methods should support method chaining for ergonomic usage.

```rust
// GOOD
let app = App::new()
    .add_plugin(CorePlugin::default())
    .add_plugin(RenderPlugin::default())
    .add_resource(WindowConfig { width: 800, height: 600 })
    .run();

// BAD
let mut app = App::new();
app.add_plugin(CorePlugin::default());
app.add_plugin(RenderPlugin::default());
app.add_resource(WindowConfig { width: 800, height: 600 });
app.run();
```

## Plugin API Conventions

### Rule 5: Plugin Registration
All plugins must implement the `Plugin` trait and follow a consistent registration pattern.

```rust
// GOOD
pub struct MyPlugin;

impl Plugin for MyPlugin {
    fn build(&self, app: &mut App) {
        // Register components, systems, etc.
        app.register_component::<Position>()
           .add_system(update_position);
    }
}

// Usage
app.add_plugin(MyPlugin);
```

### Rule 6: Plugin Configuration
Plugins that require configuration should use builder patterns or configuration structs.

```rust
// GOOD
pub struct RenderPluginConfig {
    pub vsync: bool,
    pub msaa_samples: u8,
}

impl Default for RenderPluginConfig {
    fn default() -> Self {
        Self {
            vsync: true,
            msaa_samples: 4,
        }
    }
}

pub struct RenderPlugin {
    config: RenderPluginConfig,
}

impl RenderPlugin {
    pub fn new(config: RenderPluginConfig) -> Self {
        Self { config }
    }
    
    pub fn with_vsync(mut self, enabled: bool) -> Self {
        self.config.vsync = enabled;
        self
    }
    
    pub fn with_msaa(mut self, samples: u8) -> Self {
        self.config.msaa_samples = samples;
        self
    }
}

impl Default for RenderPlugin {
    fn default() -> Self {
        Self::new(RenderPluginConfig::default())
    }
}

impl Plugin for RenderPlugin {
    fn build(&self, app: &mut App) {
        // Implementation using self.config
    }
}
```

## ECS API Conventions

### Rule 7: Component Design
Components should be data-only structures without behavior.

```rust
// GOOD
#[derive(Component)]
pub struct Position {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

// BAD
#[derive(Component)]
pub struct PositionWithBehavior {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    
    // Bad: Component with behavior
    pub fn move_forward(&mut self, distance: f32) {
        self.z += distance;
    }
}
```

### Rule 8: System Parameter Types
System parameters should be explicit about mutability and access patterns.

```rust
// GOOD
fn update_position(
    time: Res<Time>,
    mut query: Query<(&mut Position, &Velocity)>,
) {
    for (mut position, velocity) in query.iter_mut() {
        position.x += velocity.x * time.delta_seconds();
        position.y += velocity.y * time.delta_seconds();
        position.z += velocity.z * time.delta_seconds();
    }
}

// BAD - Requesting mutable access when not needed
fn read_only_system(
    mut time: ResMut<Time>, // Bad: Requesting mutable access but not modifying
    mut query: Query<&Position>, // Bad: Using mut when not needed
) {
    // ...
}
```

## Event API Conventions

### Rule 9: Event Naming
Events should be named with a clear noun or noun phrase describing what happened.

```rust
// GOOD
pub struct CollisionEvent {
    pub entity_a: Entity,
    pub entity_b: Entity,
    pub contact_point: Vec3,
}

// BAD - Unclear name
pub struct ThingHappened {
    // ...
}
```

### Rule 10: Event Reading
Event readers should be explicit about consumption.

```rust
// GOOD
fn process_collisions(
    mut collision_events: EventReader<CollisionEvent>,
    mut sound_system: ResMut<SoundSystem>,
) {
    for collision in collision_events.read() {
        sound_system.play_sound("collision.wav", collision.contact_point);
    }
}
```

## Documentation Conventions

### Rule 11: Public API Documentation
All public items must be documented with examples.

```rust
/// A 3D position component.
///
/// This component represents the position of an entity in 3D space.
///
/// # Example
///
/// ```
/// # use rustica::prelude::*;
/// let mut world = World::new();
/// let entity = world.spawn()
///     .insert(Position { x: 0.0, y: 0.0, z: 0.0 })
///     .id();
/// ```
#[derive(Component)]
pub struct Position {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
```

### Rule 12: Internal Documentation
Internal code should include explanatory comments for complex logic.

```rust
// GOOD
// Calculate the distance using a fast approximate square root
// This is about 30% faster than std::f32::sqrt but has a maximum
// error of 0.001 for the range we care about
fn fast_distance(a: Vec3, b: Vec3) -> f32 {
    let dx = b.x - a.x;
    let dy = b.y - a.y;
    let dz = b.z - a.z;
    let square_sum = dx * dx + dy * dy + dz * dz;
    fast_inverse_sqrt(square_sum)
}
```

## API Stability Rules

### Rule 13: Breaking Changes
Breaking changes must follow Rust's SemVer rules and be clearly documented.

- Major version bumps (e.g., 1.0.0 -> 2.0.0) can include breaking changes
- Minor and patch versions must maintain backward compatibility
- Deprecated functionality should be marked with `#[deprecated]` before removal

### Rule 14: Feature Flags
Optional functionality should be behind feature flags.

```toml
# In Cargo.toml
[features]
default = ["renderer"]
renderer = ["wgpu", "winit"]
headless = []
```

```rust
// In code
#[cfg(feature = "renderer")]
pub mod rendering {
    // Rendering code here
}

#[cfg(feature = "headless")]
pub mod headless {
    // Headless mode implementation
}
```

## Performance Guidelines

### Rule 15: Avoid Allocations in Hot Paths
Systems that run frequently should avoid allocations.

```rust
// GOOD
fn update_positions(mut query: Query<(&mut Position, &Velocity)>) {
    for (mut position, velocity) in query.iter_mut() {
        position.x += velocity.x;
        position.y += velocity.y;
        position.z += velocity.z;
    }
}

// BAD - Allocating in a hot path
fn update_positions_bad(mut query: Query<(&mut Position, &Velocity)>) {
    let positions: Vec<_> = query.iter_mut().collect(); // Unnecessary allocation
    for (mut position, velocity) in positions {
        position.x += velocity.x;
        position.y += velocity.y;
        position.z += velocity.z;
    }
}
```

### Rule 16: Batched Operations
Prefer batched operations over individual ones for performance.

```rust
// GOOD
fn spawn_many_entities(mut commands: Commands) {
    commands.spawn_batch((0..1000).map(|i| {
        (
            Position { x: i as f32, y: 0.0, z: 0.0 },
            Velocity { x: 0.0, y: 0.0, z: 0.0 },
        )
    }));
}

// BAD - Individual spawns in a loop
fn spawn_many_entities_bad(mut commands: Commands) {
    for i in 0..1000 {
        commands.spawn((
            Position { x: i as f32, y: 0.0, z: 0.0 },
            Velocity { x: 0.0, y: 0.0, z: 0.0 },
        ));
    }
}
```
