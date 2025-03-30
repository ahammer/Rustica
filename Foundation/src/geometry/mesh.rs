// Mesh definition for Rustica engine

use cgmath::{Vector3, InnerSpace};
use super::vertex::StandardVertex;
use super::face::Face;

/// A 3D mesh with vertices and faces
#[derive(Clone, Debug)]
pub struct Mesh {
    /// Vertices of the mesh
    pub vertices: Vec<StandardVertex>,
    /// Faces of the mesh
    pub faces: Vec<Face>,
}

impl Mesh {
    /// Create a new empty mesh
    pub fn new() -> Self {
        Self {
            vertices: Vec::new(),
            faces: Vec::new(),
        }
    }

    /// Create a mesh from vertices and faces
    pub fn from_vertices_and_faces(vertices: Vec<StandardVertex>, faces: Vec<Face>) -> Self {
        Self {
            vertices,
            faces,
        }
    }

    /// Get the indices as a flat vector
    pub fn indices(&self) -> Vec<u32> {
        let mut indices = Vec::with_capacity(self.faces.len() * 3);
        for face in &self.faces {
            indices.extend_from_slice(&face.indices);
        }
        indices
    }

    /// Calculate vertex normals based on face normals
    pub fn calculate_vertex_normals(&mut self) {
        // Initialize normal accumulators
        let mut normal_sums: Vec<Vector3<f32>> = vec![Vector3::new(0.0, 0.0, 0.0); self.vertices.len()];
        let mut normal_counts: Vec<u32> = vec![0; self.vertices.len()];
        
        // Calculate face normals and accumulate them for each vertex
        for face in &self.faces {
            let normal = face.calculate_normal(&self.vertices);
            
            for &index in &face.indices {
                let idx = index as usize;
                normal_sums[idx] += normal;
                normal_counts[idx] += 1;
            }
        }
        
        // Average the normals and update vertices
        for (i, vertex) in self.vertices.iter_mut().enumerate() {
            if normal_counts[i] > 0 {
                let avg_normal = normal_sums[i] / normal_counts[i] as f32;
                let normalized = avg_normal.normalize();
                vertex.normal = [normalized.x, normalized.y, normalized.z];
            }
        }
    }

    /// Map this mesh to a custom vertex format using a mapping function
    pub fn map<V, F>(&self, vertex_mapper: F) -> (Vec<V>, Vec<u32>)
    where
        F: Fn(&StandardVertex) -> V,
    {
        let vertices: Vec<V> = self.vertices.iter()
            .map(|v| vertex_mapper(v))
            .collect();
        
        let indices = self.indices();
        
        (vertices, indices)
    }
}
