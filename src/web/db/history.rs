use super::listing_direction::ListDirection;
use super::models::{Record, RecordRevision};
use super::DbPool;
use crate::error::AppError;
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _};
use serde::{Deserialize, Serialize};

const MAX_LIMIT: i64 = 100;

#[derive(Clone, Debug, Serialize)]
pub struct RevisionPage {
    pub revisions: Vec<RecordRevision>,
    pub previous_cursor: Option<String>,
    pub next_cursor: Option<String>,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
struct RevisionCursor {
    revision_number: i32,
}

pub async fn list_record_revisions(
    pool: &DbPool,
    record_id: &str,
    include_private: bool,
    limit: i64,
    direction: &ListDirection,
    cursor: Option<&str>,
) -> Result<RevisionPage, AppError> {
    let limit = limit.clamp(1, MAX_LIMIT);
    let cursor = decode_cursor(cursor)?;
    let mut revisions =
        query_page(pool, record_id, include_private, limit, direction, cursor).await?;
    if revisions.len() as i64 > limit {
        revisions.pop();
    }
    if matches!(direction, ListDirection::Prev) {
        revisions.reverse();
    }
    let previous_cursor = edge_cursor(
        pool,
        record_id,
        include_private,
        revisions.first().map(|item| item.revision_number),
        true,
    )
    .await?;
    let next_cursor = edge_cursor(
        pool,
        record_id,
        include_private,
        revisions.last().map(|item| item.revision_number),
        false,
    )
    .await?;
    Ok(RevisionPage {
        revisions,
        previous_cursor,
        next_cursor,
    })
}
pub async fn get_record_revision(
    pool: &DbPool,
    record_id: &str,
    revision_number: i32,
) -> Result<Option<RecordRevision>, AppError> {
    client(pool)
        .await?
        .query_opt(
            "SELECT revision_number, body, is_private, created_at FROM record_revisions \
             WHERE record_id = $1 AND revision_number = $2",
            &[&record_id, &revision_number],
        )
        .await
        .map(|row| row.map(row_to_revision))
        .map_err(|e| AppError::DatabaseError(e.to_string()))
}
pub async fn get_previous_record(
    pool: &DbPool,
    id: &str,
    include_private: bool,
) -> Result<Option<Record>, AppError> {
    adjacent_record(pool, id, include_private, true).await
}
pub async fn get_next_record(
    pool: &DbPool,
    id: &str,
    include_private: bool,
) -> Result<Option<Record>, AppError> {
    adjacent_record(pool, id, include_private, false).await
}

async fn query_page(
    pool: &DbPool,
    record_id: &str,
    include_private: bool,
    limit: i64,
    direction: &ListDirection,
    cursor: Option<i32>,
) -> Result<Vec<RecordRevision>, AppError> {
    let (predicate, order) = match direction {
        ListDirection::Next => ("revision_number < $3", "revision_number DESC"),
        ListDirection::Prev => ("revision_number > $3", "revision_number ASC"),
    };
    client(pool)
        .await?
        .query(
            &format!(
                "SELECT revision_number, body, is_private, created_at FROM record_revisions \
                 WHERE record_id = $1 AND ($2 OR is_private = FALSE) AND ($3::INT IS NULL OR {predicate}) \
                 ORDER BY {order} LIMIT $4"
            ),
            &[&record_id, &include_private, &cursor, &(limit + 1)],
        )
        .await
        .map(|rows| rows.into_iter().map(row_to_revision).collect())
        .map_err(|e| AppError::DatabaseError(e.to_string()))
}
async fn edge_cursor(
    pool: &DbPool,
    record_id: &str,
    include_private: bool,
    revision_number: Option<i32>,
    previous: bool,
) -> Result<Option<String>, AppError> {
    let Some(revision_number) = revision_number else {
        return Ok(None);
    };
    let predicate = if previous {
        "revision_number > $3"
    } else {
        "revision_number < $3"
    };
    let row = client(pool)
        .await?
        .query_opt(
            &format!(
                "SELECT 1 FROM record_revisions WHERE record_id = $1 AND ($2 OR is_private = FALSE) AND {predicate} LIMIT 1"
            ),
            &[&record_id, &include_private, &revision_number],
        )
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;
    Ok(row.map(|_| encode_cursor(revision_number)))
}
async fn adjacent_record(
    pool: &DbPool,
    id: &str,
    include_private: bool,
    older: bool,
) -> Result<Option<Record>, AppError> {
    let query = if older {
        "SELECT id, alias, title, summary, body, is_favorite, favorite_position, is_private, created_at, updated_at \
         FROM records WHERE deleted_at IS NULL AND ($2 OR is_private = FALSE) \
         AND ((created_at < (SELECT created_at FROM records WHERE id = $1 AND deleted_at IS NULL)) \
           OR (created_at = (SELECT created_at FROM records WHERE id = $1 AND deleted_at IS NULL) AND id < $1)) \
         ORDER BY created_at DESC, id DESC LIMIT 1"
    } else {
        "SELECT id, alias, title, summary, body, is_favorite, favorite_position, is_private, created_at, updated_at \
         FROM records WHERE deleted_at IS NULL AND ($2 OR is_private = FALSE) \
         AND ((created_at > (SELECT created_at FROM records WHERE id = $1 AND deleted_at IS NULL)) \
           OR (created_at = (SELECT created_at FROM records WHERE id = $1 AND deleted_at IS NULL) AND id > $1)) \
         ORDER BY created_at ASC, id ASC LIMIT 1"
    };
    client(pool)
        .await?
        .query_opt(query, &[&id, &include_private])
        .await
        .map(|row| row.map(super::records::row_to_record))
        .map_err(|e| AppError::DatabaseError(e.to_string()))
}
async fn client(pool: &DbPool) -> Result<deadpool_postgres::Object, AppError> {
    pool.get()
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))
}
fn row_to_revision(row: tokio_postgres::Row) -> RecordRevision {
    RecordRevision {
        revision_number: row.get("revision_number"),
        body: row.get("body"),
        is_private: row.get("is_private"),
        created_at: row.get("created_at"),
    }
}
fn decode_cursor(cursor: Option<&str>) -> Result<Option<i32>, AppError> {
    let Some(cursor) = cursor else {
        return Ok(None);
    };
    let raw = URL_SAFE_NO_PAD
        .decode(cursor)
        .map_err(|_| AppError::InvalidRequest("invalid cursor".to_string()))?;
    let text = String::from_utf8(raw)
        .map_err(|_| AppError::InvalidRequest("invalid cursor".to_string()))?;
    let cursor: RevisionCursor = serde_json::from_str(&text)
        .map_err(|_| AppError::InvalidRequest("invalid cursor".to_string()))?;
    Ok(Some(cursor.revision_number))
}
fn encode_cursor(revision_number: i32) -> String {
    URL_SAFE_NO_PAD.encode(serde_json::to_string(&RevisionCursor { revision_number }).unwrap())
}
