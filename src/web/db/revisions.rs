use super::listing_direction::ListDirection;
use super::models::{Record, RecordSnapshot};
use super::record_support::row_to_record;
use super::DbPool;
use crate::error::AppError;
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _};
use serde::{Deserialize, Serialize};

const MAX_LIMIT: i64 = 100;

#[derive(Clone, Debug, Serialize)]
pub struct SnapshotPage {
    pub snapshots: Vec<RecordSnapshot>,
    pub previous_cursor: Option<String>,
    pub next_cursor: Option<String>,
}

#[derive(Clone, Debug)]
pub struct SnapshotResource {
    pub record: Record,
    pub snapshot: RecordSnapshot,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct SnapshotCursor {
    snapshot_number: i32,
}

pub async fn list_record_snapshots(
    pool: &DbPool,
    record_id: &str,
    include_private: bool,
    limit: i64,
    direction: &ListDirection,
    cursor: Option<&str>,
) -> Result<SnapshotPage, AppError> {
    let limit = limit.clamp(1, MAX_LIMIT);
    let cursor = decode_cursor(cursor)?;
    let mut snapshots =
        query_page(pool, record_id, include_private, limit, direction, cursor).await?;
    if snapshots.len() as i64 > limit {
        snapshots.pop();
    }
    if matches!(direction, ListDirection::Prev) {
        snapshots.reverse();
    }
    Ok(SnapshotPage {
        previous_cursor: edge_cursor(
            pool,
            record_id,
            include_private,
            snapshots.first().map(|item| item.snapshot_number),
            true,
        )
        .await?,
        next_cursor: edge_cursor(
            pool,
            record_id,
            include_private,
            snapshots.last().map(|item| item.snapshot_number),
            false,
        )
        .await?,
        snapshots,
    })
}

pub async fn get_snapshot_resource(
    pool: &DbPool,
    snapshot_id: &str,
) -> Result<Option<SnapshotResource>, AppError> {
    client(pool)
        .await?
        .query_opt(
            "SELECT r.id, r.alias, r.title, r.summary, r.body, r.is_favorite, r.favorite_position, \
             r.is_private, r.view_count_total, r.last_viewed_at, r.created_at, r.updated_at, \
             rr.id AS snapshot_id, rr.snapshot_number, rr.alias AS snapshot_alias, \
             rr.title AS snapshot_title, rr.summary AS snapshot_summary, rr.body AS snapshot_body, \
             rr.is_private AS snapshot_is_private, rr.created_at AS snapshot_created_at \
             FROM record_revisions rr \
             JOIN records r ON r.id = rr.record_id \
             WHERE rr.id = $1 AND r.deleted_at IS NULL",
            &[&snapshot_id],
        )
        .await
        .map(|row| row.map(row_to_snapshot_resource))
        .map_err(|e| AppError::DatabaseError(e.to_string()))
}

async fn query_page(
    pool: &DbPool,
    record_id: &str,
    include_private: bool,
    limit: i64,
    direction: &ListDirection,
    cursor: Option<i32>,
) -> Result<Vec<RecordSnapshot>, AppError> {
    let (predicate, order) = match direction {
        ListDirection::Next => ("snapshot_number < $3", "snapshot_number DESC"),
        ListDirection::Prev => ("snapshot_number > $3", "snapshot_number ASC"),
    };
    client(pool)
        .await?
        .query(
            &format!(
                "SELECT id, snapshot_number, alias, title, summary, body, is_private, created_at \
                 FROM record_revisions \
                 WHERE record_id = $1 AND ($2 OR is_private = FALSE) AND ($3::INT IS NULL OR {predicate}) \
                 ORDER BY {order} LIMIT $4"
            ),
            &[&record_id, &include_private, &cursor, &(limit + 1)],
        )
        .await
        .map(|rows| rows.into_iter().map(row_to_snapshot).collect())
        .map_err(|e| AppError::DatabaseError(e.to_string()))
}

async fn edge_cursor(
    pool: &DbPool,
    record_id: &str,
    include_private: bool,
    snapshot_number: Option<i32>,
    previous: bool,
) -> Result<Option<String>, AppError> {
    let Some(snapshot_number) = snapshot_number else {
        return Ok(None);
    };
    let predicate = if previous {
        "snapshot_number > $3"
    } else {
        "snapshot_number < $3"
    };
    let row = client(pool)
        .await?
        .query_opt(
            &format!(
                "SELECT 1 FROM record_revisions \
                 WHERE record_id = $1 AND ($2 OR is_private = FALSE) AND {predicate} LIMIT 1"
            ),
            &[&record_id, &include_private, &snapshot_number],
        )
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;
    Ok(row.map(|_| encode_cursor(snapshot_number)))
}

fn row_to_snapshot(row: tokio_postgres::Row) -> RecordSnapshot {
    RecordSnapshot {
        id: row.get("id"),
        snapshot_number: row.get("snapshot_number"),
        alias: row.get("alias"),
        title: row.get("title"),
        summary: row.get("summary"),
        body: row.get("body"),
        is_private: row.get("is_private"),
        created_at: row.get("created_at"),
    }
}

fn row_to_snapshot_resource(row: tokio_postgres::Row) -> SnapshotResource {
    SnapshotResource {
        record: row_to_record(row.clone()),
        snapshot: RecordSnapshot {
            id: row.get("snapshot_id"),
            snapshot_number: row.get("snapshot_number"),
            alias: row.get("snapshot_alias"),
            title: row.get("snapshot_title"),
            summary: row.get("snapshot_summary"),
            body: row.get("snapshot_body"),
            is_private: row.get("snapshot_is_private"),
            created_at: row.get("snapshot_created_at"),
        },
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
    let cursor: SnapshotCursor = serde_json::from_str(&text)
        .map_err(|_| AppError::InvalidRequest("invalid cursor".to_string()))?;
    Ok(Some(cursor.snapshot_number))
}

fn encode_cursor(snapshot_number: i32) -> String {
    URL_SAFE_NO_PAD.encode(serde_json::to_string(&SnapshotCursor { snapshot_number }).unwrap())
}

async fn client(pool: &DbPool) -> Result<deadpool_postgres::Object, AppError> {
    pool.get()
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))
}
