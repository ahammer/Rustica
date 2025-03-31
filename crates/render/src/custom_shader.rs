// Custom shader implementation

use std::collections::HashMap;
use wgpu::{
    BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayout, BindGroupLayoutDescriptor,
    BindGroupLayoutEntry, BindingType, Buffer, BufferBindingType, BufferDescriptor,
    BufferUsages, Device, PipelineLayoutDescriptor, RenderPipeline,
    RenderPipelineDescriptor, ShaderModuleDescriptor, ShaderSource, ShaderStages,
    TextureFormat, VertexBufferLayout, VertexFormat, VertexState, FragmentState, ColorTargetState,
    BlendState, ColorWrites, PrimitiveState, PrimitiveTopology, FrontFace, Face, PolygonMode,
    MultisampleState, Queue,
};

// Import core geometry traits from Foundation
use rustica_foundation::geometry::{Vertex, VertexAttributeProvider, VertexAttribute, Triangle};

/// Uniform parameter descriptor
pub struct UniformParameter {
    /// Name of the parameter
    pub name: String,
    /// Binding index
    pub binding: u32,
    /// Size of the parameter in bytes
    pub size: u64,
}

/// Descriptor for a custom WGSL shader
pub struct ShaderDescriptor {
    /// Name of the shader
    pub name: String,
    /// WGSL shader source code
    pub shader_source: String,
    /// Vertex attributes
    pub vertex_attributes: Vec<VertexAttribute>,
    /// Uniform parameters (if any)
    pub uniforms: Vec<UniformParameter>,
}

/// A custom shader implementation
pub struct CustomShader {
    /// Name of the shader
    pub name: String,
    /// Compiled pipeline
    pipeline: Option<RenderPipeline>,
    /// Bind group layout
    bind_group_layout: Option<BindGroupLayout>,
    /// Bind group
    bind_group: Option<BindGroup>,
    /// Uniform buffers (if any)
    uniform_buffers: HashMap<String, Buffer>,
    /// Original descriptor (for deferred initialization)
    descriptor: Option<ShaderDescriptor>,
}

/// Create the instance buffer attributes for model matrices (4 rows of vec4)
/// Plus an instance color attribute at the end
/// Since these are static attributes, we'll use a static array
pub fn get_instance_attributes() -> [wgpu::VertexAttribute; 5] {
    use std::mem;
    
    [
        wgpu::VertexAttribute {
            offset: 0,
            shader_location: 3, // Start after the main vertex attributes
            format: wgpu::VertexFormat::Float32x4,
        },
        wgpu::VertexAttribute {
            offset: mem::size_of::<[f32; 4]>() as wgpu::BufferAddress,
            shader_location: 4,
            format: wgpu::VertexFormat::Float32x4,
        },
        wgpu::VertexAttribute {
            offset: mem::size_of::<[f32; 8]>() as wgpu::BufferAddress,
            shader_location: 5,
            format: wgpu::VertexFormat::Float32x4,
        },
        wgpu::VertexAttribute {
            offset: mem::size_of::<[f32; 12]>() as wgpu::BufferAddress,
            shader_location: 6,
            format: wgpu::VertexFormat::Float32x4,
        },
        wgpu::VertexAttribute {
            offset: mem::size_of::<[f32; 16]>() as wgpu::BufferAddress,
            shader_location: 7, // Location for instance color
            format: wgpu::VertexFormat::Float32x3,
        },
    ]
}

/// Create an instance buffer layout for model matrices (4 rows of vec4)
pub fn create_instance_buffer_layout<'a>() -> wgpu::VertexBufferLayout<'a> {
    use std::mem;
    
    // Create a static reference to the attributes
    static INSTANCE_ATTRIBUTES: once_cell::sync::Lazy<[wgpu::VertexAttribute; 5]> = 
        once_cell::sync::Lazy::new(get_instance_attributes);
    
    // The stride needs to include both the 4x4 matrix (16 floats) and the color (3 floats)
    // Plus padding (1 float) for alignment, totaling 20 floats
    wgpu::VertexBufferLayout {
        array_stride: mem::size_of::<[f32; 20]>() as wgpu::BufferAddress,
        step_mode: wgpu::VertexStepMode::Instance,
        attributes: &INSTANCE_ATTRIBUTES[..],
    }
}

impl CustomShader {
    /// Create a new custom shader from a descriptor
    pub fn new(device: &Device, format: TextureFormat, descriptor: ShaderDescriptor) -> Self {
        // Create shader module from the provided source code
        let shader_source = ShaderSource::Wgsl(descriptor.shader_source.into());
        
        let shader_module = device.create_shader_module(ShaderModuleDescriptor {
            label: Some(&format!("{} Shader", descriptor.name)),
            source: shader_source,
        });

        // Create bind group layout and uniform buffers if needed
        let (bind_group_layout, bind_group, uniform_buffers) = if !descriptor.uniforms.is_empty() {
            // Create bind group layout entries
            let mut entries = Vec::new();
            let mut uniform_buffers = HashMap::new();
            let mut bind_group_entries = Vec::new();

            for uniform in &descriptor.uniforms {
                // Create bind group layout entry
                entries.push(BindGroupLayoutEntry {
                    binding: uniform.binding,
                    visibility: ShaderStages::VERTEX_FRAGMENT,
                    ty: BindingType::Buffer {
                        ty: BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                });

                // Create uniform buffer
                let buffer = device.create_buffer(&BufferDescriptor {
                    label: Some(&format!("{} Uniform Buffer", uniform.name)),
                    size: uniform.size,
                    usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
                    mapped_at_creation: false,
                });

                // Add to uniform buffers map
                uniform_buffers.insert(uniform.name.clone(), buffer);
            }

            // Create bind group layout
            let bind_group_layout = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
                label: Some(&format!("{} Bind Group Layout", descriptor.name)),
                entries: &entries,
            });

            // Create bind group entries
            for uniform in &descriptor.uniforms {
                if let Some(buffer) = uniform_buffers.get(&uniform.name) {
                    bind_group_entries.push(BindGroupEntry {
                        binding: uniform.binding,
                        resource: buffer.as_entire_binding(),
                    });
                }
            }

            // Create bind group
            let bind_group = device.create_bind_group(&BindGroupDescriptor {
                label: Some(&format!("{} Bind Group", descriptor.name)),
                layout: &bind_group_layout,
                entries: &bind_group_entries,
            });

            (Some(bind_group_layout), Some(bind_group), uniform_buffers)
        } else {
            (None, None, HashMap::new())
        };

        // Create pipeline layout
        let pipeline_layout = if let Some(ref layout) = bind_group_layout {
            let bind_group_layouts = &[layout];
            device.create_pipeline_layout(&PipelineLayoutDescriptor {
                label: Some(&format!("{} Pipeline Layout", descriptor.name)),
                bind_group_layouts,
                push_constant_ranges: &[],
            })
        } else {
            device.create_pipeline_layout(&PipelineLayoutDescriptor {
                label: Some(&format!("{} Pipeline Layout", descriptor.name)),
                bind_group_layouts: &[],
                push_constant_ranges: &[],
            })
        };

        // Create vertex buffer layout
        let vertex_buffer_layout = VertexBufferLayout {
            array_stride: descriptor.vertex_attributes.iter().map(|attr| attr.offset + attr.format.size()).max().unwrap_or(0),
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &descriptor.vertex_attributes.iter().map(|attr| wgpu::VertexAttribute {
                offset: attr.offset,
                shader_location: attr.location,
                format: attr.format,
            }).collect::<Vec<_>>(),
        };

        // Add the instance buffer layout
        let instance_layout = create_instance_buffer_layout();
        
        // We need to collect the buffer layouts into a Vec because they need to live long enough
        let mut buffer_layouts = Vec::new();
        buffer_layouts.push(vertex_buffer_layout);
        buffer_layouts.push(instance_layout);
        
        // Create render pipeline with both vertex and instance buffers
        let pipeline = device.create_render_pipeline(&RenderPipelineDescriptor {
            label: Some(&format!("{} Pipeline", descriptor.name)),
            layout: Some(&pipeline_layout),
            vertex: VertexState {
                module: &shader_module,
                entry_point: Some("vs_main"),
                buffers: &buffer_layouts,
                compilation_options: Default::default(),
            },
            fragment: Some(FragmentState {
                module: &shader_module,
                entry_point: Some("fs_main"),
                targets: &[Some(ColorTargetState {
                    format,
                    blend: Some(BlendState::REPLACE),
                    write_mask: ColorWrites::ALL,
                })],
                compilation_options: Default::default(),
            }),
            primitive: PrimitiveState {
                topology: PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: FrontFace::Ccw,
                cull_mode: Some(Face::Back),
                polygon_mode: PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: Some(wgpu::DepthStencilState {
                format: TextureFormat::Depth32Float,
                depth_write_enabled: true,
                depth_compare: wgpu::CompareFunction::Less, // Standard depth test
                stencil: wgpu::StencilState::default(),
                bias: wgpu::DepthBiasState::default(),
            }),
            multisample: MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
            cache: None,
        });

        Self {
            name: descriptor.name.clone(),
            pipeline: Some(pipeline),
            bind_group_layout,
            bind_group,
            uniform_buffers,
            descriptor: None, // No need to store the descriptor since we've already initialized
        }
    }
    
    /// Create a placeholder shader that will be initialized later
    pub fn new_placeholder(descriptor: ShaderDescriptor) -> Self {
        Self {
            name: descriptor.name.clone(),
            pipeline: None,
            bind_group_layout: None,
            bind_group: None,
            uniform_buffers: HashMap::new(),
            descriptor: Some(descriptor),
        }
    }
    
    /// Initialize the shader with a device and format
    pub fn initialize(&mut self, device: &Device, format: TextureFormat) -> bool {
        if self.pipeline.is_some() {
            return false; // Already initialized
        }
        
        if let Some(descriptor) = self.descriptor.take() {
            let initialized = Self::new(device, format, descriptor);
            self.pipeline = initialized.pipeline;
            self.bind_group_layout = initialized.bind_group_layout;
            self.bind_group = initialized.bind_group;
            self.uniform_buffers = initialized.uniform_buffers;
            true
        } else {
            false
        }
    }

    /// Set a uniform parameter value
    pub fn set_uniform<T: bytemuck::Pod>(&self, name: &str, value: T, queue: Option<&Queue>) {
        if let (Some(buffer), Some(q)) = (self.uniform_buffers.get(name), queue) {
            q.write_buffer(buffer, 0, bytemuck::cast_slice(&[value]));
        }
    }

    /// Get the render pipeline
    pub fn pipeline(&self) -> Option<&RenderPipeline> {
        self.pipeline.as_ref()
    }

    /// Get the bind group
    pub fn bind_group(&self) -> Option<&BindGroup> {
        self.bind_group.as_ref()
    }
    
    /// Check if the shader is initialized
    pub fn is_initialized(&self) -> bool {
        self.pipeline.is_some()
    }
}
