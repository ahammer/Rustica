# Rustica Window

The Window crate provides window management functionality for the Rustica engine.

## Overview

Rustica Window is a thin wrapper around the winit crate, providing a simplified API for window creation and event handling. It serves as the foundation for the rendering system, handling window creation, input events, and the main application loop.

## Features

- **Window Creation**: Simple API for creating application windows
- **Event Handling**: Built-in event loop with sensible defaults
- **Raw Window Handles**: Access to raw window and display handles for graphics API integration
- **Cross-Platform**: Works on Windows, macOS, Linux, and Web (via WebAssembly)

## Usage

### Creating a Basic Window

```rust
use rustica_window::WindowApp;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a window application
    let app = WindowApp::new("My Rustica App", 800, 600);
    
    // Run the application
    app.run()?;
    
    Ok(())
}
```

### Accessing Raw Window Handles

```rust
use rustica_window::WindowApp;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut app = WindowApp::new("Raw Handle Example", 800, 600);
    
    // Run the application with a custom event handler
    app.run()?;
    
    // Access raw window handle (useful for graphics APIs)
    if let Some(window) = app.window() {
        let raw_window_handle = window.window_handle().unwrap();
        let raw_display_handle = window.display_handle().unwrap();
        
        // Use raw handles with graphics API...
    }
    
    Ok(())
}
```

## Integration

Rustica Window is used by:
- **Render Crate**: Builds upon window management for rendering
