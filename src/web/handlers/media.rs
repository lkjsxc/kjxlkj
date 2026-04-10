use super::media_input::parse_media_form;
use super::media_support::{detect_media_family, initial_body, object_key, sha256_hex};
use super::resource_payload::ResourcePayload;
use crate::core::normalize_alias;
use crate::error::AppError;
use crate::storage::Storage;
use crate::web::db::{self, MediaBlob};
use actix_multipart::Multipart;
use actix_web::{post, web, HttpRequest, HttpResponse};

#[post("/resources/media")]
pub async fn create(
    pool: web::Data<db::DbPool>,
    storage: web::Data<Storage>,
    req: HttpRequest,
    payload: Multipart,
) -> Result<HttpResponse, AppError> {
    super::session::require_session(&req, &pool).await?;
    let form = parse_media_form(payload).await?;
    let id = db::generate_resource_id(&pool).await?;
    let file_key = object_key(&id, &form.file.original_filename);
    let body = initial_body(&form.file.original_filename);
    storage
        .put_object(&file_key, form.file.bytes.clone(), &form.file.content_type)
        .await?;
    let sha256_hex = sha256_hex(&form.file.bytes);
    let blob = MediaBlob {
        media_family: detect_media_family(&form.file.content_type, &form.file.original_filename)?,
        file_key: &file_key,
        content_type: &form.file.content_type,
        byte_size: form.file.bytes.len() as i64,
        sha256_hex: &sha256_hex,
        original_filename: &form.file.original_filename,
        width: None,
        height: None,
        duration_ms: None,
        media_variants: None,
    };
    let result = db::create_media(
        &pool,
        &id,
        normalize_alias(form.alias.as_deref())?.as_deref(),
        &body,
        &blob,
        form.is_favorite.unwrap_or(false),
        form.is_private.unwrap_or(
            db::get_settings(&pool)
                .await?
                .default_new_resource_is_private,
        ),
    )
    .await;
    match result {
        Ok(resource) => Ok(HttpResponse::Created().json(ResourcePayload::from_resource(resource))),
        Err(error) => {
            let _ = storage.delete_object(&file_key).await;
            Err(error)
        }
    }
}
