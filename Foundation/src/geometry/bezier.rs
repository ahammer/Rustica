// Bezier curve and patch implementations for Rustica engine

use cgmath::{Vector2, Vector3, InnerSpace};
use super::vertex::StandardVertex;
use super::face::Face;
use super::mesh::Mesh;

/// A generic Bezier curve with control points of type T
#[derive(Clone, Debug)]
pub struct BezierCurve<T> {
    /// Control points of the curve
    pub control_points: Vec<T>,
}

impl<T: Clone + std::ops::Add<Output = T> + std::ops::Mul<f32, Output = T>> BezierCurve<T> {
    /// Create a new Bezier curve with the given control points
    pub fn new(control_points: Vec<T>) -> Self {
        Self { control_points }
    }

    /// Get the degree of the curve (number of control points - 1)
    pub fn degree(&self) -> usize {
        self.control_points.len() - 1
    }

    /// Evaluate the curve at parameter t (0.0 to 1.0)
    pub fn evaluate(&self, t: f32) -> T {
        // Handle edge cases
        if t <= 0.0 {
            return self.control_points[0].clone();
        }
        if t >= 1.0 {
            return self.control_points.last().unwrap().clone();
        }

        // De Casteljau's algorithm for evaluating a Bezier curve
        let mut points = self.control_points.clone();
        let n = points.len();

        for r in 1..n {
            for i in 0..(n - r) {
                points[i] = points[i].clone() * (1.0 - t) + points[i + 1].clone() * t;
            }
        }

        points[0].clone()
    }

    /// Tessellate the curve into line segments
    pub fn tessellate(&self, segments: usize) -> Vec<T> {
        let mut points = Vec::with_capacity(segments + 1);
        
        for i in 0..=segments {
            let t = i as f32 / segments as f32;
            points.push(self.evaluate(t));
        }
        
        points
    }
}

/// A cubic Bezier curve in 2D space
pub type CubicBezier2D = BezierCurve<Vector2<f32>>;

/// A cubic Bezier curve in 3D space
pub type CubicBezier3D = BezierCurve<Vector3<f32>>;

/// A Bezier patch (surface) defined by a grid of control points
#[derive(Clone, Debug)]
pub struct BezierPatch {
    /// Control points of the patch (row-major order)
    pub control_points: Vec<Vec<Vector3<f32>>>,
}

impl BezierPatch {
    /// Create a new Bezier patch with the given control points
    pub fn new(control_points: Vec<Vec<Vector3<f32>>>) -> Self {
        Self { control_points }
    }

    /// Get the degree in the u direction
    pub fn u_degree(&self) -> usize {
        self.control_points.len() - 1
    }

    /// Get the degree in the v direction
    pub fn v_degree(&self) -> usize {
        if self.control_points.is_empty() {
            0
        } else {
            self.control_points[0].len() - 1
        }
    }

    /// Evaluate the patch at parameters u, v (both 0.0 to 1.0)
    pub fn evaluate(&self, u: f32, v: f32) -> Vector3<f32> {
        // First, evaluate each row at parameter v
        let mut row_points = Vec::with_capacity(self.control_points.len());
        
        for row in &self.control_points {
            let curve = BezierCurve::new(row.clone());
            row_points.push(curve.evaluate(v));
        }
        
        // Then, evaluate the resulting curve at parameter u
        let curve = BezierCurve::new(row_points);
        curve.evaluate(u)
    }

    /// Calculate the normal vector at parameters u, v
    pub fn normal(&self, u: f32, v: f32) -> Vector3<f32> {
        // Calculate partial derivatives
        let delta = 0.001;
        
        let p = self.evaluate(u, v);
        let p_u = self.evaluate(u + delta, v);
        let p_v = self.evaluate(u, v + delta);
        
        let du = (p_u - p) / delta;
        let dv = (p_v - p) / delta;
        
        // Cross product of partial derivatives gives the normal
        du.cross(dv).normalize()
    }

    /// Tessellate the patch into a mesh
    pub fn tessellate(&self, u_segments: usize, v_segments: usize) -> Mesh {
        let mut vertices = Vec::new();
        let mut faces = Vec::new();
        
        // Generate vertices
        for i in 0..=u_segments {
            let u = i as f32 / u_segments as f32;
            
            for j in 0..=v_segments {
                let v = j as f32 / v_segments as f32;
                
                let position = self.evaluate(u, v);
                let normal = self.normal(u, v);
                
                // Generate color based on position (normalized to 0-1 range)
                let color = [
                    0.5 + 0.5 * (position.x / 2.0).min(1.0).max(-1.0),
                    0.5 + 0.5 * (position.y / 2.0).min(1.0).max(-1.0),
                    0.5 + 0.5 * (position.z / 2.0).min(1.0).max(-1.0),
                ];
                
                vertices.push(StandardVertex {
                    position: [position.x, position.y, position.z],
                    normal: [normal.x, normal.y, normal.z],
                    color,
                    tex_coords: [u, v],
                });
            }
        }
        
        // Generate faces
        for i in 0..u_segments {
            for j in 0..v_segments {
                let v0 = i * (v_segments + 1) + j;
                let v1 = v0 + 1;
                let v2 = (i + 1) * (v_segments + 1) + j;
                let v3 = v2 + 1;
                
                // Create two triangles for each quad
                faces.push(Face::new(v0 as u32, v2 as u32, v1 as u32));
                faces.push(Face::new(v1 as u32, v2 as u32, v3 as u32));
            }
        }
        
        Mesh::from_vertices_and_faces(vertices, faces)
    }
}
