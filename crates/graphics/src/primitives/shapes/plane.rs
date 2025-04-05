// // Plane mesh implementation for Rustica graphics crate

// use rustica_foundation::geometry::{Mesh as FoundationMesh, StandardVertex, Face};

// /// Create a plane mesh
// pub fn create_plane(width: f32, depth: f32) -> FoundationMesh {
//     let hw = width / 2.0;
//     let hd = depth / 2.0;
    
//     // Define the 4 corners of the plane
//     let positions = [
//         [-hw, 0.0, -hd], // 0: back-left
//         [hw, 0.0, -hd],  // 1: back-right
//         [hw, 0.0, hd],   // 2: front-right
//         [-hw, 0.0, hd],  // 3: front-left
//     ];
    
//     // Define normal (pointing up)
//     let normal = [0.0, 1.0, 0.0];
    
//     // Define texture coordinates
//     let tex_coords = [
//         [0.0, 0.0], // back-left
//         [1.0, 0.0], // back-right
//         [1.0, 1.0], // front-right
//         [0.0, 1.0], // front-left
//     ];
    
//     // Define colors (grayscale)
//     let colors = [
//         [0.5, 0.5, 0.5], // gray
//         [0.6, 0.6, 0.6], // light gray
//         [0.7, 0.7, 0.7], // lighter gray
//         [0.5, 0.5, 0.5], // gray
//     ];
    
//     // Create vertices
//     let vertices = vec![
//         StandardVertex { position: positions[0], normal, tex_coords: tex_coords[0], color: colors[0] },
//         StandardVertex { position: positions[1], normal, tex_coords: tex_coords[1], color: colors[1] },
//         StandardVertex { position: positions[2], normal, tex_coords: tex_coords[2], color: colors[2] },
//         StandardVertex { position: positions[3], normal, tex_coords: tex_coords[3], color: colors[3] },
//     ];
    
//     // Define faces (2 triangles)
//     let faces = vec![
//         Face::new(0, 1, 2),
//         Face::new(0, 2, 3),
//     ];
    
//     FoundationMesh::from_vertices_and_faces(vertices, faces)
// }
