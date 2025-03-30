// Window management crate for Rustica engine

use raw_window_handle::{HasWindowHandle, HasDisplayHandle};
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, EventLoop},
    window::{Window as WinitWindow, WindowAttributes, WindowId},
};

/// A window application that handles window creation and events
#[derive(Default)]
pub struct WindowApp {
    window: Option<WinitWindow>,
    title: String,
    width: u32,
    height: u32,
}

impl WindowApp {
    /// Create a new window application with the given title and size
    pub fn new(title: &str, width: u32, height: u32) -> Self {
        Self {
            window: None,
            title: title.to_string(),
            width,
            height,
            ..Default::default()
        }
    }
    
    /// Get a reference to the window
    pub fn window(&self) -> Option<&WinitWindow> {
        self.window.as_ref()
    }
    
    /// Get the raw window handle - useful for graphics APIs
    pub fn raw_window_handle(&self) -> Option<raw_window_handle::WindowHandle<'_>> {
        self.window.as_ref().map(|window| window.window_handle().unwrap())
    }
    
    /// Get the raw display handle - also needed for graphics APIs
    pub fn raw_display_handle(&self) -> Option<raw_window_handle::DisplayHandle<'_>> {
        self.window.as_ref().map(|window| window.display_handle().unwrap())
    }
    
/// Run the window application
    pub fn run(mut self) -> Result<(), Box<dyn std::error::Error>> {
        let event_loop = EventLoop::new()?;
        event_loop.run_app(&mut self)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_window_app_new() {
        let app = WindowApp::new("Test Window", 800, 600);
        assert_eq!(app.title, "Test Window");
        assert_eq!(app.width, 800);
        assert_eq!(app.height, 600);
        assert!(app.window.is_none());
    }

    #[test]
    fn test_window_getters_with_no_window() {
        let app = WindowApp::new("Test Window", 800, 600);
        assert!(app.window().is_none());
        assert!(app.raw_window_handle().is_none());
        assert!(app.raw_display_handle().is_none());
    }
}

impl ApplicationHandler for WindowApp {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        // Create window attributes
        let window_attributes = WindowAttributes::default()
            .with_title(&self.title)
            .with_inner_size(winit::dpi::LogicalSize::new(self.width as f64, self.height as f64));
        
        // Create the window
        self.window = Some(event_loop.create_window(window_attributes).unwrap());
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            _ => (),
        }
    }
}
