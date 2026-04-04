use super::models::Record;
use super::record_support::{
    current_favorite_state, map_write_error, next_position, resolve_position, row_to_record,
    RETURNING_RECORD, SELECT_RECORD,
};
use super::resource_ids::next_resource_id;
use super::DbPool;
use crate::core::{derive_summary, derive_title};
use crate::error::AppError;
use deadpool_postgres::GenericClient;

pub async fn get_record(pool: &DbPool, id: &str) -> Result<Option<Record>, AppError> {
    get_record_where(pool, "id = $1", &[&id]).await
}

pub async fn get_record_by_alias(pool: &DbPool, alias: &str) -> Result<Option<Record>, AppError> {
    get_record_where(pool, "alias = $1", &[&alias]).await
}

pub async fn get_record_by_ref(pool: &DbPool, reference: &str) -> Result<Option<Record>, AppError> {
    if let Some(record) = get_record_by_alias(pool, reference).await? {
        return Ok(Some(record));
    }
    get_record(pool, reference).await
}

pub async fn create_record(
    pool: &DbPool,
    id: &str,
    alias: Option<&str>,
    body: &str,
    is_favorite: bool,
    is_private: bool,
) -> Result<Record, AppError> {
    let mut db = client(pool).await?;
    let tx = db
        .transaction()
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;
    let row = tx
        .query_one(
            &format!(
                "INSERT INTO records (id, alias, title, summary, body, is_favorite, favorite_position, is_private) \
                 VALUES ($1, $2, $3, $4, $5, $6, $7, $8) {RETURNING_RECORD}"
            ),
            &[
                &id,
                &alias,
                &derive_title(body),
                &derive_summary(body),
                &body,
                &is_favorite,
                &next_position(&tx, is_favorite).await?,
                &is_private,
            ],
        )
        .await
        .map_err(map_write_error)?;
    let record = row_to_record(row);
    create_snapshot(&tx, &record, next_snapshot_number(&tx, &record.id).await?).await?;
    tx.commit()
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;
    Ok(record)
}

pub async fn update_record(
    pool: &DbPool,
    id: &str,
    alias: Option<&str>,
    body: &str,
    is_favorite: bool,
    is_private: bool,
) -> Result<Option<Record>, AppError> {
    let mut db = client(pool).await?;
    let tx = db
        .transaction()
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;
    let Some((was_favorite, current_position)) = current_favorite_state(&tx, id).await? else {
        return Ok(None);
    };
    let row = tx
        .query_one(
            &format!(
                "UPDATE records SET alias = $2, title = $3, summary = $4, body = $5, \
                 is_favorite = $6, favorite_position = $7, is_private = $8, updated_at = NOW() \
                 WHERE id = $1 AND deleted_at IS NULL {RETURNING_RECORD}"
            ),
            &[
                &id,
                &alias,
                &derive_title(body),
                &derive_summary(body),
                &body,
                &is_favorite,
                &resolve_position(&tx, was_favorite, current_position, is_favorite).await?,
                &is_private,
            ],
        )
        .await
        .map_err(map_write_error)?;
    let record = row_to_record(row);
    create_snapshot(&tx, &record, next_snapshot_number(&tx, &record.id).await?).await?;
    tx.commit()
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;
    Ok(Some(record))
}

pub async fn delete_record(pool: &DbPool, id: &str) -> Result<bool, AppError> {
    let count = client(pool)
        .await?
        .execute(
            "UPDATE records SET deleted_at = NOW() WHERE id = $1 AND deleted_at IS NULL",
            &[&id],
        )
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;
    Ok(count > 0)
}

async fn get_record_where(
    pool: &DbPool,
    predicate: &str,
    params: &[&(dyn tokio_postgres::types::ToSql + Sync)],
) -> Result<Option<Record>, AppError> {
    client(pool)
        .await?
        .query_opt(
            &format!("{SELECT_RECORD} FROM records WHERE {predicate} AND deleted_at IS NULL"),
            params,
        )
        .await
        .map(|row| row.map(row_to_record))
        .map_err(|e| AppError::DatabaseError(e.to_string()))
}

async fn next_snapshot_number<C: GenericClient>(db: &C, record_id: &str) -> Result<i32, AppError> {
    db.query_one(
        "SELECT COALESCE(MAX(snapshot_number), 0) + 1 AS snapshot_number \
         FROM record_revisions WHERE record_id = $1",
        &[&record_id],
    )
    .await
    .map(|row| row.get("snapshot_number"))
    .map_err(|e| AppError::DatabaseError(e.to_string()))
}

async fn create_snapshot<C: GenericClient>(
    db: &C,
    record: &Record,
    snapshot_number: i32,
) -> Result<(), AppError> {
    let snapshot_id = next_resource_id(db).await?;
    db.execute(
        "INSERT INTO record_revisions \
         (id, record_id, snapshot_number, alias, title, summary, body, is_private) \
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8)",
        &[
            &snapshot_id,
            &record.id,
            &snapshot_number,
            &record.alias,
            &record.title,
            &record.summary,
            &record.body,
            &record.is_private,
        ],
    )
    .await
    .map(|_| ())
    .map_err(|e| AppError::DatabaseError(e.to_string()))
}

async fn client(pool: &DbPool) -> Result<deadpool_postgres::Object, AppError> {
    pool.get()
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))
}
