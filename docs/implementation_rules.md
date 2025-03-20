# Rustica Implementation Rules

This document outlines the implementation guidelines and rules for the Rustica game engine. These rules help maintain code quality, performance, and maintainability.

## Code Structure Guidelines

### Rule 1: Module Organization

Each module should have a clear, single responsibility:

```
crates/rustica_ecs/
├── src/
│   ├── lib.rs           # Public exports and documentation
│   ├── entity.rs        # Entity type and related functions
│   ├── component.rs     # Component trait and storage
│   ├── world.rs         # World container for entities/components
│   └── query.rs         # Query system for accessing components
```

### Rule 2: Public API Structure

Each crate's `lib.rs` file should clearly define its public API:

```rust
//! # Rustica ECS
//!
//! Entity-Component-System implementation for the Rustica game engine.

// Re-export public items
pub use crate::entity::{Entity, EntityBuilder};
pub use crate::component::{Component, ComponentStorage};
pub use crate::world::{World, WorldBuilder};
pub use crate::query::{Query, QueryIter};

// Module definitions
mod entity;
mod component;
mod world;
mod query;

// Public modules
pub mod prelude;
```

### Rule 3: Prelude Module

Include a prelude module with the most commonly used types:

```rust
//! Common imports for working with Rustica ECS.

pub use crate::Entity;
pub use crate::Component;
pub use crate::World;
pub use crate::Query;
```

## Performance Guidelines

### Rule 4: Memory Layout Optimization

Optimize memory layout for cache efficiency:

```rust
// GOOD: Data-oriented design with struct-of-arrays
pub struct PositionStorage {
    x: Vec<f32>,
    y: Vec<f32>,
    z: Vec<f32>,
}

// BAD: Array-of-structs design
pub struct PositionStorageBad {
    positions: Vec<Position>, // Each Position is x, y, z fields
}
```

### Rule 5: Avoid Virtual Dispatch in Hot Paths

Prefer static dispatch over dynamic dispatch in performance-critical code:

```rust
// GOOD: Static dispatch with generics
pub fn update_system<T: System>(system: T, world: &mut World) {
    system.run(world);
}

// BAD: Dynamic dispatch with trait objects in hot paths
pub fn update_system_dyn(system: &mut dyn System, world: &mut World) {
    system.run(world);
}
```

### Rule 6: Batch Operations

Perform operations in batches rather than one at a time:

```rust
// GOOD: Batch update
fn update_positions(world: &mut World) {
    let (positions, velocities) = world.query_mut::<(&mut Position, &Velocity)>();
    for (pos, vel) in positions.iter_mut().zip(velocities.iter()) {
        pos.x += vel.x;
        pos.y += vel.y;
        pos.z += vel.z;
    }
}

// BAD: Individual entity updates
fn update_positions_bad(world: &mut World) {
    for entity in world.entities() {
        if let (Some(pos), Some(vel)) = (
            world.get_mut::<Position>(entity),
            world.get::<Velocity>(entity),
        ) {
            pos.x += vel.x;
            pos.y += vel.y;
            pos.z += vel.z;
        }
    }
}
```

## Error Handling

### Rule 7: Error Types

Define clear error types for each module:

```rust
// In render crate
#[derive(Debug, thiserror::Error)]
pub enum RenderError {
    #[error("Failed to create window: {0}")]
    WindowCreationError(String),
    
    #[error("Shader compilation failed: {0}")]
    ShaderCompilationError(String),
    
    #[error("Texture loading failed: {0}")]
    TextureLoadingError(String),
}

// Then use it consistently
pub fn load_texture(path: &str) -> Result<Texture, RenderError> {
    // Implementation
}
```

### Rule 8: Panic vs. Result

Use `Result` for recoverable errors and only panic in truly unrecoverable situations:

```rust
// GOOD: Return Result for operations that might fail
pub fn load_shader(path: &str) -> Result<Shader, RenderError> {
    let source = std::fs::read_to_string(path)
        .map_err(|e| RenderError::ShaderCompilationError(e.to_string()))?;
    
    // Compile shader...
    Ok(Shader { /* ... */ })
}

// Only panic in truly unrecoverable situations or programming errors
fn get_renderer() -> &'static Renderer {
    RENDERER.as_ref().expect("Renderer not initialized. Call initialize_renderer() first.")
}
```

## Safety Guidelines

### Rule 9: Unsafe Code

Minimize unsafe code and document its usage:

```rust
/// Returns a mutable reference to a component for the given entity.
///
/// # Safety
///
/// This function is unsafe because it allows getting multiple mutable
/// references to the same component if called with the same entity
/// multiple times. The caller must ensure this doesn't happen.
pub unsafe fn get_component_unchecked<T: Component>(&mut self, entity: Entity) -> &mut T {
    let storage = self.storages.get_mut::<T>().unwrap_unchecked();
    storage.get_mut(entity.id()).unwrap_unchecked()
}
```

### Rule 10: Thread Safety

Ensure all public types are Send and Sync where appropriate:

```rust
// GOOD: Explicitly mark thread-safety requirements
pub trait System: Send + Sync {
    fn run(&mut self, world: &mut World);
}

// Types with internal mutability should use appropriate synchronization
pub struct SharedResource<T> {
    inner: std::sync::RwLock<T>,
}
```

## Plugin Implementation

### Rule 11: Plugin Modularity

Each plugin should be self-contained:

```rust
pub struct PhysicsPlugin {
    gravity: Vec3,
    iterations: u32,
}

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        // Register all components
        app.register_component::<RigidBody>()
           .register_component::<Collider>();
        
        // Add all systems
        app.add_system(update_physics)
           .add_system(detect_collisions)
           .add_system(resolve_collisions);
        
        // Add resources
        app.insert_resource(PhysicsConfig {
            gravity: self.gravity,
            iterations: self.iterations,
        });
    }
}
```

### Rule 12: Plugin Defaults

Provide sensible defaults for all plugin configurations:

```rust
impl Default for PhysicsPlugin {
    fn default() -> Self {
        Self {
            gravity: Vec3::new(0.0, -9.81, 0.0),
            iterations: 4,
        }
    }
}

// Usage:
app.add_plugin(PhysicsPlugin::default());
// or
app.add_plugin(PhysicsPlugin {
    gravity: Vec3::new(0.0, -1.62, 0.0), // Moon gravity
    ..Default::default()
});
```

## Resource Management

### Rule 13: Resource Acquisition

Resources should be acquired early and released late:

```rust
// GOOD: Acquire in build, release in drop
impl RenderPlugin {
    fn build(&self, app: &mut App) {
        let window = WindowBuilder::new().build().expect("Failed to create window");
        let device = Device::new(&window).expect("Failed to create device");
        
        app.insert_resource(window)
           .insert_resource(device);
    }
}

// Resources are automatically dropped when the app is dropped
```

### Rule 14: Resource Cleanup

Provide explicit cleanup methods for resources that need it:

```rust
impl Drop for RenderBackend {
    fn drop(&mut self) {
        // Explicit cleanup
        self.device.wait_idle().expect("Failed to wait for device idle");
        // Release resources in the correct order
        self.pipelines.clear();
        self.buffers.clear();
        // etc.
    }
}
```

## System Implementation

### Rule 15: System Parameters

Be explicit about system parameter types and access patterns:

```rust
// GOOD: Clear parameter types
fn update_physics(
    time: Res<Time>,              // Read-only resource
    config: Res<PhysicsConfig>,   // Read-only resource
    mut positions: Query<&mut Position>,  // Mutable component access
    velocities: Query<&Velocity>,  // Read-only component access
) {
    // Implementation
}
```

### Rule 16: System Registration

Register systems with explicit dependencies:

```rust
fn register_systems(app: &mut App) {
    app.add_system(update_physics)
       .add_system(detect_collisions.after(update_physics))
       .add_system(resolve_collisions.after(detect_collisions));
}
```

## Memory Management

### Rule 17: Arena Allocation

Use arena allocation for frequently allocated objects:

```rust
// GOOD: Arena allocation for entities
pub struct World {
    entities: EntityAllocator,
    components: ComponentStorage,
}

impl World {
    pub fn spawn(&mut self) -> Entity {
        self.entities.allocate()
    }
    
    pub fn despawn(&mut self, entity: Entity) {
        self.entities.deallocate(entity);
        self.components.remove_all(entity);
    }
}
```

### Rule 18: Custom Allocators

Consider custom allocators for performance-critical parts:

```rust
// Example using a custom allocator for component storage
#[cfg(feature = "custom_allocator")]
pub struct ComponentVec<T> {
    data: Vec<T, CustomAllocator>,
}

#[cfg(not(feature = "custom_allocator"))]
pub struct ComponentVec<T> {
    data: Vec<T>,
}
```

## Concurrency

### Rule 19: Parallel Execution

Design systems to support parallel execution:

```rust
// Systems that can run in parallel
fn position_update(mut query: Query<(&mut Position, &Velocity)>) {
    // Only accesses Position and Velocity, can run in parallel with
    // systems that don't modify these components
}

fn animation_update(mut query: Query<&mut Animation>) {
    // Only accesses Animation, can run in parallel with position_update
}
```

### Rule 20: Lock-Free Data Structures

Use lock-free data structures for shared state:

```rust
use crossbeam::queue::ArrayQueue;

// Thread-safe, lock-free event queue
pub struct EventQueue<T> {
    queue: ArrayQueue<T>,
}

impl<T: Clone> EventQueue<T> {
    pub fn new(capacity: usize) -> Self {
        Self {
            queue: ArrayQueue::new(capacity),
        }
    }
    
    pub fn push(&self, event: T) -> Result<(), T> {
        self.queue.push(event)
    }
    
    pub fn pop(&self) -> Option<T> {
        self.queue.pop()
    }
}
```

## Cross-Platform Compatibility

### Rule 21: Platform Abstraction

Abstract platform-specific code:

```rust
// Platform-agnostic interface
pub trait Window {
    fn width(&self) -> u32;
    fn height(&self) -> u32;
    fn set_title(&mut self, title: &str);
    fn is_open(&self) -> bool;
    fn poll_events(&mut self) -> Vec<Event>;
}

// Platform-specific implementations
#[cfg(target_os = "windows")]
mod windows {
    pub struct WindowsWindow { /* ... */ }
    impl super::Window for WindowsWindow { /* ... */ }
}

#[cfg(target_os = "macos")]
mod macos {
    pub struct MacOSWindow { /* ... */ }
    impl super::Window for MacOSWindow { /* ... */ }
}

// Factory function
pub fn create_window(width: u32, height: u32, title: &str) -> Box<dyn Window> {
    #[cfg(target_os = "windows")]
    return Box::new(windows::WindowsWindow::new(width, height, title));
    
    #[cfg(target_os = "macos")]
    return Box::new(macos::MacOSWindow::new(width, height, title));
    
    #[cfg(not(any(target_os = "windows", target_os = "macos")))]
    compile_error!("Unsupported platform");
}
```

### Rule 22: Feature Flags for Platform Support

Use feature flags for platform-specific features:

```toml
# In Cargo.toml
[features]
default = ["vulkan"]
vulkan = ["dep:vulkan-rs"]
metal = ["dep:metal-rs"]
d3d12 = ["dep:d3d12-rs"]
```

```rust
// In code
#[cfg(feature = "vulkan")]
pub mod vulkan;

#[cfg(feature = "metal")]
pub mod metal;

#[cfg(feature = "d3d12")]
pub mod d3d12;

pub fn create_renderer() -> Box<dyn Renderer> {
    #[cfg(feature = "vulkan")]
    return Box::new(vulkan::VulkanRenderer::new());
    
    #[cfg(all(feature = "metal", not(feature = "vulkan")))]
    return Box::new(metal::MetalRenderer::new());
    
    #[cfg(all(feature = "d3d12", not(any(feature = "vulkan", feature = "metal"))))]
    return Box::new(d3d12::D3D12Renderer::new());
    
    #[cfg(not(any(feature = "vulkan", feature = "metal", feature = "d3d12")))]
    compile_error!("At least one rendering backend must be enabled");
}
```

## Optimizations

### Rule 23: Hot-Path Optimization

Optimize hot paths extensively:

```rust
// GOOD: Optimized hot path
#[inline]
pub fn entity_index(entity: Entity) -> usize {
    entity.id() as usize & ENTITY_INDEX_MASK
}

// Mark cold paths to help the compiler
#[cold]
fn handle_error(error: &str) {
    log::error!("Error: {}", error);
    panic!("Critical error: {}", error);
}
```

### Rule 24: Benchmark-Driven Optimization

Only optimize based on benchmarks:

```rust
// Before optimization, write benchmarks
#[bench]
fn bench_entity_iteration(b: &mut Bencher) {
    let world = create_test_world();
    b.iter(|| {
        for entity in world.entities() {
            test::black_box(entity);
        }
    });
}

// Then optimize and verify improvements
```

## Documentation

### Rule 25: Document Performance Characteristics

Document performance characteristics and complexity:

```rust
/// Finds all entities with the specified components.
///
/// # Complexity
///
/// Time: O(min(n_a, n_b)) where n_a and n_b are the number of entities with components A and B.
/// Space: O(min(n_a, n_b)) for the result vector.
///
/// # Examples
///
/// ```
/// let entities = world.find_entities_with::<(Position, Velocity)>();
/// ```
pub fn find_entities_with<T: ComponentQuery>(&self) -> Vec<Entity> {
    // Implementation
}
```

### Rule 26: Document Constraints and Invariants

Document constraints and invariants:

```rust
/// Updates the physics world.
///
/// # Constraints
///
/// - Must be called exactly once per frame.
/// - Must be called before `render()`.
/// - The time step should ideally be constant for stability.
///
/// # Examples
///
/// ```
/// physics.update(1.0 / 60.0);
/// ```
pub fn update(&mut self, dt: f32) {
    // Implementation
}
```

## Conclusion

Follow these implementation rules to ensure that Rustica remains high-quality, performant, and maintainable. Remember:

1. Prioritize clear, well-structured code
2. Optimize based on measurements
3. Document thoroughly, especially invariants and performance characteristics
4. Design for testability and modularity
5. Use Rust's type system to enforce correctness at compile time
