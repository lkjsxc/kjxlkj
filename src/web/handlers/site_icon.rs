use super::session;
use crate::error::AppError;
use crate::storage::Storage;
use crate::web::db::{self, DbPool};
use actix_multipart::{Field, Multipart};
use actix_web::{post, web, HttpRequest, HttpResponse};
use futures_util::TryStreamExt;
use std::path::Path;
use uuid::Uuid;

const MAX_ICON_BYTES: usize = 2 * 1024 * 1024;

struct IconUpload {
    bytes: Vec<u8>,
    filename: String,
    content_type: String,
}

#[post("/admin/site-icon")]
pub async fn upload(
    pool: web::Data<DbPool>,
    storage: web::Data<Storage>,
    req: HttpRequest,
    payload: Multipart,
) -> Result<HttpResponse, AppError> {
    session::require_session(&req, &pool).await?;
    let upload = parse_icon(payload).await?;
    validate_icon(&upload)?;
    let key = format!(
        "site-icons/{}-{}",
        Uuid::new_v4(),
        safe_name(&upload.filename)
    );
    storage
        .put_object(&key, upload.bytes, &upload.content_type)
        .await?;
    let mut settings = db::get_settings(&pool).await?;
    let old_key = settings.site_icon_key.replace(key.clone());
    settings.site_icon_content_type = Some(upload.content_type);
    let result = db::update_settings(&pool, &settings).await;
    if result.is_err() {
        let _ = storage.delete_object(&key).await;
    } else if let Some(old_key) = old_key.filter(|old_key| old_key != &key) {
        let _ = storage.delete_object(&old_key).await;
    }
    result?;
    Ok(HttpResponse::SeeOther()
        .append_header(("Location", "/admin/settings"))
        .finish())
}

async fn parse_icon(mut payload: Multipart) -> Result<IconUpload, AppError> {
    let mut icon = None;
    while let Some(mut field) = payload.try_next().await.map_err(invalid_payload)? {
        if field.name() == Some("icon") {
            icon = Some(read_icon(&mut field).await?);
        } else {
            let _ = field.bytes(1024).await;
        }
    }
    icon.ok_or_else(|| invalid("icon image is required"))
}

async fn read_icon(field: &mut Field) -> Result<IconUpload, AppError> {
    let bytes = field
        .bytes(MAX_ICON_BYTES)
        .await
        .map_err(|_| invalid("icon image exceeds upload limit"))?
        .map_err(|error| invalid(&format!("could not read icon image: {error}")))?;
    if bytes.is_empty() {
        return Err(invalid("icon image is required"));
    }
    Ok(IconUpload {
        bytes: bytes.to_vec(),
        filename: field
            .content_disposition()
            .and_then(|item| item.get_filename())
            .unwrap_or("site-icon")
            .to_string(),
        content_type: field
            .content_type()
            .map(|value| value.to_string())
            .unwrap_or_else(|| "application/octet-stream".to_string()),
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

fn invalid_payload(error: actix_multipart::MultipartError) -> AppError {
    invalid(&format!("invalid multipart payload: {error}"))
}

fn invalid(message: &str) -> AppError {
    AppError::InvalidRequest(message.to_string())
}
