/// HTTP middleware per /docs/spec/security/csrf.md
/// Request ID generation and CSRF validation.
use axum::http::Request;
use uuid::Uuid;

/// Generate a request ID for correlation
pub fn generate_request_id() -> String {
    Uuid::new_v4().to_string()
}

/// Placeholder CSRF validation.
/// Per /docs/spec/security/csrf.md: state-changing ops must validate CSRF
/// for browser sessions. GET is exempt.
pub fn is_csrf_exempt(method: &str) -> bool {
    method == "GET" || method == "HEAD" || method == "OPTIONS"
}
