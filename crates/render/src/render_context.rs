// Internal rendering context that manages WGPU resources

use std::time::Instant;
use wgpu::{
    Backends, Color, CommandEncoderDescriptor, Device, Features, Instance, InstanceDescriptor, 
    Limits, LoadOp, MemoryHints, Operations, PowerPreference, Queue, RenderPassColorAttachment, 
    RenderPassDescriptor, RenderPipeline, RequestAdapterOptions, StoreOp, Surface, 
    SurfaceConfiguration, SurfaceTargetUnsafe, TextureUsages, TextureViewDescriptor
};
use wgpu::util::DeviceExt;
use winit::dpi::PhysicalSize;
use rustica_window::WindowApp;
use cgmath::Matrix4;

use crate::shader_types::ShaderType;
use crate::draw_commands::DrawCommand;
use crate::shaders;
use crate::custom_shader::CustomShader;

/// Internal rendering context that manages WGPU resources
pub struct RenderContext {
    instance: Instance,
    pub(crate) surface: Option<Surface<'static>>,
    pub(crate) device: Option<Device>,
    pub(crate) queue: Option<Queue>,
    pub(crate) config: Option<SurfaceConfiguration>,
    clear_color: Color,
    shader_pipelines: std::collections::HashMap<ShaderType, RenderPipeline>,
    pub(crate) start_time: Instant,
    custom_shaders: Vec<CustomShader>,
}

impl RenderContext {
    pub fn new() -> Self {
        let instance = Instance::new(&InstanceDescriptor {
            backends: Backends::all(),
            ..Default::default()
        });

        Self {
            instance,
            surface: None,
            device: None,
            queue: None,
            config: None,
            clear_color: Color {
                r: 0.0,
                g: 0.0,
                b: 0.0,
                a: 1.0,
            },
            shader_pipelines: std::collections::HashMap::new(),
            start_time: Instant::now(),
            custom_shaders: Vec::new(),
        }
    }
    
    /// Register a custom shader and return its ID
    pub fn register_shader(&mut self, shader: CustomShader) -> usize {
        let id = self.custom_shaders.len();
        self.custom_shaders.push(shader);
        id
    }
    
    /// Get a mutable reference to a custom shader by ID
    pub fn get_shader_mut(&mut self, id: usize) -> Option<&mut CustomShader> {
        self.custom_shaders.get_mut(id)
    }

    pub fn initialize(&mut self, window_app: &WindowApp) -> Result<(), Box<dyn std::error::Error>> {
        let window = window_app.window().expect("Window should be created");
        let st = unsafe { SurfaceTargetUnsafe::from_window(window)? };
        let surface = unsafe { self.instance.create_surface_unsafe(st)? };

        let adapter = pollster::block_on(self.instance.request_adapter(&RequestAdapterOptions {
            power_preference: PowerPreference::default(),
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
        }))
        .ok_or("Failed to find an appropriate adapter")?;

        let (device, queue) = pollster::block_on(adapter.request_device(
            &wgpu::DeviceDescriptor {
                label: None,
                memory_hints: MemoryHints::default(),
                required_features: Features::empty(),
                required_limits: Limits::default(),
            },
            None,
        ))?;

        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps
            .formats
            .iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(surface_caps.formats[0]);

        let window_size = window.inner_size();
        let config = SurfaceConfiguration {
            usage: TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: window_size.width,
            height: window_size.height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };

        surface.configure(&device, &config);

        self.surface = Some(surface);
        self.device = Some(device);
        self.queue = Some(queue);
        self.config = Some(config);
        
        // Initialize shaders
        self.initialize_shaders();
        
        // Initialize any placeholder custom shaders
        if let (Some(device), Some(config)) = (&self.device, &self.config) {
            for shader in &mut self.custom_shaders {
                if !shader.is_initialized() {
                    shader.initialize(device, config.format);
                }
            }
        }

        Ok(())
    }

    pub fn resize(&mut self, new_size: PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            if let (Some(config), Some(surface), Some(device)) =
                (&mut self.config, &self.surface, &self.device)
            {
                config.width = new_size.width;
                config.height = new_size.height;
                surface.configure(device, config);
            }
        }
    }

    pub fn set_clear_color(&mut self, r: f64, g: f64, b: f64, a: f64) {
        self.clear_color = Color {
            r,
            g,
            b,
            a,
        };
    }
    
    fn initialize_shaders(&mut self) {
        if self.device.is_none() || self.config.is_none() {
            return;
        }
        
        let device_ref = self.device.as_ref().unwrap();
        let config_ref = self.config.as_ref().unwrap();
        let format = config_ref.format;
        
        // Initialize shaders using the shader module
        self.shader_pipelines = shaders::initialize_shaders(device_ref, format);
    }
    
    pub fn process_draw_commands(&mut self, commands: &[DrawCommand]) -> Result<(), wgpu::SurfaceError> {
        if self.device.is_none() || self.surface.is_none() || self.queue.is_none() {
            return Ok(());
        }
        
        let device = self.device.as_ref().unwrap();
        let surface = self.surface.as_ref().unwrap();
        let queue = self.queue.as_ref().unwrap();
        
        let output = surface.get_current_texture()?;
        let view = output.texture.create_view(&TextureViewDescriptor::default());
        
        let mut encoder = device.create_command_encoder(&CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });
        
        {
            let mut render_pass = encoder.begin_render_pass(&RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: Operations {
                        load: LoadOp::Clear(self.clear_color),
                        store: StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });
            
            // Process each draw command
            for command in commands {
                match command {
                    DrawCommand::Triangle { points, colors, shader } => {
                        if let Some(pipeline) = self.shader_pipelines.get(shader) {
                            // Create vertices from points and colors
                            let vertices = [
                                shaders::Vertex { 
                                    position: [points[0].x, points[0].y, points[0].z], 
                                    color: [colors[0].x, colors[0].y, colors[0].z] 
                                },
                                shaders::Vertex { 
                                    position: [points[1].x, points[1].y, points[1].z], 
                                    color: [colors[1].x, colors[1].y, colors[1].z] 
                                },
                                shaders::Vertex { 
                                    position: [points[2].x, points[2].y, points[2].z], 
                                    color: [colors[2].x, colors[2].y, colors[2].z] 
                                },
                            ];
                            
                            // Create vertex buffer
                            let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                                label: Some("Vertex Buffer"),
                                contents: bytemuck::cast_slice(&vertices),
                                usage: wgpu::BufferUsages::VERTEX,
                            });
                            
                            // Draw the triangle
                            render_pass.set_pipeline(pipeline);
                            render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));
                            render_pass.draw(0..3, 0..1);
                        }
                    },
                    DrawCommand::CustomTriangles { shader_id, vertices, vertex_count, uniforms } => {
                        // Get the custom shader from the registry
                        if let Some(shader) = self.custom_shaders.get(*shader_id) {
                            // Apply uniforms if any
                            if !uniforms.is_empty() && self.queue.is_some() {
                                let queue = self.queue.as_ref().unwrap();
                                for (name, value) in uniforms {
                                    match value {
                                        crate::draw_commands::UniformValue::Float(val) => {
                                            shader.set_uniform(name, *val, Some(queue));
                                        },
                                        crate::draw_commands::UniformValue::Vec2(val) => {
                                            shader.set_uniform(name, *val, Some(queue));
                                        },
                                        crate::draw_commands::UniformValue::Vec3(val) => {
                                            shader.set_uniform(name, *val, Some(queue));
                                        },
                                        crate::draw_commands::UniformValue::Vec4(val) => {
                                            shader.set_uniform(name, *val, Some(queue));
                                        },
                                        crate::draw_commands::UniformValue::Mat4(val) => {
                                            shader.set_uniform(name, *val, Some(queue));
                                        },
                                        crate::draw_commands::UniformValue::Int(val) => {
                                            shader.set_uniform(name, *val, Some(queue));
                                        },
                                        crate::draw_commands::UniformValue::UInt(val) => {
                                            shader.set_uniform(name, *val, Some(queue));
                                        },
                                    }
                                }
                            }
                            
                            // Create vertex buffer
                            let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                                label: Some(&format!("{} Vertex Buffer", shader.name)),
                                contents: vertices,
                                usage: wgpu::BufferUsages::VERTEX,
                            });
                            
                            // Draw the triangles if the shader is initialized
                            if let Some(pipeline) = shader.pipeline() {
                                render_pass.set_pipeline(pipeline);
                                render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));
                                
                                // Set bind group if available
                                if let Some(bind_group) = shader.bind_group() {
                                    render_pass.set_bind_group(0, bind_group, &[]);
                                }
                                
                                render_pass.draw(0..*vertex_count, 0..1);
                            }
                        }
                    }
                }
            }
        }
        
        queue.submit(std::iter::once(encoder.finish()));
        output.present();
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_render_context_new() {
        let context = RenderContext::new();
        assert!(context.surface.is_none());
        assert!(context.device.is_none());
        assert!(context.queue.is_none());
        assert!(context.config.is_none());
        
        // Check default clear color
        assert_eq!(context.clear_color.r, 0.0);
        assert_eq!(context.clear_color.g, 0.0);
        assert_eq!(context.clear_color.b, 0.0);
        assert_eq!(context.clear_color.a, 1.0);
    }

    #[test]
    fn test_render_context_set_clear_color() {
        let mut context = RenderContext::new();
        context.set_clear_color(0.5, 0.6, 0.7, 0.8);
        
        assert_eq!(context.clear_color.r, 0.5);
        assert_eq!(context.clear_color.g, 0.6);
        assert_eq!(context.clear_color.b, 0.7);
        assert_eq!(context.clear_color.a, 0.8);
    }
}
