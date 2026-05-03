use super::models::{MediaFamily, ResourceKind};
use super::{ListedResource, Resource};
use crate::media::media_variants_from_json;

pub(crate) fn row_to_listed_resource(row: tokio_postgres::Row) -> ListedResource {
    ListedResource {
        resource: Resource {
            id: row.get("id"),
            space_slug: row.try_get("space_slug").unwrap_or_default(),
            kind: ResourceKind::from_db(&row.get::<_, String>("kind")),
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
            media_variants: media_variants_from_json(row.get("media_variants")),
            owner_note_id: row.get("owner_note_id"),
            is_favorite: row.get("is_favorite"),
            favorite_position: row.get("favorite_position"),
            is_private: row.get("is_private"),
            view_count_total: row.get("view_count_total"),
            last_viewed_at: row.get("last_viewed_at"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        },
        preview: row.get("preview"),
        popular_views: row.try_get("popular_views").ok(),
    }
}
