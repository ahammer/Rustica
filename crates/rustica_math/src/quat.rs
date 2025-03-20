//! Quaternion implementation for 3D rotations

use std::ops::{Add, Sub, Mul, Div, Neg};
use std::fmt;
use crate::error::MathError;
use crate::approx_eq;
use crate::vec3::Vec3;
use crate::mat4::Mat4;

/// A quaternion representing a 3D rotation.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Quat {
    /// The real part (scalar component) of the quaternion.
    pub w: f32,
    /// The first imaginary component (i).
    pub x: f32,
    /// The second imaginary component (j).
    pub y: f32,
    /// The third imaginary component (k).
    pub z: f32,
}

/// Create a new quaternion with the given w, x, y, and z components.
#[inline]
pub fn quat(w: f32, x: f32, y: f32, z: f32) -> Quat {
    Quat::new(w, x, y, z)
}

/// Create a new identity quaternion.
#[inline]
pub fn quat_identity() -> Quat {
    Quat::identity()
}

impl Quat {
    /// Create a new quaternion with the given w, x, y, and z components.
    #[inline]
    pub fn new(w: f32, x: f32, y: f32, z: f32) -> Self {
        Self { w, x, y, z }
    }
    
    /// Create a new identity quaternion (no rotation).
    #[inline]
    pub fn identity() -> Self {
        Self { w: 1.0, x: 0.0, y: 0.0, z: 0.0 }
    }
    
    /// Create a quaternion from an axis-angle rotation.
    pub fn from_axis_angle(axis: Vec3, angle_radians: f32) -> Self {
        let half_angle = angle_radians * 0.5;
        let s = half_angle.sin();
        let c = half_angle.cos();
        
        let axis = match axis.normalize() {
            Ok(a) => a,
            Err(_) => return Self::identity(),
        };
        
        Self {
            w: c,
            x: axis.x * s,
            y: axis.y * s,
            z: axis.z * s,
        }
    }
    
    /// Create a quaternion from Euler angles (pitch, yaw, roll).
    /// The rotation order is YXZ: first yaw around Y, then pitch around X, then roll around Z.
    pub fn from_euler_angles(pitch: f32, yaw: f32, roll: f32) -> Self {
        let cy = (yaw * 0.5).cos();
        let sy = (yaw * 0.5).sin();
        let cp = (pitch * 0.5).cos();
        let sp = (pitch * 0.5).sin();
        let cr = (roll * 0.5).cos();
        let sr = (roll * 0.5).sin();
        
        Self {
            w: cr * cp * cy + sr * sp * sy,
            x: cr * sp * cy + sr * cp * sy,
            y: cr * cp * sy - sr * sp * cy,
            z: sr * cp * cy - cr * sp * sy,
        }
    }
    
    /// Create a quaternion from a rotation matrix.
    pub fn from_rotation_matrix(mat: &Mat4) -> Self {
        let m00 = mat.get(0, 0);
        let m11 = mat.get(1, 1);
        let m22 = mat.get(2, 2);
        let trace = m00 + m11 + m22;
        
        if trace > 0.0 {
            let s = 0.5 / (trace + 1.0).sqrt();
            Self {
                w: 0.25 / s,
                x: (mat.get(2, 1) - mat.get(1, 2)) * s,
                y: (mat.get(0, 2) - mat.get(2, 0)) * s,
                z: (mat.get(1, 0) - mat.get(0, 1)) * s,
            }
        } else if m00 > m11 && m00 > m22 {
            let s = 2.0 * (1.0 + m00 - m11 - m22).sqrt();
            Self {
                w: (mat.get(2, 1) - mat.get(1, 2)) / s,
                x: 0.25 * s,
                y: (mat.get(0, 1) + mat.get(1, 0)) / s,
                z: (mat.get(0, 2) + mat.get(2, 0)) / s,
            }
        } else if m11 > m22 {
            let s = 2.0 * (1.0 + m11 - m00 - m22).sqrt();
            Self {
                w: (mat.get(0, 2) - mat.get(2, 0)) / s,
                x: (mat.get(0, 1) + mat.get(1, 0)) / s,
                y: 0.25 * s,
                z: (mat.get(1, 2) + mat.get(2, 1)) / s,
            }
        } else {
            let s = 2.0 * (1.0 + m22 - m00 - m11).sqrt();
            Self {
                w: (mat.get(1, 0) - mat.get(0, 1)) / s,
                x: (mat.get(0, 2) + mat.get(2, 0)) / s,
                y: (mat.get(1, 2) + mat.get(2, 1)) / s,
                z: 0.25 * s,
            }
        }
    }
    
    /// Calculate the length (magnitude) of the quaternion.
    #[inline]
    pub fn length(&self) -> f32 {
        (self.w * self.w + self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }
    
    /// Calculate the squared length of the quaternion.
    #[inline]
    pub fn length_squared(&self) -> f32 {
        self.w * self.w + self.x * self.x + self.y * self.y + self.z * self.z
    }
    
    /// Normalize the quaternion to a unit quaternion.
    /// Returns an error if the quaternion has zero length.
    pub fn normalize(&self) -> Result<Self, MathError> {
        let len = self.length();
        if approx_eq(len, 0.0) {
            return Err(MathError::ZeroVectorNormalize);
        }
        
        let inv_len = 1.0 / len;
        Ok(Self {
            w: self.w * inv_len,
            x: self.x * inv_len,
            y: self.y * inv_len,
            z: self.z * inv_len,
        })
    }
    
    /// Returns a normalized quaternion or `Quat::identity()` if the quaternion has zero length.
    pub fn normalize_or_identity(&self) -> Self {
        self.normalize().unwrap_or_else(|_| Self::identity())
    }
    
    /// Calculate the conjugate of the quaternion (inverse of the imaginary parts).
    #[inline]
    pub fn conjugate(&self) -> Self {
        Self {
            w: self.w,
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
    
    /// Calculate the inverse of the quaternion.
    /// For a unit quaternion, this is the same as the conjugate.
    pub fn inverse(&self) -> Result<Self, MathError> {
        let len_sq = self.length_squared();
        if approx_eq(len_sq, 0.0) {
            return Err(MathError::ZeroVectorNormalize);
        }
        
        let inv_len_sq = 1.0 / len_sq;
        Ok(Self {
            w: self.w * inv_len_sq,
            x: -self.x * inv_len_sq,
            y: -self.y * inv_len_sq,
            z: -self.z * inv_len_sq,
        })
    }
    
    /// Calculate the dot product with another quaternion.
    #[inline]
    pub fn dot(&self, other: &Self) -> f32 {
        self.w * other.w + self.x * other.x + self.y * other.y + self.z * other.z
    }
    
    /// Linear interpolation between this quaternion and another quaternion.
    /// The parameter `t` is clamped to the range [0, 1].
    pub fn lerp(&self, other: &Self, t: f32) -> Self {
        let t = t.clamp(0.0, 1.0);
        let mut result = Self {
            w: self.w * (1.0 - t) + other.w * t,
            x: self.x * (1.0 - t) + other.x * t,
            y: self.y * (1.0 - t) + other.y * t,
            z: self.z * (1.0 - t) + other.z * t,
        };
        
        // Normalize the result
        result = result.normalize_or_identity();
        result
    }
    
    /// Spherical linear interpolation between this quaternion and another quaternion.
    /// The parameter `t` is clamped to the range [0, 1].
    pub fn slerp(&self, other: &Self, t: f32) -> Self {
        let t = t.clamp(0.0, 1.0);
        
        // Check if quaternions are equal or opposite
        let mut dot = self.dot(other);
        
        // Use shortest path
        let mut other_copy = *other;
        if dot < 0.0 {
            other_copy = -other_copy;
            dot = -dot;
        }
        
        // If quaternions are close, use linear interpolation
        if dot > 0.9995 {
            return self.lerp(&other_copy, t);
        }
        
        // Spherical interpolation
        let theta_0 = dot.acos();
        let theta = theta_0 * t;
        
        let sin_theta = theta.sin();
        let sin_theta_0 = theta_0.sin();
        
        let s0 = (theta_0 - theta).cos() / sin_theta_0;
        let s1 = sin_theta / sin_theta_0;
        
        Self {
            w: self.w * s0 + other_copy.w * s1,
            x: self.x * s0 + other_copy.x * s1,
            y: self.y * s0 + other_copy.y * s1,
            z: self.z * s0 + other_copy.z * s1,
        }
    }
    
    /// Convert the quaternion to an axis-angle rotation.
    /// Returns the axis and angle (in radians).
    pub fn to_axis_angle(&self) -> (Vec3, f32) {
        let normalized = self.normalize_or_identity();
        
        let angle = 2.0 * normalized.w.acos();
        let sin_half_angle = (1.0 - normalized.w * normalized.w).sqrt();
        
        if approx_eq(sin_half_angle, 0.0) {
            (Vec3::unit_x(), 0.0) // Any axis with zero angle
        } else {
            let axis = Vec3::new(
                normalized.x / sin_half_angle,
                normalized.y / sin_half_angle,
                normalized.z / sin_half_angle,
            );
            (axis, angle)
        }
    }
    
    /// Convert the quaternion to Euler angles (pitch, yaw, roll).
    /// Returns (pitch, yaw, roll) in radians.
    pub fn to_euler_angles(&self) -> (f32, f32, f32) {
        let normalized = self.normalize_or_identity();
        
        // Extract components for convenience
        let w = normalized.w;
        let x = normalized.x;
        let y = normalized.y;
        let z = normalized.z;
        
        // Calculate pitch (X rotation)
        let sin_pitch = 2.0 * (w * x - y * z);
        let pitch = if sin_pitch.abs() >= 1.0 {
            -std::f32::consts::FRAC_PI_2 * sin_pitch.signum()
        } else {
            sin_pitch.asin()
        };
        
        // Calculate yaw (Y rotation)
        let yaw = (2.0 * (w * y + x * z)).atan2(1.0 - 2.0 * (x * x + y * y));
        
        // Calculate roll (Z rotation)
        let roll = (2.0 * (w * z + x * y)).atan2(1.0 - 2.0 * (y * y + z * z));
        
        (pitch, yaw, roll)
    }
    
    /// Convert the quaternion to a rotation matrix.
    pub fn to_rotation_matrix(&self) -> Mat4 {
        let normalized = self.normalize_or_identity();
        
        // Extract components for convenience
        let w = normalized.w;
        let x = normalized.x;
        let y = normalized.y;
        let z = normalized.z;
        
        // Calculate components of the rotation matrix
        let xx = x * x;
        let xy = x * y;
        let xz = x * z;
        let xw = x * w;
        
        let yy = y * y;
        let yz = y * z;
        let yw = y * w;
        
        let zz = z * z;
        let zw = z * w;
        
        // Build the rotation matrix
        Mat4::from_cols(
            [
                1.0 - 2.0 * (yy + zz),
                2.0 * (xy + zw),
                2.0 * (xz - yw),
                0.0,
            ],
            [
                2.0 * (xy - zw),
                1.0 - 2.0 * (xx + zz),
                2.0 * (yz + xw),
                0.0,
            ],
            [
                2.0 * (xz + yw),
                2.0 * (yz - xw),
                1.0 - 2.0 * (xx + yy),
                0.0,
            ],
            [0.0, 0.0, 0.0, 1.0],
        )
    }
    
    /// Rotate a vector by this quaternion.
    pub fn rotate_vec3(&self, vector: Vec3) -> Vec3 {
        let q = self.normalize_or_identity();
        
        // Formula: q * v * q^-1 (where v is treated as a quaternion with w=0)
        let q_vector = Self::new(0.0, vector.x, vector.y, vector.z);
        let q_result = q * q_vector * q.conjugate();
        
        Vec3::new(q_result.x, q_result.y, q_result.z)
    }
    
    /// Returns true if the quaternion is approximately equal to another quaternion.
    pub fn approx_eq(&self, other: &Self) -> bool {
        approx_eq(self.w, other.w) && 
        approx_eq(self.x, other.x) && 
        approx_eq(self.y, other.y) && 
        approx_eq(self.z, other.z)
    }
}

// Implement operator overloads for Quat

impl Add for Quat {
    type Output = Self;
    
    fn add(self, other: Self) -> Self {
        Self {
            w: self.w + other.w,
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub for Quat {
    type Output = Self;
    
    fn sub(self, other: Self) -> Self {
        Self {
            w: self.w - other.w,
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Mul for Quat {
    type Output = Self;
    
    fn mul(self, other: Self) -> Self {
        Self {
            w: self.w * other.w - self.x * other.x - self.y * other.y - self.z * other.z,
            x: self.w * other.x + self.x * other.w + self.y * other.z - self.z * other.y,
            y: self.w * other.y - self.x * other.z + self.y * other.w + self.z * other.x,
            z: self.w * other.z + self.x * other.y - self.y * other.x + self.z * other.w,
        }
    }
}

impl Mul<f32> for Quat {
    type Output = Self;
    
    fn mul(self, scalar: f32) -> Self {
        Self {
            w: self.w * scalar,
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }
}

impl Mul<Quat> for f32 {
    type Output = Quat;
    
    fn mul(self, q: Quat) -> Quat {
        q * self
    }
}

impl Div<f32> for Quat {
    type Output = Self;
    
    fn div(self, scalar: f32) -> Self {
        if scalar == 0.0 {
            panic!("Division by zero");
        }
        let inv_scalar = 1.0 / scalar;
        Self {
            w: self.w * inv_scalar,
            x: self.x * inv_scalar,
            y: self.y * inv_scalar,
            z: self.z * inv_scalar,
        }
    }
}

impl Neg for Quat {
    type Output = Self;
    
    fn neg(self) -> Self {
        Self {
            w: -self.w,
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl fmt::Display for Quat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Quat({}, {}, {}, {})", self.w, self.x, self.y, self.z)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::to_radians;
    
    #[test]
    fn test_quat_constructors() {
        let q1 = Quat::new(1.0, 2.0, 3.0, 4.0);
        assert_eq!(q1.w, 1.0);
        assert_eq!(q1.x, 2.0);
        assert_eq!(q1.y, 3.0);
        assert_eq!(q1.z, 4.0);
        
        let q2 = quat(1.0, 2.0, 3.0, 4.0);
        assert_eq!(q1, q2);
        
        let identity = Quat::identity();
        assert_eq!(identity.w, 1.0);
        assert_eq!(identity.x, 0.0);
        assert_eq!(identity.y, 0.0);
        assert_eq!(identity.z, 0.0);
        
        let identity2 = quat_identity();
        assert_eq!(identity, identity2);
    }
    
    #[test]
    fn test_quat_length_and_normalize() {
        let q = Quat::new(1.0, 2.0, 3.0, 4.0);
        let len = (1.0f32 + 4.0 + 9.0 + 16.0).sqrt();
        assert_eq!(q.length(), len);
        assert_eq!(q.length_squared(), 30.0);
        
        let normalized = q.normalize().unwrap();
        assert!(approx_eq(normalized.length(), 1.0));
        
        let zero = Quat::new(0.0, 0.0, 0.0, 0.0);
        assert!(zero.normalize().is_err());
        assert_eq!(zero.normalize_or_identity(), Quat::identity());
    }
    
    #[test]
    fn test_quat_conjugate_and_inverse() {
        let q = Quat::new(1.0, 2.0, 3.0, 4.0);
        let q_conj = q.conjugate();
        assert_eq!(q_conj, Quat::new(1.0, -2.0, -3.0, -4.0));
        
        let unit_q = Quat::new(0.5, 0.5, 0.5, 0.5).normalize().unwrap();
        let unit_q_inv = unit_q.inverse().unwrap();
        assert!(unit_q_inv.approx_eq(&unit_q.conjugate()));
        
        let product = unit_q * unit_q_inv;
        assert!(product.approx_eq(&Quat::identity()));
    }
    
    #[test]
    fn test_quat_operations() {
        let q1 = Quat::new(1.0, 2.0, 3.0, 4.0);
        let q2 = Quat::new(5.0, 6.0, 7.0, 8.0);
        
        assert_eq!(q1 + q2, Quat::new(6.0, 8.0, 10.0, 12.0));
        assert_eq!(q1 - q2, Quat::new(-4.0, -4.0, -4.0, -4.0));
        
        assert_eq!(q1 * 2.0, Quat::new(2.0, 4.0, 6.0, 8.0));
        assert_eq!(2.0 * q1, Quat::new(2.0, 4.0, 6.0, 8.0));
        
        assert_eq!(q1 / 2.0, Quat::new(0.5, 1.0, 1.5, 2.0));
        
        assert_eq!(-q1, Quat::new(-1.0, -2.0, -3.0, -4.0));
        
        // Quaternion multiplication
        let q1 = Quat::new(1.0, 2.0, 3.0, 4.0).normalize().unwrap();
        let q2 = Quat::new(5.0, 6.0, 7.0, 8.0).normalize().unwrap();
        let q3 = q1 * q2;
        
        // Test that quaternion multiplication preserves unit length
        assert!(approx_eq(q3.length(), 1.0));
    }
    
    #[test]
    fn test_quat_rotation() {
        // Test rotation around X axis
        let rot_x = Quat::from_axis_angle(Vec3::unit_x(), to_radians(90.0));
        let v = Vec3::unit_y();
        let rotated = rot_x.rotate_vec3(v);
        assert!(rotated.approx_eq(Vec3::unit_z()));
        
        // Test rotation around Y axis
        let rot_y = Quat::from_axis_angle(Vec3::unit_y(), to_radians(90.0));
        let v = Vec3::unit_z();
        let rotated = rot_y.rotate_vec3(v);
        assert!(rotated.approx_eq(Vec3::unit_x()));
        
        // Test rotation around Z axis
        let rot_z = Quat::from_axis_angle(Vec3::unit_z(), to_radians(90.0));
        let v = Vec3::unit_x();
        let rotated = rot_z.rotate_vec3(v);
        assert!(rotated.approx_eq(Vec3::unit_y()));
    }
    
    #[test]
    fn test_quat_conversions() {
        // Test axis-angle to quaternion and back
        let axis = Vec3::unit_y();
        let angle = to_radians(90.0);
        let q = Quat::from_axis_angle(axis, angle);
        let (axis2, angle2) = q.to_axis_angle();
        
        assert!(axis2.approx_eq(axis));
        assert!(approx_eq(angle2, angle));
        
        // Test Euler angles
        let pitch = to_radians(30.0);
        let yaw = to_radians(45.0);
        let roll = to_radians(60.0);
        
        let q = Quat::from_euler_angles(pitch, yaw, roll);
        let (pitch2, yaw2, roll2) = q.to_euler_angles();
        
        assert!(approx_eq(pitch2, pitch));
        assert!(approx_eq(yaw2, yaw));
        assert!(approx_eq(roll2, roll));
        
        // Test rotation matrix conversion
        let matrix = q.to_rotation_matrix();
        let q2 = Quat::from_rotation_matrix(&matrix);
        
        // Quaternions can be negated and still represent the same rotation
        let dot = q.dot(&q2).abs();
        assert!(approx_eq(dot, 1.0));
    }
    
    #[test]
    fn test_quat_interpolation() {
        let q1 = Quat::identity();
        let q2 = Quat::from_axis_angle(Vec3::unit_y(), to_radians(180.0));
        
        // Test lerp
        let half_lerp = q1.lerp(&q2, 0.5);
        // Half-way point should have w close to sqrt(0.5) 
        assert!(approx_eq(half_lerp.w.abs(), (0.5f32).sqrt()));
        
        // Test slerp
        let half_slerp = q1.slerp(&q2, 0.5);
        // For this rotation, slerp is similar to lerp
        assert!(half_slerp.approx_eq(&half_lerp));
        
        // Test extreme cases
        assert!(q1.slerp(&q2, 0.0).approx_eq(&q1));
        assert!(q1.slerp(&q2, 1.0).approx_eq(&q2));
    }
    
    #[test]
    #[should_panic(expected = "Division by zero")]
    fn test_quat_div_by_zero() {
        let q = Quat::new(1.0, 2.0, 3.0, 4.0);
        let _ = q / 0.0;
    }
}
