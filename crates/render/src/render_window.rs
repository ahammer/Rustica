// Window with rendering capabilities

use std::time::Instant;
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, EventLoop},
    window::WindowId,
};
use rustica_window::WindowApp;

use crate::canvas::Canvas;
use crate::render_context::RenderContext;
use crate::custom_shader::{CustomShader, ShaderDescriptor};

/// A window with rendering capabilities
pub struct RenderWindow {
    window_app: WindowApp,
    render_context: RenderContext,
    frame_callback: Option<Box<dyn FnMut(&mut Canvas) + 'static>>,
}

impl RenderWindow {
    /// Create a new render window with the given title and size
    pub fn new(title: &str, width: u32, height: u32) -> Self {
        Self {
            window_app: WindowApp::new(title, width, height),
            render_context: RenderContext::new(),
            frame_callback: None,
        }
    }

    /// Get a reference to the underlying window
    pub fn window(&self) -> Option<&winit::window::Window> {
        self.window_app.window()
    }

    /// Set the clear color for the rendering surface
    pub fn set_clear_color(&mut self, r: f64, g: f64, b: f64, a: f64) {
        self.render_context.set_clear_color(r, g, b, a);
    }
    
    /// Register a custom shader and return its ID
    pub fn register_shader(&mut self, descriptor: ShaderDescriptor) -> usize {
        if let (Some(device), Some(config)) = (&self.render_context.device, &self.render_context.config) {
            let shader = CustomShader::new(device, config.format, descriptor);
            self.render_context.register_shader(shader)
        } else {
            // If the device isn't initialized yet, create a placeholder shader
            // that will be properly initialized later
            let shader = CustomShader::new_placeholder(descriptor);
            self.render_context.register_shader(shader)
        }
    }
    
    /// Set a callback function to be called each frame
    pub fn with_frame_callback<F>(mut self, callback: F) -> Self 
    where 
        F: FnMut(&mut Canvas) + 'static 
    {
        self.frame_callback = Some(Box::new(callback));
        self
    }

    /// Run the render window application
    pub fn run(mut self) -> Result<(), Box<dyn std::error::Error>> {
        let event_loop = EventLoop::new()?;
        event_loop.run_app(&mut self)?;
        Ok(())
    }
    
    // Internal method to execute the frame callback
    fn execute_frame(&mut self) -> Result<(), wgpu::SurfaceError> {
        if let Some(callback) = &mut self.frame_callback {
            // Get the elapsed time
            let elapsed = Instant::now().duration_since(self.render_context.start_time);
            
            // Create a canvas and collect draw commands
            let mut canvas = Canvas::new(elapsed);
            
            // Set the queue reference if available
            if let Some(queue) = &self.render_context.queue {
                canvas.set_queue(queue);
            }
            
            // Execute the callback
            callback(&mut canvas);
            
            // Process the draw commands
            self.render_context.process_draw_commands(&canvas.commands)?;
        }
        
        Ok(())
    }
}

impl ApplicationHandler for RenderWindow {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        self.window_app.resumed(event_loop);
        if let Err(e) = self.render_context.initialize(&self.window_app) {
            eprintln!("Render context initialization error: {:?}", e);
        }
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    ) {
        self.window_app.window_event(event_loop, window_id, event.clone());
        match event {
            WindowEvent::Resized(size) => self.render_context.resize(size),
            WindowEvent::RedrawRequested => {
                if self.render_context.surface.is_some() {
                    match self.execute_frame() {
                        Ok(_) => {}
                        Err(wgpu::SurfaceError::Lost) => {
                            if let (Some(config), Some(surface), Some(device)) = (
                                &self.render_context.config,
                                &self.render_context.surface,
                                &self.render_context.device,
                            ) {
                                surface.configure(device, config);
                            }
                        }
                        Err(wgpu::SurfaceError::OutOfMemory) => event_loop.exit(),
                        Err(e) => eprintln!("Render error: {:?}", e),
                    }
                }
            }
            _ => {}
        }
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        if let Some(window) = self.window_app.window() {
            window.request_redraw();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_render_window_new() {
        let window = RenderWindow::new("Test Render Window", 800, 600);
        assert!(window.window().is_none());
    }

    #[test]
    fn test_render_window_set_clear_color() {
        let mut window = RenderWindow::new("Test Render Window", 800, 600);
        window.set_clear_color(1.0, 0.5, 0.25, 1.0);
        // We can't directly test the clear color as it's in a private field,
        // but we can at least verify the method doesn't panic
    }
}
