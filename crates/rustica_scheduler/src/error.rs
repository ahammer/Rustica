//! Error types for the scheduler module.

use thiserror::Error;

/// Error types that can occur in scheduler operations.
#[derive(Error, Debug, Clone)]
pub enum SchedulerError {
    /// Error when a system with the specified name is not found.
    #[error("System not found: {0}")]
    SystemNotFound(String),
    
    /// Error when a system with the same name is already registered.
    #[error("System already exists: {0}")]
    SystemAlreadyExists(String),
    
    /// Error when there's a cycle in the system dependencies.
    #[error("Dependency cycle detected in systems")]
    DependencyCycle,
    
    /// Error when a system depends on a system that doesn't exist.
    #[error("Dependency not found: {0}")]
    DependencyNotFound(String),
    
    /// Error when a system execution fails.
    #[error("System execution failed: {0}")]
    SystemExecutionFailed(String),
    
    /// Error from the ECS module.
    #[error("ECS error: {0}")]
    EcsError(#[from] rustica_ecs::error::EcsError),
}
