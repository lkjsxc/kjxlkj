use sqlx::PgPool;
use uuid::Uuid;
use kjxlkj_db::repo_user;
use crate::password;

/// Register the first owner account.
/// Returns error if an owner already exists (setup-lock).
pub async fn register_owner(
    pool: &PgPool,
    username: &str,
    display_name: &str,
    password_raw: &str,
) -> Result<Uuid, SetupError> {
    // Setup-lock: check if an owner already exists
    let exists = repo_user::owner_exists(pool)
        .await
        .map_err(SetupError::Db)?;
    if exists {
        return Err(SetupError::Locked);
    }

    let id = Uuid::now_v7();
    let hash = password::hash_password(password_raw)
        .map_err(|_| SetupError::Internal("password hash failed".into()))?;

    repo_user::create_user(pool, id, username, display_name, &hash, true)
        .await
        .map_err(SetupError::Db)?;

    Ok(id)
}

#[derive(Debug)]
pub enum SetupError {
    Locked,
    Db(sqlx::Error),
    Internal(String),
}
