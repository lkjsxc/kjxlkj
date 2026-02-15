use kjxlkj_domain::ids::UserId;
use kjxlkj_domain::user::Role;
use sqlx::prelude::FromRow;
use sqlx::PgPool;
use time::OffsetDateTime;
use uuid::Uuid;

/// User row from database.
#[derive(FromRow)]
pub struct UserRow {
    pub id: Uuid,
    pub email: String,
    pub display_name: String,
    pub role: String,
    pub status: String,
    pub password_hash: String,
    pub created_at: OffsetDateTime,
}

#[derive(FromRow)]
struct CountRow {
    count: i64,
}

pub async fn count_owners(pool: &PgPool) -> Result<i64, sqlx::Error> {
    let row: CountRow = sqlx::query_as(
        "SELECT COUNT(*) as count FROM users WHERE role = 'owner'"
    )
    .fetch_one(pool)
    .await?;
    Ok(row.count)
}

pub async fn create_user(
    pool: &PgPool,
    id: UserId,
    email: &str,
    display_name: &str,
    role: Role,
    password_hash: &str,
) -> Result<(), sqlx::Error> {
    let role_str = match role {
        Role::Owner => "owner",
        Role::Admin => "admin",
        Role::Editor => "editor",
        Role::Viewer => "viewer",
    };
    sqlx::query(
        "INSERT INTO users (id, email, display_name, role, password_hash)
         VALUES ($1, $2, $3, $4, $5)",
    )
    .bind(id.0)
    .bind(email)
    .bind(display_name)
    .bind(role_str)
    .bind(password_hash)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn find_user_by_email(
    pool: &PgPool,
    email: &str,
) -> Result<Option<UserRow>, sqlx::Error> {
    sqlx::query_as::<_, UserRow>(
        "SELECT id, email, display_name, role, status,
                password_hash, created_at
         FROM users WHERE email = $1",
    )
    .bind(email)
    .fetch_optional(pool)
    .await
}

pub async fn find_user_by_id(
    pool: &PgPool,
    id: UserId,
) -> Result<Option<UserRow>, sqlx::Error> {
    sqlx::query_as::<_, UserRow>(
        "SELECT id, email, display_name, role, status,
                password_hash, created_at
         FROM users WHERE id = $1",
    )
    .bind(id.0)
    .fetch_optional(pool)
    .await
}

pub async fn list_users(pool: &PgPool) -> Result<Vec<UserRow>, sqlx::Error> {
    sqlx::query_as::<_, UserRow>(
        "SELECT id, email, display_name, role, status,
                password_hash, created_at
         FROM users ORDER BY created_at",
    )
    .fetch_all(pool)
    .await
}

pub async fn update_user_role(
    pool: &PgPool,
    id: UserId,
    role: Role,
) -> Result<bool, sqlx::Error> {
    let role_str = match role {
        Role::Owner => "owner",
        Role::Admin => "admin",
        Role::Editor => "editor",
        Role::Viewer => "viewer",
    };
    let result = sqlx::query(
        "UPDATE users SET role = $2 WHERE id = $1",
    )
    .bind(id.0)
    .bind(role_str)
    .execute(pool)
    .await?;
    Ok(result.rows_affected() > 0)
}

pub async fn disable_user(
    pool: &PgPool,
    id: UserId,
) -> Result<bool, sqlx::Error> {
    let result = sqlx::query(
        "UPDATE users SET status = 'disabled' WHERE id = $1",
    )
    .bind(id.0)
    .execute(pool)
    .await?;
    Ok(result.rows_affected() > 0)
}
