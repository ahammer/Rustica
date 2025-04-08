use glam::{Vec2, Vec3};
use rustica_foundation::geometry::{Geometry, GeometryBuilder};
use rustica_standard_shader::StandardShaderVertex;

// Include the improved sphere implementation
mod improved_sphere;

// Re-export the improved sphere creation function
pub use improved_sphere::create_improved_uv_sphere;

/// Factory for creating standard primitive geometries.
///
/// Uses `GeometryBuilder` internally to construct geometry data
/// compatible with `StandardShaderVertex`.
pub struct GeometryFactory;

impl GeometryFactory {
    /// Creates a new cube geometry centered at the origin.
    ///
    /// # Arguments
    ///
    /// * `size` - The length of the cube's sides.
    /// * `color` - A single color to apply to all vertices.
    ///
    /// # Returns
    ///
    /// A `Geometry<StandardShaderVertex>` containing the vertex and index data for the cube.
    pub fn cube(size: f32, color: Vec3) -> Geometry<StandardShaderVertex> {
        let half_size = size / 2.0;
        let mut builder = GeometryBuilder::with_capacity(24, 36); // 6 faces * 4 vertices/face, 6 faces * 2 triangles/face * 3 indices/triangle

        // Define the 8 corner positions
        let p = [
            Vec3::new(-half_size, -half_size, half_size), // 0: front-bottom-left
            Vec3::new(half_size, -half_size, half_size),  // 1: front-bottom-right
            Vec3::new(half_size, half_size, half_size),   // 2: front-top-right
            Vec3::new(-half_size, half_size, half_size),  // 3: front-top-left
            Vec3::new(-half_size, -half_size, -half_size), // 4: back-bottom-left
            Vec3::new(half_size, -half_size, -half_size), // 5: back-bottom-right
            Vec3::new(half_size, half_size, -half_size),  // 6: back-top-right
            Vec3::new(-half_size, half_size, -half_size), // 7: back-top-left
        ];

        // Define normals for each face
        let n = [
            Vec3::new(0.0, 0.0, 1.0),  // front
            Vec3::new(0.0, 0.0, -1.0), // back
            Vec3::new(-1.0, 0.0, 0.0), // left
            Vec3::new(1.0, 0.0, 0.0),  // right
            Vec3::new(0.0, 1.0, 0.0),  // top
            Vec3::new(0.0, -1.0, 0.0), // bottom
        ];

        // Define texture coordinates (using standard box mapping)
        let uv = [
            Vec2::new(0.0, 1.0), // bottom-left
            Vec2::new(1.0, 1.0), // bottom-right
            Vec2::new(1.0, 0.0), // top-right
            Vec2::new(0.0, 0.0), // top-left
        ];

        // Helper to create a vertex
        let v = |pos: Vec3, normal: Vec3, tex_coord: Vec2| StandardShaderVertex {
            position: pos.into(),
            normal: normal.into(),
            color: color.into(), // Use the provided color
            uv: tex_coord.into(),
        };

        // Build faces using triangles
        // Front face (+Z)
        builder.triangle(v(p[0], n[0], uv[0]), v(p[1], n[0], uv[1]), v(p[2], n[0], uv[2]));
        builder.triangle(v(p[0], n[0], uv[0]), v(p[2], n[0], uv[2]), v(p[3], n[0], uv[3]));

        // Back face (-Z)
        builder.triangle(v(p[4], n[1], uv[1]), v(p[6], n[1], uv[3]), v(p[5], n[1], uv[2])); // Adjusted UVs for back
        builder.triangle(v(p[4], n[1], uv[1]), v(p[7], n[1], uv[0]), v(p[6], n[1], uv[3])); // Adjusted UVs for back

        // Left face (-X)
        builder.triangle(v(p[4], n[2], uv[0]), v(p[0], n[2], uv[1]), v(p[3], n[2], uv[2]));
        builder.triangle(v(p[4], n[2], uv[0]), v(p[3], n[2], uv[2]), v(p[7], n[2], uv[3]));

        // Right face (+X)
        builder.triangle(v(p[1], n[3], uv[0]), v(p[5], n[3], uv[1]), v(p[6], n[3], uv[2]));
        builder.triangle(v(p[1], n[3], uv[0]), v(p[6], n[3], uv[2]), v(p[2], n[3], uv[3]));

        // Top face (+Y)
        builder.triangle(v(p[3], n[4], uv[0]), v(p[2], n[4], uv[1]), v(p[6], n[4], uv[2]));
        builder.triangle(v(p[3], n[4], uv[0]), v(p[6], n[4], uv[2]), v(p[7], n[4], uv[3]));

        // Bottom face (-Y)
        builder.triangle(v(p[4], n[5], uv[0]), v(p[5], n[5], uv[1]), v(p[1], n[5], uv[2]));
        builder.triangle(v(p[4], n[5], uv[0]), v(p[1], n[5], uv[2]), v(p[0], n[5], uv[3]));

        builder.build()
    }

    /// Creates a new plane geometry centered at the origin, lying on the XZ plane.
    ///
    /// # Arguments
    ///
    /// * `size` - The width and depth of the plane.
    /// * `color` - A single color to apply to all vertices.
    ///
    /// # Returns
    ///
    /// A `Geometry<StandardShaderVertex>` containing the vertex and index data for the plane.
    pub fn plane(size: f32, color: Vec3) -> Geometry<StandardShaderVertex> {
        let half_size = size / 2.0;
        let mut builder = GeometryBuilder::with_capacity(4, 6); // 4 vertices, 2 triangles * 3 indices

        // Define the 4 corner positions on the XZ plane
        let p = [
            Vec3::new(-half_size, 0.0, -half_size), // 0: back-left
            Vec3::new(half_size, 0.0, -half_size),  // 1: back-right
            Vec3::new(half_size, 0.0, half_size),   // 2: front-right
            Vec3::new(-half_size, 0.0, half_size),  // 3: front-left
        ];

        // Normal points up
        let n = Vec3::new(0.0, 1.0, 0.0);

        // Define texture coordinates
        let uv = [
            Vec2::new(0.0, 0.0), // back-left
            Vec2::new(1.0, 0.0), // back-right
            Vec2::new(1.0, 1.0), // front-right
            Vec2::new(0.0, 1.0), // front-left
        ];

        // Helper to create a vertex
        let v = |pos: Vec3, tex_coord: Vec2| StandardShaderVertex {
            position: pos.into(),
            normal: n.into(),
            color: color.into(),
            uv: tex_coord.into(),
        };

        // Build the two triangles
        builder.triangle(v(p[0], uv[0]), v(p[1], uv[1]), v(p[2], uv[2]));
        builder.triangle(v(p[0], uv[0]), v(p[2], uv[2]), v(p[3], uv[3]));

        builder.build()
    }

    /// Creates a new UV sphere geometry centered at the origin.
    ///
    /// # Arguments
    ///
    /// * `radius` - The radius of the sphere.
    /// * `sectors` - The number of longitudinal segments (around the equator).
    /// * `stacks` - The number of latitudinal segments (from pole to pole).
    /// * `color` - A single color to apply to all vertices.
    ///
    /// # Returns
    ///
    /// A `Geometry<StandardShaderVertex>` containing the vertex and index data for the sphere.
    pub fn uv_sphere(radius: f32, sectors: u32, stacks: u32, color: Vec3) -> Geometry<StandardShaderVertex> {
        use std::f32::consts::PI;
        let mut builder = GeometryBuilder::new(); // Capacity estimation is tricky, let it grow

        // Generate vertices row by row, stack by stack
        let mut vertices: Vec<StandardShaderVertex> = Vec::with_capacity(((stacks + 1) * (sectors + 1)) as usize);

        for i in 0..=stacks {
            let stack_angle = PI / 2.0 - (i as f32 * PI / stacks as f32); // From PI/2 to -PI/2
            let xy = radius * stack_angle.cos();
            let z = radius * stack_angle.sin();

            for j in 0..=sectors {
                let sector_angle = j as f32 * 2.0 * PI / sectors as f32; // From 0 to 2PI

                let x = xy * sector_angle.cos();
                let y = xy * sector_angle.sin();
                let pos = Vec3::new(x, y, z);

                let normal = pos.normalize();
                let uv_coords = Vec2::new(j as f32 / sectors as f32, i as f32 / stacks as f32);

                vertices.push(StandardShaderVertex {
                    position: pos.into(),
                    normal: normal.into(),
                    color: color.into(),
                    uv: uv_coords.into(),
                });
            }
        }

        // Generate indices for triangles
        // Iterate over stacks and sectors to form quads, then split into triangles
        for i in 0..stacks {
            let k1 = i * (sectors + 1); // beginning of current stack
            let k2 = k1 + sectors + 1; // beginning of next stack

            for j in 0..sectors {
                // Get indices of vertices forming the quad
                let idx1 = k1 + j;
                let idx2 = k2 + j;
                let idx3 = k1 + j + 1;
                let idx4 = k2 + j + 1;

                // Get the actual vertices using the indices
                let v1 = vertices[idx1 as usize];
                let v2 = vertices[idx2 as usize];
                let v3 = vertices[idx3 as usize];
                let v4 = vertices[idx4 as usize];

                // Add two triangles for the quad
                if i != 0 { // Top cap triangles
                    builder.triangle(v1, v2, v4);
                }
                if i != (stacks - 1) { // Bottom cap triangles
                     builder.triangle(v1, v4, v3);
                }
            }
        }

        builder.build()
    }
}
