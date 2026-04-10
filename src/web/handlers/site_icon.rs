use super::session;
use crate::error::AppError;
use crate::web::db;
use crate::web::handlers::http;
use crate::web::handlers::media_input::{discard_field, field_bytes_limited};
use crate::web::routes::AppState;
use axum::extract::multipart::Field;
use axum::extract::{Multipart, State};
use axum::http::HeaderMap;
use axum::response::Response;
use std::path::Path;
use uuid::Uuid;

struct IconUpload {
    bytes: Vec<u8>,
    filename: String,
    content_type: String,
}

pub async fn upload(
    State(state): State<AppState>,
    headers: HeaderMap,
    payload: Multipart,
) -> Result<Response, AppError> {
    let pool = &state.pool;
    let storage = &state.storage;
    session::require_session(&headers, pool).await?;
    let upload = parse_icon(payload, state.site_icon_upload_max_bytes).await?;
    validate_icon(&upload)?;
    let key = format!(
        "site-icons/{}-{}",
        Uuid::new_v4(),
        safe_name(&upload.filename)
    );
    storage
        .put_object(&key, upload.bytes, &upload.content_type)
        .await?;
    let mut settings = db::get_settings(pool).await?;
    let old_key = settings.site_icon_key.replace(key.clone());
    settings.site_icon_content_type = Some(upload.content_type);
    let result = db::update_settings(pool, &settings).await;
    if result.is_err() {
        let _ = storage.delete_object(&key).await;
    } else if let Some(old_key) = old_key.filter(|old_key| old_key != &key) {
        let _ = storage.delete_object(&old_key).await;
    }
    result?;
    Ok(http::see_other("/admin/settings"))
}

async fn parse_icon(mut payload: Multipart, max_bytes: usize) -> Result<IconUpload, AppError> {
    let mut icon = None;
    while let Some(field) = payload
        .next_field()
        .await
        .map_err(|error| invalid(&format!("invalid multipart payload: {error}")))?
    {
        if field.name() == Some("icon") {
            icon = Some(read_icon(field, max_bytes).await?);
        } else {
            discard_field(field, 16 * 1024).await?;
        }
    }
    icon.ok_or_else(|| invalid("icon image is required"))
}

async fn read_icon(field: Field<'_>, max_bytes: usize) -> Result<IconUpload, AppError> {
    let filename = field
        .file_name()
        .map(str::to_string)
        .unwrap_or_else(|| "site-icon".to_string());
    let content_type = field
        .content_type()
        .map(str::to_string)
        .unwrap_or_else(|| "application/octet-stream".to_string());
    let bytes = field_bytes_limited(field, max_bytes, "icon image exceeds upload limit").await?;
    if bytes.is_empty() {
        return Err(invalid("icon image is required"));
    }
    Ok(IconUpload {
        bytes,
        filename,
        content_type,
    })
}

fn validate_icon(icon: &IconUpload) -> Result<(), AppError> {
    if icon.content_type.starts_with("image/")
        || extension(&icon.filename).is_some_and(|value| value.eq_ignore_ascii_case("ico"))
    {
        Ok(())
    } else {
        Err(invalid("site icon must be an image"))
    }
}

fn safe_name(filename: &str) -> String {
    filename
        .chars()
        .map(|ch| {
            if ch.is_ascii_alphanumeric() || ".-_".contains(ch) {
                ch
            } else {
                '-'
            }
        })
        .collect::<String>()
        .trim_matches('-')
        .to_string()
}

fn extension(filename: &str) -> Option<&str> {
    Path::new(filename)
        .extension()
        .and_then(|value| value.to_str())
}

fn invalid(message: &str) -> AppError {
    AppError::InvalidRequest(message.to_string())
}
