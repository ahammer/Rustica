use glam::{Mat4, Vec3, Vec4};
use rustica_shader_bindings::pbr_shader::{
    CameraUniform, MaterialUniform, MaterialUniformInit, ModelUniform,
    VertexInput, WgpuBindGroup0, WgpuBindGroup0Entries,
    WgpuBindGroup1, WgpuBindGroup1Entries,
    WgpuBindGroup2, WgpuBindGroup2Entries,
    create_pipeline_layout, create_shader_module, vs_main_entry, fs_main_entry,
};
use std::time::Instant;
use winit::{
    event::WindowEvent, // Removed unnecessary braces
    event_loop::{EventLoop, EventLoopWindowTarget},
    window::{Window, WindowBuilder},
    application::ApplicationHandler,
};
use wgpu::util::DeviceExt;

// State structure to manage rendering resources, now with lifetime 'a
struct State<'a> {
    surface: wgpu::Surface<'a>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: winit::dpi::PhysicalSize<u32>,
    render_pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    num_vertices: u32,
    start_time: Instant,
    window: &'a Window, // Store a reference to the window

    // PBR shader bind groups
    camera_bind_group: WgpuBindGroup0,
    camera_uniform_buffer: wgpu::Buffer,
    model_bind_group: WgpuBindGroup1,
    model_uniform_buffer: wgpu::Buffer,
    material_bind_group: WgpuBindGroup2,
    material_uniform_buffer: wgpu::Buffer,
}

// Triangle vertex data
const VERTICES: &[VertexInput] = &[
    VertexInput {
        position: [0.0, 0.5, 0.0, 1.0],
        tex_coords: [0.5, 0.0],
        normal: [0.0, 0.0, 1.0],
        tangent: [1.0, 0.0, 0.0],
    },
    VertexInput {
        position: [-0.5, -0.5, 0.0, 1.0],
        tex_coords: [0.0, 1.0],
        normal: [0.0, 0.0, 1.0],
        tangent: [1.0, 0.0, 0.0],
    },
    VertexInput {
        position: [0.5, -0.5, 0.0, 1.0],
        tex_coords: [1.0, 1.0],
        normal: [0.0, 0.0, 1.0],
        tangent: [1.0, 0.0, 0.0],
    },
];

impl<'a> State<'a> {
    async fn new(window: &'a Window) -> Self {
        let size = window.inner_size();

        // Initialize wgpu instance, adapter, device, and queue
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            flags: Default::default(),
            instance_flags: wgpu::InstanceFlags::default(),
            dx12_shader_compiler: wgpu::Dx12Compiler::Fxc,
        });

        // The surface needs to live as long as the window that created it.
        // State owns the window reference, so this should be safe.
        let surface = instance.create_surface(window).unwrap();

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
                None, // Trace path
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
            present_mode: surface_caps.present_modes[0], // Fifo is generally preferred
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2, // Added default value
        };
        surface.configure(&device, &config);

        // Create shader module and pipeline layout using PBR bindings
        let shader_module = create_shader_module(&device);
        let pipeline_layout = create_pipeline_layout(&device);

        // Create the render pipeline
        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader_module,
                entry_point: vs_main_entry(),
                buffers: &[VertexInput::desc()], // Use VertexInput descriptor
                compilation_options: wgpu::PipelineCompilationOptions::default(), // Added
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader_module,
                entry_point: fs_main_entry(),
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: wgpu::PipelineCompilationOptions::default(), // Added
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
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
            cache: None, // Added
        });

        // Create vertex buffer
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(VERTICES),
            usage: wgpu::BufferUsages::VERTEX,
        });
        let num_vertices = VERTICES.len() as u32;

        // Create uniform buffers and bind groups
        let camera_uniform = CameraUniform {
            view_proj: Mat4::IDENTITY.to_cols_array_2d(),
            position: Vec4::ZERO.to_array(), // Add position field
        };
        let camera_uniform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Camera Uniform Buffer"),
            contents: bytemuck::cast_slice(&[camera_uniform]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });
        let camera_bind_group = WgpuBindGroup0::from_bindings(
            &device,
            WgpuBindGroup0Entries {
                camera: camera_uniform_buffer.as_entire_buffer_binding(),
            },
        );

        let model_uniform = ModelUniform {
            model: Mat4::IDENTITY.to_cols_array_2d(),
        };
        let model_uniform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Model Uniform Buffer"),
            contents: bytemuck::cast_slice(&[model_uniform]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });
        let model_bind_group = WgpuBindGroup1::from_bindings(
            &device,
            WgpuBindGroup1Entries {
                model: model_uniform_buffer.as_entire_buffer_binding(),
            },
        );

        let material_uniform = MaterialUniform::new(MaterialUniformInit { // Use MaterialUniform::new
            color: Vec4::new(1.0, 0.0, 0.0, 1.0).to_array(), // Red triangle
            ..Default::default() // Use default for other fields
        });
        let material_uniform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Material Uniform Buffer"),
            contents: bytemuck::cast_slice(&[material_uniform]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });
        let material_bind_group = WgpuBindGroup2::from_bindings(
            &device,
            WgpuBindGroup2Entries {
                material: material_uniform_buffer.as_entire_buffer_binding(),
            },
        );

        Self {
            surface,
            device,
            queue,
            config,
            size,
            render_pipeline,
            vertex_buffer,
            num_vertices,
            start_time: Instant::now(),
            window, // Store the window reference
            camera_bind_group,
            camera_uniform_buffer,
            model_bind_group,
            model_uniform_buffer,
            material_bind_group,
            material_uniform_buffer,
        }
    }

    fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
        }
    }

    fn update(&mut self) {
        let elapsed = self.start_time.elapsed().as_secs_f32();

        // Simple camera setup (looking down Z-axis)
        let eye = Vec3::new(0.0, 0.0, 2.0);
        let target = Vec3::ZERO;
        let up = Vec3::Y;
        let view = Mat4::look_at_rh(eye, target, up);
        let proj = Mat4::perspective_rh_gl(
            45.0f32.to_radians(),
            self.size.width as f32 / self.size.height as f32,
            0.1,
            100.0,
        );
        let view_proj = proj * view;

        let camera_uniform = CameraUniform {
            view_proj: view_proj.to_cols_array_2d(),
            position: Vec4::new(eye.x, eye.y, eye.z, 1.0).to_array(),
        };
        self.queue.write_buffer(
            &self.camera_uniform_buffer,
            0,
            bytemuck::cast_slice(&[camera_uniform]),
        );

        // Simple model rotation
        let model_matrix = Mat4::from_rotation_y(elapsed * 0.5); // Rotate around Y
        let model_uniform = ModelUniform {
            model: model_matrix.to_cols_array_2d(),
        };
        self.queue.write_buffer(
            &self.model_uniform_buffer,
            0,
            bytemuck::cast_slice(&[model_uniform]),
        );
    }

    fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let output = self.surface.get_current_texture()?;
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
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

            // Set bind groups using the PBR shader binding helpers
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

// Implement ApplicationHandler for modern winit
struct App<'a> { // Add lifetime
    state: Option<State<'a>>,
    window: Option<Window>, // Store the window owned by the event loop
}

impl ApplicationHandler for App<'_> { // Use anonymous lifetime
    fn resumed(&mut self, event_loop: &EventLoopWindowTarget<()>) {
        let window = WindowBuilder::new().build(event_loop).unwrap();
        // Need to store the window somewhere it can live long enough
        // A simple way for this example is to store it in App
        let window_ref = self.window.insert(window);
        self.state = Some(pollster::block_on(State::new(window_ref)));
    }

    fn window_event(
        &mut self,
        event_loop: &EventLoopWindowTarget<()>,
        _window_id: winit::window::WindowId,
        event: WindowEvent,
    ) {
        let state = match self.state.as_mut() {
            Some(s) => s,
            None => return, // Should not happen if resumed was called
        };

        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::Resized(physical_size) => {
                state.resize(physical_size);
            }
            WindowEvent::RedrawRequested => {
                state.update();
                match state.render() {
                    Ok(_) => {}
                    // Reconfigure the surface if lost
                    Err(wgpu::SurfaceError::Lost) => state.resize(state.size),
                    // The system is out of memory, we should probably quit
                    Err(wgpu::SurfaceError::OutOfMemory) => event_loop.exit(),
                    // All other errors (Outdated, Timeout) should be resolved by the next frame
                    Err(e) => eprintln!("{:?}", e),
                }
                // Request the next frame
                state.window.request_redraw();
            }
            _ => {}
        }
    }
}

fn main() {
    env_logger::init(); // Initialize logging
    let event_loop = EventLoop::new().unwrap();
    let mut app = App { state: None, window: None }; // Initialize App state
    event_loop.run_app(&mut app).unwrap(); // Run the application
}
