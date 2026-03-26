//! Searchable note listing queries

use super::{DbPool, Record};
use crate::error::AppError;
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _};
use chrono::{DateTime, Utc};

const DEFAULT_LIMIT: i64 = 50;
const MAX_LIMIT: i64 = 100;

#[derive(Clone, Debug)]
pub struct ListRequest {
    pub include_private: bool,
    pub limit: i64,
    pub query: Option<String>,
    pub cursor: Option<String>,
}

#[derive(Clone, Debug)]
pub struct ListPage {
    pub records: Vec<Record>,
    pub next_cursor: Option<String>,
}

pub async fn list_records(pool: &DbPool, request: &ListRequest) -> Result<ListPage, AppError> {
    let client = pool
        .get()
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;
    let limit = request.limit.clamp(1, MAX_LIMIT);
    let query = request
        .query
        .as_deref()
        .map(str::trim)
        .filter(|q| !q.is_empty());
    let cursor = request.cursor.as_deref().map(decode_cursor).transpose()?;
    let rows = client
        .query(
            "SELECT id, title, summary, body, is_private, created_at, updated_at FROM records \
             WHERE deleted_at IS NULL \
             AND ($1 OR is_private = FALSE) \
             AND ($2::TEXT IS NULL OR search_document @@ websearch_to_tsquery('simple', $2)) \
             AND ($3::TIMESTAMPTZ IS NULL OR updated_at < $3 OR (updated_at = $3 AND id > $4)) \
             ORDER BY updated_at DESC, id ASC LIMIT $5",
            &[
                &request.include_private,
                &query,
                &cursor.as_ref().map(|item| item.updated_at),
                &cursor.as_ref().map(|item| item.id.as_str()),
                &(limit + 1),
            ],
        )
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;
    let mut records: Vec<Record> = rows
        .into_iter()
        .map(super::records::row_to_record)
        .collect();
    let next_cursor = if records.len() as i64 > limit {
        records
            .pop()
            .map(|record| encode_cursor(&record.updated_at, &record.id))
    } else {
        None
    };
    Ok(ListPage {
        records,
        next_cursor,
    })
}

#[derive(Clone, Debug)]
struct Cursor {
    updated_at: DateTime<Utc>,
    id: String,
}

fn encode_cursor(updated_at: &DateTime<Utc>, id: &str) -> String {
    URL_SAFE_NO_PAD.encode(format!("{}|{id}", updated_at.to_rfc3339()))
}

fn decode_cursor(cursor: &str) -> Result<Cursor, AppError> {
    let raw = URL_SAFE_NO_PAD
        .decode(cursor)
        .map_err(|_| AppError::InvalidRequest("invalid cursor".to_string()))?;
    let text = String::from_utf8(raw)
        .map_err(|_| AppError::InvalidRequest("invalid cursor".to_string()))?;
    let Some((updated_at, id)) = text.split_once('|') else {
        return Err(AppError::InvalidRequest("invalid cursor".to_string()));
    };
    let updated_at = updated_at
        .parse()
        .map_err(|_| AppError::InvalidRequest("invalid cursor".to_string()))?;
    Ok(Cursor {
        updated_at,
        id: id.to_string(),
    })
}

impl Default for ListRequest {
    fn default() -> Self {
        Self {
            include_private: false,
            limit: DEFAULT_LIMIT,
            query: None,
            cursor: None,
        }
    }
}
