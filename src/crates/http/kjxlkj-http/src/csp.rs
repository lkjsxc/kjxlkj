/// Content Security Policy middleware per /docs/spec/security/transport.md (IMP-SEC-01)
///
/// Adds CSP header with nonce for inline scripts.
/// Per spec: script execution hardening via nonce strategy.
use axum::{
    extract::Request,
    middleware::Next,
    response::Response,
};
use uuid::Uuid;

/// CSP nonce header for downstream consumers (e.g. HTML templates).
pub const CSP_NONCE_HEADER: &str = "x-csp-nonce";

/// Generate a random CSP nonce (base64-url-safe UUID).
fn generate_nonce() -> String {
    Uuid::new_v4().to_string().replace('-', "")
}

/// CSP middleware adds Content-Security-Policy header per IMP-SEC-01.
///
/// Policy rules:
/// - default-src 'self'
/// - script-src 'self' with nonce for inline scripts
/// - style-src 'self' 'unsafe-inline' (needed for editor styling)
/// - connect-src 'self' (for API + WS)
/// - img-src 'self' data: (for embedded images)
/// - frame-ancestors 'none' (clickjacking protection)
/// - base-uri 'self'
/// - form-action 'self'
pub async fn csp_middleware(req: Request, next: Next) -> Response {
    let nonce = generate_nonce();

    let mut response = next.run(req).await;

    let csp_value = format!(
        "default-src 'self'; \
         script-src 'self' 'nonce-{}'; \
         style-src 'self' 'unsafe-inline'; \
         connect-src 'self'; \
         img-src 'self' data:; \
         frame-ancestors 'none'; \
         base-uri 'self'; \
         form-action 'self'",
        nonce
    );

    // Set CSP header
    if let Ok(hv) = csp_value.parse() {
        response
            .headers_mut()
            .insert("content-security-policy", hv);
    }

    // Expose nonce via custom header for SSR/template injection
    if let Ok(hv) = nonce.parse() {
        response
            .headers_mut()
            .insert(CSP_NONCE_HEADER, hv);
    }

    response
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nonce_is_32_hex_chars() {
        let nonce = generate_nonce();
        assert_eq!(nonce.len(), 32);
        assert!(nonce.chars().all(|c| c.is_ascii_hexdigit()));
    }

    #[test]
    fn csp_nonce_header_constant() {
        assert_eq!(CSP_NONCE_HEADER, "x-csp-nonce");
    }
}
