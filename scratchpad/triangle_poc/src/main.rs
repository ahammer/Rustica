use glam::{Mat4, Vec3A, Vec4};
use rustica_shader_bindings::pbr_shader::*;
use rustica_render_utils::{
    create_camera_resources, update_camera,
    create_model_resources, update_model_transform,
    create_material_resources,
    create_vertex_buffer,
    create_pipeline,
    create_orthographic_projection,
};
use rustica_window::{
    run_application, // Import the new run function
    Window, WindowConfig, ApplicationEvent, 
    RusticaApplication, ApplicationError
};
use std::time::Instant;
use wgpu::SurfaceError;
// Import the necessary traits from raw-window-handle
use raw_window_handle::{HasDisplayHandle, HasWindowHandle};

struct TriangleDemo {
    state: Option<RenderState>,
    start_time: Instant,
}

struct RenderState {
    surface: wgpu::Surface<'static>, 
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    render_pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    num_vertices: u32,
    
    camera_bind_group: WgpuBindGroup0,
    camera_uniform_buffer: wgpu::Buffer,
    model_bind_group: WgpuBindGroup1,
    model_uniform_buffer: wgpu::Buffer,
    material_bind_group: WgpuBindGroup2,
}

const VERTICES: &[VertexInput] = &[
    VertexInput {
        position: Vec3A::new(0.0, 0.5, 0.0), 
        uv: [0.5, 0.0],
        normal: Vec3A::new(0.0, 0.0, 1.0), 
    },
    VertexInput {
        position: Vec3A::new(-0.5, -0.5, 0.0),
        uv: [0.0, 1.0],
        normal: Vec3A::new(0.0, 0.0, 1.0),
    },
    VertexInput {
        position: Vec3A::new(0.5, -0.5, 0.0),
        uv: [1.0, 1.0],
        normal: Vec3A::new(0.0, 0.0, 1.0),
    },
];

impl TriangleDemo {
    async fn init_render_state(&mut self, window: &Window) -> Result<(), ApplicationError> {
        let size = window.inner_size();

        // Initialize WGPU
        let instance_descriptor = wgpu::InstanceDescriptor {
            backends: wgpu::Backends::PRIMARY,
            flags: wgpu::InstanceFlags::default(),
            backend_options: Default::default(),
        };
        let instance = wgpu::Instance::new(&instance_descriptor);
        
        // Use the traits and call .as_raw()
        let surface = unsafe {
            let window_handle = window.window_handle()
                .map_err(|e| ApplicationError::Initialization(format!("Window handle error: {}", e)))?
                .as_raw(); // Convert to raw handle
            let display_handle = window.display_handle()
                .map_err(|e| ApplicationError::Initialization(format!("Display handle error: {}", e)))?
                .as_raw(); // Convert to raw handle
            
            let target = wgpu::SurfaceTargetUnsafe::RawHandle {
                raw_display_handle: display_handle,
                raw_window_handle: window_handle,
            };
            instance.create_surface_unsafe(target)
                .map_err(|e| ApplicationError::Initialization(format!("Surface creation error: {}", e)))?
        };

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .unwrap();

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: Some("Device"),
                    required_features: wgpu::Features::empty(),
                    required_limits: wgpu::Limits::default(), 
                    memory_hints: wgpu::MemoryHints::default(),
                },
                None, 
            )
            .await
            .unwrap();

        // Configure surface
        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps.formats.iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(surface_caps.formats[0]);

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo, 
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };
        surface.configure(&device, &config);

        // Create pipeline using utility function
        let render_pipeline = create_pipeline(&device, config.format, None);
        
        // Create vertex buffer using utility function
        let (vertex_buffer, num_vertices) = create_vertex_buffer(&device, VERTICES);
        
        // Create camera resources using utility function
        let (camera_uniform_buffer, camera_bind_group) = create_camera_resources(&device);
        
        // Create model resources using utility function
        let (model_uniform_buffer, model_bind_group) = create_model_resources(&device);
        
        // Create material resources using utility function
        // Ignore the unused material_uniform_buffer
        let (_, material_bind_group) = create_material_resources(
            &device,
            Vec4::new(1.0, 0.0, 0.0, 1.0), // red color
            0.0,  // non-metallic
            1.0   // rough
        );

        self.state = Some(RenderState { 
            surface, 
            device,
            queue,
            config,
            render_pipeline,
            vertex_buffer,
            num_vertices,
            camera_bind_group,
            camera_uniform_buffer,
            model_bind_group,
            model_uniform_buffer,
            material_bind_group,
        });

        Ok(())
    }

    fn update(&mut self, size: winit::dpi::PhysicalSize<u32>) {
        if let Some(state) = &mut self.state {
            // Calculate rotation based on elapsed time
            let elapsed = self.start_time.elapsed().as_secs_f32();
            let angle = elapsed * std::f32::consts::PI / 4.0; 
            let rotation = Mat4::from_rotation_z(angle);
            
            // Update model transform using utility function
            update_model_transform(&state.queue, &state.model_uniform_buffer, rotation);
    
            // Create orthographic projection using utility function
            let view_proj = create_orthographic_projection(size.width, size.height);
            
            // Update camera using utility function
            update_camera(
                &state.queue,
                &state.camera_uniform_buffer,
                view_proj, 
                Vec3A::new(0.0, 0.0, 1.0)
            );
        }
    }

    fn resize(&mut self, size: winit::dpi::PhysicalSize<u32>) {
        if let Some(state) = &mut self.state {
            if size.width > 0 && size.height > 0 {
                state.config.width = size.width;
                state.config.height = size.height;
                state.surface.configure(&state.device, &state.config);
            }
        }
    }

    fn render(&mut self) -> Result<(), SurfaceError> {
        if let Some(state) = &mut self.state {
            let output = state.surface.get_current_texture()?;
            let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());
    
            let mut encoder = state.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });
    
            {
                let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: Some("Render Pass"),
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                        view: &view,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(wgpu::Color {
                                r: 0.1,
                                g: 0.2,
                                b: 0.3,
                                a: 1.0,
                            }),
                            store: wgpu::StoreOp::Store,
                        },
                    })],
                    depth_stencil_attachment: None, 
                    timestamp_writes: None,
                    occlusion_query_set: None,
                });
    
                render_pass.set_pipeline(&state.render_pipeline);
                
                state.camera_bind_group.set(&mut render_pass);
                state.model_bind_group.set(&mut render_pass);
                state.material_bind_group.set(&mut render_pass);
                
                render_pass.set_vertex_buffer(0, state.vertex_buffer.slice(..));
                render_pass.draw(0..state.num_vertices, 0..1); 
            }
    
            state.queue.submit(std::iter::once(encoder.finish()));
            output.present();
        }

        Ok(())
    }
}


impl RusticaApplication for TriangleDemo {
    fn create() -> Self {
        Self {
            state: None,
            start_time: Instant::now(),
        }
    }
    
    fn init(&mut self, window: &Window) -> Result<(), ApplicationError> {
        // Use pollster::block_on for the async init_render_state
        pollster::block_on(self.init_render_state(window))
    }
    
    fn handle_event(&mut self, event: ApplicationEvent, window: &Window) -> Result<(), ApplicationError> {
        match event {
            ApplicationEvent::Update => {
                self.update(window.inner_size());
            },
            ApplicationEvent::Resize(new_size) => {
                self.resize(new_size);
            },
            ApplicationEvent::RedrawRequested => {
                match self.render() {
                    Ok(_) => {},
                    // Reconfigure surface on Lost, handled by resize
                    Err(SurfaceError::Lost) => self.resize(window.inner_size()), 
                    // Map OutOfMemory to an ApplicationError
                    Err(SurfaceError::OutOfMemory) => return Err(ApplicationError::EventHandler(
                        "Graphics system out of memory".into()
                    )),
                    // Log other errors
                    Err(e) => log::error!("Render error: {:?}", e),
                }
            },
            ApplicationEvent::Exit => {
                log::info!("Application exiting");
                // Cleanup logic can go here if needed, 
                // but AppHandler drops the app state automatically.
            },
        }
        Ok(())
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    
    let window_config = WindowConfig {
        title: "Rustica Triangle Demo".into(),
        width: 800,
        height: 600,
        resizable: true,
        maximized: false,
    };
    
    // Call the standalone run_application function
    run_application::<TriangleDemo>(window_config)?;
    
    Ok(())
}