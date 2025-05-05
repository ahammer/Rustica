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
use std::time::Instant;
use winit::{ 
    application::ApplicationHandler, 
    event::WindowEvent,
    event_loop::ActiveEventLoop,
    window::{Window, WindowAttributes, WindowId}, 
};
// No longer needed with render utils
use winit::platform::windows::EventLoopBuilderExtWindows; 
use raw_window_handle::{HasDisplayHandle, HasWindowHandle};

struct State {
    surface: wgpu::Surface<'static>, 
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: winit::dpi::PhysicalSize<u32>,
    render_pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    num_vertices: u32,
    start_time: Instant,
    

    
    camera_bind_group: WgpuBindGroup0,
    camera_uniform_buffer: wgpu::Buffer,
    model_bind_group: WgpuBindGroup1,
    model_uniform_buffer: wgpu::Buffer,
    material_bind_group: WgpuBindGroup2,
    material_uniform_buffer: wgpu::Buffer,
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

impl State {
    
    async fn new(window_attributes: WindowAttributes, event_loop: &ActiveEventLoop) -> (Window, Self) {
        let window = event_loop.create_window(window_attributes).unwrap();
        let size = window.inner_size();

        
        let instance_descriptor = wgpu::InstanceDescriptor {
             backends: wgpu::Backends::PRIMARY,
             flags: wgpu::InstanceFlags::default(),
             backend_options: Default::default(), 
        };
        let instance = wgpu::Instance::new(&instance_descriptor);

        
        let surface = unsafe {
            
            let window_handle = window.window_handle()
                .map(|handle| handle.as_raw())
                .expect("Window handle unavailable");
            let display_handle = window.display_handle()
                .map(|handle| handle.as_raw())
                .expect("Display handle unavailable");

            
            let target = wgpu::SurfaceTargetUnsafe::RawHandle {
                raw_display_handle: display_handle,
                raw_window_handle: window_handle,
            };
            instance.create_surface_unsafe(target)
                .expect("Failed to create surface unsafely")
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
        };        surface.configure(&device, &config);

        // Create pipeline using utility function
        let render_pipeline = create_pipeline(&device, config.format, None);        // Create vertex buffer using utility function
        let (vertex_buffer, num_vertices) = create_vertex_buffer(&device, VERTICES);        // Create camera resources using utility function
        let (camera_uniform_buffer, camera_bind_group) = create_camera_resources(&device);        // Create model resources using utility function
        let (model_uniform_buffer, model_bind_group) = create_model_resources(&device);        // Create material resources using utility function
        let (material_uniform_buffer, material_bind_group) = create_material_resources(
            &device,
            Vec4::new(1.0, 0.0, 0.0, 1.0), // red color
            0.0,  // non-metallic
            1.0   // rough
        );

        (window, Self { 
            surface, 
            device,
            queue,
            config,
            size,
            render_pipeline,
            vertex_buffer,
            num_vertices,
            start_time: Instant::now(),
            camera_bind_group,
            camera_uniform_buffer,
            model_bind_group,
            model_uniform_buffer,
            material_bind_group,
            material_uniform_buffer,
        })
    }

    fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
        }
    }    fn update(&mut self) {
        // Calculate rotation based on elapsed time
        let elapsed = self.start_time.elapsed().as_secs_f32();
        let angle = elapsed * std::f32::consts::PI / 4.0; 
        let rotation = Mat4::from_rotation_z(angle);
        
        // Update model transform using utility function
        update_model_transform(&self.queue, &self.model_uniform_buffer, rotation);

        // Create orthographic projection using utility function
        let view_proj = create_orthographic_projection(self.size.width, self.size.height);
        
        // Update camera using utility function
        update_camera(
            &self.queue,
            &self.camera_uniform_buffer,
            view_proj, 
            Vec3A::new(0.0, 0.0, 1.0)
        );
    }

    fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let output = self.surface.get_current_texture()?;
        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
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

            render_pass.set_pipeline(&self.render_pipeline);
            
            self.camera_bind_group.set(&mut render_pass);
            self.model_bind_group.set(&mut render_pass);
            self.material_bind_group.set(&mut render_pass);
            
            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            
            render_pass.draw(0..self.num_vertices, 0..1); 
        }

        
        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}


#[derive(Default)]
struct App {
    window: Option<Window>,
    state: Option<State>,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_none() {
            let window_attributes = WindowAttributes::default()
                .with_title("Rustica Triangle POC")
                .with_inner_size(winit::dpi::LogicalSize::new(800.0, 600.0));

            
            let (window, state) = pollster::block_on(State::new(window_attributes, event_loop));

            self.window = Some(window);
            self.state = Some(state);
            println!("Window and WGPU State Initialized.");
        }
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId, 
        event: WindowEvent,
    ) {
        if let Some(state) = self.state.as_mut() {
            match event {
                WindowEvent::CloseRequested => {
                    println!("Close requested");
                    event_loop.exit();
                }
                WindowEvent::Resized(physical_size) => {
                    println!("Window resized to: {:?}", physical_size);
                    state.resize(physical_size);
                    
                    if let Some(window) = self.window.as_ref() {
                        window.request_redraw();
                    }
                }
                WindowEvent::ScaleFactorChanged { .. } => { 
                    if let Some(window) = self.window.as_ref() {
                        let new_inner_size = window.inner_size();
                        println!("Scale factor changed, new size: {:?}", new_inner_size);
                        state.resize(new_inner_size);
                        window.request_redraw();
                    }
                }
                WindowEvent::RedrawRequested => {
                    
                    state.update();
                    
                    match state.render() {
                        Ok(_) => {}
                        
                        Err(wgpu::SurfaceError::Lost) => {
                            println!("Surface lost, reconfiguring.");
                            state.resize(state.size)
                        },
                        
                        Err(wgpu::SurfaceError::OutOfMemory) => {
                            eprintln!("Out of memory error!");
                            event_loop.exit();
                        },
                        
                        Err(e) => eprintln!("Error rendering frame: {:?}", e),
                    }
                    
                    if let Some(window) = self.window.as_ref() {
                        window.request_redraw();
                    }
                }
                _ => (),
            }
        }
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        
        
        
        
        if let Some(window) = self.window.as_ref() {
             window.request_redraw();
        }
    }

    fn exiting(&mut self, _event_loop: &ActiveEventLoop) {
        println!("Exiting application.");
    }
}


fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init(); 
    
    let event_loop = winit::event_loop::EventLoop::builder()
        .with_any_thread(true)
        .build()?;
    let mut app = App::default();

    
    event_loop.run_app(&mut app)?;


    Ok(())
}