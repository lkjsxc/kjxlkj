// Auth middleware per /docs/spec/security/csrf.md
use actix_web::{HttpRequest, HttpResponse};
use kjxlkj_domain::error::DomainError;
use serde::Serialize;
use uuid::Uuid;

/// Error response envelope per /docs/spec/api/errors.md
#[derive(Serialize)]
pub struct ErrorResponse {
    pub code: String,
    pub message: String,
    pub details: Option<serde_json::Value>,
    pub request_id: String,
}

/// Build error response from DomainError.
pub fn error_response(err: &DomainError) -> HttpResponse {
    let request_id = Uuid::now_v7().to_string();
    let body = ErrorResponse {
        code: err.code().to_string(),
        message: err.to_string(),
        details: None,
        request_id,
    };
    HttpResponse::build(
        actix_web::http::StatusCode::from_u16(err.status_code())
            .unwrap_or(actix_web::http::StatusCode::INTERNAL_SERVER_ERROR),
    )
    .json(body)
}

/// Extract session token from cookie.
pub fn extract_session_token(req: &HttpRequest) -> Option<String> {
    req.cookie("session")
        .map(|c| c.value().to_string())
}

/// Validate CSRF token from header against session CSRF.
pub fn validate_csrf(req: &HttpRequest, expected: &str) -> bool {
    req.headers()
        .get("X-CSRF-Token")
        .and_then(|v| v.to_str().ok())
        .map(|v| v == expected)
        .unwrap_or(false)
}
