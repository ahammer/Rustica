//! Window management for the Rustica engine.
//!
//! This module provides functionality for creating and managing windows
//! using the winit library.

use std::sync::{Arc, Mutex};
use winit::event_loop::{EventLoop, EventLoopBuilder};
use winit::window::{Window as WinitWindow, WindowBuilder};
use winit::dpi::{LogicalSize, PhysicalSize};
use crate::error::{Error, Result};
use log::{info, error};

/// Window configuration options.
#[derive(Debug, Clone)]
pub struct WindowConfig {
    /// The width of the window.
    pub width: u32,
    /// The height of the window.
    pub height: u32,
    /// The title of the window.
    pub title: String,
    /// Whether the window is resizable.
    pub resizable: bool,
    /// Whether the window is decorated (has titlebar, borders, etc.).
    pub decorated: bool,
    /// Whether the window is maximized.
    pub maximized: bool,
    /// Whether the window is visible.
    pub visible: bool,
}

impl Default for WindowConfig {
    fn default() -> Self {
        Self {
            width: 800,
            height: 600,
            title: "Rustica Engine".to_string(),
            resizable: true,
            decorated: true,
            maximized: false,
            visible: true,
        }
    }
}

/// Window manager for the Rustica engine.
pub struct WindowManager {
    /// The window instance.
    window: Option<Arc<WinitWindow>>,
    /// The event loop for handling window events.
    event_loop: Option<EventLoop<()>>,
    /// Window configuration.
    config: WindowConfig,
}

impl WindowManager {
    /// Create a new window manager with default configuration.
    pub fn new() -> Self {
        Self {
            window: None,
            event_loop: None,
            config: WindowConfig::default(),
        }
    }

    /// Create a new window manager with custom configuration.
    pub fn with_config(config: WindowConfig) -> Self {
        Self {
            window: None,
            event_loop: None,
            config,
        }
    }

    /// Initialize the window manager.
    pub fn initialize(&mut self) -> Result<()> {
        info!("Initializing window manager");
        
        // Create event loop
        let event_loop = EventLoopBuilder::new().build();
        
        // Create window
        let window = WindowBuilder::new()
            .with_title(&self.config.title)
            .with_inner_size(LogicalSize::new(self.config.width, self.config.height))
            .with_resizable(self.config.resizable)
            .with_decorations(self.config.decorated)
            .with_maximized(self.config.maximized)
            .with_visible(self.config.visible)
            .build(&event_loop)
            .map_err(|e| Error::WindowError(format!("Failed to create window: {}", e)))?;
        
        self.window = Some(Arc::new(window));
        self.event_loop = Some(event_loop);
        
        info!("Window created with size {}x{}", self.config.width, self.config.height);
        Ok(())
    }

    /// Get a reference to the window.
    pub fn window(&self) -> Option<Arc<WinitWindow>> {
        self.window.clone()
    }

    /// Get a reference to the event loop.
    pub fn event_loop(&self) -> Option<&EventLoop<()>> {
        self.event_loop.as_ref()
    }

    /// Set the window title.
    pub fn set_title(&self, title: &str) {
        if let Some(window) = &self.window {
            window.set_title(title);
        }
    }

    /// Resize the window.
    pub fn resize(&self, width: u32, height: u32) {
        if let Some(window) = &self.window {
            window.set_inner_size(LogicalSize::new(width, height));
        }
    }

    /// Get the window size.
    pub fn size(&self) -> (u32, u32) {
        if let Some(window) = &self.window {
            let size = window.inner_size();
            (size.width, size.height)
        } else {
            (self.config.width, self.config.height)
        }
    }

    /// Request that the window be redrawn.
    pub fn request_redraw(&self) {
        if let Some(window) = &self.window {
            window.request_redraw();
        }
    }
}

/// A resource that holds the window manager.
#[derive(Clone)]
pub struct WindowResource {
    /// The window manager.
    manager: Arc<Mutex<WindowManager>>,
}

impl WindowResource {
    /// Create a new window resource.
    pub fn new() -> Self {
        Self {
            manager: Arc::new(Mutex::new(WindowManager::new())),
        }
    }

    /// Create a new window resource with custom configuration.
    pub fn with_config(config: WindowConfig) -> Self {
        Self {
            manager: Arc::new(Mutex::new(WindowManager::with_config(config))),
        }
    }

    /// Initialize the window resource.
    pub fn initialize(&self) -> Result<()> {
        if let Ok(mut manager) = self.manager.lock() {
            manager.initialize()
        } else {
            Err(Error::WindowError("Failed to lock window manager".to_string()))
        }
    }

    /// Get a reference to the window manager.
    pub fn manager(&self) -> Arc<Mutex<WindowManager>> {
        self.manager.clone()
    }
}

impl Default for WindowResource {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_window_config_default() {
        let config = WindowConfig::default();
        assert_eq!(config.width, 800);
        assert_eq!(config.height, 600);
        assert_eq!(config.title, "Rustica Engine");
        assert!(config.resizable);
        assert!(config.decorated);
        assert!(!config.maximized);
        assert!(config.visible);
    }

    #[test]
    fn test_window_manager_with_config() {
        let config = WindowConfig {
            width: 1024,
            height: 768,
            title: "Test Window".to_string(),
            resizable: false,
            decorated: false,
            maximized: true,
            visible: false,
        };

        let manager = WindowManager::with_config(config.clone());
        assert_eq!(manager.config.width, 1024);
        assert_eq!(manager.config.height, 768);
        assert_eq!(manager.config.title, "Test Window");
        assert!(!manager.config.resizable);
        assert!(!manager.config.decorated);
        assert!(manager.config.maximized);
        assert!(!manager.config.visible);
    }
}
