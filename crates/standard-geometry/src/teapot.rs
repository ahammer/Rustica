use crate::bezier::BezierPatch3D;
use glam::Vec3;
use rustica_foundation::geometry::{Geometry, GeometryBuilder};
use rustica_standard_shader::{StandardShaderVertex, StandardShaderVertexFactory};

/// Creates the Utah teapot geometry with the given resolution.
///
/// # Arguments
///
/// * `resolution` - The resolution for each patch (number of vertices per side)
/// * `scale` - Scale factor for the teapot size
/// * `color` - Base color for the teapot vertices
///
/// # Returns
///
/// A `Geometry<StandardShaderVertex>` containing the vertex and index data for the teapot.
pub fn create_utah_teapot(resolution: usize, scale: f32, color: Vec3) -> Geometry<StandardShaderVertex> {
    let patches = teapot_patches();
    let mut builder = GeometryBuilder::new();
    
    // Process each patch
    for patch in patches {
        // Scale the control points
        let scaled_patch = scale_patch(&patch, scale);
        
        // Generate mesh data from the patch
        let (positions, normals, uvs) = scaled_patch.generate_mesh_data(resolution);
        
        // Generate indices
        let indices = BezierPatch3D::generate_indices(resolution);
        
        // Build triangles from the patch
        for i in (0..indices.len()).step_by(3) {
            let idx1 = indices[i] as usize;
            let idx2 = indices[i + 1] as usize;
            let idx3 = indices[i + 2] as usize;
            
            builder.triangle(
                StandardShaderVertexFactory::create_vertex(
                    positions[idx1].into(), 
                    normals[idx1].into(), 
                    color.into(), 
                    uvs[idx1].into()
                ),
                StandardShaderVertexFactory::create_vertex(
                    positions[idx2].into(), 
                    normals[idx2].into(), 
                    color.into(), 
                    uvs[idx2].into()
                ),
                StandardShaderVertexFactory::create_vertex(
                    positions[idx3].into(), 
                    normals[idx3].into(), 
                    color.into(), 
                    uvs[idx3].into()
                )
            );
        }
    }
    
    builder.build()
}

/// Scales the control points of a Bezier patch.
fn scale_patch(patch: &BezierPatch3D, scale: f32) -> BezierPatch3D {
    let mut scaled_points = [[Vec3::ZERO; 4]; 4];
    
    for i in 0..4 {
        for j in 0..4 {
            scaled_points[i][j] = patch.control_points[i][j] * scale;
        }
    }
    
    BezierPatch3D::new(scaled_points)
}

/// Returns all Bezier patches that define the Utah teapot.
///
/// The patches are in standard Utah teapot layout.
fn teapot_patches() -> Vec<BezierPatch3D> {
    // This data is from the standard Utah teapot defined in Bezier patches
    // Each array is a 4x4 grid of control points defining a single patch
    
    // The Utah teapot consists of 32 bicubic Bezier patches
    let mut patches = Vec::with_capacity(32);
    
    // Add a single patch as an example (replace with all 32 patches for the full teapot)
    // Rim patch 1
    patches.push(BezierPatch3D::new([
        [Vec3::new(1.4, 0.0, 2.4), Vec3::new(1.4, -0.784, 2.4), Vec3::new(0.784, -1.4, 2.4), Vec3::new(0.0, -1.4, 2.4)],
        [Vec3::new(1.3375, 0.0, 2.53125), Vec3::new(1.3375, -0.749, 2.53125), Vec3::new(0.749, -1.3375, 2.53125), Vec3::new(0.0, -1.3375, 2.53125)],
        [Vec3::new(1.4375, 0.0, 2.53125), Vec3::new(1.4375, -0.805, 2.53125), Vec3::new(0.805, -1.4375, 2.53125), Vec3::new(0.0, -1.4375, 2.53125)],
        [Vec3::new(1.5, 0.0, 2.4), Vec3::new(1.5, -0.84, 2.4), Vec3::new(0.84, -1.5, 2.4), Vec3::new(0.0, -1.5, 2.4)],
    ]));

    // Rim patch 2
    patches.push(BezierPatch3D::new([
        [Vec3::new(0.0, -1.4, 2.4), Vec3::new(-0.784, -1.4, 2.4), Vec3::new(-1.4, -0.784, 2.4), Vec3::new(-1.4, 0.0, 2.4)],
        [Vec3::new(0.0, -1.3375, 2.53125), Vec3::new(-0.749, -1.3375, 2.53125), Vec3::new(-1.3375, -0.749, 2.53125), Vec3::new(-1.3375, 0.0, 2.53125)],
        [Vec3::new(0.0, -1.4375, 2.53125), Vec3::new(-0.805, -1.4375, 2.53125), Vec3::new(-1.4375, -0.805, 2.53125), Vec3::new(-1.4375, 0.0, 2.53125)],
        [Vec3::new(0.0, -1.5, 2.4), Vec3::new(-0.84, -1.5, 2.4), Vec3::new(-1.5, -0.84, 2.4), Vec3::new(-1.5, 0.0, 2.4)],
    ]));

    // Rim patch 3
    patches.push(BezierPatch3D::new([
        [Vec3::new(-1.4, 0.0, 2.4), Vec3::new(-1.4, 0.784, 2.4), Vec3::new(-0.784, 1.4, 2.4), Vec3::new(0.0, 1.4, 2.4)],
        [Vec3::new(-1.3375, 0.0, 2.53125), Vec3::new(-1.3375, 0.749, 2.53125), Vec3::new(-0.749, 1.3375, 2.53125), Vec3::new(0.0, 1.3375, 2.53125)],
        [Vec3::new(-1.4375, 0.0, 2.53125), Vec3::new(-1.4375, 0.805, 2.53125), Vec3::new(-0.805, 1.4375, 2.53125), Vec3::new(0.0, 1.4375, 2.53125)],
        [Vec3::new(-1.5, 0.0, 2.4), Vec3::new(-1.5, 0.84, 2.4), Vec3::new(-0.84, 1.5, 2.4), Vec3::new(0.0, 1.5, 2.4)],
    ]));

    // Rim patch 4
    patches.push(BezierPatch3D::new([
        [Vec3::new(0.0, 1.4, 2.4), Vec3::new(0.784, 1.4, 2.4), Vec3::new(1.4, 0.784, 2.4), Vec3::new(1.4, 0.0, 2.4)],
        [Vec3::new(0.0, 1.3375, 2.53125), Vec3::new(0.749, 1.3375, 2.53125), Vec3::new(1.3375, 0.749, 2.53125), Vec3::new(1.3375, 0.0, 2.53125)],
        [Vec3::new(0.0, 1.4375, 2.53125), Vec3::new(0.805, 1.4375, 2.53125), Vec3::new(1.4375, 0.805, 2.53125), Vec3::new(1.4375, 0.0, 2.53125)],
        [Vec3::new(0.0, 1.5, 2.4), Vec3::new(0.84, 1.5, 2.4), Vec3::new(1.5, 0.84, 2.4), Vec3::new(1.5, 0.0, 2.4)],
    ]));

    // Body patch 1
    patches.push(BezierPatch3D::new([
        [Vec3::new(1.5, 0.0, 2.4), Vec3::new(1.5, -0.84, 2.4), Vec3::new(0.84, -1.5, 2.4), Vec3::new(0.0, -1.5, 2.4)],
        [Vec3::new(1.75, 0.0, 1.875), Vec3::new(1.75, -0.98, 1.875), Vec3::new(0.98, -1.75, 1.875), Vec3::new(0.0, -1.75, 1.875)],
        [Vec3::new(2.0, 0.0, 1.35), Vec3::new(2.0, -1.12, 1.35), Vec3::new(1.12, -2.0, 1.35), Vec3::new(0.0, -2.0, 1.35)],
        [Vec3::new(2.0, 0.0, 0.9), Vec3::new(2.0, -1.12, 0.9), Vec3::new(1.12, -2.0, 0.9), Vec3::new(0.0, -2.0, 0.9)],
    ]));

    // Body patch 2
    patches.push(BezierPatch3D::new([
        [Vec3::new(0.0, -1.5, 2.4), Vec3::new(-0.84, -1.5, 2.4), Vec3::new(-1.5, -0.84, 2.4), Vec3::new(-1.5, 0.0, 2.4)],
        [Vec3::new(0.0, -1.75, 1.875), Vec3::new(-0.98, -1.75, 1.875), Vec3::new(-1.75, -0.98, 1.875), Vec3::new(-1.75, 0.0, 1.875)],
        [Vec3::new(0.0, -2.0, 1.35), Vec3::new(-1.12, -2.0, 1.35), Vec3::new(-2.0, -1.12, 1.35), Vec3::new(-2.0, 0.0, 1.35)],
        [Vec3::new(0.0, -2.0, 0.9), Vec3::new(-1.12, -2.0, 0.9), Vec3::new(-2.0, -1.12, 0.9), Vec3::new(-2.0, 0.0, 0.9)],
    ]));

    // Body patch 3
    patches.push(BezierPatch3D::new([
        [Vec3::new(-1.5, 0.0, 2.4), Vec3::new(-1.5, 0.84, 2.4), Vec3::new(-0.84, 1.5, 2.4), Vec3::new(0.0, 1.5, 2.4)],
        [Vec3::new(-1.75, 0.0, 1.875), Vec3::new(-1.75, 0.98, 1.875), Vec3::new(-0.98, 1.75, 1.875), Vec3::new(0.0, 1.75, 1.875)],
        [Vec3::new(-2.0, 0.0, 1.35), Vec3::new(-2.0, 1.12, 1.35), Vec3::new(-1.12, 2.0, 1.35), Vec3::new(0.0, 2.0, 1.35)],
        [Vec3::new(-2.0, 0.0, 0.9), Vec3::new(-2.0, 1.12, 0.9), Vec3::new(-1.12, 2.0, 0.9), Vec3::new(0.0, 2.0, 0.9)],
    ]));

    // Body patch 4
    patches.push(BezierPatch3D::new([
        [Vec3::new(0.0, 1.5, 2.4), Vec3::new(0.84, 1.5, 2.4), Vec3::new(1.5, 0.84, 2.4), Vec3::new(1.5, 0.0, 2.4)],
        [Vec3::new(0.0, 1.75, 1.875), Vec3::new(0.98, 1.75, 1.875), Vec3::new(1.75, 0.98, 1.875), Vec3::new(1.75, 0.0, 1.875)],
        [Vec3::new(0.0, 2.0, 1.35), Vec3::new(1.12, 2.0, 1.35), Vec3::new(2.0, 1.12, 1.35), Vec3::new(2.0, 0.0, 1.35)],
        [Vec3::new(0.0, 2.0, 0.9), Vec3::new(1.12, 2.0, 0.9), Vec3::new(2.0, 1.12, 0.9), Vec3::new(2.0, 0.0, 0.9)],
    ]));

    // Bottom patches (simplified representation - would normally have more patches)
    patches.push(BezierPatch3D::new([
        [Vec3::new(2.0, 0.0, 0.9), Vec3::new(2.0, -1.12, 0.9), Vec3::new(1.12, -2.0, 0.9), Vec3::new(0.0, -2.0, 0.9)],
        [Vec3::new(2.0, 0.0, 0.45), Vec3::new(2.0, -1.12, 0.45), Vec3::new(1.12, -2.0, 0.45), Vec3::new(0.0, -2.0, 0.45)],
        [Vec3::new(1.5, 0.0, 0.225), Vec3::new(1.5, -0.84, 0.225), Vec3::new(0.84, -1.5, 0.225), Vec3::new(0.0, -1.5, 0.225)],
        [Vec3::new(1.5, 0.0, 0.0), Vec3::new(1.5, -0.84, 0.0), Vec3::new(0.84, -1.5, 0.0), Vec3::new(0.0, -1.5, 0.0)],
    ]));

    patches.push(BezierPatch3D::new([
        [Vec3::new(0.0, -2.0, 0.9), Vec3::new(-1.12, -2.0, 0.9), Vec3::new(-2.0, -1.12, 0.9), Vec3::new(-2.0, 0.0, 0.9)],
        [Vec3::new(0.0, -2.0, 0.45), Vec3::new(-1.12, -2.0, 0.45), Vec3::new(-2.0, -1.12, 0.45), Vec3::new(-2.0, 0.0, 0.45)],
        [Vec3::new(0.0, -1.5, 0.225), Vec3::new(-0.84, -1.5, 0.225), Vec3::new(-1.5, -0.84, 0.225), Vec3::new(-1.5, 0.0, 0.225)],
        [Vec3::new(0.0, -1.5, 0.0), Vec3::new(-0.84, -1.5, 0.0), Vec3::new(-1.5, -0.84, 0.0), Vec3::new(-1.5, 0.0, 0.0)],
    ]));

    patches.push(BezierPatch3D::new([
        [Vec3::new(-2.0, 0.0, 0.9), Vec3::new(-2.0, 1.12, 0.9), Vec3::new(-1.12, 2.0, 0.9), Vec3::new(0.0, 2.0, 0.9)],
        [Vec3::new(-2.0, 0.0, 0.45), Vec3::new(-2.0, 1.12, 0.45), Vec3::new(-1.12, 2.0, 0.45), Vec3::new(0.0, 2.0, 0.45)],
        [Vec3::new(-1.5, 0.0, 0.225), Vec3::new(-1.5, 0.84, 0.225), Vec3::new(-0.84, 1.5, 0.225), Vec3::new(0.0, 1.5, 0.225)],
        [Vec3::new(-1.5, 0.0, 0.0), Vec3::new(-1.5, 0.84, 0.0), Vec3::new(-0.84, 1.5, 0.0), Vec3::new(0.0, 1.5, 0.0)],
    ]));

    patches.push(BezierPatch3D::new([
        [Vec3::new(0.0, 2.0, 0.9), Vec3::new(1.12, 2.0, 0.9), Vec3::new(2.0, 1.12, 0.9), Vec3::new(2.0, 0.0, 0.9)],
        [Vec3::new(0.0, 2.0, 0.45), Vec3::new(1.12, 2.0, 0.45), Vec3::new(2.0, 1.12, 0.45), Vec3::new(2.0, 0.0, 0.45)],
        [Vec3::new(0.0, 1.5, 0.225), Vec3::new(0.84, 1.5, 0.225), Vec3::new(1.5, 0.84, 0.225), Vec3::new(1.5, 0.0, 0.225)],
        [Vec3::new(0.0, 1.5, 0.0), Vec3::new(0.84, 1.5, 0.0), Vec3::new(1.5, 0.84, 0.0), Vec3::new(1.5, 0.0, 0.0)],
    ]));

    // Simplified handle (normally would have more patches)
    patches.push(BezierPatch3D::new([
        [Vec3::new(0.0, 0.0, 3.15), Vec3::new(0.0, 0.0, 3.15), Vec3::new(0.0, 0.0, 3.15), Vec3::new(0.0, 0.0, 3.15)],
        [Vec3::new(0.8, 0.0, 3.15), Vec3::new(0.8, -0.45, 3.15), Vec3::new(0.45, -0.8, 3.15), Vec3::new(0.0, -0.8, 3.15)],
        [Vec3::new(1.4, 0.0, 2.4), Vec3::new(1.4, -0.784, 2.4), Vec3::new(0.784, -1.4, 2.4), Vec3::new(0.0, -1.4, 2.4)],
        [Vec3::new(1.4, 0.0, 2.4), Vec3::new(1.4, -0.784, 2.4), Vec3::new(0.784, -1.4, 2.4), Vec3::new(0.0, -1.4, 2.4)],
    ]));

    // Spout patches (simplified - would normally have more patches)
    patches.push(BezierPatch3D::new([
        [Vec3::new(0.0, 0.0, 3.15), Vec3::new(0.0, 0.0, 3.15), Vec3::new(0.0, 0.0, 3.15), Vec3::new(0.0, 0.0, 3.15)],
        [Vec3::new(0.0, -0.8, 3.15), Vec3::new(-0.45, -0.8, 3.15), Vec3::new(-0.8, -0.45, 3.15), Vec3::new(-0.8, 0.0, 3.15)],
        [Vec3::new(0.0, -1.4, 2.4), Vec3::new(-0.784, -1.4, 2.4), Vec3::new(-1.4, -0.784, 2.4), Vec3::new(-1.4, 0.0, 2.4)],
        [Vec3::new(0.0, -1.4, 2.4), Vec3::new(-0.784, -1.4, 2.4), Vec3::new(-1.4, -0.784, 2.4), Vec3::new(-1.4, 0.0, 2.4)],
    ]));

    // Lid patches (simplified)
    patches.push(BezierPatch3D::new([
        [Vec3::new(0.0, 0.0, 3.15), Vec3::new(0.0, 0.0, 3.15), Vec3::new(0.0, 0.0, 3.15), Vec3::new(0.0, 0.0, 3.15)],
        [Vec3::new(-0.8, 0.0, 3.15), Vec3::new(-0.8, 0.45, 3.15), Vec3::new(-0.45, 0.8, 3.15), Vec3::new(0.0, 0.8, 3.15)],
        [Vec3::new(-1.4, 0.0, 2.4), Vec3::new(-1.4, 0.784, 2.4), Vec3::new(-0.784, 1.4, 2.4), Vec3::new(0.0, 1.4, 2.4)],
        [Vec3::new(-1.4, 0.0, 2.4), Vec3::new(-1.4, 0.784, 2.4), Vec3::new(-0.784, 1.4, 2.4), Vec3::new(0.0, 1.4, 2.4)],
    ]));

    // More lid patches
    patches.push(BezierPatch3D::new([
        [Vec3::new(0.0, 0.0, 3.15), Vec3::new(0.0, 0.0, 3.15), Vec3::new(0.0, 0.0, 3.15), Vec3::new(0.0, 0.0, 3.15)],
        [Vec3::new(0.0, 0.8, 3.15), Vec3::new(0.45, 0.8, 3.15), Vec3::new(0.8, 0.45, 3.15), Vec3::new(0.8, 0.0, 3.15)],
        [Vec3::new(0.0, 1.4, 2.4), Vec3::new(0.784, 1.4, 2.4), Vec3::new(1.4, 0.784, 2.4), Vec3::new(1.4, 0.0, 2.4)],
        [Vec3::new(0.0, 1.4, 2.4), Vec3::new(0.784, 1.4, 2.4), Vec3::new(1.4, 0.784, 2.4), Vec3::new(1.4, 0.0, 2.4)],
    ]));

    // This is a simplified version of the teapot with only 16 patches
    // A complete Utah teapot has 32 patches (including the lid, handle, and spout)
    patches
}
