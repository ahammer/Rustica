//! 3D vector implementation

use std::ops::{Add, Sub, Mul, Div, AddAssign, SubAssign, MulAssign, DivAssign, Neg};
use std::fmt;
use crate::error::MathError;
use crate::approx_eq;
use crate::vec2::Vec2;

/// A 3D vector with x, y, and z components.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

/// Create a new Vec3 with the given x, y, and z components.
#[inline]
pub fn vec3(x: f32, y: f32, z: f32) -> Vec3 {
    Vec3::new(x, y, z)
}

impl Vec3 {
    /// Create a new Vec3 with the given x, y, and z components.
    #[inline]
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }
    
    /// Create a new Vec3 from a Vec2 and a z component.
    #[inline]
    pub fn from_vec2(v: Vec2, z: f32) -> Self {
        Self { x: v.x, y: v.y, z }
    }
    
    /// Convert to a Vec2 by dropping the z component.
    #[inline]
    pub fn to_vec2(&self) -> Vec2 {
        Vec2::new(self.x, self.y)
    }
    
    /// Create a new Vec3 with all components set to zero.
    #[inline]
    pub fn zero() -> Self {
        Self { x: 0.0, y: 0.0, z: 0.0 }
    }
    
    /// Create a new Vec3 with all components set to one.
    #[inline]
    pub fn one() -> Self {
        Self { x: 1.0, y: 1.0, z: 1.0 }
    }
    
    /// Create a unit vector pointing along the X axis.
    #[inline]
    pub fn unit_x() -> Self {
        Self { x: 1.0, y: 0.0, z: 0.0 }
    }
    
    /// Create a unit vector pointing along the Y axis.
    #[inline]
    pub fn unit_y() -> Self {
        Self { x: 0.0, y: 1.0, z: 0.0 }
    }
    
    /// Create a unit vector pointing along the Z axis.
    #[inline]
    pub fn unit_z() -> Self {
        Self { x: 0.0, y: 0.0, z: 1.0 }
    }
    
    /// Calculate the length (magnitude) of the vector.
    #[inline]
    pub fn length(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }
    
    /// Calculate the squared length of the vector.
    /// This is faster than `length()` when you only need to compare distances.
    #[inline]
    pub fn length_squared(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }
    
    /// Calculate the distance to another vector.
    #[inline]
    pub fn distance(&self, other: Vec3) -> f32 {
        (*self - other).length()
    }
    
    /// Calculate the squared distance to another vector.
    #[inline]
    pub fn distance_squared(&self, other: Vec3) -> f32 {
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
            z: self.z / len,
        })
    }
    
    /// Returns a normalized vector or `Vec3::zero()` if the vector has zero length.
    pub fn normalize_or_zero(&self) -> Self {
        self.normalize().unwrap_or_else(|_| Self::zero())
    }
    
    /// Calculate the dot product with another vector.
    #[inline]
    pub fn dot(&self, other: Vec3) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
    
    /// Calculate the cross product with another vector.
    #[inline]
    pub fn cross(&self, other: Vec3) -> Self {
        Self {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }
    
    /// Linearly interpolate between this vector and another vector.
    /// The parameter `t` is clamped to the range [0, 1].
    pub fn lerp(&self, other: Vec3, t: f32) -> Self {
        let t = t.clamp(0.0, 1.0);
        *self * (1.0 - t) + other * t
    }
    
    /// Reflect this vector around a normal vector.
    pub fn reflect(&self, normal: Vec3) -> Self {
        *self - normal * 2.0 * self.dot(normal)
    }
    
    /// Returns true if the vector is approximately equal to another vector.
    pub fn approx_eq(&self, other: Vec3) -> bool {
        approx_eq(self.x, other.x) && approx_eq(self.y, other.y) && approx_eq(self.z, other.z)
    }
}

// Implement operator overloads for Vec3

impl Add for Vec3 {
    type Output = Self;
    
    #[inline]
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub for Vec3 {
    type Output = Self;
    
    #[inline]
    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Mul<f32> for Vec3 {
    type Output = Self;
    
    #[inline]
    fn mul(self, scalar: f32) -> Self {
        Self {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }
}

impl Mul<Vec3> for f32 {
    type Output = Vec3;
    
    #[inline]
    fn mul(self, vector: Vec3) -> Vec3 {
        Vec3 {
            x: vector.x * self,
            y: vector.y * self,
            z: vector.z * self,
        }
    }
}

impl Div<f32> for Vec3 {
    type Output = Self;
    
    #[inline]
    fn div(self, scalar: f32) -> Self {
        if scalar == 0.0 {
            panic!("Division by zero");
        }
        Self {
            x: self.x / scalar,
            y: self.y / scalar,
            z: self.z / scalar,
        }
    }
}

impl AddAssign for Vec3 {
    #[inline]
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

impl SubAssign for Vec3 {
    #[inline]
    fn sub_assign(&mut self, other: Self) {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
    }
}

impl MulAssign<f32> for Vec3 {
    #[inline]
    fn mul_assign(&mut self, scalar: f32) {
        self.x *= scalar;
        self.y *= scalar;
        self.z *= scalar;
    }
}

impl DivAssign<f32> for Vec3 {
    #[inline]
    fn div_assign(&mut self, scalar: f32) {
        if scalar == 0.0 {
            panic!("Division by zero");
        }
        self.x /= scalar;
        self.y /= scalar;
        self.z /= scalar;
    }
}

impl Neg for Vec3 {
    type Output = Self;
    
    #[inline]
    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Vec3({}, {}, {})", self.x, self.y, self.z)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_vec3_constructors() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(v1.x, 1.0);
        assert_eq!(v1.y, 2.0);
        assert_eq!(v1.z, 3.0);
        
        let v2 = vec3(4.0, 5.0, 6.0);
        assert_eq!(v2.x, 4.0);
        assert_eq!(v2.y, 5.0);
        assert_eq!(v2.z, 6.0);
        
        assert_eq!(Vec3::zero(), vec3(0.0, 0.0, 0.0));
        assert_eq!(Vec3::one(), vec3(1.0, 1.0, 1.0));
        assert_eq!(Vec3::unit_x(), vec3(1.0, 0.0, 0.0));
        assert_eq!(Vec3::unit_y(), vec3(0.0, 1.0, 0.0));
        assert_eq!(Vec3::unit_z(), vec3(0.0, 0.0, 1.0));
        
        let v2d = Vec2::new(1.0, 2.0);
        let v3d = Vec3::from_vec2(v2d, 3.0);
        assert_eq!(v3d, vec3(1.0, 2.0, 3.0));
        assert_eq!(v3d.to_vec2(), v2d);
    }
    
    #[test]
    fn test_vec3_length() {
        let v = vec3(3.0, 4.0, 12.0);
        assert_eq!(v.length(), 13.0);
        assert_eq!(v.length_squared(), 169.0);
    }
    
    #[test]
    fn test_vec3_normalize() {
        let v = vec3(3.0, 4.0, 0.0);
        let normalized = v.normalize().unwrap();
        assert!(normalized.approx_eq(vec3(3.0 / 5.0, 4.0 / 5.0, 0.0)));
        assert!(approx_eq(normalized.length(), 1.0));
        
        let zero = Vec3::zero();
        assert!(zero.normalize().is_err());
        assert_eq!(zero.normalize_or_zero(), zero);
    }
    
    #[test]
    fn test_vec3_dot() {
        let v1 = vec3(1.0, 2.0, 3.0);
        let v2 = vec3(4.0, 5.0, 6.0);
        assert_eq!(v1.dot(v2), 32.0);
    }
    
    #[test]
    fn test_vec3_cross() {
        let v1 = vec3(1.0, 0.0, 0.0);
        let v2 = vec3(0.0, 1.0, 0.0);
        assert_eq!(v1.cross(v2), vec3(0.0, 0.0, 1.0));
        assert_eq!(v2.cross(v1), vec3(0.0, 0.0, -1.0));
        
        let v3 = vec3(2.0, 3.0, 4.0);
        let v4 = vec3(5.0, 6.0, 7.0);
        assert_eq!(v3.cross(v4), vec3(-3.0, 6.0, -3.0));
    }
    
    #[test]
    fn test_vec3_operations() {
        let v1 = vec3(1.0, 2.0, 3.0);
        let v2 = vec3(4.0, 5.0, 6.0);
        
        assert_eq!(v1 + v2, vec3(5.0, 7.0, 9.0));
        assert_eq!(v1 - v2, vec3(-3.0, -3.0, -3.0));
        assert_eq!(v1 * 2.0, vec3(2.0, 4.0, 6.0));
        assert_eq!(2.0 * v1, vec3(2.0, 4.0, 6.0));
        assert_eq!(v2 / 2.0, vec3(2.0, 2.5, 3.0));
        
        let mut v3 = v1;
        v3 += v2;
        assert_eq!(v3, vec3(5.0, 7.0, 9.0));
        
        let mut v4 = v1;
        v4 -= v2;
        assert_eq!(v4, vec3(-3.0, -3.0, -3.0));
        
        let mut v5 = v1;
        v5 *= 2.0;
        assert_eq!(v5, vec3(2.0, 4.0, 6.0));
        
        let mut v6 = v2;
        v6 /= 2.0;
        assert_eq!(v6, vec3(2.0, 2.5, 3.0));
        
        assert_eq!(-v1, vec3(-1.0, -2.0, -3.0));
    }
    
    #[test]
    fn test_vec3_distance() {
        let v1 = vec3(1.0, 2.0, 3.0);
        let v2 = vec3(4.0, 6.0, 3.0);
        assert_eq!(v1.distance(v2), 5.0);
        assert_eq!(v1.distance_squared(v2), 25.0);
    }
    
    #[test]
    fn test_vec3_lerp() {
        let v1 = vec3(1.0, 2.0, 3.0);
        let v2 = vec3(3.0, 4.0, 5.0);
        assert_eq!(v1.lerp(v2, 0.0), v1);
        assert_eq!(v1.lerp(v2, 1.0), v2);
        assert_eq!(v1.lerp(v2, 0.5), vec3(2.0, 3.0, 4.0));
        
        // Test clamping
        assert_eq!(v1.lerp(v2, -1.0), v1);
        assert_eq!(v1.lerp(v2, 2.0), v2);
    }
    
    #[test]
    fn test_vec3_reflect() {
        let v = vec3(1.0, -1.0, 0.0);
        let n = vec3(0.0, 1.0, 0.0).normalize().unwrap();
        let r = v.reflect(n);
        assert!(r.approx_eq(vec3(1.0, 1.0, 0.0)));
    }
    
    #[test]
    fn test_vec3_approx_eq() {
        let v1 = vec3(1.0, 2.0, 3.0);
        let v2 = vec3(1.0, 2.0, 3.0);
        let v3 = vec3(1.0 + 0.0000001, 2.0, 3.0);
        let v4 = vec3(1.1, 2.0, 3.0);
        
        assert!(v1.approx_eq(v2));
        assert!(v1.approx_eq(v3));
        assert!(!v1.approx_eq(v4));
    }
    
    #[test]
    #[should_panic(expected = "Division by zero")]
    fn test_vec3_div_by_zero() {
        let v = vec3(1.0, 2.0, 3.0);
        let _ = v / 0.0;
    }
    
    #[test]
    #[should_panic(expected = "Division by zero")]
    fn test_vec3_div_assign_by_zero() {
        let mut v = vec3(1.0, 2.0, 3.0);
        v /= 0.0;
    }
}
