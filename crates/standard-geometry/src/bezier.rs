use glam::{Vec2, Vec3};

/// Evaluates a cubic Bernstein polynomial.
///
/// # Arguments
///
/// * `i` - The control point index (0-3)
/// * `t` - The parameter value in range [0, 1]
///
/// # Returns
///
/// The Bernstein basis function value.
fn bernstein(i: usize, t: f32) -> f32 {
    match i {
        0 => (1.0 - t).powi(3),
        1 => 3.0 * t * (1.0 - t).powi(2),
        2 => 3.0 * t.powi(2) * (1.0 - t),
        3 => t.powi(3),
        _ => panic!("Bernstein index must be 0-3 for cubic curves"),
    }
}

/// A generic 2D Bezier patch with 4x4 control points (bicubic).
#[derive(Clone, Debug)]
pub struct BezierPatch2D {
    pub control_points: [[Vec2; 4]; 4],
}

impl BezierPatch2D {
    /// Creates a new 2D Bezier patch.
    pub fn new(control_points: [[Vec2; 4]; 4]) -> Self {
        Self { control_points }
    }

    /// Evaluates the 2D Bezier patch at parameters (u, v).
    ///
    /// # Arguments
    ///
    /// * `u` - The parameter in the first direction, in range [0, 1]
    /// * `v` - The parameter in the second direction, in range [0, 1]
    ///
    /// # Returns
    ///
    /// The evaluated point at (u, v).
    pub fn evaluate(&self, u: f32, v: f32) -> Vec2 {
        let mut point = Vec2::ZERO;
        for i in 0..4 {
            for j in 0..4 {
                let bernstein_u = bernstein(i, u);
                let bernstein_v = bernstein(j, v);
                point += self.control_points[i][j] * bernstein_u * bernstein_v;
            }
        }
        point
    }
}

/// A generic 3D Bezier patch with 4x4 control points (bicubic).
#[derive(Clone, Debug)]
pub struct BezierPatch3D {
    pub control_points: [[Vec3; 4]; 4],
}

impl BezierPatch3D {
    /// Creates a new 3D Bezier patch.
    pub fn new(control_points: [[Vec3; 4]; 4]) -> Self {
        Self { control_points }
    }

    /// Evaluates the 3D Bezier patch at parameters (u, v).
    ///
    /// # Arguments
    ///
    /// * `u` - The parameter in the first direction, in range [0, 1]
    /// * `v` - The parameter in the second direction, in range [0, 1]
    ///
    /// # Returns
    ///
    /// The evaluated point at (u, v).
    pub fn evaluate(&self, u: f32, v: f32) -> Vec3 {
        let mut point = Vec3::ZERO;
        for i in 0..4 {
            for j in 0..4 {
                let bernstein_u = bernstein(i, u);
                let bernstein_v = bernstein(j, v);
                point += self.control_points[i][j] * bernstein_u * bernstein_v;
            }
        }
        point
    }

    /// Evaluates the normal at parameters (u, v) using partial derivatives.
    ///
    /// # Arguments
    ///
    /// * `u` - The parameter in the first direction, in range [0, 1]
    /// * `v` - The parameter in the second direction, in range [0, 1]
    ///
    /// # Returns
    ///
    /// The normal vector at (u, v).
    pub fn normal(&self, u: f32, v: f32) -> Vec3 {
        // Calculate partial derivatives using central difference
        let epsilon = 0.001;
        let half_epsilon = epsilon / 2.0;
        
        let u1 = (u - half_epsilon).max(0.0);
        let u2 = (u + half_epsilon).min(1.0);
        let v1 = (v - half_epsilon).max(0.0);
        let v2 = (v + half_epsilon).min(1.0);
        
        let du = if u2 > u1 {
            (self.evaluate(u2, v) - self.evaluate(u1, v)) / (u2 - u1)
        } else {
            Vec3::X // Fallback if we're at the edge
        };
        
        let dv = if v2 > v1 {
            (self.evaluate(u, v2) - self.evaluate(u, v1)) / (v2 - v1)
        } else {
            Vec3::Z // Fallback if we're at the edge
        };
        
        // Normal is cross product of partial derivatives
        du.cross(dv).normalize()
    }

    /// Generate vertices for the patch with given resolution.
    ///
    /// # Arguments
    ///
    /// * `resolution` - The number of points to generate in each direction
    ///
    /// # Returns
    ///
    /// A vector of 3D points that form a grid of the evaluated patch.
    pub fn generate_vertices(&self, resolution: usize) -> Vec<Vec3> {
        let step = 1.0 / (resolution as f32 - 1.0);
        let mut vertices = Vec::with_capacity(resolution * resolution);
        
        for i in 0..resolution {
            for j in 0..resolution {
                let u = i as f32 * step;
                let v = j as f32 * step;
                vertices.push(self.evaluate(u, v));
            }
        }
        
        vertices
    }

    /// Generate vertices with normals and UV coordinates for the patch.
    ///
    /// # Arguments
    ///
    /// * `resolution` - The number of points to generate in each direction
    ///
    /// # Returns
    ///
    /// A tuple of vectors containing (positions, normals, uvs).
    pub fn generate_mesh_data(&self, resolution: usize) -> (Vec<Vec3>, Vec<Vec3>, Vec<Vec2>) {
        let step = 1.0 / (resolution as f32 - 1.0);
        let mesh_size = resolution * resolution;
        
        let mut positions = Vec::with_capacity(mesh_size);
        let mut normals = Vec::with_capacity(mesh_size);
        let mut uvs = Vec::with_capacity(mesh_size);
        
        for i in 0..resolution {
            for j in 0..resolution {
                let u = i as f32 * step;
                let v = j as f32 * step;
                
                positions.push(self.evaluate(u, v));
                normals.push(self.normal(u, v));
                uvs.push(Vec2::new(u, v));
            }
        }
        
        (positions, normals, uvs)
    }

    /// Generate indices for a grid of vertices.
    ///
    /// # Arguments
    ///
    /// * `resolution` - The grid resolution (number of vertices per side)
    ///
    /// # Returns
    ///
    /// A vector of indices that define triangles for the patch.
    pub fn generate_indices(resolution: usize) -> Vec<u32> {
        let mut indices = Vec::with_capacity((resolution - 1) * (resolution - 1) * 6);
        
        for i in 0..resolution - 1 {
            for j in 0..resolution - 1 {
                let current = (i * resolution + j) as u32;
                let next_row = ((i + 1) * resolution + j) as u32;
                
                // First triangle
                indices.push(current);
                indices.push(current + 1);
                indices.push(next_row + 1);
                
                // Second triangle
                indices.push(current);
                indices.push(next_row + 1);
                indices.push(next_row);
            }
        }
        
        indices
    }
}
