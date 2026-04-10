use super::SnapshotTarget;
use crate::media::media_variants_from_json;
use crate::web::db::models::{MediaFamily, ResourceKind, ResourceSnapshot};
use crate::web::db::resource_support::row_to_resource;

pub(super) fn row_to_snapshot(row: tokio_postgres::Row) -> ResourceSnapshot {
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
        media_variants: media_variants_from_json(row.get("media_variants")),
        is_private: row.get("is_private"),
        created_at: row.get("created_at"),
    }
}

pub(super) fn row_to_snapshot_target(row: tokio_postgres::Row) -> SnapshotTarget {
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
            media_variants: media_variants_from_json(row.get("snapshot_media_variants")),
            is_private: row.get("snapshot_is_private"),
            created_at: row.get("snapshot_created_at"),
        },
    }
}
