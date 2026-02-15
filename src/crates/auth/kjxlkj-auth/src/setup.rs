use kjxlkj_db::repo_user;
use kjxlkj_domain::error::DomainError;
use kjxlkj_domain::ids::UserId;
use kjxlkj_domain::user::Role;
use sqlx::PgPool;
use uuid::Uuid;

use crate::password::hash_password;

/// First-run owner registration per /docs/spec/security/auth.md.
/// POST /setup/register is enabled only when no owner account exists.
/// After first owner creation, setup route is locked.
pub async fn register_owner(
    pool: &PgPool,
    email: &str,
    display_name: &str,
    password: &str,
) -> Result<UserId, DomainError> {
    let owner_count = repo_user::count_owners(pool)
        .await
        .map_err(|e| DomainError::Internal(e.to_string()))?;

    if owner_count > 0 {
        return Err(DomainError::SetupLocked);
    }

    let password_hash =
        hash_password(password).map_err(|e| DomainError::Internal(e.to_string()))?;

    let user_id = UserId(Uuid::now_v7());
    repo_user::create_user(pool, user_id, email, display_name, Role::Owner, &password_hash)
        .await
        .map_err(|e| DomainError::Internal(e.to_string()))?;

    Ok(user_id)
}

/// Check if setup is locked (owner exists).
pub async fn is_setup_locked(pool: &PgPool) -> Result<bool, DomainError> {
    let count = repo_user::count_owners(pool)
        .await
        .map_err(|e| DomainError::Internal(e.to_string()))?;
    Ok(count > 0)
}
