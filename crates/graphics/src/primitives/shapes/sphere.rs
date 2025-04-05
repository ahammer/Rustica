// // UV Sphere implementation for Rustica graphics crate

// use std::f32::consts::PI;
// use cgmath::{Vector3, InnerSpace};
// use rustica_foundation::geometry::{Mesh as FoundationMesh, StandardVertex, Face};

// /// Create a UV Sphere mesh
// /// 
// /// # Arguments
// /// * `radius` - Radius of the sphere
// /// * `longitude_segments` - Number of segments around the equator
// /// * `latitude_segments` - Number of segments from pole to pole
// /// * `color` - Optional color for the sphere (defaults to gradient based on position)
// pub fn create_uv_sphere(
//     radius: f32, 
//     longitude_segments: u32, 
//     latitude_segments: u32,
//     color: Option<[f32; 3]>
// ) -> FoundationMesh {
//     let mut vertices = Vec::new();
//     let mut faces = Vec::new();
    
//     // Generate vertices
//     for lat in 0..=latitude_segments {
//         // Latitude ranges from 0 (north pole) to PI (south pole)
//         let phi = lat as f32 * PI / latitude_segments as f32;
//         let sin_phi = phi.sin();
//         let cos_phi = phi.cos();
        
//         for lon in 0..=longitude_segments {
//             // Longitude ranges from 0 to 2*PI
//             let theta = lon as f32 * 2.0 * PI / longitude_segments as f32;
//             let sin_theta = theta.sin();
//             let cos_theta = theta.cos();
            
//             // Calculate vertex position
//             let x = radius * sin_phi * cos_theta;
//             let y = radius * cos_phi;
//             let z = radius * sin_phi * sin_theta;
            
//             // Calculate normal (pointing outward from center)
//             let normal = Vector3::new(x, y, z).normalize();
            
//             // Calculate texture coordinates
//             let u = lon as f32 / longitude_segments as f32;
//             let v = lat as f32 / latitude_segments as f32;
            
//             // Calculate color (gradient based on position or use provided color)
//             let vertex_color = color.unwrap_or([
//                 0.5 + 0.5 * (x / radius),
//                 0.5 + 0.5 * (y / radius),
//                 0.5 + 0.5 * (z / radius),
//             ]);
            
//             // Add vertex
//             vertices.push(StandardVertex {
//                 position: [x, y, z],
//                 normal: [normal.x, normal.y, normal.z],
//                 tex_coords: [u, v],
//                 color: vertex_color,
//             });
//         }
//     }
    
//     // Generate faces
//     for lat in 0..latitude_segments {
//         for lon in 0..longitude_segments {
//             let current = lat * (longitude_segments + 1) + lon;
//             let next_lon = current + 1;
//             let next_lat = current + (longitude_segments + 1);
//             let next_diag = next_lat + 1;
            
//             // Each quad is split into two triangles
//             // For the first triangle
//             faces.push(Face::new(
//                 current as u32,
//                 next_lon as u32,
//                 next_diag as u32,
//             ));
            
//             // For the second triangle
//             faces.push(Face::new(
//                 current as u32,
//                 next_diag as u32,
//                 next_lat as u32,
//             ));
//         }
//     }
    
//     FoundationMesh::from_vertices_and_faces(vertices, faces)
// }

// /// Create a UV Sphere with a solid color
// pub fn create_solid_sphere(
//     radius: f32,
//     longitude_segments: u32,
//     latitude_segments: u32,
//     color: [f32; 3]
// ) -> FoundationMesh {
//     create_uv_sphere(radius, longitude_segments, latitude_segments, Some(color))
// }

// /// Create a UV Sphere with default parameters (radius 1.0, 32x16 segments, gradient color)
// pub fn create_default_sphere() -> FoundationMesh {
//     create_uv_sphere(1.0, 32, 16, None)
// }
