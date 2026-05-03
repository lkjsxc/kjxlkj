use super::DbPool;
use crate::error::AppError;

async fn space_exists(pool: &DbPool, slug: &str) -> Result<bool, AppError> {
    client(pool)
        .await?
        .query_opt("SELECT 1 FROM spaces WHERE slug = $1::CITEXT", &[&slug])
        .await
        .map(|row| row.is_some())
        .map_err(|e| AppError::DatabaseError(e.to_string()))
}

pub async fn require_space(pool: &DbPool, slug: &str) -> Result<(), AppError> {
    if space_exists(pool, slug).await? {
        Ok(())
    } else {
        Err(AppError::NotFound(format!("space '{slug}' not found")))
    }
}

async fn client(pool: &DbPool) -> Result<deadpool_postgres::Object, AppError> {
    pool.get()
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))
}
