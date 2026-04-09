use crate::web::db::Record;
use crate::web::view;
use chrono::{DateTime, Utc};
use serde::Serialize;

#[derive(Serialize)]
pub struct ResourcePayload {
    id: String,
    kind: crate::web::db::RecordKind,
    alias: Option<String>,
    body: String,
    media_family: Option<crate::web::db::MediaFamily>,
    file_href: Option<String>,
    content_type: Option<String>,
    byte_size: Option<i64>,
    sha256_hex: Option<String>,
    original_filename: Option<String>,
    width: Option<i32>,
    height: Option<i32>,
    duration_ms: Option<i64>,
    is_favorite: bool,
    favorite_position: Option<i64>,
    is_private: bool,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl ResourcePayload {
    pub fn from_record(record: Record) -> Self {
        let file_href =
            (record.kind == crate::web::db::RecordKind::Media).then(|| view::file_href(&record));
        Self {
            id: record.id,
            kind: record.kind,
            alias: record.alias,
            body: record.body,
            media_family: record.media_family,
            file_href,
            content_type: record.content_type,
            byte_size: record.byte_size,
            sha256_hex: record.sha256_hex,
            original_filename: record.original_filename,
            width: record.width,
            height: record.height,
            duration_ms: record.duration_ms,
            is_favorite: record.is_favorite,
            favorite_position: record.favorite_position,
            is_private: record.is_private,
            created_at: record.created_at,
            updated_at: record.updated_at,
        }
    }
}
