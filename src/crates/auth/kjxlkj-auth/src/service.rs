/// Auth service per /docs/spec/security/auth.md
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use chrono::{Duration, Utc};
use kjxlkj_domain::permission::Role;
use kjxlkj_domain::DomainError;
use uuid::Uuid;

use kjxlkj_db::user_repo::{SessionRecord, UserRecord};

/// Auth service providing register/login/logout/session operations.
pub struct AuthService;

impl AuthService {
    /// Hash password with Argon2 per /docs/spec/security/auth.md
    pub fn hash_password(password: &str) -> Result<String, DomainError> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        argon2
            .hash_password(password.as_bytes(), &salt)
            .map(|h| h.to_string())
            .map_err(|e| DomainError::Internal(format!("hash: {e}")))
    }

    /// Verify password against stored hash
    pub fn verify_password(password: &str, hash: &str) -> Result<bool, DomainError> {
        let parsed =
            PasswordHash::new(hash).map_err(|e| DomainError::Internal(format!("parse hash: {e}")))?;
        Ok(Argon2::default()
            .verify_password(password.as_bytes(), &parsed)
            .is_ok())
    }

    /// Generate a session token
    pub fn generate_session_token() -> String {
        format!("{}-{}", Uuid::new_v4(), Uuid::new_v4())
    }

    /// Build a registration user record
    pub fn build_owner_user(
        username: &str,
        password: &str,
    ) -> Result<UserRecord, DomainError> {
        let hash = Self::hash_password(password)?;
        Ok(UserRecord {
            id: Uuid::new_v4(),
            username: username.to_string(),
            password_hash: hash,
            role: Role::Owner,
            disabled: false,
            created_at: Utc::now().naive_utc(),
        })
    }

    /// Build a session record with 7-day TTL per /docs/spec/security/sessions.md
    pub fn build_session(user_id: Uuid, role: Role) -> SessionRecord {
        SessionRecord {
            id: Uuid::new_v4(),
            user_id,
            token: Self::generate_session_token(),
            role,
            expires_at: (Utc::now() + Duration::days(7)).naive_utc(),
            created_at: Utc::now().naive_utc(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_password_hash_verify() {
        let hash = AuthService::hash_password("test123").unwrap();
        assert!(AuthService::verify_password("test123", &hash).unwrap());
        assert!(!AuthService::verify_password("wrong", &hash).unwrap());
    }

    #[test]
    fn test_session_token_uniqueness() {
        let t1 = AuthService::generate_session_token();
        let t2 = AuthService::generate_session_token();
        assert_ne!(t1, t2);
    }

    #[test]
    fn test_build_owner() {
        let user = AuthService::build_owner_user("admin", "pass").unwrap();
        assert_eq!(user.role, Role::Owner);
        assert!(!user.disabled);
    }

    #[test]
    fn test_session_ttl() {
        let session = AuthService::build_session(Uuid::new_v4(), Role::Owner);
        let diff = session.expires_at - session.created_at;
        // Should be approximately 7 days
        assert!(diff.num_days() >= 6 && diff.num_days() <= 8);
    }
}
