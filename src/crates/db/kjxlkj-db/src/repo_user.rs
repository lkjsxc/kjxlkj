use sqlx::{PgPool, FromRow};
use uuid::Uuid;
use time::OffsetDateTime;

/// User row for authentication.
#[derive(FromRow)]
pub struct UserRow {
    pub id: Uuid,
    pub username: String,
    pub display_name: String,
    pub password_hash: String,
    pub is_disabled: bool,
    pub is_owner: bool,
    pub created_at: OffsetDateTime,
}

/// Create a user in the database.
pub async fn create_user(
    pool: &PgPool,
    id: Uuid,
    username: &str,
    display_name: &str,
    password_hash: &str,
    is_owner: bool,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        "INSERT INTO users (id, username, display_name, password_hash, is_owner)
         VALUES ($1, $2, $3, $4, $5)"
    )
    .bind(id)
    .bind(username)
    .bind(display_name)
    .bind(password_hash)
    .bind(is_owner)
    .execute(pool)
    .await?;
    Ok(())
}

/// Find user by username for login.
pub async fn find_user_by_username(
    pool: &PgPool,
    username: &str,
) -> Result<Option<UserRow>, sqlx::Error> {
    let row = sqlx::query_as::<_, UserRow>(
        "SELECT id, username, display_name, password_hash,
                is_disabled, is_owner, created_at
         FROM users WHERE username = $1"
    )
    .bind(username)
    .fetch_optional(pool)
    .await?;
    Ok(row)
}

/// Check if any owner account exists (for setup-lock).
pub async fn owner_exists(pool: &PgPool) -> Result<bool, sqlx::Error> {
    let row: (bool,) = sqlx::query_as(
        "SELECT EXISTS(SELECT 1 FROM users WHERE is_owner = true)"
    )
    .fetch_one(pool)
    .await?;
    Ok(row.0)
}
