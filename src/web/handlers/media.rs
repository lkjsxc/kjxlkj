use super::media_input::parse_media_form;
use super::resource_payload::ResourcePayload;
use crate::core::{generate_id, normalize_alias, validate_id};
use crate::error::AppError;
use crate::storage::Storage;
use crate::web::db::{self, MediaBlob, MediaFamily};
use actix_multipart::Multipart;
use actix_web::{post, put, web, HttpRequest, HttpResponse};
use sha2::{Digest, Sha256};
use std::path::Path;

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
    };
    let record = db::create_media(
        &pool,
        &id,
        normalize_alias(form.alias.as_deref())?.as_deref(),
        &body,
        &blob,
        form.is_favorite.unwrap_or(false),
        form.is_private
            .unwrap_or(db::get_settings(&pool).await?.default_new_resource_is_private),
    )
    .await?;
    Ok(HttpResponse::Created().json(ResourcePayload::from_record(record)))
}

#[put("/resources/media/{id}/file")]
pub async fn replace_file(
    pool: web::Data<db::DbPool>,
    storage: web::Data<Storage>,
    req: HttpRequest,
    path: web::Path<String>,
    payload: Multipart,
) -> Result<HttpResponse, AppError> {
    super::session::require_session(&req, &pool).await?;
    let id = path.into_inner();
    validate_id(&id)?;
    let form = parse_media_form(payload).await?;
    let file_key = object_key(&id, &form.file.original_filename);
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
    };
    match db::replace_media_file(&pool, &id, &blob).await? {
        Some(record) => Ok(HttpResponse::Ok().json(ResourcePayload::from_record(record))),
        None => Err(AppError::NotFound(format!("resource '{id}' not found"))),
    }
}

fn detect_media_family(content_type: &str, filename: &str) -> Result<MediaFamily, AppError> {
    if content_type.starts_with("image/") {
        return Ok(MediaFamily::Image);
    }
    if content_type.starts_with("video/") {
        return Ok(MediaFamily::Video);
    }
    match extension(filename).as_deref() {
        Some("png" | "jpg" | "jpeg" | "gif" | "webp" | "svg") => Ok(MediaFamily::Image),
        Some("mp4" | "webm" | "mov" | "m4v" | "ogg") => Ok(MediaFamily::Video),
        _ => Err(AppError::InvalidRequest(
            "unsupported media type; use an image or video".to_string(),
        )),
    }
}

fn initial_body(filename: &str) -> String {
    format!("# {}", pretty_stem(filename))
}

fn object_key(id: &str, filename: &str) -> String {
    format!("media/{id}/{}-{}", generate_id(), safe_name(filename))
}

fn safe_name(filename: &str) -> String {
    filename
        .chars()
        .map(|ch| if ch.is_ascii_alphanumeric() || ".-_".contains(ch) { ch } else { '-' })
        .collect::<String>()
        .trim_matches('-')
        .to_string()
}

fn pretty_stem(filename: &str) -> String {
    let stem = Path::new(filename)
        .file_stem()
        .and_then(|value| value.to_str())
        .unwrap_or("Untitled media");
    let words = stem
        .split(['-', '_', '.'])
        .filter(|part| !part.is_empty())
        .map(capitalize)
        .collect::<Vec<_>>();
    if words.is_empty() {
        "Untitled media".to_string()
    } else {
        words.join(" ")
    }
}

fn capitalize(word: &str) -> String {
    let mut chars = word.chars();
    match chars.next() {
        Some(first) => first.to_ascii_uppercase().to_string() + chars.as_str(),
        None => String::new(),
    }
}

fn extension(filename: &str) -> Option<String> {
    Path::new(filename)
        .extension()
        .and_then(|value| value.to_str())
        .map(|value| value.to_ascii_lowercase())
}

fn sha256_hex(bytes: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    format!("{:x}", hasher.finalize())
}
