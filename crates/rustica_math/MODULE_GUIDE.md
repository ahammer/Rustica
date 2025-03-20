# Rustica Math Module Guide

This guide provides information about the `rustica_math` module, which implements math utilities for the Rustica game engine.

## Purpose

The math module provides fundamental mathematics functionality for 3D graphics and physics:

- Vector operations (2D and 3D)
- Matrix operations (4x4 transformation matrices)
- Quaternion operations for rotations
- Utility functions for common math operations

## Code Organization

The `rustica_math` module is organized as follows:

```
rustica_math/
├── src/
│   ├── lib.rs           # Main entry point, re-exports, utility functions
│   ├── error.rs         # Error types
│   ├── vec2.rs          # 2D vector implementation
│   ├── vec3.rs          # 3D vector implementation
│   ├── mat4.rs          # 4x4 matrix implementation
│   ├── quat.rs          # Quaternion implementation
│   └── plugin.rs        # Plugin implementation for engine integration
```

## Key Types and Functionality

### Vector Types

- `Vec2`: 2D vector with x and y components
- `Vec3`: 3D vector with x, y, and z components

Both vector types provide common operations:
- Vector arithmetic (add, subtract, multiply, divide)
- Dot product, cross product (Vec3 only)
- Normalization
- Distance calculations
- Linear interpolation

### Matrix Operations

- `Mat4`: 4x4 matrix for 3D transformations
  - Translation, rotation, and scaling transformations
  - Perspective and orthographic projections
  - View matrices (look-at)
  - Matrix inversion and multiplication

### Quaternions

- `Quat`: Quaternion for representing 3D rotations
  - Creation from axis-angle, Euler angles, and matrices
  - Rotation operations
  - Spherical linear interpolation (slerp)
  - Conversion to/from other rotation formats

### Utility Functions

- Angle conversions (degrees to radians)
- Floating point comparisons
- Linear interpolation

## Usage

```rust
use rustica_math::{Vec3, vec3, Quat, Mat4};

// Create vectors
let position = vec3(1.0, 2.0, 3.0);
let velocity = vec3(0.1, 0.2, 0.3);

// Update position
let new_position = position + velocity;

// Create a rotation
let rotation = Quat::from_axis_angle(vec3(0.0, 1.0, 0.0), 0.5);

// Rotate a vector
let rotated_vector = rotation.rotate_vec3(position);

// Create a transformation matrix
let transform = Mat4::translation(position) * rotation.to_rotation_matrix();
```

## Plugin System Integration

The `MathPlugin` allows integration with the Rustica engine's plugin system, though currently it doesn't register any resources or systems.

```rust
use rustica::prelude::*;

fn main() {
    let mut app = App::new();
    app.add_plugin(MathPlugin::default());
    // ...
}
```

## Development Guidelines

1. **Performance**: Math operations should be optimized for performance, using `#[inline]` for small functions.
2. **Safety**: Handle edge cases (e.g., division by zero, normalization of zero vectors).
3. **Testing**: Maintain comprehensive test coverage for all math operations.
4. **Documentation**: Document all public types and functions with examples.

## Future Improvements

- SIMD optimizations for vector operations
- Additional matrix types (2x2, 3x3)
- Spline interpolation
- Collision detection primitives
