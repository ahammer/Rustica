//! Core debug renderer implementation.
//!
//! This module provides the main DebugRenderer struct which uses wgpu to render
//! debug visualization primitives.

use std::sync::Arc;
use wgpu::{
    Adapter, Backends, BindGroup, BindGroupLayout, Buffer, ComputePipeline, Device, Instance, InstanceDescriptor, PipelineLayout, Queue, RenderPipeline, Sampler, ShaderModule, Surface, SurfaceConfiguration, TextureView
};
use winit::window::Window;

use crate::error::{Error, Result};
use crate::primitives::{Line, Point, Rectangle, Vertex};

// Shader code for 2D primitives
const SHADER_SOURCE: &str = r#"
// Vertex shader
struct VertexInput {
    @location(0) position: vec2<f32>,
    @location(1) color: vec4<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec4<f32>,
};

struct Uniforms {
    view_size: vec2<f32>,
    padding: vec2<f32>,
};

@group(0) @binding(0)
var<uniform> uniforms: Uniforms;

@vertex
fn vs_main(in: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    
    // Convert screen space to clip space [-1, 1]
    let clip_position = vec2<f32>(
        (in.position.x / uniforms.view_size.x) * 2.0 - 1.0,
        1.0 - (in.position.y / uniforms.view_size.y) * 2.0
    );
    
    out.clip_position = vec4<f32>(clip_position, 0.0, 1.0);
    out.color = in.color;
    
    return out;
}

// Fragment shader
@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return in.color;
}
"#;

/// Configuration for the debug renderer.
#[derive(Debug, Clone)]
pub struct DebugRendererConfig {
    /// The initial width of the renderer's viewport.
    pub width: u32,
    /// The initial height of the renderer's viewport.
    pub height: u32,
    /// Whether to use vsync.
    pub vsync: bool,
    /// The maximum number of vertices that can be rendered in a batch.
    pub max_vertices: u32,
}

impl Default for DebugRendererConfig {
    fn default() -> Self {
        Self {
            width: 800,
            height: 600,
            vsync: true,
            max_vertices: 10000,
        }
    }
}

/// The main debug renderer.
pub struct DebugRenderer {
    /// Instance of wgpu.
    instance: Instance,
    /// Surface to render to.
    surface: Surface,
    /// Adapter for graphics hardware.
    adapter: Adapter,
    /// Graphics device.
    device: Device,
    /// Command queue.
    queue: Queue,
    /// Surface configuration.
    surface_config: SurfaceConfiguration,
    /// Debug renderer configuration.
    config: DebugRendererConfig,
    /// Render pipeline.
    render_pipeline: RenderPipeline,
    /// Uniform buffer.
    uniform_buffer: Buffer,
    /// Bind group.
    bind_group: BindGroup,
    /// Vertex buffer.
    vertex_buffer: Buffer,
    /// Shader module.
    shader: ShaderModule,
}

impl DebugRenderer {
    /// Create a new debug renderer.
    pub async fn new(window: &Window, config: Option<DebugRendererConfig>) -> Result<Self> {
        let config = config.unwrap_or_default();
        
        // Create the wgpu instance
        let instance = Instance::new( InstanceDescriptor {
            backends: Backends::all(),
            ..Default::default()
        });
        
        // Create the surface from the window
        let surface = unsafe { instance.create_surface(&window) }?;
        
        // Request an adapter
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .ok_or_else(|| Error::InitializationError("Failed to find an appropriate adapter".to_string()))?;
        
        // Request a device and queue
        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: Some("Debug Renderer Device"),
                    features: wgpu::Features::empty(),
                    limits: wgpu::Limits::default(),
                },
                None,
            )
            .await?;
        
        // Get surface capabilities and choose a surface format
        let surface_caps = surface.get_capabilities(&adapter);
        // In wgpu 0.15, we need to manually check for sRGB formats
        let surface_format = surface_caps
            .formats
            .iter()
            .find(|format| {
                matches!(
                    format,
                    wgpu::TextureFormat::Rgba8UnormSrgb | wgpu::TextureFormat::Bgra8UnormSrgb
                )
            })
            .copied()
            .unwrap_or(surface_caps.formats[0]);
        
        // Configure the surface
        let present_mode = if config.vsync {
            wgpu::PresentMode::Fifo
        } else {
            wgpu::PresentMode::Immediate
        };
        
        let surface_config = wgpu::SurfaceConfiguration {            
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: config.width,
            height: config.height,
            present_mode,
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
        };
        surface.configure(&device, &surface_config);
        
        // Create the shader module
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Debug Shader"),
            source: wgpu::ShaderSource::Wgsl(SHADER_SOURCE.into()),
        });
        
        // Create uniform buffer for view transforms
        let uniform_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Debug Uniform Buffer"),
            size: 16, // 2 vec2<f32> (8 bytes each)
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        
        // Update uniform buffer with initial values
        let uniform_data = [
            config.width as f32,
            config.height as f32,
            0.0, // padding
            0.0, // padding
        ];
        queue.write_buffer(&uniform_buffer, 0, bytemuck::cast_slice(&uniform_data));
        
        // Create bind group layout
        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Debug Bind Group Layout"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        });
        
        // Create bind group
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Debug Bind Group"),
            layout: &bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: uniform_buffer.as_entire_binding(),
            }],
        });
        
        // Create pipeline layout
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Debug Pipeline Layout"),
            bind_group_layouts: &[&bind_group_layout],
            push_constant_ranges: &[],
        });
        
        // Create vertex buffer
        let vertex_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Debug Vertex Buffer"),
            size: (std::mem::size_of::<Vertex>() as u64) * (config.max_vertices as u64),
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        
        // Create render pipeline
        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Debug Render Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[wgpu::VertexBufferLayout {
                    array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
                    step_mode: wgpu::VertexStepMode::Vertex,
                    attributes: &[
                        // Position
                        wgpu::VertexAttribute {
                            offset: 0,
                            shader_location: 0,
                            format: wgpu::VertexFormat::Float32x2,
                        },
                        // Color
                        wgpu::VertexAttribute {
                            offset: 8, // 2 floats * 4 bytes
                            shader_location: 1,
                            format: wgpu::VertexFormat::Float32x4,
                        },
                    ],
                }],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: surface_format,
                    blend: Some(wgpu::BlendState {
                        color: wgpu::BlendComponent {
                            src_factor: wgpu::BlendFactor::SrcAlpha,
                            dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
                            operation: wgpu::BlendOperation::Add,
                        },
                        alpha: wgpu::BlendComponent {
                            src_factor: wgpu::BlendFactor::SrcAlpha,
                            dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
                            operation: wgpu::BlendOperation::Add,
                        },
                    }),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: None,
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
        });
        
        Ok(Self {
            instance,
            surface,
            adapter,
            device,
            queue,
            surface_config,
            config,
            render_pipeline,
            uniform_buffer,
            bind_group,
            vertex_buffer,
            shader,
        })
    }
    
    /// Handle window resize events.
    pub fn resize(&mut self, width: u32, height: u32) {
        if width > 0 && height > 0 {
            self.surface_config.width = width;
            self.surface_config.height = height;
            self.surface.configure(&self.device, &self.surface_config);
            
            // Update uniform buffer with new dimensions
            let uniform_data = [
                width as f32,
                height as f32,
                0.0, // padding
                0.0, // padding
            ];
            self.queue.write_buffer(&self.uniform_buffer, 0, bytemuck::cast_slice(&uniform_data));
        }
    }
    
    /// Begin a new frame.
    pub fn begin_frame(&mut self) -> Result<wgpu::SurfaceTexture> {
        let frame = self.surface.get_current_texture()?;
        Ok(frame)
    }
    
    /// End the current frame and present it.
    pub fn end_frame(&mut self, frame: wgpu::SurfaceTexture) {
        frame.present();
    }
    
    /// Draw a batch of vertices.
    pub fn draw_vertices(&mut self, vertices: &[Vertex], frame: &wgpu::SurfaceTexture) -> Result<()> {
        if vertices.is_empty() {
            return Ok(());
        }
        
        // Create a command encoder for recording commands
        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Debug Render Encoder"),
        });
        
        // Write vertices to the vertex buffer
        self.queue.write_buffer(&self.vertex_buffer, 0, bytemuck::cast_slice(vertices));
        
        // Create a render pass and draw the vertices
        let view = frame.texture.create_view(&wgpu::TextureViewDescriptor::default());
        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Debug Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Load, // Don't clear, add to existing frame
                        store: true,
                    },
                })],
                depth_stencil_attachment: None,
            });
            
            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.set_bind_group(0, &self.bind_group, &[]);
            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(0..((vertices.len() * std::mem::size_of::<Vertex>()) as u64)));
            render_pass.draw(0..vertices.len() as u32, 0..1);
        }
        
        // Submit the command buffer
        self.queue.submit(std::iter::once(encoder.finish()));
        
        Ok(())
    }
    
    /// Clear the screen with a color.
    pub fn clear(&mut self, color: [f32; 4], frame: &wgpu::SurfaceTexture) -> Result<()> {
        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Debug Clear Encoder"),
        });
        
        let view = frame.texture.create_view(&wgpu::TextureViewDescriptor::default());
        {
            let _render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Debug Clear Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: color[0] as f64,
                            g: color[1] as f64,
                            b: color[2] as f64,
                            a: color[3] as f64,
                        }),
                        store: true,
                    },
                })],
                depth_stencil_attachment: None,
            });
        }
        
        self.queue.submit(std::iter::once(encoder.finish()));
        
        Ok(())
    }
    
    /// Draw a point.
    pub fn draw_point(&mut self, point: &Point, frame: &wgpu::SurfaceTexture) -> Result<()> {
        let vertices = point.to_vertices();
        self.draw_vertices(&vertices, frame)
    }
    
    /// Draw a line.
    pub fn draw_line(&mut self, line: &Line, frame: &wgpu::SurfaceTexture) -> Result<()> {
        let vertices = line.to_vertices();
        self.draw_vertices(&vertices, frame)
    }
    
    /// Draw a rectangle.
    pub fn draw_rect(&mut self, rect: &Rectangle, frame: &wgpu::SurfaceTexture) -> Result<()> {
        let vertices = rect.to_vertices();
        self.draw_vertices(&vertices, frame)
    }
}
