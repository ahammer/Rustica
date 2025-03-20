//! 2D vector implementation

use std::ops::{Add, Sub, Mul, Div, AddAssign, SubAssign, MulAssign, DivAssign, Neg};
use std::fmt;
use crate::error::MathError;
use crate::approx_eq;

/// A 2D vector with x and y components.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

/// Create a new Vec2 with the given x and y components.
#[inline]
pub fn vec2(x: f32, y: f32) -> Vec2 {
    Vec2::new(x, y)
}

impl Vec2 {
    /// Create a new Vec2 with the given x and y components.
    #[inline]
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
    
    /// Create a new Vec2 with both components set to zero.
    #[inline]
    pub fn zero() -> Self {
        Self { x: 0.0, y: 0.0 }
    }
    
    /// Create a new Vec2 with both components set to one.
    #[inline]
    pub fn one() -> Self {
        Self { x: 1.0, y: 1.0 }
    }
    
    /// Create a unit vector pointing along the X axis.
    #[inline]
    pub fn unit_x() -> Self {
        Self { x: 1.0, y: 0.0 }
    }
    
    /// Create a unit vector pointing along the Y axis.
    #[inline]
    pub fn unit_y() -> Self {
        Self { x: 0.0, y: 1.0 }
    }
    
    /// Calculate the length (magnitude) of the vector.
    #[inline]
    pub fn length(&self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }
    
    /// Calculate the squared length of the vector.
    /// This is faster than `length()` when you only need to compare distances.
    #[inline]
    pub fn length_squared(&self) -> f32 {
        self.x * self.x + self.y * self.y
    }
    
    /// Calculate the distance to another vector.
    #[inline]
    pub fn distance(&self, other: Vec2) -> f32 {
        (*self - other).length()
    }
    
    /// Calculate the squared distance to another vector.
    #[inline]
    pub fn distance_squared(&self, other: Vec2) -> f32 {
        (*self - other).length_squared()
    }
    
    /// Normalize the vector to a unit vector (length of 1).
    /// Returns an error if the vector has zero length.
    pub fn normalize(&self) -> Result<Self, MathError> {
        let len = self.length();
        if len == 0.0 {
            return Err(MathError::ZeroVectorNormalize);
        }
        Ok(Self {
            x: self.x / len,
            y: self.y / len,
        })
    }
    
    /// Returns a normalized vector or `Vec2::zero()` if the vector has zero length.
    pub fn normalize_or_zero(&self) -> Self {
        self.normalize().unwrap_or_else(|_| Self::zero())
    }
    
    /// Calculate the dot product with another vector.
    #[inline]
    pub fn dot(&self, other: Vec2) -> f32 {
        self.x * other.x + self.y * other.y
    }
    
    /// Linearly interpolate between this vector and another vector.
    /// The parameter `t` is clamped to the range [0, 1].
    pub fn lerp(&self, other: Vec2, t: f32) -> Self {
        let t = t.clamp(0.0, 1.0);
        *self * (1.0 - t) + other * t
    }
    
    /// Reflect this vector around a normal vector.
    pub fn reflect(&self, normal: Vec2) -> Self {
        *self - normal * 2.0 * self.dot(normal)
    }
    
    /// Returns true if the vector is approximately equal to another vector.
    pub fn approx_eq(&self, other: Vec2) -> bool {
        approx_eq(self.x, other.x) && approx_eq(self.y, other.y)
    }
}

// Implement operator overloads for Vec2

impl Add for Vec2 {
    type Output = Self;
    
    #[inline]
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Vec2 {
    type Output = Self;
    
    #[inline]
    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Mul<f32> for Vec2 {
    type Output = Self;
    
    #[inline]
    fn mul(self, scalar: f32) -> Self {
        Self {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }
}

impl Mul<Vec2> for f32 {
    type Output = Vec2;
    
    #[inline]
    fn mul(self, vector: Vec2) -> Vec2 {
        Vec2 {
            x: vector.x * self,
            y: vector.y * self,
        }
    }
}

impl Div<f32> for Vec2 {
    type Output = Self;
    
    #[inline]
    fn div(self, scalar: f32) -> Self {
        if scalar == 0.0 {
            panic!("Division by zero");
        }
        Self {
            x: self.x / scalar,
            y: self.y / scalar,
        }
    }
}

impl AddAssign for Vec2 {
    #[inline]
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl SubAssign for Vec2 {
    #[inline]
    fn sub_assign(&mut self, other: Self) {
        self.x -= other.x;
        self.y -= other.y;
    }
}

impl MulAssign<f32> for Vec2 {
    #[inline]
    fn mul_assign(&mut self, scalar: f32) {
        self.x *= scalar;
        self.y *= scalar;
    }
}

impl DivAssign<f32> for Vec2 {
    #[inline]
    fn div_assign(&mut self, scalar: f32) {
        if scalar == 0.0 {
            panic!("Division by zero");
        }
        self.x /= scalar;
        self.y /= scalar;
    }
}

impl Neg for Vec2 {
    type Output = Self;
    
    #[inline]
    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl fmt::Display for Vec2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Vec2({}, {})", self.x, self.y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_vec2_constructors() {
        let v1 = Vec2::new(1.0, 2.0);
        assert_eq!(v1.x, 1.0);
        assert_eq!(v1.y, 2.0);
        
        let v2 = vec2(3.0, 4.0);
        assert_eq!(v2.x, 3.0);
        assert_eq!(v2.y, 4.0);
        
        assert_eq!(Vec2::zero(), vec2(0.0, 0.0));
        assert_eq!(Vec2::one(), vec2(1.0, 1.0));
        assert_eq!(Vec2::unit_x(), vec2(1.0, 0.0));
        assert_eq!(Vec2::unit_y(), vec2(0.0, 1.0));
    }
    
    #[test]
    fn test_vec2_length() {
        let v = vec2(3.0, 4.0);
        assert_eq!(v.length(), 5.0);
        assert_eq!(v.length_squared(), 25.0);
    }
    
    #[test]
    fn test_vec2_normalize() {
        let v = vec2(3.0, 4.0);
        let normalized = v.normalize().unwrap();
        assert!(normalized.approx_eq(vec2(3.0 / 5.0, 4.0 / 5.0)));
        assert!(approx_eq(normalized.length(), 1.0));
        
        let zero = Vec2::zero();
        assert!(zero.normalize().is_err());
        assert_eq!(zero.normalize_or_zero(), zero);
    }
    
    #[test]
    fn test_vec2_dot() {
        let v1 = vec2(1.0, 2.0);
        let v2 = vec2(3.0, 4.0);
        assert_eq!(v1.dot(v2), 11.0);
    }
    
    #[test]
    fn test_vec2_operations() {
        let v1 = vec2(1.0, 2.0);
        let v2 = vec2(3.0, 4.0);
        
        assert_eq!(v1 + v2, vec2(4.0, 6.0));
        assert_eq!(v1 - v2, vec2(-2.0, -2.0));
        assert_eq!(v1 * 2.0, vec2(2.0, 4.0));
        assert_eq!(2.0 * v1, vec2(2.0, 4.0));
        assert_eq!(v2 / 2.0, vec2(1.5, 2.0));
        
        let mut v3 = v1;
        v3 += v2;
        assert_eq!(v3, vec2(4.0, 6.0));
        
        let mut v4 = v1;
        v4 -= v2;
        assert_eq!(v4, vec2(-2.0, -2.0));
        
        let mut v5 = v1;
        v5 *= 2.0;
        assert_eq!(v5, vec2(2.0, 4.0));
        
        let mut v6 = v2;
        v6 /= 2.0;
        assert_eq!(v6, vec2(1.5, 2.0));
        
        assert_eq!(-v1, vec2(-1.0, -2.0));
    }
    
    #[test]
    fn test_vec2_distance() {
        let v1 = vec2(1.0, 2.0);
        let v2 = vec2(4.0, 6.0);
        assert_eq!(v1.distance(v2), 5.0);
        assert_eq!(v1.distance_squared(v2), 25.0);
    }
    
    #[test]
    fn test_vec2_lerp() {
        let v1 = vec2(1.0, 2.0);
        let v2 = vec2(3.0, 4.0);
        assert_eq!(v1.lerp(v2, 0.0), v1);
        assert_eq!(v1.lerp(v2, 1.0), v2);
        assert_eq!(v1.lerp(v2, 0.5), vec2(2.0, 3.0));
        
        // Test clamping
        assert_eq!(v1.lerp(v2, -1.0), v1);
        assert_eq!(v1.lerp(v2, 2.0), v2);
    }
    
    #[test]
    fn test_vec2_reflect() {
        let v = vec2(1.0, -1.0);
        let n = vec2(0.0, 1.0).normalize().unwrap();
        let r = v.reflect(n);
        assert!(r.approx_eq(vec2(1.0, 1.0)));
    }
    
    #[test]
    fn test_vec2_approx_eq() {
        let v1 = vec2(1.0, 2.0);
        let v2 = vec2(1.0, 2.0);
        let v3 = vec2(1.0 + 0.0000001, 2.0);
        let v4 = vec2(1.1, 2.0);
        
        assert!(v1.approx_eq(v2));
        assert!(v1.approx_eq(v3));
        assert!(!v1.approx_eq(v4));
    }
    
    #[test]
    #[should_panic(expected = "Division by zero")]
    fn test_vec2_div_by_zero() {
        let v = vec2(1.0, 2.0);
        let _ = v / 0.0;
    }
    
    #[test]
    #[should_panic(expected = "Division by zero")]
    fn test_vec2_div_assign_by_zero() {
        let mut v = vec2(1.0, 2.0);
        v /= 0.0;
    }
}
