use crate::web::db::Resource;
use crate::web::view;
use chrono::{DateTime, Utc};
use serde::Serialize;

#[derive(Serialize)]
pub struct ResourcePayload {
    id: String,
    kind: crate::web::db::ResourceKind,
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
    pub fn from_resource(resource: Resource) -> Self {
        let file_href = (resource.kind == crate::web::db::ResourceKind::Media)
            .then(|| view::file_href(&resource));
        Self {
            id: resource.id,
            kind: resource.kind,
            alias: resource.alias,
            body: resource.body,
            media_family: resource.media_family,
            file_href,
            content_type: resource.content_type,
            byte_size: resource.byte_size,
            sha256_hex: resource.sha256_hex,
            original_filename: resource.original_filename,
            width: resource.width,
            height: resource.height,
            duration_ms: resource.duration_ms,
            is_favorite: resource.is_favorite,
            favorite_position: resource.favorite_position,
            is_private: resource.is_private,
            created_at: resource.created_at,
            updated_at: resource.updated_at,
        }
    }
}
