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

    #[test]
    fn test_error_crashed() {
        let err = ServiceError::Crashed("panic".to_string());
        assert!(err.to_string().contains("crashed"));
        assert!(err.to_string().contains("panic"));
    }

    #[test]
    fn test_error_channel() {
        let err = ServiceError::Channel("disconnected".to_string());
        assert!(err.to_string().contains("Channel"));
    }

    #[test]
    fn test_error_timeout() {
        let err = ServiceError::Timeout("5s".to_string());
        assert!(err.to_string().contains("Timeout"));
        assert!(err.to_string().contains("5s"));
    }

    #[test]
    fn test_error_debug() {
        let err = ServiceError::StartFailed("test".to_string());
        let debug = format!("{:?}", err);
        assert!(debug.contains("StartFailed"));
    }

    #[test]
    fn test_service_result_ok() {
        let result: ServiceResult<i32> = Ok(42);
        assert_eq!(result.unwrap(), 42);
    }

    #[test]
    fn test_service_result_err() {
        let result: ServiceResult<i32> = Err(ServiceError::Timeout("test".to_string()));
        assert!(result.is_err());
    }
}
