use super::media_insert::InsertResult;
use super::{http, resource_payload::ResourcePayload};
use crate::error::AppError;
use crate::storage::Storage;
use crate::web::{db, view};
use axum::http::StatusCode;
use axum::response::Response;
use serde::Serialize;

#[derive(Serialize)]
struct AttachmentRefPayload {
    id: String,
    kind: db::ResourceKind,
    alias: Option<String>,
    owner_note_id: Option<String>,
    file_href: Option<String>,
}

#[derive(Serialize)]
struct AttachmentResponse {
    current_resource: ResourcePayload,
    inserted_markdown: String,
    selection_fallback: bool,
    cursor_utf8: usize,
    created_media: Vec<AttachmentRefPayload>,
}

pub(super) async fn attachment_response(
    storage: &Storage,
    keys: Vec<String>,
    result: Result<db::AttachmentBatchResult, AppError>,
    inserted_markdown: String,
    insertion: InsertResult,
) -> Result<Response, AppError> {
    match result {
        Ok(result) => Ok(http::json_status(
            StatusCode::OK,
            AttachmentResponse {
                current_resource: ResourcePayload::from_resource(result.current_resource),
                inserted_markdown,
                selection_fallback: insertion.selection_fallback,
                cursor_utf8: insertion.cursor_utf8,
                created_media: result.created_media.into_iter().map(media_ref).collect(),
            },
        )),
        Err(error) => {
            cleanup_objects(storage, &keys).await;
            Err(error)
        }
    }
}

fn media_ref(resource: db::Resource) -> AttachmentRefPayload {
    AttachmentRefPayload {
        file_href: Some(view::file_href(&resource)),
        id: resource.id,
        kind: resource.kind,
        alias: resource.alias,
        owner_note_id: resource.owner_note_id,
    }
}

async fn cleanup_objects(storage: &Storage, keys: &[String]) {
    for key in keys {
        let _ = storage.delete_object(key).await;
    }
}
