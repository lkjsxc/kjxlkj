use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use password_hash::rand_core::OsRng;
use password_hash::SaltString;
use thiserror::Error;
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Debug, Error)]
pub enum AuthError {
    #[error("password hash failed")]
    Hash,
    #[error("password verify failed")]
    Verify,
}

pub fn hash_password(password: &str) -> Result<String, AuthError> {
    let salt = SaltString::generate(&mut OsRng);
    Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .map(|result| result.to_string())
        .map_err(|_| AuthError::Hash)
}

pub fn verify_password(password_hash: &str, password: &str) -> Result<bool, AuthError> {
    let parsed_hash = PasswordHash::new(password_hash).map_err(|_| AuthError::Verify)?;
    Ok(Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok())
}

pub fn new_session_id() -> Uuid {
    Uuid::now_v7()
}

pub fn new_csrf_token() -> String {
    Uuid::now_v7().to_string()
}

pub fn session_expiry(days: i64) -> OffsetDateTime {
    OffsetDateTime::now_utc() + time::Duration::days(days)
}
