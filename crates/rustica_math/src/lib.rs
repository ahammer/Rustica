//! # Rustica Math
//! 
//! Math utilities for the Rustica game engine. This crate provides
//! types and functions for 2D and 3D math operations commonly used in games.
//!
//! ## Key Features
//!
//! - Vector types (Vec2, Vec3)
//! - Matrix operations (Mat4)
//! - Quaternion rotations (Quat)
//! - Common math utilities
//!
//! ## Examples
//!
//! ```rust
//! use rustica_math::{Vec3, vec3};
//!
//! // Create a new vector
//! let position = vec3(1.0, 2.0, 3.0);
//! let velocity = vec3(0.1, 0.0, -0.5);
//!
//! // Update position
//! let new_position = position + velocity;
//! assert_eq!(new_position, vec3(1.1, 2.0, 2.5));
//! ```

use std::ops::{Add, Sub, Mul, Div, AddAssign, SubAssign, MulAssign, DivAssign, Neg};
use std::fmt;

mod error;
mod vec2;
pub mod vec3;
mod mat4;
mod quat;
mod plugin;

pub use error::MathError;
pub use vec2::{Vec2, vec2};
pub use vec3::{Vec3, vec3};
pub use mat4::{Mat4, mat4_identity};
pub use quat::{Quat, quat_identity};
pub use plugin::MathPlugin;

/// A small value used for floating point comparisons
pub const EPSILON: f32 = 1e-6;

/// Checks if two floating point values are approximately equal
#[inline]
pub fn approx_eq(a: f32, b: f32) -> bool {
    (a - b).abs() < EPSILON
}

/// Linearly interpolate between two values
#[inline]
pub fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t.clamp(0.0, 1.0)
}

/// Convert degrees to radians
#[inline]
pub fn to_radians(degrees: f32) -> f32 {
    degrees * std::f32::consts::PI / 180.0
}

/// Convert radians to degrees
#[inline]
pub fn to_degrees(radians: f32) -> f32 {
    radians * 180.0 / std::f32::consts::PI
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_approx_eq() {
        assert!(approx_eq(1.0, 1.0));
        assert!(approx_eq(1.0, 1.0 + EPSILON / 2.0));
        assert!(!approx_eq(1.0, 1.1));
    }

    #[test]
    fn test_lerp() {
        assert_eq!(lerp(0.0, 10.0, 0.0), 0.0);
        assert_eq!(lerp(0.0, 10.0, 1.0), 10.0);
        assert_eq!(lerp(0.0, 10.0, 0.5), 5.0);
        
        // Test clamping
        assert_eq!(lerp(0.0, 10.0, 2.0), 10.0);
        assert_eq!(lerp(0.0, 10.0, -1.0), 0.0);
    }

    #[test]
    fn test_angle_conversion() {
        assert!(approx_eq(to_radians(180.0), std::f32::consts::PI));
        assert!(approx_eq(to_degrees(std::f32::consts::PI), 180.0));
    }
}
