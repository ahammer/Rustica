// Prelude module - Re-exports commonly used types from the Window crate

pub use crate::WindowApp;
pub use winit::event::WindowEvent;
pub use winit::event_loop::EventLoop;
pub use raw_window_handle::{HasWindowHandle, HasDisplayHandle};