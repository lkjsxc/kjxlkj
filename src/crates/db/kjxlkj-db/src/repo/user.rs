//! User repository per /docs/spec/api/http.md setup/auth routes.

use sqlx::{FromRow, PgPool};
use uuid::Uuid;

#[derive(Debug, FromRow)]
pub struct UserRow {
    pub id: Uuid,
    pub email: String,
    pub display_name: String,
    pub role: String,
    pub status: String,
    pub password_hash: String,
    pub created_at: String,
}

/// Check if any owner exists (for setup-lock).
pub async fn owner_exists(pool: &PgPool) -> Result<bool, sqlx::Error> {
    let row: (i64,) =
        sqlx::query_as("SELECT COUNT(*) FROM users WHERE role = 'owner'")
            .fetch_one(pool)
            .await?;
    Ok(row.0 > 0)
}

/// Create user.
pub async fn create_user(
    pool: &PgPool,
    id: Uuid,
    email: &str,
    display_name: &str,
    role: &str,
    password_hash: &str,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        "INSERT INTO users (id, email, display_name, role, password_hash) \
         VALUES ($1, $2, $3, $4, $5)",
    )
    .bind(id)
    .bind(email)
    .bind(display_name)
    .bind(role)
    .bind(password_hash)
    .execute(pool)
    .await?;
    Ok(())
}

/// Find user by email.
pub async fn find_by_email(
    pool: &PgPool,
    email: &str,
) -> Result<Option<UserRow>, sqlx::Error> {
    sqlx::query_as::<_, UserRow>(
        "SELECT id, email, display_name, role, status, \
         password_hash, created_at::text as created_at \
         FROM users WHERE email = $1",
    )
    .bind(email)
    .fetch_optional(pool)
    .await
}

/// Find user by id.
pub async fn find_by_id(
    pool: &PgPool,
    id: Uuid,
) -> Result<Option<UserRow>, sqlx::Error> {
    sqlx::query_as::<_, UserRow>(
        "SELECT id, email, display_name, role, status, \
         password_hash, created_at::text as created_at \
         FROM users WHERE id = $1",
    )
    .bind(id)
    .fetch_optional(pool)
    .await
}

/// List all users.
pub async fn list_users(pool: &PgPool) -> Result<Vec<UserRow>, sqlx::Error> {
    sqlx::query_as::<_, UserRow>(
        "SELECT id, email, display_name, role, status, \
         password_hash, created_at::text as created_at \
         FROM users ORDER BY created_at",
    )
    .fetch_all(pool)
    .await
}

/// Update user role.
pub async fn update_role(
    pool: &PgPool,
    id: Uuid,
    role: &str,
) -> Result<bool, sqlx::Error> {
    let result = sqlx::query("UPDATE users SET role = $1 WHERE id = $2")
        .bind(role)
        .bind(id)
        .execute(pool)
        .await?;
    Ok(result.rows_affected() > 0)
}

/// Disable user.
pub async fn disable_user(
    pool: &PgPool,
    id: Uuid,
) -> Result<bool, sqlx::Error> {
    let result =
        sqlx::query("UPDATE users SET status = 'disabled' WHERE id = $1")
            .bind(id)
            .execute(pool)
            .await?;
    Ok(result.rows_affected() > 0)
}
