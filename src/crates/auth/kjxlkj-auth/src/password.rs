use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use kjxlkj_domain::errors::DomainError;

/// Hash a plaintext password using Argon2id (memory-hard per auth.md).
pub fn hash_password(password: &str) -> Result<String, DomainError> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|e| DomainError::Internal(format!("password hash error: {e}")))?;
    Ok(hash.to_string())
}

/// Verify a plaintext password against a stored hash.
pub fn verify_password(password: &str, hash: &str) -> Result<bool, DomainError> {
    let parsed = PasswordHash::new(hash)
        .map_err(|e| DomainError::Internal(format!("password parse error: {e}")))?;
    Ok(Argon2::default()
        .verify_password(password.as_bytes(), &parsed)
        .is_ok())
}
