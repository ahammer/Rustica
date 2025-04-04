use crate::geometry::Vertex;

/// Geometry container with vertices and indices for triangle lists
#[derive(Debug)]
pub struct Geometry<V: Vertex> {
    pub vertices: Vec<V>,
    pub indices: Vec<u32>,
}

/// Builder for geometry using triangle lists
pub struct GeometryBuilder<V: Vertex> {
    vertices: Vec<V>,
    indices: Vec<u32>,
}

impl<V: Vertex> GeometryBuilder<V> {
    /// Create a new geometry builder
    pub fn new() -> Self {
        Self {
            vertices: Vec::new(),
            indices: Vec::new(),
        }
    }
    
    /// Create a builder with pre-allocated capacity
    pub fn with_capacity(vertices: usize, indices: usize) -> Self {
        Self {
            vertices: Vec::with_capacity(vertices),
            indices: Vec::with_capacity(indices),
        }
    }
    
    /// Add a single triangle to the geometry
    pub fn triangle(&mut self, a: V, b: V, c: V) -> &mut Self {
        let base_index = self.vertices.len() as u32;
        self.vertices.push(a);
        self.vertices.push(b);
        self.vertices.push(c);
        
        self.indices.push(base_index);
        self.indices.push(base_index + 1);
        self.indices.push(base_index + 2);
        
        self
    }
    
    /// Add a triangle strip to the geometry
    /// 
    /// Takes vertices that would form a triangle strip but internally converts them
    /// to a triangle list with proper indices to maintain consistent winding order.
    pub fn triangle_strip(&mut self, vertices: &[V]) -> &mut Self {
        if vertices.len() < 3 {
            return self; // Need at least 3 vertices for a strip
        }
        
        // Add all vertices
        let base_index = self.vertices.len() as u32;
        self.vertices.extend_from_slice(vertices);
        
        // Generate triangle list indices that replicate a strip
        // Ensuring consistent winding order for all triangles
        for i in 0..(vertices.len() - 2) {
            if i % 2 == 0 {
                // Even triangles — use natural winding
                self.indices.push(base_index + i as u32);
                self.indices.push(base_index + (i + 1) as u32);
                self.indices.push(base_index + (i + 2) as u32);
            } else {
                // Odd triangles — flip first two to preserve winding
                self.indices.push(base_index + (i + 1) as u32);
                self.indices.push(base_index + i as u32);
                self.indices.push(base_index + (i + 2) as u32);
            }
            
        }
        
        self
    }
    
    /// Build the final geometry
    pub fn build(self) -> Geometry<V> {
        Geometry {
            vertices: self.vertices,
            indices: self.indices,
        }
    }
}
