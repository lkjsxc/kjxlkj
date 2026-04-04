use super::DbPool;
use crate::core::generate_id;
use crate::error::AppError;
use deadpool_postgres::GenericClient;

pub async fn generate_resource_id(pool: &DbPool) -> Result<String, AppError> {
    let db = client(pool).await?;
    next_resource_id(&db).await
}

pub async fn next_resource_id<C: GenericClient>(db: &C) -> Result<String, AppError> {
    for _ in 0..10 {
        let id = generate_id();
        if !id_exists(db, &id).await? {
            return Ok(id);
        }
    }
    Err(AppError::StorageError(
        "could not generate unique id".to_string(),
    ))
}

async fn id_exists<C: GenericClient>(db: &C, id: &str) -> Result<bool, AppError> {
    db.query_opt(
        "SELECT 1 FROM records WHERE id = $1 \
         UNION ALL \
         SELECT 1 FROM record_revisions WHERE id = $1 \
         LIMIT 1",
        &[&id],
    )
    .await
    .map(|row| row.is_some())
    .map_err(|e| AppError::DatabaseError(e.to_string()))
}

async fn client(pool: &DbPool) -> Result<deadpool_postgres::Object, AppError> {
    pool.get()
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))
}
