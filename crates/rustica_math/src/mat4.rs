//! 4x4 matrix implementation for 3D transformations

use std::ops::{Add, Sub, Mul, Index, IndexMut};
use std::fmt;
use crate::error::MathError;
use crate::approx_eq;
use crate::vec3::Vec3;

/// A 4x4 matrix stored in column-major order.
#[derive(Debug, Clone, Copy)]
pub struct Mat4 {
    // Column-major storage: [column][row]
    // [0][0] [1][0] [2][0] [3][0]
    // [0][1] [1][1] [2][1] [3][1]
    // [0][2] [1][2] [2][2] [3][2]
    // [0][3] [1][3] [2][3] [3][3]
    data: [[f32; 4]; 4],
}

/// Create a new identity matrix.
#[inline]
pub fn mat4_identity() -> Mat4 {
    Mat4::identity()
}

impl Mat4 {
    /// Create a new matrix with all elements set to zero.
    #[inline]
    pub fn zero() -> Self {
        Self {
            data: [[0.0; 4]; 4],
        }
    }
    
    /// Create a new identity matrix.
    #[inline]
    pub fn identity() -> Self {
        let mut mat = Self::zero();
        mat.data[0][0] = 1.0;
        mat.data[1][1] = 1.0;
        mat.data[2][2] = 1.0;
        mat.data[3][3] = 1.0;
        mat
    }
    
    /// Create a new matrix from raw data in column-major order.
    #[inline]
    pub fn from_cols(c0: [f32; 4], c1: [f32; 4], c2: [f32; 4], c3: [f32; 4]) -> Self {
        Self {
            data: [c0, c1, c2, c3],
        }
    }
    
    /// Create a new matrix from raw data in row-major order.
    #[inline]
    pub fn from_rows(r0: [f32; 4], r1: [f32; 4], r2: [f32; 4], r3: [f32; 4]) -> Self {
        Self {
            data: [
                [r0[0], r1[0], r2[0], r3[0]],
                [r0[1], r1[1], r2[1], r3[1]],
                [r0[2], r1[2], r2[2], r3[2]],
                [r0[3], r1[3], r2[3], r3[3]],
            ],
        }
    }
    
    /// Get a specific element at [row, column].
    #[inline]
    pub fn get(&self, row: usize, col: usize) -> f32 {
        self.data[col][row]
    }
    
    /// Set a specific element at [row, column].
    #[inline]
    pub fn set(&mut self, row: usize, col: usize, value: f32) {
        self.data[col][row] = value;
    }
    
    /// Create a translation matrix.
    pub fn translation(translation: Vec3) -> Self {
        let mut mat = Self::identity();
        mat.data[3][0] = translation.x;
        mat.data[3][1] = translation.y;
        mat.data[3][2] = translation.z;
        mat
    }
    
    /// Create a scale matrix.
    pub fn scaling(scale: Vec3) -> Self {
        let mut mat = Self::zero();
        mat.data[0][0] = scale.x;
        mat.data[1][1] = scale.y;
        mat.data[2][2] = scale.z;
        mat.data[3][3] = 1.0;
        mat
    }
    
    /// Create a uniform scale matrix.
    pub fn uniform_scaling(scale: f32) -> Self {
        Self::scaling(Vec3::new(scale, scale, scale))
    }
    
    /// Create a rotation matrix around the X axis.
    pub fn rotation_x(angle_radians: f32) -> Self {
        let c = angle_radians.cos();
        let s = angle_radians.sin();
        
        let mut mat = Self::identity();
        mat.data[1][1] = c;
        mat.data[1][2] = s;
        mat.data[2][1] = -s;
        mat.data[2][2] = c;
        mat
    }
    
    /// Create a rotation matrix around the Y axis.
    pub fn rotation_y(angle_radians: f32) -> Self {
        let c = angle_radians.cos();
        let s = angle_radians.sin();
        
        let mut mat = Self::identity();
        mat.data[0][0] = c;
        mat.data[0][2] = -s;
        mat.data[2][0] = s;
        mat.data[2][2] = c;
        mat
    }
    
    /// Create a rotation matrix around the Z axis.
    pub fn rotation_z(angle_radians: f32) -> Self {
        let c = angle_radians.cos();
        let s = angle_radians.sin();
        
        let mut mat = Self::identity();
        mat.data[0][0] = c;
        mat.data[0][1] = s;
        mat.data[1][0] = -s;
        mat.data[1][1] = c;
        mat
    }
    
    /// Create a perspective projection matrix.
    pub fn perspective(fov_y_radians: f32, aspect_ratio: f32, near: f32, far: f32) -> Self {
        let tan_half_fovy = (fov_y_radians / 2.0).tan();
        let mut mat = Self::zero();
        
        mat.data[0][0] = 1.0 / (aspect_ratio * tan_half_fovy);
        mat.data[1][1] = 1.0 / tan_half_fovy;
        mat.data[2][2] = -(far + near) / (far - near);
        mat.data[2][3] = -1.0;
        mat.data[3][2] = -(2.0 * far * near) / (far - near);
        
        mat
    }
    
    /// Create an orthographic projection matrix.
    pub fn orthographic(left: f32, right: f32, bottom: f32, top: f32, near: f32, far: f32) -> Self {
        let mut mat = Self::identity();
        
        mat.data[0][0] = 2.0 / (right - left);
        mat.data[1][1] = 2.0 / (top - bottom);
        mat.data[2][2] = -2.0 / (far - near);
        
        mat.data[3][0] = -(right + left) / (right - left);
        mat.data[3][1] = -(top + bottom) / (top - bottom);
        mat.data[3][2] = -(far + near) / (far - near);
        
        mat
    }
    
    /// Create a look-at matrix (view matrix).
    pub fn look_at(eye: Vec3, target: Vec3, up: Vec3) -> Self {
        let forward = (eye - target).normalize().unwrap_or(Vec3::unit_z());
        let right = up.cross(forward).normalize().unwrap_or(Vec3::unit_x());
        let up = forward.cross(right);
        
        let mut mat = Self::identity();
        
        mat.data[0][0] = right.x;
        mat.data[0][1] = right.y;
        mat.data[0][2] = right.z;
        
        mat.data[1][0] = up.x;
        mat.data[1][1] = up.y;
        mat.data[1][2] = up.z;
        
        mat.data[2][0] = forward.x;
        mat.data[2][1] = forward.y;
        mat.data[2][2] = forward.z;
        
        mat.data[3][0] = -right.dot(eye);
        mat.data[3][1] = -up.dot(eye);
        mat.data[3][2] = -forward.dot(eye);
        
        mat
    }
    
    /// Transpose the matrix.
    pub fn transpose(&self) -> Self {
        let mut result = Self::zero();
        for row in 0..4 {
            for col in 0..4 {
                result.data[row][col] = self.data[col][row];
            }
        }
        result
    }
    
    /// Calculate the determinant of the matrix.
    pub fn determinant(&self) -> f32 {
        let a = self.data[0][0];
        let b = self.data[1][0];
        let c = self.data[2][0];
        let d = self.data[3][0];
        
        let e = self.data[0][1];
        let f = self.data[1][1];
        let g = self.data[2][1];
        let h = self.data[3][1];
        
        let i = self.data[0][2];
        let j = self.data[1][2];
        let k = self.data[2][2];
        let l = self.data[3][2];
        
        let m = self.data[0][3];
        let n = self.data[1][3];
        let o = self.data[2][3];
        let p = self.data[3][3];
        
        // Calculate determinant using cofactor expansion
        a * (
            f * (k * p - l * o) -
            g * (j * p - l * n) +
            h * (j * o - k * n)
        ) - b * (
            e * (k * p - l * o) -
            g * (i * p - l * m) +
            h * (i * o - k * m)
        ) + c * (
            e * (j * p - l * n) -
            f * (i * p - l * m) +
            h * (i * n - j * m)
        ) - d * (
            e * (j * o - k * n) -
            f * (i * o - k * m) +
            g * (i * n - j * m)
        )
    }
    
    /// Invert the matrix.
    /// Returns an error if the matrix is singular (determinant is zero).
    pub fn inverse(&self) -> Result<Self, MathError> {
        let det = self.determinant();
        if approx_eq(det, 0.0) {
            return Err(MathError::SingularMatrix);
        }
        
        let inv_det = 1.0 / det;
        
        let a = self.data[0][0];
        let b = self.data[1][0];
        let c = self.data[2][0];
        let d = self.data[3][0];
        
        let e = self.data[0][1];
        let f = self.data[1][1];
        let g = self.data[2][1];
        let h = self.data[3][1];
        
        let i = self.data[0][2];
        let j = self.data[1][2];
        let k = self.data[2][2];
        let l = self.data[3][2];
        
        let m = self.data[0][3];
        let n = self.data[1][3];
        let o = self.data[2][3];
        let p = self.data[3][3];
        
        // Calculate cofactors
        let c00 = f * (k * p - l * o) - g * (j * p - l * n) + h * (j * o - k * n);
        let c01 = -(e * (k * p - l * o) - g * (i * p - l * m) + h * (i * o - k * m));
        let c02 = e * (j * p - l * n) - f * (i * p - l * m) + h * (i * n - j * m);
        let c03 = -(e * (j * o - k * n) - f * (i * o - k * m) + g * (i * n - j * m));
        
        let c10 = -(b * (k * p - l * o) - c * (j * p - l * n) + d * (j * o - k * n));
        let c11 = a * (k * p - l * o) - c * (i * p - l * m) + d * (i * o - k * m);
        let c12 = -(a * (j * p - l * n) - b * (i * p - l * m) + d * (i * n - j * m));
        let c13 = a * (j * o - k * n) - b * (i * o - k * m) + c * (i * n - j * m);
        
        let c20 = b * (g * p - h * o) - c * (f * p - h * n) + d * (f * o - g * n);
        let c21 = -(a * (g * p - h * o) - c * (e * p - h * m) + d * (e * o - g * m));
        let c22 = a * (f * p - h * n) - b * (e * p - h * m) + d * (e * n - f * m);
        let c23 = -(a * (f * o - g * n) - b * (e * o - g * m) + c * (e * n - f * m));
        
        let c30 = -(b * (g * l - h * k) - c * (f * l - h * j) + d * (f * k - g * j));
        let c31 = a * (g * l - h * k) - c * (e * l - h * i) + d * (e * k - g * i);
        let c32 = -(a * (f * l - h * j) - b * (e * l - h * i) + d * (e * j - f * i));
        let c33 = a * (f * k - g * j) - b * (e * k - g * i) + c * (e * j - f * i);
        
        // Adjugate matrix multiplied by 1/determinant
        let mut result = Self::zero();
        
        // Note: Transposed because we're using column-major storage
        result.data[0][0] = c00 * inv_det;
        result.data[0][1] = c10 * inv_det;
        result.data[0][2] = c20 * inv_det;
        result.data[0][3] = c30 * inv_det;
        
        result.data[1][0] = c01 * inv_det;
        result.data[1][1] = c11 * inv_det;
        result.data[1][2] = c21 * inv_det;
        result.data[1][3] = c31 * inv_det;
        
        result.data[2][0] = c02 * inv_det;
        result.data[2][1] = c12 * inv_det;
        result.data[2][2] = c22 * inv_det;
        result.data[2][3] = c32 * inv_det;
        
        result.data[3][0] = c03 * inv_det;
        result.data[3][1] = c13 * inv_det;
        result.data[3][2] = c23 * inv_det;
        result.data[3][3] = c33 * inv_det;
        
        Ok(result)
    }
    
    /// Transform a Vec3 point by this matrix, applying perspective division.
    /// The Vec3 is treated as a 3D point with an implicit w-component of 1.0.
    pub fn transform_point(&self, point: Vec3) -> Vec3 {
        let x = self.data[0][0] * point.x + self.data[1][0] * point.y + self.data[2][0] * point.z + self.data[3][0];
        let y = self.data[0][1] * point.x + self.data[1][1] * point.y + self.data[2][1] * point.z + self.data[3][1];
        let z = self.data[0][2] * point.x + self.data[1][2] * point.y + self.data[2][2] * point.z + self.data[3][2];
        let w = self.data[0][3] * point.x + self.data[1][3] * point.y + self.data[2][3] * point.z + self.data[3][3];
        
        if approx_eq(w, 0.0) {
            return Vec3::new(x, y, z);
        }
        
        let inv_w = 1.0 / w;
        Vec3::new(x * inv_w, y * inv_w, z * inv_w)
    }
    
    /// Transform a Vec3 vector (direction) by this matrix, ignoring translation.
    /// The Vec3 is treated as a 3D vector with an implicit w-component of 0.0.
    pub fn transform_vector(&self, vector: Vec3) -> Vec3 {
        let x = self.data[0][0] * vector.x + self.data[1][0] * vector.y + self.data[2][0] * vector.z;
        let y = self.data[0][1] * vector.x + self.data[1][1] * vector.y + self.data[2][1] * vector.z;
        let z = self.data[0][2] * vector.x + self.data[1][2] * vector.y + self.data[2][2] * vector.z;
        
        Vec3::new(x, y, z)
    }
    
    /// Returns true if the matrix is approximately equal to another matrix.
    pub fn approx_eq(&self, other: &Self) -> bool {
        for col in 0..4 {
            for row in 0..4 {
                if !approx_eq(self.data[col][row], other.data[col][row]) {
                    return false;
                }
            }
        }
        true
    }
}

// Implement operator overloads for Mat4

impl Add for Mat4 {
    type Output = Self;
    
    fn add(self, other: Self) -> Self {
        let mut result = Self::zero();
        for col in 0..4 {
            for row in 0..4 {
                result.data[col][row] = self.data[col][row] + other.data[col][row];
            }
        }
        result
    }
}

impl Sub for Mat4 {
    type Output = Self;
    
    fn sub(self, other: Self) -> Self {
        let mut result = Self::zero();
        for col in 0..4 {
            for row in 0..4 {
                result.data[col][row] = self.data[col][row] - other.data[col][row];
            }
        }
        result
    }
}

impl Mul for Mat4 {
    type Output = Self;
    
    fn mul(self, other: Self) -> Self {
        let mut result = Self::zero();
        
        for col in 0..4 {
            for row in 0..4 {
                let mut sum = 0.0;
                for i in 0..4 {
                    sum += self.data[i][row] * other.data[col][i];
                }
                result.data[col][row] = sum;
            }
        }
        
        result
    }
}

impl Mul<Vec3> for Mat4 {
    type Output = Vec3;
    
    fn mul(self, vector: Vec3) -> Vec3 {
        self.transform_point(vector)
    }
}

impl Index<(usize, usize)> for Mat4 {
    type Output = f32;
    
    fn index(&self, (row, col): (usize, usize)) -> &Self::Output {
        &self.data[col][row]
    }
}

impl IndexMut<(usize, usize)> for Mat4 {
    fn index_mut(&mut self, (row, col): (usize, usize)) -> &mut Self::Output {
        &mut self.data[col][row]
    }
}

impl PartialEq for Mat4 {
    fn eq(&self, other: &Self) -> bool {
        for col in 0..4 {
            for row in 0..4 {
                if self.data[col][row] != other.data[col][row] {
                    return false;
                }
            }
        }
        true
    }
}

impl fmt::Display for Mat4 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Mat4[")?;
        for row in 0..4 {
            write!(f, "  [")?;
            for col in 0..4 {
                if col > 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{:.3}", self.data[col][row])?;
            }
            writeln!(f, "]")?;
        }
        write!(f, "]")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::to_radians;
    
    #[test]
    fn test_mat4_constructors() {
        let identity = Mat4::identity();
        assert_eq!(identity.get(0, 0), 1.0);
        assert_eq!(identity.get(1, 1), 1.0);
        assert_eq!(identity.get(2, 2), 1.0);
        assert_eq!(identity.get(3, 3), 1.0);
        assert_eq!(identity.get(0, 1), 0.0);
        
        let zero = Mat4::zero();
        for row in 0..4 {
            for col in 0..4 {
                assert_eq!(zero.get(row, col), 0.0);
            }
        }
        
        let from_cols = Mat4::from_cols(
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 10.0, 11.0, 12.0],
            [13.0, 14.0, 15.0, 16.0],
        );
        
        assert_eq!(from_cols.get(0, 0), 1.0);
        assert_eq!(from_cols.get(1, 0), 5.0);
        assert_eq!(from_cols.get(2, 0), 9.0);
        assert_eq!(from_cols.get(3, 0), 13.0);
        assert_eq!(from_cols.get(0, 1), 2.0);
        
        let from_rows = Mat4::from_rows(
            [1.0, 5.0, 9.0, 13.0],
            [2.0, 6.0, 10.0, 14.0],
            [3.0, 7.0, 11.0, 15.0],
            [4.0, 8.0, 12.0, 16.0],
        );
        
        assert_eq!(from_rows, from_cols);
    }
    
    #[test]
    fn test_mat4_indexing() {
        let mut mat = Mat4::identity();
        assert_eq!(mat[(0, 0)], 1.0);
        assert_eq!(mat[(1, 1)], 1.0);
        assert_eq!(mat[(0, 1)], 0.0);
        
        mat[(0, 1)] = 5.0;
        assert_eq!(mat[(0, 1)], 5.0);
        
        mat.set(2, 3, 7.0);
        assert_eq!(mat.get(2, 3), 7.0);
    }
    
    #[test]
    fn test_mat4_transformation_matrices() {
        let translation = Mat4::translation(Vec3::new(1.0, 2.0, 3.0));
        let point = Vec3::new(0.0, 0.0, 0.0);
        let translated = translation.transform_point(point);
        assert_eq!(translated, Vec3::new(1.0, 2.0, 3.0));
        
        let scale = Mat4::scaling(Vec3::new(2.0, 3.0, 4.0));
        let point = Vec3::new(1.0, 1.0, 1.0);
        let scaled = scale.transform_point(point);
        assert_eq!(scaled, Vec3::new(2.0, 3.0, 4.0));
        
        let rot_x = Mat4::rotation_x(to_radians(90.0));
        let point = Vec3::new(0.0, 1.0, 0.0);
        let rotated = rot_x.transform_point(point);
        assert!(rotated.approx_eq(Vec3::new(0.0, 0.0, 1.0)));
        
        let rot_y = Mat4::rotation_y(to_radians(90.0));
        let point = Vec3::new(0.0, 0.0, 1.0);
        let rotated = rot_y.transform_point(point);
        assert!(rotated.approx_eq(Vec3::new(1.0, 0.0, 0.0)));
        
        let rot_z = Mat4::rotation_z(to_radians(90.0));
        let point = Vec3::new(1.0, 0.0, 0.0);
        let rotated = rot_z.transform_point(point);
        assert!(rotated.approx_eq(Vec3::new(0.0, 1.0, 0.0)));
    }
    
    #[test]
    fn test_mat4_operations() {
        let a = Mat4::from_cols(
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 10.0, 11.0, 12.0],
            [13.0, 14.0, 15.0, 16.0],
        );
        
        let b = Mat4::from_cols(
            [17.0, 18.0, 19.0, 20.0],
            [21.0, 22.0, 23.0, 24.0],
            [25.0, 26.0, 27.0, 28.0],
            [29.0, 30.0, 31.0, 32.0],
        );
        
        // Addition
        let sum = a + b;
        assert_eq!(sum.get(0, 0), 18.0);
        assert_eq!(sum.get(3, 3), 48.0);
        
        // Subtraction
        let diff = b - a;
        assert_eq!(diff.get(0, 0), 16.0);
        assert_eq!(diff.get(3, 3), 16.0);
        
        // Matrix multiplication
        let identity = Mat4::identity();
        assert_eq!(a * identity, a);
        
        let c = a * b;
        assert_eq!(c.get(0, 0), 17.0*1.0 + 21.0*5.0 + 25.0*9.0 + 29.0*13.0);
        assert_eq!(c.get(3, 3), 20.0*4.0 + 24.0*8.0 + 28.0*12.0 + 32.0*16.0);
    }
    
    #[test]
    fn test_mat4_transpose() {
        let a = Mat4::from_cols(
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 10.0, 11.0, 12.0],
            [13.0, 14.0, 15.0, 16.0],
        );
        
        let transposed = a.transpose();
        
        for row in 0..4 {
            for col in 0..4 {
                assert_eq!(a.get(row, col), transposed.get(col, row));
            }
        }
        
        // Double transpose should be the original matrix
        assert_eq!(a.transpose().transpose(), a);
    }
    
    #[test]
    fn test_mat4_determinant_and_inverse() {
        let identity = Mat4::identity();
        assert_eq!(identity.determinant(), 1.0);
        assert_eq!(identity.inverse().unwrap(), identity);
        
        let translation = Mat4::translation(Vec3::new(1.0, 2.0, 3.0));
        let inverse_translation = translation.inverse().unwrap();
        assert!(inverse_translation.approx_eq(&Mat4::translation(Vec3::new(-1.0, -2.0, -3.0))));
        
        let scale = Mat4::scaling(Vec3::new(2.0, 3.0, 4.0));
        let inverse_scale = scale.inverse().unwrap();
        assert!(inverse_scale.approx_eq(&Mat4::scaling(Vec3::new(0.5, 1.0/3.0, 0.25))));
        
        // Singular matrix (determinant = 0)
        let singular = Mat4::from_cols(
            [1.0, 2.0, 3.0, 4.0],
            [2.0, 4.0, 6.0, 8.0], // 2 * first column
            [3.0, 6.0, 9.0, 12.0], // 3 * first column
            [4.0, 8.0, 12.0, 16.0], // 4 * first column
        );
        assert_eq!(singular.determinant(), 0.0);
        assert!(singular.inverse().is_err());
    }
    
    #[test]
    fn test_mat4_transform() {
        let translation = Mat4::translation(Vec3::new(1.0, 2.0, 3.0));
        let point = Vec3::new(0.0, 0.0, 0.0);
        let translated_point = translation.transform_point(point);
        assert_eq!(translated_point, Vec3::new(1.0, 2.0, 3.0));
        
        let vector = Vec3::new(1.0, 1.0, 1.0);
        let translated_vector = translation.transform_vector(vector);
        assert_eq!(translated_vector, vector); // Vectors shouldn't be affected by translation
        
        let look = Mat4::look_at(
            Vec3::new(0.0, 0.0, 5.0),
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
        );
        let transformed = look.transform_point(Vec3::new(0.0, 0.0, 0.0));
        assert!(transformed.approx_eq(Vec3::new(0.0, 0.0, -5.0)));
    }
}
