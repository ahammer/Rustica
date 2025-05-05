use glam::{Mat3A, Mat4, Vec3A, Vec4};
use rustica_shader_bindings::pbr_shader::*;
use wgpu::util::DeviceExt;

/// Creates and initializes a camera uniform buffer and bind group
///
/// # Example
/// ```
/// # use rustica_render_utils::create_camera_resources;
/// # use wgpu::Device;
/// # fn example(device: &Device) {
/// let (buffer, bind_group) = create_camera_resources(device);
/// # }
/// ```
pub fn create_camera_resources(
    device: &wgpu::Device,
) -> (wgpu::Buffer, WgpuBindGroup0) {
    let camera_uniform = CameraUniform {
        view_proj: Mat4::IDENTITY,
        position: Vec3A::ZERO,
    };
    
    let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Camera Uniform Buffer"),
        contents: bytemuck::cast_slice(&[camera_uniform]),
        usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
    });
    
    let bind_group = WgpuBindGroup0::from_bindings(
        device,
        WgpuBindGroup0Entries {
            camera: wgpu::BindGroupEntry {
                binding: 0,
                resource: buffer.as_entire_binding(),
            },
        },
    );
    
    (buffer, bind_group)
}

/// Updates camera uniform buffer with new view-projection matrix and position
///
/// # Example
/// ```
/// # use rustica_render_utils::update_camera;
/// # use glam::{Mat4, Vec3A};
/// # fn example(queue: &wgpu::Queue, buffer: &wgpu::Buffer) {
/// let view_proj = Mat4::IDENTITY;
/// let position = Vec3A::new(0.0, 0.0, 1.0);
/// update_camera(queue, buffer, view_proj, position);
/// # }
/// ```
pub fn update_camera(
    queue: &wgpu::Queue,
    buffer: &wgpu::Buffer,
    view_proj: Mat4,
    position: Vec3A,
) {
    let camera_uniform = CameraUniform {
        view_proj,
        position,
    };
    
    queue.write_buffer(buffer, 0, bytemuck::cast_slice(&[camera_uniform]));
}

/// Creates and initializes a model uniform buffer and bind group
///
/// # Example
/// ```
/// # use rustica_render_utils::create_model_resources;
/// # fn example(device: &wgpu::Device) {
/// let (buffer, bind_group) = create_model_resources(device);
/// # }
/// ```
pub fn create_model_resources(
    device: &wgpu::Device,
) -> (wgpu::Buffer, WgpuBindGroup1) {
    let model_uniform = ModelUniform {
        model: Mat4::IDENTITY,
        normal_transform: Mat3A::IDENTITY,
    };
    
    let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Model Uniform Buffer"),
        contents: bytemuck::cast_slice(&[model_uniform]),
        usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
    });
    
    let bind_group = WgpuBindGroup1::from_bindings(
        device,
        WgpuBindGroup1Entries {
            model: wgpu::BindGroupEntry {
                binding: 0,
                resource: buffer.as_entire_binding(),
            },
        },
    );
    
    (buffer, bind_group)
}

/// Updates model uniform buffer with new transform
///
/// # Example
/// ```
/// # use rustica_render_utils::update_model_transform;
/// # use glam::Mat4;
/// # fn example(queue: &wgpu::Queue, buffer: &wgpu::Buffer) {
/// let transform = Mat4::IDENTITY;
/// update_model_transform(queue, buffer, transform);
/// # }
/// ```
pub fn update_model_transform(
    queue: &wgpu::Queue,
    buffer: &wgpu::Buffer,
    transform: Mat4,
) {
    let normal_transform = Mat3A::from_mat4(transform).inverse().transpose();
    let model_uniform = ModelUniform {
        model: transform,
        normal_transform,
    };
    
    queue.write_buffer(buffer, 0, bytemuck::cast_slice(&[model_uniform]));
}

/// Creates and initializes a material uniform buffer and bind group
///
/// # Example
/// ```
/// # use rustica_render_utils::create_material_resources;
/// # use glam::Vec4;
/// # fn example(device: &wgpu::Device) {
/// let (buffer, bind_group) = create_material_resources(
///     device, 
///     Vec4::new(1.0, 0.0, 0.0, 1.0),  // red
///     0.0,   // non-metallic
///     1.0    // rough
/// );
/// # }
/// ```
pub fn create_material_resources(
    device: &wgpu::Device,
    base_color: Vec4,
    metallic: f32,
    roughness: f32,
) -> (wgpu::Buffer, WgpuBindGroup2) {
    let material_uniform_init = MaterialUniformInit {
        base_color_factor: base_color,
        metallic_factor: metallic,
        roughness_factor: roughness,
    };
    let material_uniform: MaterialUniform = material_uniform_init.into();
    
    let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Material Uniform Buffer"),
        contents: bytemuck::cast_slice(&[material_uniform]),
        usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
    });
    
    let bind_group = WgpuBindGroup2::from_bindings(
        device,
        WgpuBindGroup2Entries {
            material: wgpu::BindGroupEntry {
                binding: 0,
                resource: buffer.as_entire_binding(),
            },
        },
    );
    
    (buffer, bind_group)
}
