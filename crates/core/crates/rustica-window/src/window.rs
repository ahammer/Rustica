use thiserror::Error;
use winit::{
    dpi::PhysicalSize,
    event_loop::ActiveEventLoop,
    window::{Window as WinitWindow, WindowAttributes, WindowId},
};
// Import the correct handle types
use raw_window_handle::{
    HasDisplayHandle, HasWindowHandle, HandleError, 
    DisplayHandle, WindowHandle // Keep these non-raw handle types
};

/// Common errors that can occur with window operations
#[derive(Debug, Error)]
pub enum WindowError {
    /// Failed to create window
    #[error("Failed to create window: {0}")]
    Creation(String),
    
    /// Window operation failed
    #[error("Window operation failed: {0}")]
    Operation(String),
}

/// A window configuration for initial creation
#[derive(Debug, Clone)]
pub struct WindowConfig {
    /// Title of the window
    pub title: String,
    /// Initial width in logical pixels
    pub width: u32,
    /// Initial height in logical pixels
    pub height: u32,
    /// Whether the window should be resizable
    pub resizable: bool,
    /// Whether the window should start maximized
    pub maximized: bool,
}

impl Default for WindowConfig {
    fn default() -> Self {
        Self {
            title: "Rustica Application".to_string(),
            width: 800,
            height: 600,
            resizable: true,
            maximized: false,
        }
    }
}

/// Represents an application window.
pub struct Window {
    /// The underlying winit window
    winit_window: WinitWindow,
}

impl Window {
    /// Creates a new window.
    ///
    /// # Example
    /// 
    /// ```
    /// # use rustica_window::{Window, WindowConfig};
    /// # use winit::event_loop::EventLoopBuilder;
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let event_loop = EventLoopBuilder::new().build()?;
    /// let config = WindowConfig::default();
    /// let window = Window::new(&event_loop, config)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn new(event_loop: &ActiveEventLoop, config: WindowConfig) -> Result<Self, WindowError> {
        let attributes = WindowAttributes::default()
            .with_title(config.title)
            .with_inner_size(PhysicalSize::new(config.width, config.height))
            .with_resizable(config.resizable)
            .with_maximized(config.maximized);

        let winit_window = event_loop
            .create_window(attributes)
            .map_err(|e| WindowError::Creation(e.to_string()))?;

        Ok(Self { winit_window })
    }

    /// Returns the unique identifier of the window.
    pub fn id(&self) -> WindowId {
        self.winit_window.id()
    }

    /// Returns the current inner size of the window's client area.
    pub fn inner_size(&self) -> PhysicalSize<u32> {
        self.winit_window.inner_size()
    }

    /// Requests that the window be redrawn.
    pub fn request_redraw(&self) {
        self.winit_window.request_redraw();
    }
}

// Implement raw window handle traits (remove unsafe)
impl HasWindowHandle for Window {
    // Return the non-raw handle type as required by the trait
    fn window_handle(&self) -> Result<WindowHandle<'_>, HandleError> {
        self.winit_window.window_handle()
    }
}

impl HasDisplayHandle for Window {
    // Return the non-raw handle type as required by the trait
    fn display_handle(&self) -> Result<DisplayHandle<'_>, HandleError> {
        self.winit_window.display_handle()
    }
}
