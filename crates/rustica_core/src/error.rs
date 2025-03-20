//! # Error types for the Rustica Core
//! 
//! This module defines the error types used by the Rustica Core.
//!
//! ## AGENT DIRECTIVES
//! 
//! - DOC_ROOT: /docs/
//! - ARCH_DOC: /docs/architecture.md#ErrorHandling
//! - API_RULES: /docs/api_conventions.md#ErrorHandling
//! 
//! ## Critical Rules
//! 
//! 1. All errors must be clearly documented and include context
//! 2. Fatal vs. recoverable errors should be clearly distinguished
//! 3. Error messages should be user-friendly and actionable

// === REGION: IMPORTS ===
use thiserror::Error;

// === REGION: ERROR DEFINITION ===

/// Errors that can occur in the Rustica Core.
#[derive(Debug, Error)]
pub enum CoreError {
    /// Error during plugin registration or execution.
    #[error("Plugin error: {0}")]
    PluginError(String),
    
    /// Error when a plugin is registered multiple times.
    #[error("Duplicate plugin: {0}")]
    DuplicatePlugin(String),
    
    /// Error when a plugin dependency is missing.
    #[error("Missing plugin dependency: {0} requires {1}")]
    MissingDependency(String, String),
    
    /// Generic engine initialization error.
    #[error("Engine initialization error: {0}")]
    InitializationError(String),
    
    /// Resource not found error.
    #[error("Resource not found: {0}")]
    ResourceNotFound(String),
}

// === REGION: TESTS ===
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_error_messages() {
        let plugin_error = CoreError::PluginError("test error".to_string());
        assert!(plugin_error.to_string().contains("Plugin error"));
        
        let duplicate_error = CoreError::DuplicatePlugin("TestPlugin".to_string());
        assert!(duplicate_error.to_string().contains("Duplicate plugin"));
        
        let dependency_error = CoreError::MissingDependency(
            "TestPlugin".to_string(), 
            "CorePlugin".to_string()
        );
        assert!(dependency_error.to_string().contains("Missing plugin dependency"));
    }
}
