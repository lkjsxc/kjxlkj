/// HTTP middleware per /docs/spec/security/csrf.md and sessions.md
///
/// - Request ID generation for structured log correlation
/// - CSRF validation for state-changing browser requests
/// - Session extraction from cookie/Authorization header
use axum::{
    extract::Request,
    http::{header, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
};
use uuid::Uuid;

/// Generate a request ID for correlation.
/// Per /docs/spec/technical/operations.md: structured logs include request_id.
pub fn generate_request_id() -> String {
    Uuid::new_v4().to_string()
}

/// Check whether a method is CSRF-exempt.
/// Per /docs/spec/security/csrf.md: GET/HEAD/OPTIONS are exempt.
pub fn is_csrf_exempt(method: &str) -> bool {
    matches!(method, "GET" | "HEAD" | "OPTIONS")
}

/// Session token extracted from cookie or Authorization header.
/// Per /docs/spec/security/sessions.md
pub const SESSION_COOKIE_NAME: &str = "kjxlkj_session";

/// Extract session token from request.
/// Checks: 1) cookie, 2) Authorization: Bearer header.
pub fn extract_session_token<B>(req: &Request<B>) -> Option<String> {
    // Check cookie first
    if let Some(cookie_header) = req.headers().get(header::COOKIE) {
        if let Ok(cookie_str) = cookie_header.to_str() {
            for pair in cookie_str.split(';') {
                let pair = pair.trim();
                if let Some(val) = pair.strip_prefix("kjxlkj_session=") {
                    return Some(val.to_string());
                }
            }
        }
    }
    // Fallback to Authorization: Bearer
    if let Some(auth) = req.headers().get(header::AUTHORIZATION) {
        if let Ok(auth_str) = auth.to_str() {
            if let Some(token) = auth_str.strip_prefix("Bearer ") {
                return Some(token.to_string());
            }
        }
    }
    None
}

/// CSRF validation middleware.
/// Per /docs/spec/security/csrf.md: mutating requests must include
/// X-CSRF-Token header matching session-bound token.
/// Public/setup endpoints are exempt.
pub async fn csrf_middleware(req: Request, next: Next) -> Response {
    let method = req.method().as_str().to_string();
    let path = req.uri().path().to_string();
    // Exempt: safe methods, setup/login/health
    if is_csrf_exempt(&method)
        || path.starts_with("/api/setup/")
        || path.starts_with("/api/auth/login")
        || path.starts_with("/api/healthz")
        || path.starts_with("/api/readyz")
    {
        return next.run(req).await;
    }
    // For now: if session token is present but no CSRF header, reject.
    // This is a stub — full implementation ties CSRF token to session.
    let has_session = extract_session_token(&req).is_some();
    let has_csrf = req.headers().contains_key("x-csrf-token");
    if has_session && !has_csrf {
        return StatusCode::FORBIDDEN.into_response();
    }
    next.run(req).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn csrf_exempt_methods() {
        assert!(is_csrf_exempt("GET"));
        assert!(is_csrf_exempt("HEAD"));
        assert!(is_csrf_exempt("OPTIONS"));
        assert!(!is_csrf_exempt("POST"));
        assert!(!is_csrf_exempt("PATCH"));
        assert!(!is_csrf_exempt("DELETE"));
    }

    #[test]
    fn extract_token_from_cookie() {
        let req = Request::builder()
            .header("cookie", "kjxlkj_session=tok123; other=val")
            .body(())
            .unwrap();
        assert_eq!(extract_session_token(&req), Some("tok123".into()));
    }

    #[test]
    fn extract_token_from_bearer() {
        let req = Request::builder()
            .header("authorization", "Bearer mytoken")
            .body(())
            .unwrap();
        assert_eq!(extract_session_token(&req), Some("mytoken".into()));
    }

    #[test]
    fn extract_token_none() {
        let req = Request::builder().body(()).unwrap();
        assert_eq!(extract_session_token(&req), None);
    }
}

