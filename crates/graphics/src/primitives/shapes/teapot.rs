// // Utah teapot implementation for Rustica graphics crate

// use cgmath::Vector3;
// use rustica_foundation::geometry::{Mesh as FoundationMesh, BezierPatch, Face};


// /// Utah teapot data - control points for the Bezier patches
// pub struct UtahTeapot;

// impl UtahTeapot {
//     /// Get the control points for the Utah teapot
//     pub fn patches() -> Vec<[[Vector3<f32>; 4]; 4]> {
//         // The classic Utah teapot consists of 32 bicubic Bezier patches
//         // Here we define the control points for each patch
        
//         // This is the original data from the Utah teapot, scaled and adjusted
//         // Each patch is a 4x4 grid of control points
//         vec![
//             // Patch 0: Rim
//             [
//                 [Vector3::new(1.4, 0.0, 2.4), Vector3::new(1.4, -0.784, 2.4), Vector3::new(0.784, -1.4, 2.4), Vector3::new(0.0, -1.4, 2.4)],
//                 [Vector3::new(1.3375, 0.0, 2.53125), Vector3::new(1.3375, -0.749, 2.53125), Vector3::new(0.749, -1.3375, 2.53125), Vector3::new(0.0, -1.3375, 2.53125)],
//                 [Vector3::new(1.4375, 0.0, 2.53125), Vector3::new(1.4375, -0.805, 2.53125), Vector3::new(0.805, -1.4375, 2.53125), Vector3::new(0.0, -1.4375, 2.53125)],
//                 [Vector3::new(1.5, 0.0, 2.4), Vector3::new(1.5, -0.84, 2.4), Vector3::new(0.84, -1.5, 2.4), Vector3::new(0.0, -1.5, 2.4)],
//             ],
//             // Patch 1: Rim
//             [
//                 [Vector3::new(0.0, -1.4, 2.4), Vector3::new(-0.784, -1.4, 2.4), Vector3::new(-1.4, -0.784, 2.4), Vector3::new(-1.4, 0.0, 2.4)],
//                 [Vector3::new(0.0, -1.3375, 2.53125), Vector3::new(-0.749, -1.3375, 2.53125), Vector3::new(-1.3375, -0.749, 2.53125), Vector3::new(-1.3375, 0.0, 2.53125)],
//                 [Vector3::new(0.0, -1.4375, 2.53125), Vector3::new(-0.805, -1.4375, 2.53125), Vector3::new(-1.4375, -0.805, 2.53125), Vector3::new(-1.4375, 0.0, 2.53125)],
//                 [Vector3::new(0.0, -1.5, 2.4), Vector3::new(-0.84, -1.5, 2.4), Vector3::new(-1.5, -0.84, 2.4), Vector3::new(-1.5, 0.0, 2.4)],
//             ],
//             // Patch 2: Rim
//             [
//                 [Vector3::new(-1.4, 0.0, 2.4), Vector3::new(-1.4, 0.784, 2.4), Vector3::new(-0.784, 1.4, 2.4), Vector3::new(0.0, 1.4, 2.4)],
//                 [Vector3::new(-1.3375, 0.0, 2.53125), Vector3::new(-1.3375, 0.749, 2.53125), Vector3::new(-0.749, 1.3375, 2.53125), Vector3::new(0.0, 1.3375, 2.53125)],
//                 [Vector3::new(-1.4375, 0.0, 2.53125), Vector3::new(-1.4375, 0.805, 2.53125), Vector3::new(-0.805, 1.4375, 2.53125), Vector3::new(0.0, 1.4375, 2.53125)],
//                 [Vector3::new(-1.5, 0.0, 2.4), Vector3::new(-1.5, 0.84, 2.4), Vector3::new(-0.84, 1.5, 2.4), Vector3::new(0.0, 1.5, 2.4)],
//             ],
//             // Patch 3: Rim
//             [
//                 [Vector3::new(0.0, 1.4, 2.4), Vector3::new(0.784, 1.4, 2.4), Vector3::new(1.4, 0.784, 2.4), Vector3::new(1.4, 0.0, 2.4)],
//                 [Vector3::new(0.0, 1.3375, 2.53125), Vector3::new(0.749, 1.3375, 2.53125), Vector3::new(1.3375, 0.749, 2.53125), Vector3::new(1.3375, 0.0, 2.53125)],
//                 [Vector3::new(0.0, 1.4375, 2.53125), Vector3::new(0.805, 1.4375, 2.53125), Vector3::new(1.4375, 0.805, 2.53125), Vector3::new(1.4375, 0.0, 2.53125)],
//                 [Vector3::new(0.0, 1.5, 2.4), Vector3::new(0.84, 1.5, 2.4), Vector3::new(1.5, 0.84, 2.4), Vector3::new(1.5, 0.0, 2.4)],
//             ],
//             // Patch 4: Body
//             [
//                 [Vector3::new(1.5, 0.0, 2.4), Vector3::new(1.5, -0.84, 2.4), Vector3::new(0.84, -1.5, 2.4), Vector3::new(0.0, -1.5, 2.4)],
//                 [Vector3::new(1.75, 0.0, 1.875), Vector3::new(1.75, -0.98, 1.875), Vector3::new(0.98, -1.75, 1.875), Vector3::new(0.0, -1.75, 1.875)],
//                 [Vector3::new(2.0, 0.0, 1.35), Vector3::new(2.0, -1.12, 1.35), Vector3::new(1.12, -2.0, 1.35), Vector3::new(0.0, -2.0, 1.35)],
//                 [Vector3::new(2.0, 0.0, 0.9), Vector3::new(2.0, -1.12, 0.9), Vector3::new(1.12, -2.0, 0.9), Vector3::new(0.0, -2.0, 0.9)],
//             ],
//             // Patch 5: Body
//             [
//                 [Vector3::new(0.0, -1.5, 2.4), Vector3::new(-0.84, -1.5, 2.4), Vector3::new(-1.5, -0.84, 2.4), Vector3::new(-1.5, 0.0, 2.4)],
//                 [Vector3::new(0.0, -1.75, 1.875), Vector3::new(-0.98, -1.75, 1.875), Vector3::new(-1.75, -0.98, 1.875), Vector3::new(-1.75, 0.0, 1.875)],
//                 [Vector3::new(0.0, -2.0, 1.35), Vector3::new(-1.12, -2.0, 1.35), Vector3::new(-2.0, -1.12, 1.35), Vector3::new(-2.0, 0.0, 1.35)],
//                 [Vector3::new(0.0, -2.0, 0.9), Vector3::new(-1.12, -2.0, 0.9), Vector3::new(-2.0, -1.12, 0.9), Vector3::new(-2.0, 0.0, 0.9)],
//             ],
//             // Patch 6: Body
//             [
//                 [Vector3::new(-1.5, 0.0, 2.4), Vector3::new(-1.5, 0.84, 2.4), Vector3::new(-0.84, 1.5, 2.4), Vector3::new(0.0, 1.5, 2.4)],
//                 [Vector3::new(-1.75, 0.0, 1.875), Vector3::new(-1.75, 0.98, 1.875), Vector3::new(-0.98, 1.75, 1.875), Vector3::new(0.0, 1.75, 1.875)],
//                 [Vector3::new(-2.0, 0.0, 1.35), Vector3::new(-2.0, 1.12, 1.35), Vector3::new(-1.12, 2.0, 1.35), Vector3::new(0.0, 2.0, 1.35)],
//                 [Vector3::new(-2.0, 0.0, 0.9), Vector3::new(-2.0, 1.12, 0.9), Vector3::new(-1.12, 2.0, 0.9), Vector3::new(0.0, 2.0, 0.9)],
//             ],
//             // Patch 7: Body
//             [
//                 [Vector3::new(0.0, 1.5, 2.4), Vector3::new(0.84, 1.5, 2.4), Vector3::new(1.5, 0.84, 2.4), Vector3::new(1.5, 0.0, 2.4)],
//                 [Vector3::new(0.0, 1.75, 1.875), Vector3::new(0.98, 1.75, 1.875), Vector3::new(1.75, 0.98, 1.875), Vector3::new(1.75, 0.0, 1.875)],
//                 [Vector3::new(0.0, 2.0, 1.35), Vector3::new(1.12, 2.0, 1.35), Vector3::new(2.0, 1.12, 1.35), Vector3::new(2.0, 0.0, 1.35)],
//                 [Vector3::new(0.0, 2.0, 0.9), Vector3::new(1.12, 2.0, 0.9), Vector3::new(2.0, 1.12, 0.9), Vector3::new(2.0, 0.0, 0.9)],
//             ],
//             // Patch 8: Body
//             [
//                 [Vector3::new(2.0, 0.0, 0.9), Vector3::new(2.0, -1.12, 0.9), Vector3::new(1.12, -2.0, 0.9), Vector3::new(0.0, -2.0, 0.9)],
//                 [Vector3::new(2.0, 0.0, 0.45), Vector3::new(2.0, -1.12, 0.45), Vector3::new(1.12, -2.0, 0.45), Vector3::new(0.0, -2.0, 0.45)],
//                 [Vector3::new(1.5, 0.0, 0.225), Vector3::new(1.5, -0.84, 0.225), Vector3::new(0.84, -1.5, 0.225), Vector3::new(0.0, -1.5, 0.225)],
//                 [Vector3::new(1.5, 0.0, 0.15), Vector3::new(1.5, -0.84, 0.15), Vector3::new(0.84, -1.5, 0.15), Vector3::new(0.0, -1.5, 0.15)],
//             ],
//             // Patch 9: Body
//             [
//                 [Vector3::new(0.0, -2.0, 0.9), Vector3::new(-1.12, -2.0, 0.9), Vector3::new(-2.0, -1.12, 0.9), Vector3::new(-2.0, 0.0, 0.9)],
//                 [Vector3::new(0.0, -2.0, 0.45), Vector3::new(-1.12, -2.0, 0.45), Vector3::new(-2.0, -1.12, 0.45), Vector3::new(-2.0, 0.0, 0.45)],
//                 [Vector3::new(0.0, -1.5, 0.225), Vector3::new(-0.84, -1.5, 0.225), Vector3::new(-1.5, -0.84, 0.225), Vector3::new(-1.5, 0.0, 0.225)],
//                 [Vector3::new(0.0, -1.5, 0.15), Vector3::new(-0.84, -1.5, 0.15), Vector3::new(-1.5, -0.84, 0.15), Vector3::new(-1.5, 0.0, 0.15)],
//             ],
//             // Patch 10: Body
//             [
//                 [Vector3::new(-2.0, 0.0, 0.9), Vector3::new(-2.0, 1.12, 0.9), Vector3::new(-1.12, 2.0, 0.9), Vector3::new(0.0, 2.0, 0.9)],
//                 [Vector3::new(-2.0, 0.0, 0.45), Vector3::new(-2.0, 1.12, 0.45), Vector3::new(-1.12, 2.0, 0.45), Vector3::new(0.0, 2.0, 0.45)],
//                 [Vector3::new(-1.5, 0.0, 0.225), Vector3::new(-1.5, 0.84, 0.225), Vector3::new(-0.84, 1.5, 0.225), Vector3::new(0.0, 1.5, 0.225)],
//                 [Vector3::new(-1.5, 0.0, 0.15), Vector3::new(-1.5, 0.84, 0.15), Vector3::new(-0.84, 1.5, 0.15), Vector3::new(0.0, 1.5, 0.15)],
//             ],
//             // Patch 11: Body
//             [
//                 [Vector3::new(0.0, 2.0, 0.9), Vector3::new(1.12, 2.0, 0.9), Vector3::new(2.0, 1.12, 0.9), Vector3::new(2.0, 0.0, 0.9)],
//                 [Vector3::new(0.0, 2.0, 0.45), Vector3::new(1.12, 2.0, 0.45), Vector3::new(2.0, 1.12, 0.45), Vector3::new(2.0, 0.0, 0.45)],
//                 [Vector3::new(0.0, 1.5, 0.225), Vector3::new(0.84, 1.5, 0.225), Vector3::new(1.5, 0.84, 0.225), Vector3::new(1.5, 0.0, 0.225)],
//                 [Vector3::new(0.0, 1.5, 0.15), Vector3::new(0.84, 1.5, 0.15), Vector3::new(1.5, 0.84, 0.15), Vector3::new(1.5, 0.0, 0.15)],
//             ],
//             // Patch 12: Bottom
//             [
//                 [Vector3::new(1.5, 0.0, 0.15), Vector3::new(1.5, -0.84, 0.15), Vector3::new(0.84, -1.5, 0.15), Vector3::new(0.0, -1.5, 0.15)],
//                 [Vector3::new(1.5, 0.0, 0.075), Vector3::new(1.5, -0.84, 0.075), Vector3::new(0.84, -1.5, 0.075), Vector3::new(0.0, -1.5, 0.075)],
//                 [Vector3::new(1.425, 0.0, 0.0), Vector3::new(1.425, -0.798, 0.0), Vector3::new(0.798, -1.425, 0.0), Vector3::new(0.0, -1.425, 0.0)],
//                 [Vector3::new(1.5, 0.0, -0.075), Vector3::new(1.5, -0.84, -0.075), Vector3::new(0.84, -1.5, -0.075), Vector3::new(0.0, -1.5, -0.075)],
//             ],
//             // Patch 13: Bottom
//             [
//                 [Vector3::new(0.0, -1.5, 0.15), Vector3::new(-0.84, -1.5, 0.15), Vector3::new(-1.5, -0.84, 0.15), Vector3::new(-1.5, 0.0, 0.15)],
//                 [Vector3::new(0.0, -1.5, 0.075), Vector3::new(-0.84, -1.5, 0.075), Vector3::new(-1.5, -0.84, 0.075), Vector3::new(-1.5, 0.0, 0.075)],
//                 [Vector3::new(0.0, -1.425, 0.0), Vector3::new(-0.798, -1.425, 0.0), Vector3::new(-1.425, -0.798, 0.0), Vector3::new(-1.425, 0.0, 0.0)],
//                 [Vector3::new(0.0, -1.5, -0.075), Vector3::new(-0.84, -1.5, -0.075), Vector3::new(-1.5, -0.84, -0.075), Vector3::new(-1.5, 0.0, -0.075)],
//             ],
//             // Patch 14: Bottom
//             [
//                 [Vector3::new(-1.5, 0.0, 0.15), Vector3::new(-1.5, 0.84, 0.15), Vector3::new(-0.84, 1.5, 0.15), Vector3::new(0.0, 1.5, 0.15)],
//                 [Vector3::new(-1.5, 0.0, 0.075), Vector3::new(-1.5, 0.84, 0.075), Vector3::new(-0.84, 1.5, 0.075), Vector3::new(0.0, 1.5, 0.075)],
//                 [Vector3::new(-1.425, 0.0, 0.0), Vector3::new(-1.425, 0.798, 0.0), Vector3::new(-0.798, 1.425, 0.0), Vector3::new(0.0, 1.425, 0.0)],
//                 [Vector3::new(-1.5, 0.0, -0.075), Vector3::new(-1.5, 0.84, -0.075), Vector3::new(-0.84, 1.5, -0.075), Vector3::new(0.0, 1.5, -0.075)],
//             ],
//             // Patch 15: Bottom
//             [
//                 [Vector3::new(0.0, 1.5, 0.15), Vector3::new(0.84, 1.5, 0.15), Vector3::new(1.5, 0.84, 0.15), Vector3::new(1.5, 0.0, 0.15)],
//                 [Vector3::new(0.0, 1.5, 0.075), Vector3::new(0.84, 1.5, 0.075), Vector3::new(1.5, 0.84, 0.075), Vector3::new(1.5, 0.0, 0.075)],
//                 [Vector3::new(0.0, 1.425, 0.0), Vector3::new(0.798, 1.425, 0.0), Vector3::new(1.425, 0.798, 0.0), Vector3::new(1.425, 0.0, 0.0)],
//                 [Vector3::new(0.0, 1.5, -0.075), Vector3::new(0.84, 1.5, -0.075), Vector3::new(1.5, 0.84, -0.075), Vector3::new(1.5, 0.0, -0.075)],
//             ],
//             // Patch 16: Handle
//             [
//                 [Vector3::new(0.0, 0.0, 3.15), Vector3::new(0.0, 0.0, 3.15), Vector3::new(0.0, 0.0, 3.15), Vector3::new(0.0, 0.0, 3.15)],
//                 [Vector3::new(0.8, 0.0, 3.15), Vector3::new(0.8, -0.45, 3.15), Vector3::new(0.45, -0.8, 3.15), Vector3::new(0.0, -0.8, 3.15)],
//                 [Vector3::new(0.0, 0.0, 2.85), Vector3::new(1.4, 0.0, 2.85), Vector3::new(1.4, -0.784, 2.85), Vector3::new(0.784, -1.4, 2.85)],
//                 [Vector3::new(0.0, 0.0, 2.7), Vector3::new(1.3, 0.0, 2.7), Vector3::new(1.3, -0.728, 2.7), Vector3::new(0.728, -1.3, 2.7)],
//             ],
//             // Patch 17: Handle
//             [
//                 [Vector3::new(0.0, 0.0, 3.15), Vector3::new(0.0, 0.0, 3.15), Vector3::new(0.0, 0.0, 3.15), Vector3::new(0.0, 0.0, 3.15)],
//                 [Vector3::new(0.0, -0.8, 3.15), Vector3::new(-0.45, -0.8, 3.15), Vector3::new(-0.8, -0.45, 3.15), Vector3::new(-0.8, 0.0, 3.15)],
//                 [Vector3::new(0.784, -1.4, 2.85), Vector3::new(0.0, -1.4, 2.85), Vector3::new(-0.784, -1.4, 2.85), Vector3::new(-1.4, -0.784, 2.85)],
//                 [Vector3::new(0.728, -1.3, 2.7), Vector3::new(0.0, -1.3, 2.7), Vector3::new(-0.728, -1.3, 2.7), Vector3::new(-1.3, -0.728, 2.7)],
//             ],
//             // Patch 18: Handle
//             [
//                 [Vector3::new(0.0, 0.0, 3.15), Vector3::new(0.0, 0.0, 3.15), Vector3::new(0.0, 0.0, 3.15), Vector3::new(0.0, 0.0, 3.15)],
//                 [Vector3::new(-0.8, 0.0, 3.15), Vector3::new(-0.8, 0.45, 3.15), Vector3::new(-0.45, 0.8, 3.15), Vector3::new(0.0, 0.8, 3.15)],
//                 [Vector3::new(-1.4, -0.784, 2.85), Vector3::new(-1.4, 0.0, 2.85), Vector3::new(-1.4, 0.784, 2.85), Vector3::new(-0.784, 1.4, 2.85)],
//                 [Vector3::new(-1.3, -0.728, 2.7), Vector3::new(-1.3, 0.0, 2.7), Vector3::new(-1.3, 0.728, 2.7), Vector3::new(-0.728, 1.3, 2.7)],
//             ],
//             // Patch 19: Handle
//             [
//                 [Vector3::new(0.0, 0.0, 3.15), Vector3::new(0.0, 0.0, 3.15), Vector3::new(0.0, 0.0, 3.15), Vector3::new(0.0, 0.0, 3.15)],
//                 [Vector3::new(0.0, 0.8, 3.15), Vector3::new(0.45, 0.8, 3.15), Vector3::new(0.8, 0.45, 3.15), Vector3::new(0.8, 0.0, 3.15)],
//                 [Vector3::new(-0.784, 1.4, 2.85), Vector3::new(0.0, 1.4, 2.85), Vector3::new(0.784, 1.4, 2.85), Vector3::new(1.4, 0.784, 2.85)],
//                 [Vector3::new(-0.728, 1.3, 2.7), Vector3::new(0.0, 1.3, 2.7), Vector3::new(0.728, 1.3, 2.7), Vector3::new(1.3, 0.728, 2.7)],
//             ],
//             // Patch 20: Spout
//             [
//                 [Vector3::new(2.0, 0.0, 2.7), Vector3::new(2.0, -1.12, 2.7), Vector3::new(1.12, -2.0, 2.7), Vector3::new(0.0, -2.0, 2.7)],
//                 [Vector3::new(2.0, 0.0, 2.55), Vector3::new(2.0, -1.12, 2.55), Vector3::new(1.12, -2.0, 2.55), Vector3::new(0.0, -2.0, 2.55)],
//                 [Vector3::new(2.5, 0.0, 2.4), Vector3::new(2.5, -1.4, 2.4), Vector3::new(1.4, -2.5, 2.4), Vector3::new(0.0, -2.5, 2.4)],
//                 [Vector3::new(2.3, 0.0, 2.4), Vector3::new(2.3, -1.288, 2.4), Vector3::new(1.288, -2.3, 2.4), Vector3::new(0.0, -2.3, 2.4)],
//             ],
//             // Patch 21: Spout
//             [
//                 [Vector3::new(0.0, -2.0, 2.7), Vector3::new(-1.12, -2.0, 2.7), Vector3::new(-2.0, -1.12, 2.7), Vector3::new(-2.0, 0.0, 2.7)],
//                 [Vector3::new(0.0, -2.0, 2.55), Vector3::new(-1.12, -2.0, 2.55), Vector3::new(-2.0, -1.12, 2.55), Vector3::new(-2.0, 0.0, 2.55)],
//                 [Vector3::new(0.0, -2.5, 2.4), Vector3::new(-1.4, -2.5, 2.4), Vector3::new(-2.5, -1.4, 2.4), Vector3::new(-2.5, 0.0, 2.4)],
//                 [Vector3::new(0.0, -2.3, 2.4), Vector3::new(-1.288, -2.3, 2.4), Vector3::new(-2.3, -1.288, 2.4), Vector3::new(-2.3, 0.0, 2.4)],
//             ],
//             // Patch 22: Spout
//             [
//                 [Vector3::new(-2.0, 0.0, 2.7), Vector3::new(-2.0, 1.12, 2.7), Vector3::new(-1.12, 2.0, 2.7), Vector3::new(0.0, 2.0, 2.7)],
//                 [Vector3::new(-2.0, 0.0, 2.55), Vector3::new(-2.0, 1.12, 2.55), Vector3::new(-1.12, 2.0, 2.55), Vector3::new(0.0, 2.0, 2.55)],
//                 [Vector3::new(-2.5, 0.0, 2.4), Vector3::new(-2.5, 1.4, 2.4), Vector3::new(-1.4, 2.5, 2.4), Vector3::new(0.0, 2.5, 2.4)],
//                 [Vector3::new(-2.3, 0.0, 2.4), Vector3::new(-2.3, 1.288, 2.4), Vector3::new(-1.288, 2.3, 2.4), Vector3::new(0.0, 2.3, 2.4)],
//             ],
//             // Patch 23: Spout
//             [
//                 [Vector3::new(0.0, 2.0, 2.7), Vector3::new(1.12, 2.0, 2.7), Vector3::new(2.0, 1.12, 2.7), Vector3::new(2.0, 0.0, 2.7)],
//                 [Vector3::new(0.0, 2.0, 2.55), Vector3::new(1.12, 2.0, 2.55), Vector3::new(2.0, 1.12, 2.55), Vector3::new(2.0, 0.0, 2.55)],
//                 [Vector3::new(0.0, 2.5, 2.4), Vector3::new(1.4, 2.5, 2.4), Vector3::new(2.5, 1.4, 2.4), Vector3::new(2.5, 0.0, 2.4)],
//                 [Vector3::new(0.0, 2.3, 2.4), Vector3::new(1.288, 2.3, 2.4), Vector3::new(2.3, 1.288, 2.4), Vector3::new(2.3, 0.0, 2.4)],
//             ],
//             // Patch 24: Lid
//             [
//                 [Vector3::new(0.0, 0.0, 3.15), Vector3::new(0.0, 0.0, 3.15), Vector3::new(0.0, 0.0, 3.15), Vector3::new(0.0, 0.0, 3.15)],
//                 [Vector3::new(0.2, 0.0, 3.15), Vector3::new(0.2, -0.112, 3.15), Vector3::new(0.112, -0.2, 3.15), Vector3::new(0.0, -0.2, 3.15)],
//                 [Vector3::new(0.4, 0.0, 3.15), Vector3::new(0.4, -0.224, 3.15), Vector3::new(0.224, -0.4, 3.15), Vector3::new(0.0, -0.4, 3.15)],
//                 [Vector3::new(1.3, 0.0, 2.7), Vector3::new(1.3, -0.728, 2.7), Vector3::new(0.728, -1.3, 2.7), Vector3::new(0.0, -1.3, 2.7)],
//             ],
//             // Patch 25: Lid
//             [
//                 [Vector3::new(0.0, 0.0, 3.15), Vector3::new(0.0, 0.0, 3.15), Vector3::new(0.0, 0.0, 3.15), Vector3::new(0.0, 0.0, 3.15)],
//                 [Vector3::new(0.0, -0.2, 3.15), Vector3::new(-0.112, -0.2, 3.15), Vector3::new(-0.2, -0.112, 3.15), Vector3::new(-0.2, 0.0, 3.15)],
//                 [Vector3::new(0.0, -0.4, 3.15), Vector3::new(-0.224, -0.4, 3.15), Vector3::new(-0.4, -0.224, 3.15), Vector3::new(-0.4, 0.0, 3.15)],
//                 [Vector3::new(0.0, -1.3, 2.7), Vector3::new(-0.728, -1.3, 2.7), Vector3::new(-1.3, -0.728, 2.7), Vector3::new(-1.3, 0.0, 2.7)],
//             ],
//             // Patch 26: Lid
//             [
//                 [Vector3::new(0.0, 0.0, 3.15), Vector3::new(0.0, 0.0, 3.15), Vector3::new(0.0, 0.0, 3.15), Vector3::new(0.0, 0.0, 3.15)],
//                 [Vector3::new(-0.2, 0.0, 3.15), Vector3::new(-0.2, 0.112, 3.15), Vector3::new(-0.112, 0.2, 3.15), Vector3::new(0.0, 0.2, 3.15)],
//                 [Vector3::new(-0.4, 0.0, 3.15), Vector3::new(-0.4, 0.224, 3.15), Vector3::new(-0.224, 0.4, 3.15), Vector3::new(0.0, 0.4, 3.15)],
//                 [Vector3::new(-1.3, 0.0, 2.7), Vector3::new(-1.3, 0.728, 2.7), Vector3::new(-0.728, 1.3, 2.7), Vector3::new(0.0, 1.3, 2.7)],
//             ],
//             // Patch 27: Lid Knob (Top)
//             [
//                 [Vector3::new(0.0, 0.0, 3.15), Vector3::new(0.0, 0.0, 3.15), Vector3::new(0.0, 0.0, 3.15), Vector3::new(0.0, 0.0, 3.15)],
//                 [Vector3::new(0.0, 0.0, 3.15), Vector3::new(0.0, 0.0, 3.15), Vector3::new(0.0, 0.0, 3.15), Vector3::new(0.0, 0.0, 3.15)],
//                 [Vector3::new(0.0, 0.0, 3.15), Vector3::new(0.0, 0.0, 3.15), Vector3::new(0.0, 0.0, 3.15), Vector3::new(0.0, 0.0, 3.15)],
//                 [Vector3::new(0.0, 0.0, 3.15), Vector3::new(0.0, 0.0, 3.15), Vector3::new(0.0, 0.0, 3.15), Vector3::new(0.0, 0.0, 3.15)],
//             ],
//             // Patch 28: Lid Knob (Side 1)
//             [
//                 [Vector3::new(0.0, 0.0, 3.15), Vector3::new(0.0, 0.0, 3.15), Vector3::new(0.0, 0.0, 3.15), Vector3::new(0.0, 0.0, 3.15)],
//                 [Vector3::new(0.0, 0.0, 3.15), Vector3::new(0.0, -0.002, 3.15), Vector3::new(-0.002, 0.0, 3.15), Vector3::new(-0.2, 0.0, 3.6)],
//                 [Vector3::new(0.0, 0.0, 3.15), Vector3::new(0.0, -0.202, 3.15), Vector3::new(-0.202, 0.0, 3.15), Vector3::new(-0.2, 0.0, 3.6)],
//                 [Vector3::new(0.0, 0.0, 3.15), Vector3::new(0.0, 0.0, 3.15), Vector3::new(0.0, 0.0, 3.15), Vector3::new(0.0, 0.0, 3.3)],
//             ],
//             // Patch 29: Lid Knob (Side 2)
//             [
//                 [Vector3::new(0.0, 0.0, 3.15), Vector3::new(0.0, 0.0, 3.15), Vector3::new(0.0, 0.0, 3.15), Vector3::new(0.0, 0.0, 3.15)],
//                 [Vector3::new(-0.2, 0.0, 3.6), Vector3::new(-0.2, 0.0, 3.6), Vector3::new(-0.2, 0.0, 3.6), Vector3::new(-0.2, 0.0, 3.6)],
//                 [Vector3::new(-0.2, 0.0, 3.6), Vector3::new(-0.2, 0.112, 3.6), Vector3::new(-0.112, 0.2, 3.6), Vector3::new(0.0, 0.2, 3.6)],
//                 [Vector3::new(0.0, 0.0, 3.3), Vector3::new(0.0, 0.0, 3.3), Vector3::new(0.0, 0.0, 3.3), Vector3::new(0.0, 0.0, 3.3)],
//             ],
//             // Patch 30: Lid Knob (Side 3)
//             [
//                 [Vector3::new(0.0, 0.0, 3.15), Vector3::new(0.0, 0.0, 3.15), Vector3::new(0.0, 0.0, 3.15), Vector3::new(0.0, 0.0, 3.15)],
//                 [Vector3::new(0.0, 0.0, 3.15), Vector3::new(0.0, 0.002, 3.15), Vector3::new(0.002, 0.0, 3.15), Vector3::new(0.2, 0.0, 3.6)],
//                 [Vector3::new(0.0, 0.0, 3.15), Vector3::new(0.0, 0.202, 3.15), Vector3::new(0.202, 0.0, 3.15), Vector3::new(0.2, 0.0, 3.6)],
//                 [Vector3::new(0.0, 0.0, 3.15), Vector3::new(0.0, 0.0, 3.15), Vector3::new(0.0, 0.0, 3.15), Vector3::new(0.0, 0.0, 3.3)],
//             ],
//         ]
//     }

//     /// Convert the Utah teapot patches to BezierPatch objects
//     pub fn bezier_patches() -> Vec<BezierPatch> {
//         let patches_data = Self::patches();
//         let mut bezier_patches = Vec::with_capacity(patches_data.len());
        
//         for patch_data in patches_data {
//             // Convert the 4x4 array to Vec<Vec<Vector3<f32>>>
//             let control_points = patch_data.iter()
//                 .map(|row| row.to_vec())
//                 .collect();
            
//             bezier_patches.push(BezierPatch::new(control_points));
//         }
        
//         bezier_patches
//     }
// }

// /// Create a teapot mesh using Bezier patches
// /// 
// /// # Arguments
// /// * `size` - Scale factor for the teapot (1.0 is the original size)
// /// * `tessellation_level` - Number of segments to use when tessellating each patch
// pub fn create_teapot(size: f32, tessellation_level: u32) -> FoundationMesh {
//     // Get the Bezier patches for the Utah teapot
//     let patches = UtahTeapot::bezier_patches();
    
//     // Tessellate each patch and combine them into a single mesh
//     let mut combined_mesh = FoundationMesh::new();
//     let segments = tessellation_level as usize;
    
//     for patch in patches {
//         let mut patch_mesh = patch.tessellate(segments, segments);
        
//         // Ensure consistent counterclockwise winding
//         for face in &mut patch_mesh.faces {
//             // Swap indices 1 and 2 to reverse winding if needed
//             // This ensures all faces have counterclockwise winding
            
//                 let temp = face.indices[1];
//                 face.indices[1] = face.indices[2];
//                 face.indices[2] = temp;
            
//         }
        
//         // Get the current vertex count (for indexing)
//         let vertex_offset = combined_mesh.vertices.len() as u32;
        
//         // Add vertices from this patch
//         combined_mesh.vertices.extend(patch_mesh.vertices);
        
//         // Add faces from this patch (adjusting indices)
//         for face in patch_mesh.faces {
//             combined_mesh.faces.push(Face::new(
//                 face.indices[0] + vertex_offset,
//                 face.indices[1] + vertex_offset,
//                 face.indices[2] + vertex_offset
//             ));
//         }
//     }
    
//     // Scale the teapot if needed
//     if size != 1.0 {
//         for vertex in &mut combined_mesh.vertices {
//             vertex.position[0] *= size;
//             vertex.position[1] *= size;
//             vertex.position[2] *= size;
//         }
//     }
    
//     combined_mesh
// }

// /// Create a teapot mesh with default parameters
// pub fn create_default_teapot() -> FoundationMesh {
//     create_teapot(0.5, 10)
// }
