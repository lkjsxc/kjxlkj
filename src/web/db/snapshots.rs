use super::listing_direction::ListDirection;
use super::models::{MediaFamily, Resource, ResourceKind, ResourceSnapshot};
use super::resource_support::row_to_resource;
use super::snapshots_cursor::{decode_snapshot_cursor, encode_snapshot_cursor};
use super::DbPool;
use crate::error::AppError;
use serde::Serialize;

const MAX_LIMIT: i64 = 100;

#[derive(Clone, Debug, Serialize)]
pub struct SnapshotPage {
    pub snapshots: Vec<ResourceSnapshot>,
    pub previous_cursor: Option<String>,
    pub next_cursor: Option<String>,
}

#[derive(Clone, Debug)]
pub struct SnapshotTarget {
    pub resource: Resource,
    pub snapshot: ResourceSnapshot,
}

pub async fn list_resource_snapshots(
    pool: &DbPool,
    resource_id: &str,
    include_private: bool,
    limit: i64,
    direction: &ListDirection,
    cursor: Option<&str>,
) -> Result<SnapshotPage, AppError> {
    let limit = limit.clamp(1, MAX_LIMIT);
    let cursor = decode_snapshot_cursor(cursor)?;
    let mut snapshots =
        query_page(pool, resource_id, include_private, limit, direction, cursor).await?;
    if snapshots.len() as i64 > limit {
        snapshots.pop();
    }
    if matches!(direction, ListDirection::Prev) {
        snapshots.reverse();
    }
    Ok(SnapshotPage {
        previous_cursor: edge_cursor(
            pool,
            resource_id,
            include_private,
            snapshots.first().map(|item| item.snapshot_number),
            true,
        )
        .await?,
        next_cursor: edge_cursor(
            pool,
            resource_id,
            include_private,
            snapshots.last().map(|item| item.snapshot_number),
            false,
        )
        .await?,
        snapshots,
    })
}

pub async fn get_snapshot_target(
    pool: &DbPool,
    snapshot_id: &str,
) -> Result<Option<SnapshotTarget>, AppError> {
    client(pool).await?.query_opt(
        "SELECT r.id, r.kind, r.alias, r.title, r.summary, r.body, r.media_family, r.file_key, r.content_type, \
         r.byte_size, r.sha256_hex, r.original_filename, r.width, r.height, r.duration_ms, r.is_favorite, r.favorite_position, \
         r.is_private, r.view_count_total, r.last_viewed_at, r.created_at, r.updated_at, s.id AS snapshot_id, s.kind AS snapshot_kind, \
         s.snapshot_number, s.alias AS snapshot_alias, s.title AS snapshot_title, s.summary AS snapshot_summary, s.body AS snapshot_body, \
         s.media_family AS snapshot_media_family, s.file_key AS snapshot_file_key, s.content_type AS snapshot_content_type, \
         s.byte_size AS snapshot_byte_size, s.sha256_hex AS snapshot_sha256_hex, s.original_filename AS snapshot_original_filename, \
         s.width AS snapshot_width, s.height AS snapshot_height, s.duration_ms AS snapshot_duration_ms, \
         s.is_private AS snapshot_is_private, s.created_at AS snapshot_created_at \
         FROM resource_snapshots s JOIN resources r ON r.id = s.resource_id WHERE s.id = $1 AND r.deleted_at IS NULL",
        &[&snapshot_id],
    ).await.map(|row| row.map(row_to_snapshot_target)).map_err(|e| AppError::DatabaseError(e.to_string()))
}

async fn query_page(
    pool: &DbPool,
    resource_id: &str,
    include_private: bool,
    limit: i64,
    direction: &ListDirection,
    cursor: Option<i32>,
) -> Result<Vec<ResourceSnapshot>, AppError> {
    let (predicate, order) = match direction {
        ListDirection::Next => ("snapshot_number < $3", "snapshot_number DESC"),
        ListDirection::Prev => ("snapshot_number > $3", "snapshot_number ASC"),
    };
    let sql = format!(
        "SELECT id, kind, snapshot_number, alias, title, summary, body, media_family, file_key, content_type, byte_size, \
         sha256_hex, original_filename, width, height, duration_ms, is_private, created_at \
         FROM resource_snapshots WHERE resource_id = $1 AND ($2 OR is_private = FALSE) AND ($3::INT IS NULL OR {predicate}) ORDER BY {order} LIMIT $4"
    );
    client(pool)
        .await?
        .query(
            &sql,
            &[&resource_id, &include_private, &cursor, &(limit + 1)],
        )
        .await
        .map(|rows| rows.into_iter().map(row_to_snapshot).collect())
        .map_err(|e| AppError::DatabaseError(e.to_string()))
}

async fn edge_cursor(
    pool: &DbPool,
    resource_id: &str,
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
    let sql = format!(
        "SELECT 1 FROM resource_snapshots WHERE resource_id = $1 AND ($2 OR is_private = FALSE) AND {predicate} LIMIT 1"
    );
    client(pool)
        .await?
        .query_opt(&sql, &[&resource_id, &include_private, &snapshot_number])
        .await
        .map(|row| row.map(|_| encode_snapshot_cursor(snapshot_number)))
        .map_err(|e| AppError::DatabaseError(e.to_string()))
}

fn row_to_snapshot(row: tokio_postgres::Row) -> ResourceSnapshot {
    ResourceSnapshot {
        id: row.get("id"),
        kind: ResourceKind::from_db(&row.get::<_, String>("kind")),
        snapshot_number: row.get("snapshot_number"),
        alias: row.get("alias"),
        title: row.get("title"),
        summary: row.get("summary"),
        body: row.get("body"),
        media_family: MediaFamily::from_db(row.get("media_family")),
        file_key: row.get("file_key"),
        content_type: row.get("content_type"),
        byte_size: row.get("byte_size"),
        sha256_hex: row.get("sha256_hex"),
        original_filename: row.get("original_filename"),
        width: row.get("width"),
        height: row.get("height"),
        duration_ms: row.get("duration_ms"),
        is_private: row.get("is_private"),
        created_at: row.get("created_at"),
    }
}

fn row_to_snapshot_target(row: tokio_postgres::Row) -> SnapshotTarget {
    SnapshotTarget {
        resource: row_to_resource(row.clone()),
        snapshot: ResourceSnapshot {
            id: row.get("snapshot_id"),
            kind: ResourceKind::from_db(&row.get::<_, String>("snapshot_kind")),
            snapshot_number: row.get("snapshot_number"),
            alias: row.get("snapshot_alias"),
            title: row.get("snapshot_title"),
            summary: row.get("snapshot_summary"),
            body: row.get("snapshot_body"),
            media_family: MediaFamily::from_db(row.get("snapshot_media_family")),
            file_key: row.get("snapshot_file_key"),
            content_type: row.get("snapshot_content_type"),
            byte_size: row.get("snapshot_byte_size"),
            sha256_hex: row.get("snapshot_sha256_hex"),
            original_filename: row.get("snapshot_original_filename"),
            width: row.get("snapshot_width"),
            height: row.get("snapshot_height"),
            duration_ms: row.get("snapshot_duration_ms"),
            is_private: row.get("snapshot_is_private"),
            created_at: row.get("snapshot_created_at"),
        },
    }
}

async fn client(pool: &DbPool) -> Result<deadpool_postgres::Object, AppError> {
    pool.get()
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))
}
