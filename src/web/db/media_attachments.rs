use super::media::MediaBlob;
use super::models::{Resource, ResourceKind};
use super::resource_support::{
    map_write_error, resolve_position, row_to_resource, RETURNING_RECORD,
};
use super::write_support::{client, create_snapshot, next_snapshot_number};
use super::{DbPool, MediaFamily};
use crate::core::{derive_summary, derive_title, derive_title_with_fallback};
use crate::error::AppError;
use crate::media::{media_variants_to_json, GeneratedVariant};
use deadpool_postgres::GenericClient;

pub struct AttachmentCreate {
    pub media_id: String,
    pub media_body: String,
    pub media_family: MediaFamily,
    pub file_key: String,
    pub content_type: String,
    pub byte_size: i64,
    pub sha256_hex: String,
    pub original_filename: String,
    pub media_variants: Option<crate::media::MediaVariants>,
    pub generated_variants: Vec<GeneratedVariant>,
}

#[rustfmt::skip]
pub struct NoteAttachmentUpdate<'a> { pub body: &'a str, pub alias: Option<&'a str>, pub is_favorite: bool, pub is_private: bool }

#[rustfmt::skip]
pub struct AttachmentBatchResult { pub current_resource: Resource, pub created_media: Vec<Resource> }

pub async fn attach_media_to_note(
    pool: &DbPool,
    note_id: &str,
    update: &NoteAttachmentUpdate<'_>,
    attachments: &[AttachmentCreate],
) -> Result<AttachmentBatchResult, AppError> {
    let mut db = client(pool).await?;
    let tx = db
        .transaction()
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;
    let Some((kind, was_favorite, current_position)) = target_note_state(&tx, note_id).await?
    else {
        return Err(AppError::NotFound(format!(
            "resource '{note_id}' not found"
        )));
    };
    if kind != ResourceKind::Note {
        return Err(AppError::InvalidRequest(
            "media attachments require a live note".to_string(),
        ));
    }
    let created_media =
        create_media_resources(&tx, note_id, attachments, update.is_private).await?;
    let current_resource =
        update_target_note(&tx, note_id, update, was_favorite, current_position).await?;
    tx.commit()
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;
    Ok(AttachmentBatchResult {
        current_resource,
        created_media,
    })
}

async fn target_note_state<C: GenericClient>(
    db: &C,
    id: &str,
) -> Result<Option<NoteResourceState>, AppError> {
    db.query_opt(
        "SELECT kind, is_favorite, favorite_position FROM resources WHERE id = $1 AND deleted_at IS NULL FOR UPDATE",
        &[&id],
    )
    .await
    .map(|row| {
        row.map(|item| {
            (
                ResourceKind::from_db(&item.get::<_, String>("kind")),
                item.get("is_favorite"),
                item.get("favorite_position"),
            )
        })
    })
    .map_err(|e| AppError::DatabaseError(e.to_string()))
}

type NoteResourceState = (ResourceKind, bool, Option<i64>);

async fn create_media_resources<C: GenericClient>(
    db: &C,
    note_id: &str,
    attachments: &[AttachmentCreate],
    is_private: bool,
) -> Result<Vec<Resource>, AppError> {
    let mut resources = Vec::with_capacity(attachments.len());
    for attachment in attachments {
        let blob = MediaBlob {
            media_family: attachment.media_family,
            file_key: &attachment.file_key,
            content_type: &attachment.content_type,
            byte_size: attachment.byte_size,
            sha256_hex: &attachment.sha256_hex,
            original_filename: &attachment.original_filename,
            width: None,
            height: None,
            duration_ms: None,
            media_variants: attachment.media_variants.clone(),
        };
        let media_variants = media_variants_to_json(&blob.media_variants);
        let row = db
            .query_one(
                &format!(
                    "INSERT INTO resources (id, space_id, kind, title, summary, body, media_family, file_key, content_type, \
                     byte_size, sha256_hex, original_filename, width, height, duration_ms, media_variants, owner_note_id, is_favorite, favorite_position, visibility) \
                     VALUES ($1, default_space_id(), $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, NULL, NULL, NULL, $12, $13, FALSE, NULL, \
                     CASE WHEN $14 THEN 'private'::resource_visibility ELSE 'public'::resource_visibility END) {RETURNING_RECORD}"
                ),
                &[
                    &attachment.media_id,
                    &ResourceKind::Media.as_str(),
                    &derive_title_with_fallback(&attachment.media_body, "Untitled media"),
                    &derive_summary(&attachment.media_body),
                    &attachment.media_body,
                    &blob.media_family.as_str(),
                    &blob.file_key,
                    &blob.content_type,
                    &blob.byte_size,
                    &blob.sha256_hex,
                    &blob.original_filename,
                    &media_variants,
                    &note_id,
                    &is_private,
                ],
            )
            .await
            .map_err(map_write_error)?;
        let resource = row_to_resource(row);
        create_snapshot(db, &resource, 1).await?;
        resources.push(resource);
    }
    Ok(resources)
}

async fn update_target_note<C: GenericClient>(
    db: &C,
    id: &str,
    update: &NoteAttachmentUpdate<'_>,
    was_favorite: bool,
    current_position: Option<i64>,
) -> Result<Resource, AppError> {
    let row = db
        .query_one(
            &format!(
                "UPDATE resources SET alias = $2, title = $3, summary = $4, body = $5, \
                 is_favorite = $6, favorite_position = $7, \
                 visibility = CASE WHEN $8 THEN 'private'::resource_visibility ELSE 'public'::resource_visibility END, \
                 updated_at = NOW() \
                 WHERE id = $1 AND deleted_at IS NULL {RETURNING_RECORD}"
            ),
            &[
                &id,
                &update.alias,
                &derive_title(update.body),
                &derive_summary(update.body),
                &update.body,
                &update.is_favorite,
                &resolve_position(db, was_favorite, current_position, update.is_favorite).await?,
                &update.is_private,
            ],
        )
        .await
        .map_err(map_write_error)?;
    let resource = row_to_resource(row);
    create_snapshot(db, &resource, next_snapshot_number(db, &resource.id).await?).await?;
    Ok(resource)
}
