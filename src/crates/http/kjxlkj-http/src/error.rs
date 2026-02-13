//! HTTP error types.

use actix_web::{HttpResponse, ResponseError};
use thiserror::Error;

/// API error types.
#[derive(Debug, Error)]
pub enum ApiError {
    #[error("not found: {0}")]
    NotFound(String),
    #[error("unauthorized")]
    Unauthorized,
    #[error("forbidden")]
    Forbidden,
    #[error("bad request: {0}")]
    BadRequest(String),
    #[error("conflict: {0}")]
    Conflict(String),
    #[error("validation error: {0}")]
    Validation(String),
    #[error("internal error: {0}")]
    Internal(String),
}

impl ResponseError for ApiError {
    fn error_response(&self) -> HttpResponse {
        match self {
            ApiError::NotFound(msg) => HttpResponse::NotFound().json(serde_json::json!({
                "error": "not_found",
                "message": msg
            })),
            ApiError::Unauthorized => HttpResponse::Unauthorized().json(serde_json::json!({
                "error": "unauthorized",
                "message": "authentication required"
            })),
            ApiError::Forbidden => HttpResponse::Forbidden().json(serde_json::json!({
                "error": "forbidden",
                "message": "access denied"
            })),
            ApiError::BadRequest(msg) => HttpResponse::BadRequest().json(serde_json::json!({
                "error": "bad_request",
                "message": msg
            })),
            ApiError::Conflict(msg) => HttpResponse::Conflict().json(serde_json::json!({
                "error": "conflict",
                "message": msg
            })),
            ApiError::Validation(msg) => HttpResponse::UnprocessableEntity().json(serde_json::json!({
                "error": "validation_error",
                "message": msg
            })),
            ApiError::Internal(msg) => HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "internal_error",
                "message": msg
            })),
        }
    }
}

/// Result type for API handlers.
pub type ApiResult<T> = Result<T, ApiError>;
