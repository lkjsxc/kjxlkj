use super::listing::{ListPage, ListSort};
use super::{ListedRecord, Record};
use crate::error::AppError;
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub(super) struct Cursor {
    pub(super) query: Option<String>,
    pub(super) sort: String,
    pub(super) id: String,
    pub(super) updated_at: Option<DateTime<Utc>>,
    pub(super) created_at: Option<DateTime<Utc>>,
    pub(super) title_key: Option<String>,
    pub(super) rank: Option<f64>,
    pub(super) fuzzy: Option<f64>,
}

pub(super) fn page_from_rows(
    mut rows: Vec<tokio_postgres::Row>,
    limit: i64,
    query: Option<&str>,
    sort: &ListSort,
) -> ListPage {
    let next_cursor = if rows.len() as i64 > limit {
        rows.pop()
            .map(|row| encode_cursor(&cursor_from_row(&row, query, sort)))
    } else {
        None
    };
    ListPage {
        records: rows.into_iter().map(row_to_listed_record).collect(),
        next_cursor,
    }
}

pub(super) fn row_to_listed_record(row: tokio_postgres::Row) -> ListedRecord {
    ListedRecord {
        record: Record {
            id: row.get("id"),
            alias: row.get("alias"),
            title: row.get("title"),
            summary: row.get("summary"),
            body: row.get("body"),
            is_favorite: row.get("is_favorite"),
            is_private: row.get("is_private"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        },
        preview: row.get("preview"),
    }
}

pub(super) fn decode_cursor(
    cursor: Option<&str>,
    query: Option<&str>,
    sort: &ListSort,
) -> Result<Option<Cursor>, AppError> {
    let Some(cursor) = cursor else {
        return Ok(None);
    };
    let raw = URL_SAFE_NO_PAD
        .decode(cursor)
        .map_err(|_| AppError::InvalidRequest("invalid cursor".to_string()))?;
    let text = String::from_utf8(raw)
        .map_err(|_| AppError::InvalidRequest("invalid cursor".to_string()))?;
    let cursor: Cursor = serde_json::from_str(&text)
        .map_err(|_| AppError::InvalidRequest("invalid cursor".to_string()))?;
    if cursor.query.as_deref() != query || cursor.sort != sort.as_str() {
        return Err(AppError::InvalidRequest("invalid cursor".to_string()));
    }
    Ok(Some(cursor))
}

fn cursor_from_row(row: &tokio_postgres::Row, query: Option<&str>, sort: &ListSort) -> Cursor {
    Cursor {
        query: query.map(str::to_string),
        sort: sort.as_str().to_string(),
        id: row.get("id"),
        updated_at: Some(row.get("updated_at")),
        created_at: Some(row.get("created_at")),
        title_key: Some(row.get("title_key")),
        rank: matches!(sort, ListSort::Relevance).then(|| row.get("rank")),
        fuzzy: matches!(sort, ListSort::Relevance).then(|| row.get("fuzzy")),
    }
}

fn encode_cursor(cursor: &Cursor) -> String {
    URL_SAFE_NO_PAD.encode(serde_json::to_string(cursor).unwrap())
}
