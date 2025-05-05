use rustica_shader_bindings::pbr_shader::{
    create_shader_module, 
    create_pipeline_layout, 
    vs_main_entry,
    fs_main_entry,
    vertex_state,
    fragment_state
};

/// Creates an optimized pipeline for the given surface format
///
/// # Example
/// ```
/// # use rustica_render_utils::create_pipeline;
/// # fn example(device: &wgpu::Device, format: wgpu::TextureFormat) {
/// let pipeline = create_pipeline(device, format, None);
/// # }
/// ```
pub fn create_pipeline(
    device: &wgpu::Device,
    format: wgpu::TextureFormat,
    topology: Option<wgpu::PrimitiveTopology>,
) -> wgpu::RenderPipeline {
    // Create shader module and pipeline layout
    let shader_module = create_shader_module(device);
    let pipeline_layout = create_pipeline_layout(device);
    
    // Configure vertex and fragment stages
    let vs_entry = vs_main_entry(wgpu::VertexStepMode::Vertex);
    let fs_entry = fs_main_entry([Some(wgpu::ColorTargetState {
        format,
        blend: Some(wgpu::BlendState::REPLACE),
        write_mask: wgpu::ColorWrites::ALL,
    })]);
    
    let vertex_state = vertex_state(&shader_module, &vs_entry);
    let fragment_state = fragment_state(&shader_module, &fs_entry);
    
    // Create and return pipeline
    device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("PBR Pipeline"),
        layout: Some(&pipeline_layout),
        vertex: vertex_state,
        fragment: Some(fragment_state),
        primitive: wgpu::PrimitiveState {
            topology: topology.unwrap_or(wgpu::PrimitiveTopology::TriangleList),
            strip_index_format: None,
            front_face: wgpu::FrontFace::Ccw,
            cull_mode: Some(wgpu::Face::Back),
            polygon_mode: wgpu::PolygonMode::Fill,
            unclipped_depth: false,
            conservative: false,
        },
        depth_stencil: None,
        multisample: wgpu::MultisampleState {
            count: 1,
            mask: !0,
            alpha_to_coverage_enabled: false,
        },
        multiview: None,
        cache: None,
    })
}
