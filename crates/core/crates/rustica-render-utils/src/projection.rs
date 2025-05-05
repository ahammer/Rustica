use glam::Mat4;

/// Simple utility for orthographic projection matrices
///
/// # Example
/// ```
/// # use rustica_render_utils::create_orthographic_projection;
/// let width = 800;
/// let height = 600;
/// let projection = create_orthographic_projection(width, height);
/// ```
pub fn create_orthographic_projection(
    width: u32, 
    height: u32,
) -> Mat4 {
    let aspect_ratio = width as f32 / height as f32;
    let (left, right, bottom, top) = if aspect_ratio > 1.0 {
        (-1.0 * aspect_ratio, 1.0 * aspect_ratio, -1.0, 1.0)
    } else {
        (-1.0, 1.0, -1.0 / aspect_ratio, 1.0 / aspect_ratio)
    };
    Mat4::orthographic_rh(left, right, bottom, top, -1.0, 1.0)
}
