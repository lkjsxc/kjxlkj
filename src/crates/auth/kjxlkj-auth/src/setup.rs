use kjxlkj_db::repos;
use kjxlkj_domain::errors::DomainError;
use kjxlkj_domain::types::GlobalRole;
use sqlx::PgPool;
use uuid::Uuid;

use crate::password::hash_password;
use crate::session::create_session;

/// Register the first owner account. Locks setup on success per auth.md.
pub async fn register_owner(
    pool: &PgPool,
    username: &str,
    password: &str,
) -> Result<String, DomainError> {
    let locked = repos::setup::is_locked(pool)
        .await
        .map_err(|e| DomainError::Internal(e.to_string()))?;
    if locked {
        return Err(DomainError::SetupLocked);
    }
    let password_hash = hash_password(password)?;
    let user_id = Uuid::new_v4();
    let user = repos::users::create(pool, user_id, username, &password_hash, GlobalRole::Owner)
        .await
        .map_err(|e| DomainError::Internal(e.to_string()))?;
    repos::setup::lock(pool)
        .await
        .map_err(|e| DomainError::Internal(e.to_string()))?;
    let token = create_session(pool, user.id).await?;
    Ok(token)
}

/// Check whether setup is already locked.
pub async fn is_setup_locked(pool: &PgPool) -> Result<bool, DomainError> {
    repos::setup::is_locked(pool)
        .await
        .map_err(|e| DomainError::Internal(e.to_string()))
}
