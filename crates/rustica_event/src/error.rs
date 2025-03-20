//! Error types for the rustica_event crate

use thiserror::Error;

/// A specialized Result type for rustica_event operations
pub type Result<T> = std::result::Result<T, Error>;

/// Errors that can occur in the rustica_event crate
#[derive(Error, Debug)]
pub enum Error {
    /// Event system initialization failed
    #[error("Event system initialization failed: {0}")]
    InitializationError(String),
    
    /// Event registration failed
    #[error("Event registration failed: {0}")]
    RegistrationError(String),
    
    /// Event dispatch failed
    #[error("Event dispatch failed: {0}")]
    DispatchError(String),
    
    /// Generic error with message
    #[error("{0}")]
    Generic(String),
}
