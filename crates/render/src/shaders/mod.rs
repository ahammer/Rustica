// Shader module for the rendering system

use wgpu::{Device, RenderPipeline};
use crate::shader_types::ShaderType;

/// Initialize shaders and create render pipelines
/// 
/// # Deprecated
/// This function is deprecated and will be removed in a future version.
/// Use the modern shader API with `ShaderDescriptor` and `draw_with_shader` instead.
#[deprecated(
    since = "0.2.0",
    note = "This is being replaced by the custom shader API. Use the ShaderDescriptor derive macro instead."
)]
pub fn initialize_shaders(
    device: &Device,
    format: wgpu::TextureFormat,
) -> std::collections::HashMap<ShaderType, RenderPipeline> {
    let mut shader_pipelines = std::collections::HashMap::new();
    
    // Define vertex structure for the debug triangle
    #[repr(C)]
    #[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
    struct Vertex {
        position: [f32; 3],
        color: [f32; 3],
    }
    
    // Define vertex buffer layout
    let vertex_buffer_layout = wgpu::VertexBufferLayout {
        array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
        step_mode: wgpu::VertexStepMode::Vertex,
        attributes: &[
            wgpu::VertexAttribute {
                offset: 0,
                shader_location: 0,
                format: wgpu::VertexFormat::Float32x3,
            },
            wgpu::VertexAttribute {
                offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                shader_location: 1,
                format: wgpu::VertexFormat::Float32x3,
            },
        ],
    };
    
    // Create debug color shader
    let debug_color_shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("Debug Color Shader"),
        source: wgpu::ShaderSource::Wgsl(include_str!("debug_triangle.wgsl").into()),
    });
    
    // Create pipeline layout
    let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("Debug Color Pipeline Layout"),
        bind_group_layouts: &[],
        push_constant_ranges: &[],
    });
    
    // Create debug color pipeline
    let debug_color_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("Debug Color Pipeline"),
        layout: Some(&pipeline_layout),
        vertex: wgpu::VertexState {
            module: &debug_color_shader,
            entry_point: Some("vs_main"),
            buffers: &[vertex_buffer_layout],
            compilation_options: Default::default(),
        },
        fragment: Some(wgpu::FragmentState {
            module: &debug_color_shader,
            entry_point: Some("fs_main"),
            targets: &[Some(wgpu::ColorTargetState {
                format,
                blend: Some(wgpu::BlendState::REPLACE),
                write_mask: wgpu::ColorWrites::ALL,
            })],
            compilation_options: Default::default(),
        }),
        primitive: wgpu::PrimitiveState {
            topology: wgpu::PrimitiveTopology::TriangleList,
            strip_index_format: None,
            front_face: wgpu::FrontFace::Ccw,
            cull_mode: Some(wgpu::Face::Back),
            polygon_mode: wgpu::PolygonMode::Fill,
            unclipped_depth: false,
            conservative: false,
        },
        depth_stencil: Some(wgpu::DepthStencilState {
            format: wgpu::TextureFormat::Depth32Float,
            depth_write_enabled: true,
            depth_compare: wgpu::CompareFunction::Less,
            stencil: wgpu::StencilState::default(),
            bias: wgpu::DepthBiasState::default(),
        }),
        multisample: wgpu::MultisampleState {
            count: 1,
            mask: !0,
            alpha_to_coverage_enabled: false,
        },
        multiview: None,
        cache: None,
    });
    
    // Add pipelines to the shader registry
    shader_pipelines.insert(ShaderType::DebugColor, debug_color_pipeline);
    
    shader_pipelines
}

/// Vertex structure for rendering
#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub position: [f32; 3],
    pub color: [f32; 3],
}
