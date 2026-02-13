use sha2::{Digest, Sha256};

/// Generate a CSRF token bound to a session ID.
pub fn generate_csrf_token(session_id: &str, secret: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(session_id.as_bytes());
    hasher.update(b"::");
    hasher.update(secret.as_bytes());
    hex::encode(hasher.finalize())
}

/// Validate a CSRF token against the session.
pub fn validate_csrf_token(token: &str, session_id: &str, secret: &str) -> bool {
    let expected = generate_csrf_token(session_id, secret);
    constant_time_eq(token.as_bytes(), expected.as_bytes())
}

/// Constant-time comparison to prevent timing attacks.
fn constant_time_eq(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    let mut result = 0u8;
    for (x, y) in a.iter().zip(b.iter()) {
        result |= x ^ y;
    }
    result == 0
}
