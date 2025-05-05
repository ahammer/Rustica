use crate::window::{Window, WindowConfig, WindowError};
use thiserror::Error;
use winit::{
    application::ApplicationHandler,
    error::EventLoopError,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, EventLoop}, // Import EventLoop for builder
    window::WindowId,
};

/// Application specific errors
#[derive(Error, Debug)]
pub enum ApplicationError {
    #[error("Window creation failed: {0}")]
    WindowCreation(#[from] WindowError),
    #[error("Event loop creation failed: {0}")]
    EventLoopCreation(String),
    #[error("Application initialization failed: {0}")]
    Initialization(String),
    #[error("Event handling failed: {0}")]
    EventHandler(String),
    #[error("Winit event loop error: {0}")]
    EventLoopError(#[from] EventLoopError),
}

/// Events passed to the application logic
#[derive(Debug, Clone)]
pub enum ApplicationEvent {
    /// Window requested application exit
    Exit,
    /// Window resized
    Resize(winit::dpi::PhysicalSize<u32>),
    /// Window needs redraw
    RedrawRequested,
    /// Application update tick
    Update,
}

/// Trait for application implementation
pub trait RusticaApplication: Sized + 'static {
    /// Create a new instance of the application
    fn create() -> Self;

    /// Initialize application after window creation
    fn init(&mut self, window: &Window) -> Result<(), ApplicationError>;

    /// Handle application events
    fn handle_event(&mut self, event: ApplicationEvent, window: &Window) -> Result<(), ApplicationError>;
}

/// Runs the main application loop for a given RusticaApplication implementation.
pub fn run_application<App: RusticaApplication>(
    window_config: WindowConfig,
) -> Result<(), ApplicationError> {
    log::info!("Starting Rustica application loop");
    let event_loop = EventLoop::builder()
        .build()
        .map_err(|e| ApplicationError::EventLoopCreation(e.to_string()))?;

    let mut app_handler = AppHandler::<App>::new(window_config);

    event_loop
        .run_app(&mut app_handler)
        .map_err(ApplicationError::EventLoopError)
}

// Internal struct to handle winit events and bridge to RusticaApplication
pub(crate) struct AppHandler<App: RusticaApplication> {
    window_config: WindowConfig,
    window: Option<Window>,
    app: Option<App>,
}

// No visibility qualifier on impl block
impl<App: RusticaApplication> AppHandler<App> {
    // Add pub(crate) to the new function
    pub(crate) fn new(window_config: WindowConfig) -> Self {
        Self {
            window_config,
            window: None,
            app: None,
        }
    }
}

// Keep this impl internal
impl<App: RusticaApplication> ApplicationHandler for AppHandler<App> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_none() {
            log::debug!("AppHandler resumed, creating window and app");
            let window = match Window::new(event_loop, self.window_config.clone()) {
                Ok(w) => w,
                Err(e) => {
                    log::error!("Failed to create window: {}", e);
                    event_loop.exit();
                    return;
                }
            };

            let mut app = App::create();
            if let Err(e) = app.init(&window) {
                log::error!("Failed to initialize application: {}", e);
                event_loop.exit();
                return;
            }
            log::info!("Application initialized successfully");
            self.window = Some(window);
            self.app = Some(app);
        }
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    ) {
        if let (Some(window), Some(app)) = (self.window.as_ref(), self.app.as_mut()) {
            if window.id() == window_id {
                let app_event = match event {
                    WindowEvent::CloseRequested => Some(ApplicationEvent::Exit),
                    WindowEvent::Resized(size) => Some(ApplicationEvent::Resize(size)),
                    WindowEvent::RedrawRequested => Some(ApplicationEvent::RedrawRequested),
                    _ => None,
                };

                if let Some(event) = app_event {
                    log::trace!("Handling window event: {:?}", event);
                    if let Err(e) = app.handle_event(event.clone(), window) {
                        log::error!("Error handling event {:?}: {}", event, e);
                    }
                    if let ApplicationEvent::Exit = event {
                        log::info!("Exit requested by application");
                        event_loop.exit();
                    }
                }
            }
        }
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        if let (Some(window), Some(app)) = (self.window.as_ref(), self.app.as_mut()) {
            // Update app state first
            if let Err(e) = app.handle_event(ApplicationEvent::Update, window) {
                log::error!("Error handling update event: {}", e);
            }
            // Then request redraw
            window.request_redraw();
        }
    }

    fn exiting(&mut self, _event_loop: &ActiveEventLoop) {
        log::info!("Exiting application via event loop");
        // Perform any cleanup if necessary
        if let (Some(window), Some(app)) = (self.window.as_mut(), self.app.as_mut()) {
             // Use handle_event for cleanup logic if needed
             let _ = app.handle_event(ApplicationEvent::Exit, window);
        }
        self.window = None;
        self.app = None;
    }
}
