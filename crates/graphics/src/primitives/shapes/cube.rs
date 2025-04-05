// // Cube mesh implementation for Rustica graphics crate

// use rustica_foundation::geometry::{Mesh as FoundationMesh, StandardVertex, Face};

// /// Create a cube mesh
// pub fn create_cube(size: f32) -> FoundationMesh {
//     let half_size = size / 2.0;
    
//     // Define the 8 corners of the cube
//     let positions = [
//         // Front face (z = +half_size)
//         [-half_size, -half_size, half_size],  // 0: bottom-left-front
//         [half_size, -half_size, half_size],   // 1: bottom-right-front
//         [half_size, half_size, half_size],    // 2: top-right-front
//         [-half_size, half_size, half_size],   // 3: top-left-front
        
//         // Back face (z = -half_size)
//         [-half_size, -half_size, -half_size], // 4: bottom-left-back
//         [half_size, -half_size, -half_size],  // 5: bottom-right-back
//         [half_size, half_size, -half_size],   // 6: top-right-back
//         [-half_size, half_size, -half_size],  // 7: top-left-back
//     ];
    
//     // Define normals for each face
//     let normals = [
//         [0.0, 0.0, 1.0],   // front
//         [0.0, 0.0, -1.0],  // back
//         [-1.0, 0.0, 0.0],  // left
//         [1.0, 0.0, 0.0],   // right
//         [0.0, 1.0, 0.0],   // top
//         [0.0, -1.0, 0.0],  // bottom
//     ];
    
//     // Define texture coordinates for each face
//     let tex_coords = [
//         [0.0, 1.0], // bottom-left
//         [1.0, 1.0], // bottom-right
//         [1.0, 0.0], // top-right
//         [0.0, 0.0], // top-left
//     ];
    
//     // Define colors for each vertex (rainbow colors)
//     let colors = [
//         [1.0, 0.0, 0.0], // red
//         [1.0, 0.5, 0.0], // orange
//         [1.0, 1.0, 0.0], // yellow
//         [0.5, 1.0, 0.0], // lime
//         [0.0, 1.0, 0.0], // green
//         [0.0, 1.0, 0.5], // teal
//         [0.0, 0.0, 1.0], // blue
//         [0.5, 0.0, 1.0], // purple
//     ];
    
//     // Create vertices with positions, normals, texture coordinates, and colors
//     let mut vertices = Vec::new();
    
//     // Front face (vertices 0,1,2,3 with normal 0)
//     vertices.push(StandardVertex { position: positions[0], normal: normals[0], tex_coords: tex_coords[0], color: colors[0] });
//     vertices.push(StandardVertex { position: positions[1], normal: normals[0], tex_coords: tex_coords[1], color: colors[1] });
//     vertices.push(StandardVertex { position: positions[2], normal: normals[0], tex_coords: tex_coords[2], color: colors[2] });
//     vertices.push(StandardVertex { position: positions[3], normal: normals[0], tex_coords: tex_coords[3], color: colors[3] });
    
//     // Back face (vertices 4,5,6,7 with normal 1)
//     vertices.push(StandardVertex { position: positions[4], normal: normals[1], tex_coords: tex_coords[0], color: colors[4] });
//     vertices.push(StandardVertex { position: positions[5], normal: normals[1], tex_coords: tex_coords[1], color: colors[5] });
//     vertices.push(StandardVertex { position: positions[6], normal: normals[1], tex_coords: tex_coords[2], color: colors[6] });
//     vertices.push(StandardVertex { position: positions[7], normal: normals[1], tex_coords: tex_coords[3], color: colors[7] });
    
//     // Left face (vertices 0,3,7,4 with normal 2)
//     vertices.push(StandardVertex { position: positions[0], normal: normals[2], tex_coords: tex_coords[0], color: colors[0] });
//     vertices.push(StandardVertex { position: positions[3], normal: normals[2], tex_coords: tex_coords[1], color: colors[3] });
//     vertices.push(StandardVertex { position: positions[7], normal: normals[2], tex_coords: tex_coords[2], color: colors[7] });
//     vertices.push(StandardVertex { position: positions[4], normal: normals[2], tex_coords: tex_coords[3], color: colors[4] });
    
//     // Right face (vertices 1,5,6,2 with normal 3)
//     vertices.push(StandardVertex { position: positions[1], normal: normals[3], tex_coords: tex_coords[0], color: colors[1] });
//     vertices.push(StandardVertex { position: positions[5], normal: normals[3], tex_coords: tex_coords[1], color: colors[5] });
//     vertices.push(StandardVertex { position: positions[6], normal: normals[3], tex_coords: tex_coords[2], color: colors[6] });
//     vertices.push(StandardVertex { position: positions[2], normal: normals[3], tex_coords: tex_coords[3], color: colors[2] });
    
//     // Top face (vertices 3,2,6,7 with normal 4)
//     vertices.push(StandardVertex { position: positions[3], normal: normals[4], tex_coords: tex_coords[0], color: colors[3] });
//     vertices.push(StandardVertex { position: positions[2], normal: normals[4], tex_coords: tex_coords[1], color: colors[2] });
//     vertices.push(StandardVertex { position: positions[6], normal: normals[4], tex_coords: tex_coords[2], color: colors[6] });
//     vertices.push(StandardVertex { position: positions[7], normal: normals[4], tex_coords: tex_coords[3], color: colors[7] });
    
//     // Bottom face (vertices 0,4,5,1 with normal 5)
//     vertices.push(StandardVertex { position: positions[0], normal: normals[5], tex_coords: tex_coords[0], color: colors[0] });
//     vertices.push(StandardVertex { position: positions[4], normal: normals[5], tex_coords: tex_coords[1], color: colors[4] });
//     vertices.push(StandardVertex { position: positions[5], normal: normals[5], tex_coords: tex_coords[2], color: colors[5] });
//     vertices.push(StandardVertex { position: positions[1], normal: normals[5], tex_coords: tex_coords[3], color: colors[1] });
    
//     // Define faces (2 triangles per cube face)
//     let faces = vec![
//         // Front face
//         Face::new(0, 1, 2),
//         Face::new(0, 2, 3),
//         // Back face
//         Face::new(4, 6, 5),
//         Face::new(4, 7, 6),
//         // Left face
//         Face::new(8, 9, 10),
//         Face::new(8, 10, 11),
//         // Right face
//         Face::new(12, 13, 14),
//         Face::new(12, 14, 15),
//         // Top face
//         Face::new(16, 17, 18),
//         Face::new(16, 18, 19),
//         // Bottom face
//         Face::new(20, 21, 22),
//         Face::new(20, 22, 23),
//     ];
    
//     FoundationMesh::from_vertices_and_faces(vertices, faces)
// }
