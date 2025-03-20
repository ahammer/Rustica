# Rustica Testing Standards

This document outlines the testing approach and standards for the Rustica game engine. Following these guidelines ensures a robust, reliable, and testable codebase.

## Testing Philosophy

1. **Test-Driven Development**: Write tests before implementing features when possible.
2. **High Coverage**: All functionality should have corresponding tests.
3. **Isolation**: Tests should be isolated and not depend on each other.
4. **Speed**: Tests should run quickly to encourage frequent testing.
5. **Headless**: All tests should be able to run without a display.

## Test Types

### Unit Tests

Unit tests verify that individual components work correctly in isolation.

```rust
// Example unit test for a Vector3 implementation
#[test]
fn test_vector3_addition() {
    let v1 = Vec3::new(1.0, 2.0, 3.0);
    let v2 = Vec3::new(4.0, 5.0, 6.0);
    let result = v1 + v2;
    
    assert_eq!(result.x, 5.0);
    assert_eq!(result.y, 7.0);
    assert_eq!(result.z, 9.0);
}
```

#### Unit Test Rules

1. Place unit tests in the same file as the code they test, within a `#[cfg(test)]` module.
2. Test all public functions with various inputs, including edge cases.
3. Mock dependencies to isolate the unit being tested.
4. Use descriptive test names that indicate what is being tested and the expected outcome.

### Integration Tests

Integration tests verify that modules work together correctly.

```rust
// In tests/ecs_integration.rs
use rustica::prelude::*;

#[test]
fn test_entity_component_integration() {
    let mut world = World::new();
    let entity = world.spawn()
        .insert(Position { x: 1.0, y: 2.0, z: 3.0 })
        .insert(Velocity { x: 1.0, y: 0.0, z: 0.0 })
        .id();
    
    let position = world.get::<Position>(entity).unwrap();
    assert_eq!(position.x, 1.0);
    assert_eq!(position.y, 2.0);
    assert_eq!(position.z, 3.0);
}
```

#### Integration Test Rules

1. Place integration tests in a `tests/` directory at the crate root.
2. Test interactions between multiple components or subsystems.
3. Use realistic data and scenarios when possible.
4. Test both successful and error paths.

### System Tests

System tests verify that entire systems work correctly end-to-end.

```rust
// In tests/system_tests.rs
use rustica::prelude::*;

#[test]
fn test_movement_system() {
    // Create a test app with only the necessary plugins
    let mut app = App::new();
    app.add_plugin(CorePlugin);
    
    // Add test resources
    app.insert_resource(Time { delta: 1.0 });
    
    // Spawn test entities
    let entity = app.world_mut().spawn((
        Position { x: 0.0, y: 0.0, z: 0.0 },
        Velocity { x: 1.0, y: 2.0, z: 3.0 },
    )).id();
    
    // Run one update of the movement system
    app.update();
    
    // Verify the position was updated correctly
    let position = app.world().get::<Position>(entity).unwrap();
    assert_eq!(position.x, 1.0);
    assert_eq!(position.y, 2.0);
    assert_eq!(position.z, 3.0);
}
```

#### System Test Rules

1. Create a minimal App with only the necessary plugins and systems.
2. Use a controlled environment with predictable inputs.
3. Verify the end state rather than implementation details.
4. Test main user flows and scenarios.

### Headless Testing

All tests must be able to run without a display. This is especially important for rendering-related tests.

```rust
// Example of headless rendering test
#[test]
fn test_renderer_initialization() {
    // Use a headless render plugin for testing
    let mut app = App::new();
    app.add_plugin(HeadlessRenderPlugin::default());
    
    // Initialize the renderer
    app.update();
    
    // Verify renderer state through the API, not actual rendering
    let render_state = app.world().resource::<RenderState>();
    assert!(render_state.is_initialized());
}
```

#### Headless Testing Rules

1. Create mock or headless implementations of hardware-dependent features.
2. Use feature flags to switch between real and mock implementations.
3. Verify behavior through state checks rather than visual output.
4. Ensure CI environments can run all tests without a display.

## Test Organization

### Directory Structure

```
crates/rustica_core/
├── src/
│   ├── lib.rs              # Contains unit tests in #[cfg(test)] modules
│   └── app.rs              # Contains unit tests for App implementation
├── tests/                  # Integration tests
│   ├── app_lifecycle.rs    # Tests for App lifecycle
│   └── plugin_system.rs    # Tests for plugin registration and execution
```

### Test Utilities

Create test utilities to simplify common testing patterns:

```rust
// In src/testing.rs
/// Create a test app with predictable time steps
pub fn create_test_app() -> App {
    let mut app = App::new();
    app.insert_resource(Time { delta: 1.0, ..Default::default() });
    app.add_plugin(TestPlugin);
    app
}

/// Run the app for a specific number of frames
pub fn run_frames(app: &mut App, frames: usize) {
    for _ in 0..frames {
        app.update();
    }
}
```

## Code Coverage

Aim for high code coverage, with a minimum of 80% line coverage.

### Coverage Rules

1. Run coverage reports regularly using tools like `cargo tarpaulin` or `grcov`.
2. Focus on testing complex logic and error paths.
3. Document any intentionally untested code.
4. Add tests for any bugs discovered.

## Mocking

Use mocks to isolate units being tested:

```rust
// Example of a mock renderer for testing
struct MockRenderer {
    draw_calls: Vec<DrawCall>,
}

impl Renderer for MockRenderer {
    fn draw(&mut self, draw_call: DrawCall) {
        self.draw_calls.push(draw_call);
    }
    
    fn present(&mut self) -> Result<(), RenderError> {
        // Do nothing in tests
        Ok(())
    }
}

#[test]
fn test_sprite_rendering() {
    let mut mock_renderer = MockRenderer { draw_calls: Vec::new() };
    
    // Test code that uses the renderer
    render_sprite(&mut mock_renderer, sprite);
    
    // Verify the renderer received the expected draw calls
    assert_eq!(mock_renderer.draw_calls.len(), 1);
    assert_eq!(mock_renderer.draw_calls[0].texture_id, sprite.texture_id);
}
```

### Mocking Rules

1. Create mock implementations of interfaces for testing.
2. Focus on verifying the interactions rather than internal details.
3. Use clear assertions that document the expected behavior.

## Benchmarking

Performance-critical code should have benchmarks:

```rust
#[bench]
fn bench_entity_creation(b: &mut Bencher) {
    let mut world = World::new();
    
    b.iter(|| {
        for _ in 0..1000 {
            world.spawn((
                Position { x: 0.0, y: 0.0, z: 0.0 },
                Velocity { x: 0.0, y: 0.0, z: 0.0 },
            ));
        }
        world.clear();
    });
}
```

### Benchmarking Rules

1. Focus on performance-critical paths.
2. Use realistic workloads and data sizes.
3. Compare against previous versions to detect regressions.
4. Document performance characteristics and expectations.

## Continuous Integration

All tests should run in CI:

```yaml
# Example GitHub Actions workflow for testing
name: Tests

on:
  push:
    branches: [ main ]
  pull_request:
    branches: [ main ]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
    - uses: actions/checkout@v2
    - name: Install Rust
      uses: actions-rs/toolchain@v1
      with:
        profile: minimal
        toolchain: stable
    - name: Run tests
      run: cargo test --workspace
    - name: Run headless tests
      run: cargo test --workspace --features headless
```

### CI Rules

1. Run tests on all pull requests and pushes to main branches.
2. Include both regular and headless tests.
3. Run tests on multiple platforms if possible.
4. Enforce code coverage minimums.

## Test-Driven Development Process

1. Write a failing test that defines the expected behavior.
2. Implement the minimal code to make the test pass.
3. Refactor while ensuring tests continue to pass.
4. Repeat for each new feature or bug fix.

## Testing External Dependencies

For code that interacts with external dependencies:

```rust
#[test]
fn test_texture_loading() {
    // Create a test file
    let test_file = tempfile::NamedTempFile::new().unwrap();
    // Write test data to the file
    // ...
    
    // Test loading the texture
    let result = load_texture(test_file.path());
    assert!(result.is_ok());
    
    let texture = result.unwrap();
    assert_eq!(texture.width, expected_width);
    assert_eq!(texture.height, expected_height);
}
```

### External Dependency Testing Rules

1. Use temporary files or in-memory implementations when possible.
2. Mock external services to avoid network dependencies.
3. Test both successful and failure cases.
4. Use environment variables or configuration to point to test resources.

## Conclusion

Following these testing standards helps ensure that Rustica remains robust and reliable. Remember:

1. Write tests as you develop.
2. Test all functionality, especially error paths.
3. Make tests work without a display for CI.
4. Use mocks to isolate units being tested.
5. Document test patterns and utilities.
