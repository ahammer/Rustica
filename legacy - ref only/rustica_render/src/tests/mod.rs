// Test utilities for the render crate

#[cfg(test)]
pub mod mocks {
    use wgpu::{
        Adapter, Backends, Device, DeviceDescriptor, Features, Instance, InstanceDescriptor, 
        Limits, MemoryHints, PowerPreference, Queue, RequestAdapterOptions, Surface, 
        SurfaceCapabilities, SurfaceConfiguration, SurfaceError, TextureFormat,
    };
    use std::sync::{Arc, Mutex};

    // Mock implementation of wgpu components for testing
    pub struct MockWgpuContext {
        pub instance: Instance,
        pub adapter: Option<Adapter>,
        pub device: Option<Device>,
        pub queue: Option<Queue>,
    }

    impl MockWgpuContext {
        pub fn new() -> Self {
            let instance = Instance::new(&InstanceDescriptor {
                backends: Backends::all(),
                ..Default::default()
            });

            Self {
                instance,
                adapter: None,
                device: None,
                queue: None,
            }
        }

        // This function would normally be async, but for testing we can use pollster to block
        pub fn request_adapter_and_device(&mut self) -> Result<(), Box<dyn std::error::Error>> {
            // In a real test, we would mock the adapter and device
            // For now, we'll try to create a real adapter with a null surface
            let adapter = pollster::block_on(self.instance.request_adapter(&RequestAdapterOptions {
                power_preference: PowerPreference::default(),
                compatible_surface: None, // No surface for headless testing
                force_fallback_adapter: true, // Use software adapter if available
            }));

            if let Some(adapter) = adapter {
                let (device, queue) = pollster::block_on(adapter.request_device(
                    &DeviceDescriptor {
                        label: None,
                        memory_hints: MemoryHints::default(),
                        required_features: Features::empty(),
                        required_limits: Limits::default(),
                    },
                    None,
                ))?;

                self.adapter = Some(adapter);
                self.device = Some(device);
                self.queue = Some(queue);
                Ok(())
            } else {
                // If we can't create a real adapter, we'll just skip the test
                // This allows tests to pass in CI environments without a GPU
                Ok(())
            }
        }
    }

    // Mock surface for testing
    pub struct MockSurface {
        pub config: Option<SurfaceConfiguration>,
        pub error_on_get_current_texture: bool,
    }

    impl MockSurface {
        pub fn new() -> Self {
            Self {
                config: None,
                error_on_get_current_texture: false,
            }
        }

        pub fn configure(&mut self, _device: &Device, config: &SurfaceConfiguration) {
            self.config = Some(config.clone());
        }

        pub fn get_current_texture(&self) -> Result<MockSurfaceTexture, SurfaceError> {
            if self.error_on_get_current_texture {
                Err(SurfaceError::Lost)
            } else {
                Ok(MockSurfaceTexture {})
            }
        }
    }

    pub struct MockSurfaceTexture {}

    impl MockSurfaceTexture {
        pub fn present(self) {
            // Do nothing in the mock
        }
    }
}
