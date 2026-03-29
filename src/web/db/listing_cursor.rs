use super::listing::ListPage;
use super::{ListedRecord, Record};
use crate::error::AppError;
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _};
use chrono::{DateTime, Utc};

#[derive(Clone, Debug)]
pub(super) struct Cursor {
    pub(super) updated_at: DateTime<Utc>,
    pub(super) id: String,
}

pub(super) fn page_from_rows(mut rows: Vec<tokio_postgres::Row>, limit: i64) -> ListPage {
    let next_cursor = if rows.len() as i64 > limit {
        rows.pop().map(|row| {
            encode_cursor(
                &row.get::<_, DateTime<Utc>>("updated_at"),
                &row.get::<_, String>("id"),
            )
        })
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

pub(super) fn decode_cursor(cursor: &str) -> Result<Cursor, AppError> {
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

fn encode_cursor(updated_at: &DateTime<Utc>, id: &str) -> String {
    URL_SAFE_NO_PAD.encode(format!("{}|{id}", updated_at.to_rfc3339()))
}
