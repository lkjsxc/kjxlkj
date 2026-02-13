use actix_web::HttpResponse;
use serde::Serialize;
use uuid::Uuid;

use kjxlkj_domain::errors::DomainError;

/// Standard error envelope per errors.md.
#[derive(Debug, Serialize)]
pub struct ErrorEnvelope {
    pub code: String,
    pub message: String,
    pub details: Option<serde_json::Value>,
    pub request_id: String,
}

/// Convert a DomainError into an HTTP response with proper status and envelope.
pub fn domain_error_response(err: &DomainError) -> HttpResponse {
    let request_id = Uuid::new_v4().to_string();
    let (status, code, message) = match err {
        DomainError::NotFound { entity } => (
            actix_web::http::StatusCode::NOT_FOUND,
            format!("{}_NOT_FOUND", entity.to_uppercase()),
            err.to_string(),
        ),
        DomainError::VersionConflict { .. } => (
            actix_web::http::StatusCode::CONFLICT,
            "VERSION_CONFLICT".into(),
            err.to_string(),
        ),
        DomainError::SetupLocked => (
            actix_web::http::StatusCode::CONFLICT,
            "SETUP_LOCKED".into(),
            err.to_string(),
        ),
        DomainError::InvalidCredentials => (
            actix_web::http::StatusCode::UNAUTHORIZED,
            "INVALID_CREDENTIALS".into(),
            err.to_string(),
        ),
        DomainError::RoleForbidden => (
            actix_web::http::StatusCode::FORBIDDEN,
            "ROLE_FORBIDDEN".into(),
            err.to_string(),
        ),
        DomainError::BadRequest { .. } => (
            actix_web::http::StatusCode::BAD_REQUEST,
            "BAD_REQUEST".into(),
            err.to_string(),
        ),
        DomainError::RuleInvalid { .. } => (
            actix_web::http::StatusCode::UNPROCESSABLE_ENTITY,
            "RULE_INVALID".into(),
            err.to_string(),
        ),
        DomainError::RateLimited => (
            actix_web::http::StatusCode::TOO_MANY_REQUESTS,
            "RATE_LIMITED".into(),
            err.to_string(),
        ),
        DomainError::AttachmentTooLarge => (
            actix_web::http::StatusCode::PAYLOAD_TOO_LARGE,
            "ATTACHMENT_TOO_LARGE".into(),
            err.to_string(),
        ),
        DomainError::Internal(_) => (
            actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
            "INTERNAL_ERROR".into(),
            "internal error".into(),
        ),
    };
    HttpResponse::build(status).json(ErrorEnvelope {
        code,
        message,
        details: None,
        request_id,
    })
}
