use glam::{Mat3A, Mat4, Vec3A};
use rustica_shader_bindings::pbr_shader::*;
use std::time::Instant;
use winit::{ 
    application::ApplicationHandler, 
    event::WindowEvent,
    event_loop::ActiveEventLoop,
    window::{Window, WindowAttributes, WindowId}, 
};
use wgpu::util::DeviceExt;
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
        };
        surface.configure(&device, &config);

        
        let shader_module = create_shader_module(&device); 
        let pipeline_layout = create_pipeline_layout(&device); 

        
        
        let vs_entry = vs_main_entry(wgpu::VertexStepMode::Vertex);
        let fs_entry = fs_main_entry([Some(wgpu::ColorTargetState { 
            format: config.format,
            blend: Some(wgpu::BlendState::REPLACE),
            write_mask: wgpu::ColorWrites::ALL,
        })]);

        
        let vertex_state = vertex_state(&shader_module, &vs_entry);
        let fragment_state = fragment_state(&shader_module, &fs_entry);


        
        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: vertex_state, 
            fragment: Some(fragment_state), 
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
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
        });

        
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(VERTICES),
            usage: wgpu::BufferUsages::VERTEX,
        });
        let num_vertices = VERTICES.len() as u32;

        
        let camera_uniform = CameraUniform {
            view_proj: Mat4::IDENTITY, 
            position: Vec3A::ZERO, 
        };
        let camera_uniform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Camera Uniform Buffer"),
            contents: bytemuck::cast_slice(&[camera_uniform]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });
        let camera_bind_group = WgpuBindGroup0::from_bindings(
            &device,
            WgpuBindGroup0Entries {
                camera: wgpu::BindGroupEntry { 
                    binding: 0, 
                    resource: camera_uniform_buffer.as_entire_binding(), 
                },
            },
        );

        let model_uniform = ModelUniform {
            model: Mat4::IDENTITY, 
            normal_transform: Mat3A::IDENTITY, 
        };
        let model_uniform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Model Uniform Buffer"),
            contents: bytemuck::cast_slice(&[model_uniform]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });
        let model_bind_group = WgpuBindGroup1::from_bindings(
            &device,
            WgpuBindGroup1Entries {
                model: wgpu::BindGroupEntry { 
                    binding: 0, 
                    resource: model_uniform_buffer.as_entire_binding(), 
                },
            },
        );

        
        let material_uniform_init = MaterialUniformInit {
            base_color_factor: [1.0, 0.0, 0.0, 1.0].into(), 
            metallic_factor: 0.0,
            roughness_factor: 1.0,
            
        };
        let material_uniform: MaterialUniform = material_uniform_init.into(); 

        let material_uniform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Material Uniform Buffer"),
            contents: bytemuck::cast_slice(&[material_uniform]), 
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });
        let material_bind_group = WgpuBindGroup2::from_bindings(
            &device,
            WgpuBindGroup2Entries {
                material: wgpu::BindGroupEntry { 
                    binding: 0, 
                    resource: material_uniform_buffer.as_entire_binding(), 
                },
            },
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
    }

    #[allow(unused_variables)] 
    fn update(&mut self) {
        
        let elapsed = self.start_time.elapsed().as_secs_f32();

        
        let angle = elapsed * std::f32::consts::PI / 4.0; 
        let rotation = Mat4::from_rotation_z(angle);
        
        
        let normal_transform = Mat3A::from_mat4(rotation).inverse().transpose();
        let model_uniform = ModelUniform {
            model: rotation, 
            normal_transform, 
        };
        self.queue.write_buffer(&self.model_uniform_buffer, 0, bytemuck::cast_slice(&[model_uniform]));

        
        let aspect_ratio = self.size.width as f32 / self.size.height as f32;
        let (left, right, bottom, top) = if aspect_ratio > 1.0 {
            (-1.0 * aspect_ratio, 1.0 * aspect_ratio, -1.0, 1.0)
        } else {
            (-1.0, 1.0, -1.0 / aspect_ratio, 1.0 / aspect_ratio)
        };
        let view_proj = Mat4::orthographic_rh(left, right, bottom, top, -1.0, 1.0);
        let camera_uniform = CameraUniform {
            view_proj, 
            position: Vec3A::new(0.0, 0.0, 1.0), 
        };
        self.queue.write_buffer(&self.camera_uniform_buffer, 0, bytemuck::cast_slice(&[camera_uniform]));
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