//! Authentication services for kjxlkj.
//!
//! This crate contains password hashing, session management, and CSRF protection.

use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordVerifier, SaltString},
    Argon2, PasswordHasher as ArgonPasswordHasher,
};
use rand::RngCore;
use sha2::{Digest, Sha256};

use thiserror::Error;

/// Auth errors.
#[derive(Debug, Error)]
pub enum AuthError {
    #[error("password hashing error: {0}")]
    HashError(String),
    #[error("invalid credentials")]
    InvalidCredentials,
    #[error("session expired")]
    SessionExpired,
}

impl From<argon2::password_hash::Error> for AuthError {
    fn from(err: argon2::password_hash::Error) -> Self {
        AuthError::HashError(err.to_string())
    }
}

/// Password hasher using Argon2.
pub struct PasswordHasher {
    algorithm: Argon2<'static>,
}

impl Default for PasswordHasher {
    fn default() -> Self {
        Self::new()
    }
}

impl PasswordHasher {
    pub fn new() -> Self {
        Self {
            algorithm: Argon2::default(),
        }
    }

    /// Hash a password.
    pub fn hash(&self, password: &str) -> Result<String, AuthError> {
        let salt = SaltString::generate(&mut OsRng);
        let hash = self
            .algorithm
            .hash_password(password.as_bytes(), &salt)?
            .to_string();
        Ok(hash)
    }

    /// Verify a password against a hash.
    pub fn verify(&self, password: &str, hash: &str) -> Result<bool, AuthError> {
        let parsed_hash = PasswordHash::new(hash)?;
        Ok(self
            .algorithm
            .verify_password(password.as_bytes(), &parsed_hash)
            .is_ok())
    }
}

/// Generate a secure random token.
pub fn generate_token() -> String {
    let mut bytes = [0u8; 32];
    OsRng.fill_bytes(&mut bytes);
    hex::encode(bytes)
}

/// Generate a CSRF token.
pub fn generate_csrf_token() -> String {
    let mut bytes = [0u8; 16];
    OsRng.fill_bytes(&mut bytes);
    hex::encode(bytes)
}

/// Hash a token for storage.
pub fn hash_token(token: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(token.as_bytes());
    hex::encode(hasher.finalize())
}

/// Session configuration.
#[derive(Debug, Clone)]
pub struct SessionConfig {
    pub session_duration_hours: i64,
    pub cookie_name: String,
    pub cookie_secure: bool,
    pub cookie_http_only: bool,
    pub cookie_same_site: String,
}

impl Default for SessionConfig {
    fn default() -> Self {
        Self {
            session_duration_hours: 24 * 7, // 1 week
            cookie_name: "kjxlkj_session".to_string(),
            cookie_secure: true,
            cookie_http_only: true,
            cookie_same_site: "Strict".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_password_hash_and_verify() {
        let hasher = PasswordHasher::new();
        let password = "test_password_123";
        let hash = hasher.hash(password).unwrap();
        assert!(hasher.verify(password, &hash).unwrap());
        assert!(!hasher.verify("wrong_password", &hash).unwrap());
    }

    #[test]
    fn test_generate_token() {
        let token1 = generate_token();
        let token2 = generate_token();
        assert_ne!(token1, token2);
        assert_eq!(token1.len(), 64);
    }
}
