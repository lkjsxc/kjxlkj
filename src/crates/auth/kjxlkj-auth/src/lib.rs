//! kjxlkj-auth: Authentication, session, and CSRF logic.
//! Per /docs/spec/security/auth.md, /docs/spec/security/sessions.md,
//! /docs/spec/security/csrf.md.

use argon2::{password_hash::SaltString, Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use rand::rngs::OsRng;
use uuid::Uuid;

/// Hash a password with argon2 (memory-hard per /docs/spec/security/auth.md).
pub fn hash_password(password: &str) -> Result<String, String> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    argon2
        .hash_password(password.as_bytes(), &salt)
        .map(|h| h.to_string())
        .map_err(|e| format!("hash error: {e}"))
}

/// Verify a password against a stored hash.
pub fn verify_password(password: &str, hash: &str) -> bool {
    match PasswordHash::new(hash) {
        Ok(parsed) => Argon2::default()
            .verify_password(password.as_bytes(), &parsed)
            .is_ok(),
        Err(_) => false,
    }
}

/// Generate a CSRF token.
pub fn generate_csrf_token() -> String {
    Uuid::now_v7().to_string()
}

/// Generate a session expiry (7 days from now per /docs/spec/security/sessions.md).
pub fn session_expiry() -> String {
    let now = time::OffsetDateTime::now_utc();
    let expires = now + time::Duration::days(7);
    expires
        .format(&time::format_description::well_known::Rfc3339)
        .unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_password_hash_verify() {
        let hash = hash_password("test123").unwrap();
        assert!(verify_password("test123", &hash));
        assert!(!verify_password("wrong", &hash));
    }

    #[test]
    fn test_csrf_token() {
        let token = generate_csrf_token();
        assert!(!token.is_empty());
    }
}
