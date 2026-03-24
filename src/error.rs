//! Application error types

use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use serde::Serialize;
use std::fmt;

/// Application error type
#[derive(Debug)]
pub enum AppError {
    Unauthorized(String),
    InvalidRequest(String),
    NotFound(String),
    StorageError(String),
    DatabaseError(String),
}

/// JSON error response
#[derive(Serialize)]
struct ErrorResponse {
    error: &'static str,
    message: String,
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::Unauthorized(msg) => write!(f, "unauthorized: {}", msg),
            AppError::InvalidRequest(msg) => write!(f, "invalid_request: {}", msg),
            AppError::NotFound(msg) => write!(f, "not_found: {}", msg),
            AppError::StorageError(msg) => write!(f, "storage_error: {}", msg),
            AppError::DatabaseError(msg) => write!(f, "database_error: {}", msg),
        }
    }
}

impl std::error::Error for AppError {}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        let (status, error, message) = match self {
            AppError::Unauthorized(msg) => (StatusCode::UNAUTHORIZED, "unauthorized", msg.clone()),
            AppError::InvalidRequest(msg) => {
                (StatusCode::BAD_REQUEST, "invalid_request", msg.clone())
            }
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, "not_found", msg.clone()),
            AppError::StorageError(msg) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "storage_error",
                msg.clone(),
            ),
            AppError::DatabaseError(msg) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "storage_error",
                msg.clone(),
            ),
        };

        HttpResponse::build(status).json(ErrorResponse { error, message })
    }
}

impl From<crate::storage::StorageError> for AppError {
    fn from(err: crate::storage::StorageError) -> Self {
        AppError::StorageError(err.to_string())
    }
}

impl From<crate::core::IdError> for AppError {
    fn from(err: crate::core::IdError) -> Self {
        AppError::InvalidRequest(err.to_string())
    }
}
