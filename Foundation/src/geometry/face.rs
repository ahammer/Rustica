// Face definition for Rustica engine

use cgmath::{Vector3, InnerSpace};
use super::vertex::StandardVertex;

/// A triangle face with indices to three vertices
#[derive(Clone, Debug)]
pub struct Face {
    /// Indices to the three vertices that make up this face
    pub indices: [u32; 3],
    /// Optional material ID for this face
    pub material_id: Option<u32>,
}

impl Face {
    /// Create a new face with the given vertex indices
    pub fn new(a: u32, b: u32, c: u32) -> Self {
        Self {
            indices: [a, b, c],
            material_id: None,
        }
    }

    /// Create a new face with the given vertex indices and material ID
    pub fn with_material(a: u32, b: u32, c: u32, material_id: u32) -> Self {
        Self {
            indices: [a, b, c],
            material_id: Some(material_id),
        }
    }

    /// Calculate the normal vector for this face based on the vertices
    pub fn calculate_normal(&self, vertices: &[StandardVertex]) -> Vector3<f32> {
        let a = Vector3::new(
            vertices[self.indices[0] as usize].position[0],
            vertices[self.indices[0] as usize].position[1],
            vertices[self.indices[0] as usize].position[2],
        );
        
        let b = Vector3::new(
            vertices[self.indices[1] as usize].position[0],
            vertices[self.indices[1] as usize].position[1],
            vertices[self.indices[1] as usize].position[2],
        );
        
        let c = Vector3::new(
            vertices[self.indices[2] as usize].position[0],
            vertices[self.indices[2] as usize].position[1],
            vertices[self.indices[2] as usize].position[2],
        );

        // Calculate vectors from point a to b and a to c
        let ab = b - a;
        let ac = c - a;

        // Calculate cross product to get normal vector
        let normal = ab.cross(ac).normalize();
        
        normal
    }
}
