//! Error types for the math module.

use thiserror::Error;

/// Error types that can occur in math operations.
#[derive(Error, Debug, Clone, PartialEq)]
pub enum MathError {
    /// Error when trying to normalize a zero vector.
    #[error("Cannot normalize a zero vector")]
    ZeroVectorNormalize,
    
    /// Error when trying to invert a singular matrix.
    #[error("Cannot invert a singular matrix")]
    SingularMatrix,
    
    /// Division by zero.
    #[error("Division by zero")]
    DivisionByZero,
    
    /// Error when a quaternion is not normalized when it's required to be.
    #[error("Quaternion must be normalized for this operation")]
    NonNormalizedQuaternion,
}
