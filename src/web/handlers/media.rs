use super::media_input::parse_media_form;
use super::media_support::{detect_media_family, initial_body, object_key};
use super::resource_payload::ResourcePayload;
use crate::core::normalize_alias;
use crate::error::AppError;
use crate::storage::Storage;
use crate::web::db::{self, MediaBlob};
use crate::web::handlers::http;
use crate::web::routes::AppState;
use axum::extract::{Multipart, State};
use axum::http::{HeaderMap, StatusCode};
use axum::response::Response;

pub async fn create(
    State(state): State<AppState>,
    headers: HeaderMap,
    payload: Multipart,
) -> Result<Response, AppError> {
    let pool = &state.pool;
    let storage = &state.storage;
    super::session::require_session(&headers, pool).await?;
    let form = parse_media_form(payload, state.media_upload_max_bytes).await?;
    let settings = db::get_settings(pool).await?;
    let id = db::generate_resource_id(pool).await?;
    let file_key = object_key(&id, &form.file.original_filename);
    let body = initial_body(&form.file.original_filename);
    let media_family = detect_media_family(&form.file.content_type, &form.file.original_filename)?;
    let source_bytes = if media_family == db::MediaFamily::Image {
        form.file.read_bytes().await?
    } else {
        Vec::new()
    };
    let generated_variants = super::media_derivatives::build_variants(
        &id,
        media_family,
        &body,
        &source_bytes,
        settings.media_webp_quality,
    );
    storage
        .put_file(&file_key, form.file.path(), &form.file.content_type)
        .await?;
    let (media_variants, stored_variant_keys) =
        super::media_derivatives::store_variants(storage, &generated_variants).await;
    let blob = MediaBlob {
        media_family,
        file_key: &file_key,
        content_type: &form.file.content_type,
        byte_size: form.file.byte_size,
        sha256_hex: &form.file.sha256_hex,
        original_filename: &form.file.original_filename,
        width: None,
        height: None,
        duration_ms: None,
        media_variants,
    };
    let stored_keys = stored_keys(file_key.clone(), stored_variant_keys);
    let result = db::create_media(
        pool,
        &id,
        normalize_alias(form.alias.as_deref())?.as_deref(),
        &body,
        &blob,
        form.is_favorite.unwrap_or(false),
        form.is_private
            .unwrap_or(settings.default_new_resource_is_private),
    )
    .await;
    match result {
        Ok(resource) => Ok(http::json_status(
            StatusCode::CREATED,
            ResourcePayload::from_resource(resource),
        )),
        Err(error) => {
            cleanup_objects(storage, &stored_keys).await;
            Err(error)
        }
    }
}

fn stored_keys(file_key: String, variant_keys: Vec<String>) -> Vec<String> {
    std::iter::once(file_key).chain(variant_keys).collect()
}

async fn cleanup_objects(storage: &Storage, keys: &[String]) {
    for key in keys {
        let _ = storage.delete_object(key).await;
    }
}
