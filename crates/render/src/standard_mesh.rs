// Standard mesh adapter for Rustica engine

use std::sync::Arc;
use rustica_foundation::geometry::{Mesh, StandardVertex};
use wgpu::{util::DeviceExt, Buffer, BufferUsages, Device};
use rustica_foundation::geometry::{Triangle, Vertex};

/// Adapter for rendering Foundation meshes
pub struct StandardMeshAdapter<V: Vertex> {
    /// Vertex buffer
    pub vertex_buffer: Option<Buffer>,
    /// Index buffer
    pub index_buffer: Option<Buffer>,
    /// Vertex count
    pub vertex_count: usize,
    /// Index count
    pub index_count: usize,
    /// Original mesh
    pub mesh: Arc<Mesh>,
    /// Vertex mapper function
    pub vertex_mapper: Box<dyn Fn(&StandardVertex) -> V>,
}

impl<V: Vertex> StandardMeshAdapter<V> {
    /// Create a new standard mesh adapter
    pub fn new(mesh: Arc<Mesh>, vertex_mapper: impl Fn(&StandardVertex) -> V + 'static) -> Self {
        Self {
            vertex_buffer: None,
            index_buffer: None,
            vertex_count: mesh.vertices.len(),
            index_count: mesh.indices().len(),
            mesh,
            vertex_mapper: Box::new(vertex_mapper),
        }
    }

    /// Upload the mesh to the GPU
    pub fn upload(&mut self, device: &Device) {
        // Map vertices to custom format
        let (vertices, indices) = self.mesh.map(&*self.vertex_mapper);

        // Create vertex buffer
        self.vertex_buffer = Some(device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Standard Mesh Vertex Buffer"),
                contents: bytemuck::cast_slice(&vertices),
                usage: BufferUsages::VERTEX,
            }
        ));

        // Create index buffer
        self.index_buffer = Some(device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Standard Mesh Index Buffer"),
                contents: bytemuck::cast_slice(&indices),
                usage: BufferUsages::INDEX,
            }
        ));
    }

    /// Convert the mesh to triangles for rendering
    pub fn to_triangles(&self) -> Vec<Triangle<V>> {
        let mut triangles = Vec::new();
        let indices = self.mesh.indices();
        
        // Process each triangle (3 indices at a time)
        for chunk in indices.chunks(3) {
            if chunk.len() == 3 {
                triangles.push(Triangle::new([
                    (self.vertex_mapper)(&self.mesh.vertices[chunk[0] as usize]),
                    (self.vertex_mapper)(&self.mesh.vertices[chunk[1] as usize]),
                    (self.vertex_mapper)(&self.mesh.vertices[chunk[2] as usize]),
                ]));
            }
        }
        
        triangles
    }
}
