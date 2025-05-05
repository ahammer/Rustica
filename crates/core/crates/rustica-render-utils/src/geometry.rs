use rustica_shader_bindings::pbr_shader::VertexInput;
use wgpu::util::DeviceExt;

/// Creates a vertex buffer from vertex data
///
/// # Example
/// ```
/// # use rustica_render_utils::{create_vertex_buffer, VertexInput};
/// # use glam::Vec3A;
/// # fn example(device: &wgpu::Device) {
/// let vertices = &[
///     VertexInput {
///         position: Vec3A::new(0.0, 0.5, 0.0),
///         normal: Vec3A::new(0.0, 0.0, 1.0),
///         uv: [0.5, 0.0],
///     }
/// ];
/// let (buffer, count) = create_vertex_buffer(device, vertices);
/// # }
/// ```
pub fn create_vertex_buffer(
    device: &wgpu::Device,
    vertices: &[VertexInput],
) -> (wgpu::Buffer, u32) {
    let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Vertex Buffer"),
        contents: bytemuck::cast_slice(vertices),
        usage: wgpu::BufferUsages::VERTEX,
    });
    
    (buffer, vertices.len() as u32)
}
