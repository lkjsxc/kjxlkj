//! Service supervisor for kjxlkj editor.
//!
//! This crate provides a supervisor for managing async services.

mod supervisor;
mod service;

pub use supervisor::Supervisor;
pub use service::{Service, ServiceHandle, ServiceMessage, ServiceStatus};

use thiserror::Error;

/// Service error type.
#[derive(Error, Debug)]
pub enum ServiceError {
    /// Service failed to start.
    #[error("Failed to start service: {0}")]
    StartFailed(String),

    /// Service crashed.
    #[error("Service crashed: {0}")]
    Crashed(String),

    /// Channel error.
    #[error("Channel error: {0}")]
    Channel(String),

    /// Timeout error.
    #[error("Timeout: {0}")]
    Timeout(String),
}

/// Result type for services.
pub type ServiceResult<T> = Result<T, ServiceError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let err = ServiceError::StartFailed("test".to_string());
        assert!(err.to_string().contains("test"));
    }
}
