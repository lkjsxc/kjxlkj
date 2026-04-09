use super::media_insert::apply_insert;
use super::media_support::{
    attachment_note_body, detect_media_family, embed_markdown, initial_body, object_key, sha256_hex,
};
use super::note_media_input::parse_note_media_form;
use super::resource_payload::ResourcePayload;
use crate::core::{normalize_alias, validate_id};
use crate::error::AppError;
use crate::storage::Storage;
use crate::web::db::{self, AttachmentCreate, NoteAttachmentUpdate, RecordKind};
use crate::web::view;
use actix_multipart::Multipart;
use actix_web::{post, web, HttpRequest, HttpResponse};
use serde::Serialize;

#[derive(Serialize)]
struct AttachmentRefPayload {
    id: String,
    kind: RecordKind,
    alias: Option<String>,
    file_href: Option<String>,
}

#[derive(Serialize)]
struct AttachmentResponse {
    current_note: ResourcePayload,
    inserted_markdown: String,
    selection_fallback: bool,
    created_media: Vec<AttachmentRefPayload>,
    created_notes: Vec<AttachmentRefPayload>,
}

#[post("/resources/{id}/media-attachments")]
pub async fn attach_media(
    pool: web::Data<db::DbPool>,
    storage: web::Data<Storage>,
    req: HttpRequest,
    path: web::Path<String>,
    payload: Multipart,
) -> Result<HttpResponse, AppError> {
    super::session::require_session(&req, &pool).await?;
    let id = path.into_inner();
    validate_id(&id)?;
    let form = parse_note_media_form(payload).await?;
    let alias = normalize_alias(form.alias.as_deref())?;
    let attachments = build_attachments(&pool, &form.files).await?;
    let inserted_markdown = inserted_markdown(&attachments);
    let insertion = apply_insert(
        &form.body,
        form.insert_start,
        form.insert_end,
        &inserted_markdown,
    );
    let keys = attachments
        .iter()
        .map(|item| item.file_key.clone())
        .collect::<Vec<_>>();
    store_uploads(&storage, &attachments).await?;
    let result = db::attach_media_to_note(
        &pool,
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
        Ok(result) => Ok(HttpResponse::Ok().json(AttachmentResponse {
            current_note: ResourcePayload::from_record(result.current_note),
            inserted_markdown,
            selection_fallback: insertion.selection_fallback,
            created_media: result
                .created_media
                .into_iter()
                .map(|record| AttachmentRefPayload {
                    file_href: Some(view::file_href(&record)),
                    id: record.id,
                    kind: record.kind,
                    alias: record.alias,
                })
                .collect(),
            created_notes: result
                .created_notes
                .into_iter()
                .map(|record| AttachmentRefPayload {
                    file_href: None,
                    id: record.id,
                    kind: record.kind,
                    alias: record.alias,
                })
                .collect(),
        })),
        Err(error) => {
            cleanup_objects(&storage, &keys).await;
            Err(error)
        }
    }
}

async fn build_attachments(
    pool: &db::DbPool,
    files: &[super::media_input::UploadedFile],
) -> Result<Vec<AttachmentCreate>, AppError> {
    let mut attachments = Vec::with_capacity(files.len());
    for file in files {
        let media_id = db::generate_resource_id(pool).await?;
        let note_id = db::generate_resource_id(pool).await?;
        let media_family = detect_media_family(&file.content_type, &file.original_filename)?;
        attachments.push(AttachmentCreate {
            media_id: media_id.clone(),
            media_body: initial_body(&file.original_filename),
            note_id,
            note_body: attachment_note_body(&file.original_filename, &media_id, media_family),
            bytes: file.bytes.clone(),
            media_family,
            file_key: object_key(&media_id, &file.original_filename),
            content_type: file.content_type.clone(),
            byte_size: file.bytes.len() as i64,
            sha256_hex: sha256_hex(&file.bytes),
            original_filename: file.original_filename.clone(),
        });
    }
    Ok(attachments)
}

async fn store_uploads(
    storage: &Storage,
    attachments: &[AttachmentCreate],
) -> Result<(), AppError> {
    let mut stored_keys = Vec::with_capacity(attachments.len());
    for attachment in attachments {
        if let Err(error) = storage
            .put_object(
                &attachment.file_key,
                attachment.bytes.clone(),
                &attachment.content_type,
            )
            .await
        {
            cleanup_objects(storage, &stored_keys).await;
            return Err(error);
        }
        stored_keys.push(attachment.file_key.clone());
    }
    Ok(())
}

fn inserted_markdown(attachments: &[AttachmentCreate]) -> String {
    attachments
        .iter()
        .map(|attachment| embed_markdown(&attachment.media_id, attachment.media_family))
        .collect::<Vec<_>>()
        .join("\n\n")
}

async fn cleanup_objects(storage: &Storage, keys: &[String]) {
    for key in keys {
        let _ = storage.delete_object(key).await;
    }
}
