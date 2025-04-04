use crate::geometry::Vertex;

/// Primitive type for rendering
#[derive(Debug, Clone)]
pub enum PrimitiveType {
    TriangleList,
    TriangleStrip,
}

/// Geometry container with vertices and indices
#[derive(Debug)]
pub struct Geometry<V: Vertex> {
    pub vertices: Vec<V>,
    pub indices: Vec<u32>,
    pub primitive_type: PrimitiveType,
}

/// Builder for geometry with optimized primitive types
pub struct GeometryBuilder<V: Vertex> {
    vertices: Vec<V>,
    indices: Vec<u32>,
    primitive_type: PrimitiveType,
}

impl<V: Vertex> GeometryBuilder<V> {
    /// Create a new geometry builder
    pub fn new() -> Self {
        Self {
            vertices: Vec::new(),
            indices: Vec::new(),
            primitive_type: PrimitiveType::TriangleList,
        }
    }
    
    /// Create a builder with pre-allocated capacity
    pub fn with_capacity(vertices: usize, indices: usize) -> Self {
        Self {
            vertices: Vec::with_capacity(vertices),
            indices: Vec::with_capacity(indices),
            primitive_type: PrimitiveType::TriangleList,
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
    pub fn triangle_strip(&mut self, vertices: &[V]) -> &mut Self {
        if vertices.len() < 3 {
            return self; // Need at least 3 vertices for a strip
        }
        
        // Set primitive type to strip
        self.primitive_type = PrimitiveType::TriangleStrip;
        
        // Add all vertices
        let base_index = self.vertices.len() as u32;
        self.vertices.extend_from_slice(vertices);
        
        // For a triangle strip, we need indices for (n-2) triangles
        for i in 0..(vertices.len() - 2) {
            self.indices.push(base_index + i as u32);
            self.indices.push(base_index + (i + 1) as u32);
            self.indices.push(base_index + (i + 2) as u32);
        }
        
        self
    }
    
    /// Build the final geometry
    pub fn build(self) -> Geometry<V> {
        Geometry {
            vertices: self.vertices,
            indices: self.indices,
            primitive_type: self.primitive_type,
        }
    }
}
