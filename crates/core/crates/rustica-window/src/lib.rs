//! Rustica Window - Window management abstraction for graphics applications
//!
//! This crate provides a simplified API for managing windows and application 
//! lifecycle, hiding the complexity of the underlying windowing system.
//!
//! # Example
//! 
//! ```
//! use rustica_window::{Window, WindowConfig, ApplicationEvent, RusticaApplication, ApplicationError};
//! 
//! struct MyApp;
//! 
//! impl RusticaApplication for MyApp {
//!     fn create() -> Self {
//!         MyApp
//!     }
//!     
//!     fn init(&mut self, _window: Window) -> Result<(), ApplicationError> {
//!         Ok(())
//!     }
//!     
//!     fn handle_event(&mut self, event: ApplicationEvent, _window: &Window) -> Result<(), ApplicationError> {
//!         match event {
//!             ApplicationEvent::Init => println!("App initialized"),
//!             ApplicationEvent::Exit => println!("App exiting"),
//!             _ => {}
//!         }
//!         Ok(())
//!     }
//! }
//! 
//! // In main function:
//! // MyApp::run(WindowConfig::default()).unwrap();
//! ```

pub mod application;
pub mod window;

pub use application::{run_application, ApplicationError, ApplicationEvent, RusticaApplication};
pub use window::{Window, WindowConfig, WindowError};

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_window_config_default() {
        let config = WindowConfig::default();
        assert_eq!(config.width, 800);
        assert_eq!(config.height, 600);
        assert_eq!(config.title, "Rustica Application");
        assert!(config.resizable);
        assert!(!config.maximized);
    }
}
