use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;
use serde_json::{json, Value};

#[derive(Debug)]
pub struct ApiError {
    pub status: StatusCode,
    pub code: &'static str,
    pub message: String,
    pub details: Option<Value>,
    pub request_id: String,
}

#[derive(Serialize)]
struct ErrorEnvelope<'a> {
    code: &'a str,
    message: &'a str,
    details: Option<&'a Value>,
    request_id: &'a str,
}

impl ApiError {
    pub fn new(
        status: StatusCode,
        code: &'static str,
        message: impl Into<String>,
        request_id: String,
    ) -> Self {
        Self {
            status,
            code,
            message: message.into(),
            details: None,
            request_id,
        }
    }

    pub fn with_details(mut self, details: Value) -> Self {
        self.details = Some(details);
        self
    }

    pub fn auth_required(request_id: String) -> Self {
        Self::new(
            StatusCode::UNAUTHORIZED,
            "AUTH_REQUIRED",
            "authentication required",
            request_id,
        )
    }

    pub fn role_forbidden(request_id: String) -> Self {
        Self::new(
            StatusCode::FORBIDDEN,
            "ROLE_FORBIDDEN",
            "insufficient permissions",
            request_id,
        )
    }

    pub fn csrf_invalid(request_id: String) -> Self {
        Self::new(
            StatusCode::FORBIDDEN,
            "CSRF_INVALID",
            "csrf validation failed",
            request_id,
        )
    }

    pub fn not_found(code: &'static str, message: impl Into<String>, request_id: String) -> Self {
        Self::new(StatusCode::NOT_FOUND, code, message, request_id)
    }

    pub fn conflict(
        code: &'static str,
        message: impl Into<String>,
        request_id: String,
        expected_version: u64,
        current_version: u64,
    ) -> Self {
        Self::new(StatusCode::CONFLICT, code, message, request_id).with_details(json!({
            "expected_version": expected_version,
            "current_version": current_version,
        }))
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let envelope = ErrorEnvelope {
            code: self.code,
            message: &self.message,
            details: self.details.as_ref(),
            request_id: &self.request_id,
        };
        (self.status, Json(envelope)).into_response()
    }
}
