//! # ECS Error Types
//! 
//! This module defines error types specific to the ECS system.
//!
//! ## AGENT DIRECTIVES
//! 
//! - DOC_ROOT: /docs/
//! - ARCH_DOC: /docs/architecture.md#ErrorHandling
//! - API_RULES: /docs/api_conventions.md#ErrorHandling
//! 
//! ## Critical Rules
//! 
//! 1. Errors should be clear and informative
//! 2. Recovery paths should be provided where possible
//! 3. Error messages should help guide users to solutions

// === REGION: IMPORTS ===
use thiserror::Error;

// === REGION: ERROR TYPES ===

/// Errors that can occur in the ECS system.
#[derive(Debug, Error)]
pub enum EcsError {
    /// Error when an entity doesn't exist.
    #[error("Entity does not exist: {0}")]
    EntityNotFound(u64),
    
    /// Error when a component doesn't exist for an entity.
    #[error("Component not found for entity {0}")]
    ComponentNotFound(u64),
    
    /// Error when a component of the wrong type is accessed.
    #[error("Component type mismatch for entity {0}")]
    ComponentTypeMismatch(u64),
    
    /// Error when a query contains incompatible component accesses.
    #[error("Query contains incompatible component accesses: {0}")]
    IncompatibleQuery(String),
    
    /// Generic ECS error.
    #[error("ECS error: {0}")]
    Generic(String),
}

impl EcsError {
    /// Creates a new generic ECS error.
    pub fn new(msg: impl Into<String>) -> Self {
        EcsError::Generic(msg.into())
    }
}

// === REGION: TESTS ===
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_error_creation() {
        let error = EcsError::EntityNotFound(42);
        assert!(error.to_string().contains("42"));
        
        let error = EcsError::new("test error");
        assert!(error.to_string().contains("test error"));
    }
}
