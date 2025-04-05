// // ShapeFactory - A centralized API for creating primitive shapes

// use rustica_foundation::geometry::Mesh as FoundationMesh;

// use super::{
//     cube::create_cube,
//     plane::create_plane,
//     teapot::{create_teapot, create_default_teapot},
//     sphere::{create_uv_sphere, create_solid_sphere, create_default_sphere}
// };

// /// Factory for creating geometric primitive shapes
// pub struct ShapeFactory;

// impl ShapeFactory {
//     /// Create a cube with the specified size
//     pub fn cube(size: f32) -> FoundationMesh {
//         create_cube(size)
//     }
    
//     /// Create a plane with the specified width and depth
//     pub fn plane(width: f32, depth: f32) -> FoundationMesh {
//         create_plane(width, depth)
//     }
    
//     /// Create a teapot with custom size and tessellation level
//     pub fn teapot(size: f32, tessellation_level: u32) -> FoundationMesh {
//         create_teapot(size, tessellation_level)
//     }
    
//     /// Create a teapot with default parameters
//     pub fn default_teapot() -> FoundationMesh {
//         create_default_teapot()
//     }
    
//     /// Create a UV sphere with custom parameters
//     pub fn sphere(radius: f32, longitude_segments: u32, latitude_segments: u32) -> FoundationMesh {
//         create_uv_sphere(radius, longitude_segments, latitude_segments, None)
//     }
    
//     /// Create a solid-colored sphere
//     pub fn solid_sphere(radius: f32, longitude_segments: u32, latitude_segments: u32, color: [f32; 3]) -> FoundationMesh {
//         create_solid_sphere(radius, longitude_segments, latitude_segments, color)
//     }
    
//     /// Create a sphere with default parameters
//     pub fn default_sphere() -> FoundationMesh {
//         create_default_sphere()
//     }
// }
