use crate::models::user::UserRow;
use kjxlkj_domain::types::GlobalRole;
use sqlx::PgPool;
use uuid::Uuid;

pub async fn create(
    pool: &PgPool,
    id: Uuid,
    username: &str,
    password_hash: &str,
    role: GlobalRole,
) -> Result<UserRow, sqlx::Error> {
    sqlx::query_as::<_, UserRow>(
        "INSERT INTO users (id, username, password_hash, global_role)
         VALUES ($1, $2, $3, $4::text)
         RETURNING *",
    )
    .bind(id)
    .bind(username)
    .bind(password_hash)
    .bind(role)
    .fetch_one(pool)
    .await
}

pub async fn find_by_username(
    pool: &PgPool,
    username: &str,
) -> Result<Option<UserRow>, sqlx::Error> {
    sqlx::query_as::<_, UserRow>("SELECT * FROM users WHERE username = $1")
        .bind(username)
        .fetch_optional(pool)
        .await
}

pub async fn find_by_id(pool: &PgPool, id: Uuid) -> Result<Option<UserRow>, sqlx::Error> {
    sqlx::query_as::<_, UserRow>("SELECT * FROM users WHERE id = $1")
        .bind(id)
        .fetch_optional(pool)
        .await
}

pub async fn list(pool: &PgPool) -> Result<Vec<UserRow>, sqlx::Error> {
    sqlx::query_as::<_, UserRow>("SELECT * FROM users ORDER BY created_at")
        .fetch_all(pool)
        .await
}

pub async fn update_role(
    pool: &PgPool,
    id: Uuid,
    role: GlobalRole,
) -> Result<Option<UserRow>, sqlx::Error> {
    sqlx::query_as::<_, UserRow>(
        "UPDATE users SET global_role = $2::text, updated_at = now()
         WHERE id = $1 RETURNING *",
    )
    .bind(id)
    .bind(role)
    .fetch_optional(pool)
    .await
}

pub async fn disable(pool: &PgPool, id: Uuid) -> Result<bool, sqlx::Error> {
    let r = sqlx::query("UPDATE users SET disabled = true, updated_at = now() WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await?;
    Ok(r.rows_affected() > 0)
}
