use glam::{Vec2, Vec3};
use rustica_foundation::geometry::{Geometry, GeometryBuilder};
use rustica_standard_shader::StandardShaderVertex;
use std::f32::consts::PI;

/// Create an improved UV sphere with proper poles
/// 
/// Addresses the issue with missing caps in the standard implementation
/// 
/// # Arguments
/// 
/// * `radius` - The radius of the sphere
/// * `sectors` - The number of longitudinal segments (around the equator)
/// * `stacks` - The number of latitudinal segments (from pole to pole)
/// * `color` - A single color to apply to all vertices
/// 
/// # Returns
/// 
/// A complete sphere with proper poles and no gaps
pub fn create_improved_uv_sphere(radius: f32, sectors: u32, stacks: u32, color: Vec3) -> Geometry<StandardShaderVertex> {
    let mut builder = GeometryBuilder::new();

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
        let k2 = k1 + sectors + 1;  // beginning of next stack

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

            // For poles, we need to handle the triangles differently
            if i == 0 {
                // Top pole - only need one triangle per sector
                builder.triangle(v1, v2, v4);
            } else if i == stacks - 1 {
                // Bottom pole - only need one triangle per sector
                builder.triangle(v1, v4, v3);
            } else {
                // Regular latitudes - use two triangles for a quad
                builder.triangle(v1, v2, v4);
                builder.triangle(v1, v4, v3);
            }
        }
    }

    builder.build()
}
