//! Core rendering functionality

use cgmath::Vector3 as Vec3;
use rustica_ecs::component::Component;
use std::collections::HashMap;

/// A component that makes an entity renderable
#[derive(Debug, Clone)]
pub struct RenderComponent {
    /// The mesh to render (placeholder)
    pub mesh_id: usize,
    /// The material to use (placeholder)
    pub material_id: usize,
    /// Whether the component is visible
    pub visible: bool,
}

impl Default for RenderComponent {
    fn default() -> Self {
        Self {
            mesh_id: 0,
            material_id: 0,
            visible: true,
        }
    }
}

/// A render resource (textures, meshes, etc.)
#[derive(Debug)]
pub enum RenderResource {
    /// A texture resource
    Texture {
        /// Width of the texture
        width: u32,
        /// Height of the texture
        height: u32,
        /// Texture format (placeholder)
        format: String,
    },
    /// A mesh resource
    Mesh {
        /// Vertices of the mesh (placeholder)
        vertices: Vec<Vec3<f32>>,
        /// Indices of the mesh (placeholder)
        indices: Vec<u32>,
    },
    /// A material resource
    Material {
        /// Properties of the material (placeholder)
        properties: HashMap<String, f32>,
    },
}

/// The main renderer for the engine
#[derive(Debug, Default)]
pub struct Renderer {
    /// Whether the renderer is initialized
    initialized: bool,
    /// The active camera entity (placeholder)
    active_camera: Option<usize>,
    /// Resources managed by the renderer
    resources: HashMap<usize, RenderResource>,
    /// Next resource ID
    next_resource_id: usize,
}

impl Renderer {
    /// Create a new renderer
    pub fn new() -> Self {
        Self {
            initialized: false,
            active_camera: None,
            resources: HashMap::new(),
            next_resource_id: 1,
        }
    }

    /// Initialize the renderer
    pub fn initialize(&mut self) {
        self.initialized = true;
    }
    
    /// Check if the renderer is initialized
    pub fn is_initialized(&self) -> bool {
        self.initialized
    }
    
    /// Set the active camera
    pub fn set_active_camera(&mut self, camera_entity: usize) {
        self.active_camera = Some(camera_entity);
    }
    
    /// Add a resource to the renderer
    pub fn add_resource(&mut self, resource: RenderResource) -> usize {
        let id = self.next_resource_id;
        self.resources.insert(id, resource);
        self.next_resource_id += 1;
        id
    }
    
    /// Get a resource from the renderer
    pub fn get_resource(&self, id: usize) -> Option<&RenderResource> {
        self.resources.get(&id)
    }
}
