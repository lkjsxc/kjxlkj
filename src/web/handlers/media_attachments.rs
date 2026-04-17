use super::media_insert::apply_insert;
use super::media_support::{detect_media_family, embed_markdown, initial_body, object_key};
use super::note_media_input::parse_note_media_form;
use super::resource_payload::ResourcePayload;
use crate::core::{normalize_alias, validate_id};
use crate::error::AppError;
use crate::storage::Storage;
use crate::web::db::{self, AttachmentCreate, NoteAttachmentUpdate, ResourceKind};
use crate::web::handlers::http;
use crate::web::routes::AppState;
use crate::web::view;
use axum::extract::{Multipart, Path, State};
use axum::http::{HeaderMap, StatusCode};
use axum::response::Response;
use serde::Serialize;

#[derive(Serialize)]
struct AttachmentRefPayload {
    id: String,
    kind: ResourceKind,
    alias: Option<String>,
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

pub async fn attach_media(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(id): Path<String>,
    payload: Multipart,
) -> Result<Response, AppError> {
    let pool = &state.pool;
    let storage = &state.storage;
    super::session::require_session(&headers, pool).await?;
    validate_id(&id)?;
    let form = parse_note_media_form(payload, state.media_upload_max_bytes).await?;
    let alias = normalize_alias(form.alias.as_deref())?;
    let settings = db::get_settings(pool).await?;
    let mut attachments = build_attachments(pool, &form.files, settings.media_webp_quality).await?;
    let inserted_markdown = inserted_markdown(&attachments);
    let insertion = apply_insert(
        &form.body,
        form.insert_start,
        form.insert_end,
        &inserted_markdown,
    );
    let keys = store_uploads(storage, &form.files, &mut attachments).await?;
    let result = db::attach_media_to_note(
        pool,
        &id,
        &NoteAttachmentUpdate {
            body: &insertion.body,
            alias: alias.as_deref(),
            is_favorite: form.is_favorite,
            is_private: form.is_private,
        },
        &attachments,
    )
    .await;
    match result {
        Ok(result) => Ok(http::json_status(
            StatusCode::OK,
            AttachmentResponse {
                current_resource: ResourcePayload::from_resource(result.current_resource),
                inserted_markdown,
                selection_fallback: insertion.selection_fallback,
                cursor_utf8: insertion.cursor_utf8,
                created_media: result
                    .created_media
                    .into_iter()
                    .map(|resource| AttachmentRefPayload {
                        file_href: Some(view::file_href(&resource)),
                        id: resource.id,
                        kind: resource.kind,
                        alias: resource.alias,
                    })
                    .collect(),
            },
        )),
        Err(error) => {
            cleanup_objects(storage, &keys).await;
            Err(error)
        }
    }
}

async fn build_attachments(
    pool: &db::DbPool,
    files: &[super::media_input::UploadedFile],
    webp_quality: i64,
) -> Result<Vec<AttachmentCreate>, AppError> {
    let mut attachments = Vec::with_capacity(files.len());
    for file in files {
        let media_id = db::generate_resource_id(pool).await?;
        let media_family = detect_media_family(&file.content_type, &file.original_filename)?;
        let media_body = initial_body(&file.original_filename);
        let generated_variants = super::media_derivatives::build_variants(
            &media_id,
            media_family,
            file.path(),
            webp_quality,
        )
        .await;
        attachments.push(AttachmentCreate {
            media_id: media_id.clone(),
            media_body,
            media_family,
            file_key: object_key(&media_id, &file.original_filename),
            content_type: file.content_type.clone(),
            byte_size: file.byte_size,
            sha256_hex: file.sha256_hex.clone(),
            original_filename: file.original_filename.clone(),
            media_variants: None,
            generated_variants,
        });
    }
    Ok(attachments)
}

async fn store_uploads(
    storage: &Storage,
    uploads: &[super::media_input::UploadedFile],
    attachments: &mut [AttachmentCreate],
) -> Result<Vec<String>, AppError> {
    let mut stored_keys = Vec::with_capacity(attachments.len());
    for (upload, attachment) in uploads.iter().zip(attachments.iter_mut()) {
        if let Err(error) = storage
            .put_file(
                &attachment.file_key,
                upload.path(),
                &attachment.content_type,
            )
            .await
        {
            cleanup_objects(storage, &stored_keys).await;
            return Err(error);
        }
        stored_keys.push(attachment.file_key.clone());
        let (variants, variant_keys) =
            super::media_derivatives::store_variants(storage, &attachment.generated_variants).await;
        attachment.media_variants = variants;
        stored_keys.extend(variant_keys);
    }
    Ok(stored_keys)
}

fn inserted_markdown(attachments: &[AttachmentCreate]) -> String {
    attachments
        .iter()
        .map(|attachment| {
            embed_markdown(
                &attachment.media_id,
                attachment.media_family,
                &attachment.original_filename,
            )
        })
        .collect::<Vec<_>>()
        .join("\n\n")
}

async fn cleanup_objects(storage: &Storage, keys: &[String]) {
    for key in keys {
        let _ = storage.delete_object(key).await;
    }
}
