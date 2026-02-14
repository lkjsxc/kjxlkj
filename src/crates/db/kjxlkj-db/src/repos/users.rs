use crate::models::DbUser;
use sqlx::PgPool;
use uuid::Uuid;

pub async fn list_users(pool: &PgPool) -> Result<Vec<DbUser>, sqlx::Error> {
    sqlx::query_as::<_, DbUser>(
        "SELECT id, email, display_name, password_hash, role, status, created_at
         FROM users
         ORDER BY created_at ASC",
    )
    .fetch_all(pool)
    .await
}

pub async fn get_user_by_id(pool: &PgPool, user_id: Uuid) -> Result<Option<DbUser>, sqlx::Error> {
    sqlx::query_as::<_, DbUser>(
        "SELECT id, email, display_name, password_hash, role, status, created_at
         FROM users
         WHERE id = $1",
    )
    .bind(user_id)
    .fetch_optional(pool)
    .await
}

pub async fn create_user(
    pool: &PgPool,
    email: &str,
    display_name: &str,
    password_hash: &str,
    role: &str,
) -> Result<DbUser, sqlx::Error> {
    sqlx::query_as::<_, DbUser>(
        "INSERT INTO users (id, email, display_name, password_hash, role, status)
         VALUES ($1, $2, $3, $4, $5, 'active')
         RETURNING id, email, display_name, password_hash, role, status, created_at",
    )
    .bind(Uuid::now_v7())
    .bind(email)
    .bind(display_name)
    .bind(password_hash)
    .bind(role)
    .fetch_one(pool)
    .await
}

pub async fn update_user_role(pool: &PgPool, user_id: Uuid, role: &str) -> Result<DbUser, sqlx::Error> {
    sqlx::query_as::<_, DbUser>(
        "UPDATE users
         SET role = $2
         WHERE id = $1
         RETURNING id, email, display_name, password_hash, role, status, created_at",
    )
    .bind(user_id)
    .bind(role)
    .fetch_one(pool)
    .await
}
