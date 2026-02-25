//! HTTP request/response state

use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response, Json},
    Json as AxumJson,
};
use serde_json::json;

use kjxlkj_domain::ConcurrencyError;
use kjxlkj_db::DbError;

/// Application state extractor
pub type Appstate = State<crate::routes::HttpState>;

/// Error response
#[derive(Debug, thiserror::Error)]
pub enum HttpError {
    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Bad request: {0}")]
    BadRequest(String),

    #[error("Unauthorized: {0}")]
    Unauthorized(String),

    #[error("Conflict: {0}")]
    Conflict(String),

    #[error("Internal error: {0}")]
    Internal(String),
}

impl IntoResponse for HttpError {
    fn into_response(self) -> Response {
        let (status, message) = match &self {
            HttpError::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
            HttpError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg),
            HttpError::Unauthorized(msg) => (StatusCode::UNAUTHORIZED, msg),
            HttpError::Conflict(msg) => (StatusCode::CONFLICT, msg),
            HttpError::Internal(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
        };

        let body = AxumJson(json!({
            "error": format!("{:?}", self),
            "message": message,
        }));

        (status, body).into_response()
    }
}

impl From<DbError> for HttpError {
    fn from(err: DbError) -> Self {
        match err {
            DbError::NotFound(msg) => HttpError::NotFound(msg),
            DbError::ConcurrencyConflict { expected, current } => {
                HttpError::Conflict(format!(
                    "Version conflict: expected {}, got {}",
                    expected, current
                ))
            }
            _ => HttpError::Internal(err.to_string()),
        }
    }
}

impl From<ConcurrencyError> for HttpError {
    fn from(err: ConcurrencyError) -> Self {
        HttpError::Conflict(format!(
            "Version conflict: expected {}, got {}",
            err.expected_version, err.current_version
        ))
    }
}

/// Result type for HTTP handlers
pub type HttpResult<T> = Result<T, HttpError>;
