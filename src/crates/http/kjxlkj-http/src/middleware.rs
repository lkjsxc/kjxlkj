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
                    if !val.is_empty() {
                        return Some(val.to_string());
                    }
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

/// Extract CSRF token header value from request.
fn extract_csrf_header<B>(req: &Request<B>) -> Option<String> {
    req.headers()
        .get("x-csrf-token")
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string())
}

/// CSRF validation middleware.
/// Per /docs/spec/security/csrf.md: mutating requests must include
/// X-CSRF-Token header matching session-bound token.
/// Public/setup endpoints are exempt.
pub async fn csrf_middleware(
    state: axum::extract::State<crate::state::AppState>,
    req: Request,
    next: Next,
) -> Response {
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
    // If session token present via cookie (browser session),
    // CSRF header must match session's csrf_token.
    let has_cookie_session = req
        .headers()
        .get(header::COOKIE)
        .and_then(|v| v.to_str().ok())
        .map(|s| s.contains("kjxlkj_session="))
        .unwrap_or(false);
    if has_cookie_session {
        let session_token = extract_session_token(&req);
        let csrf_header = extract_csrf_header(&req);
        if let (Some(stok), Some(ctok)) = (&session_token, &csrf_header) {
            // Validate against stored session
            use kjxlkj_db::user_repo::SessionRepo;
            if let Ok(Some(sess)) = state.session_repo.get_session_by_token(stok) {
                if sess.csrf_token != *ctok {
                    return StatusCode::FORBIDDEN.into_response();
                }
            } else {
                return StatusCode::FORBIDDEN.into_response();
            }
        } else if session_token.is_some() && csrf_header.is_none() {
            return StatusCode::FORBIDDEN.into_response();
        }
    }
    // Bearer-token API requests are exempt from CSRF
    // (not browser-originated, per /docs/spec/security/csrf.md)
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

    #[test]
    fn extract_csrf_header_present() {
        let req = Request::builder()
            .header("x-csrf-token", "abc-123")
            .body(())
            .unwrap();
        assert_eq!(extract_csrf_header(&req), Some("abc-123".into()));
    }

    #[test]
    fn extract_csrf_header_absent() {
        let req = Request::builder().body(()).unwrap();
        assert_eq!(extract_csrf_header(&req), None);
    }
}

