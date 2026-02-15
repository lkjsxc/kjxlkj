use sqlx::{FromRow, PgPool};
use uuid::Uuid;

/// Per /docs/spec/technical/librarian-agent.md: operation audit log.
#[derive(Debug, FromRow)]
pub struct LibrarianOperationRow {
    pub id: Uuid,
    pub run_id: Uuid,
    pub operation_index: i32,
    pub kind: String,
    pub target_note_id: Option<Uuid>,
    pub title: Option<String>,
    pub reason: Option<String>,
    pub confidence: Option<f32>,
    pub status: String,
    pub reject_reason: Option<String>,
}

pub async fn store_operation(
    pool: &PgPool,
    id: Uuid,
    run_id: Uuid,
    index: i32,
    kind: &str,
    target_note_id: Option<Uuid>,
    title: Option<&str>,
    body_markdown: Option<&str>,
    reason: Option<&str>,
    confidence: Option<f32>,
) -> Result<LibrarianOperationRow, sqlx::Error> {
    sqlx::query_as::<_, LibrarianOperationRow>(
        "INSERT INTO librarian_operations
         (id, run_id, operation_index, kind, target_note_id, title,
          body_markdown, reason, confidence)
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
         RETURNING id, run_id, operation_index, kind, target_note_id,
                   title, reason, confidence, status, reject_reason",
    )
    .bind(id)
    .bind(run_id)
    .bind(index)
    .bind(kind)
    .bind(target_note_id)
    .bind(title)
    .bind(body_markdown)
    .bind(reason)
    .bind(confidence)
    .fetch_one(pool)
    .await
}

pub async fn list_operations(
    pool: &PgPool,
    run_id: Uuid,
) -> Result<Vec<LibrarianOperationRow>, sqlx::Error> {
    sqlx::query_as::<_, LibrarianOperationRow>(
        "SELECT id, run_id, operation_index, kind, target_note_id,
                title, reason, confidence, status, reject_reason
         FROM librarian_operations WHERE run_id = $1
         ORDER BY operation_index",
    )
    .bind(run_id)
    .fetch_all(pool)
    .await
}

/// Apply a decision (applied/rejected) to a specific operation.
pub async fn decide_operation(
    pool: &PgPool,
    id: Uuid,
    status: &str,
    reject_reason: Option<&str>,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        "UPDATE librarian_operations
         SET status = $2, reject_reason = $3
         WHERE id = $1",
    )
    .bind(id)
    .bind(status)
    .bind(reject_reason)
    .execute(pool)
    .await?;
    Ok(())
}
