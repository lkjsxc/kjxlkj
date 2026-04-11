use crate::error::AppError;
use axum::extract::multipart::Field;
use axum::extract::Multipart;
use sha2::{Digest, Sha256};
use std::io::Write;
use std::path::Path;
use tempfile::NamedTempFile;

const MAX_TEXT_BYTES: usize = 16 * 1024;

pub struct UploadedFile {
    temp_file: NamedTempFile,
    pub byte_size: i64,
    pub sha256_hex: String,
    pub original_filename: String,
    pub content_type: String,
}

impl UploadedFile {
    pub fn path(&self) -> &Path {
        self.temp_file.path()
    }
}

pub struct MediaFormInput {
    pub file: UploadedFile,
    pub alias: Option<String>,
    pub is_favorite: Option<bool>,
    pub is_private: Option<bool>,
}

pub async fn parse_media_form(
    mut payload: Multipart,
    max_file_bytes: usize,
) -> Result<MediaFormInput, AppError> {
    let mut file = None;
    let mut alias = None;
    let mut is_favorite = None;
    let mut is_private = None;
    while let Some(field) = payload
        .next_field()
        .await
        .map_err(|e| AppError::InvalidRequest(format!("invalid multipart payload: {e}")))?
    {
        match field.name() {
            Some("file") => {
                if file.is_some() {
                    return Err(invalid("file may only be provided once"));
                }
                file = Some(read_uploaded_file(field, max_file_bytes).await?);
            }
            Some("alias") => alias = text_value(field).await?,
            Some("is_favorite") => is_favorite = Some(bool_value(field).await?),
            Some("is_private") => is_private = Some(bool_value(field).await?),
            _ => {
                discard_field(field, MAX_TEXT_BYTES).await?;
            }
        }
    }
    Ok(MediaFormInput {
        file: file.ok_or_else(|| invalid("file is required"))?,
        alias,
        is_favorite,
        is_private,
    })
}

pub(super) async fn read_uploaded_file(
    mut field: Field<'_>,
    max_file_bytes: usize,
) -> Result<UploadedFile, AppError> {
    let original_filename = field
        .file_name()
        .map(str::to_string)
        .filter(|value| !value.trim().is_empty())
        .unwrap_or_else(|| "upload.bin".to_string());
    let content_type = field
        .content_type()
        .map(str::to_string)
        .unwrap_or_else(|| "application/octet-stream".to_string());
    let mut temp_file = NamedTempFile::new()
        .map_err(|e| AppError::StorageError(format!("upload temp file create failed: {e}")))?;
    let mut hasher = Sha256::new();
    let mut byte_size = 0usize;
    while let Some(chunk) = field
        .chunk()
        .await
        .map_err(|e| invalid(&format!("could not read upload: {e}")))?
    {
        byte_size = byte_size
            .checked_add(chunk.len())
            .ok_or_else(|| AppError::PayloadTooLarge("file exceeds upload limit".to_string()))?;
        if byte_size > max_file_bytes {
            return Err(AppError::PayloadTooLarge(
                "file exceeds upload limit".to_string(),
            ));
        }
        hasher.update(&chunk);
        temp_file
            .write_all(&chunk)
            .map_err(|e| AppError::StorageError(format!("upload temp file write failed: {e}")))?;
    }
    temp_file
        .flush()
        .map_err(|e| AppError::StorageError(format!("upload temp file flush failed: {e}")))?;
    if byte_size == 0 {
        return Err(invalid("file is required"));
    }
    Ok(UploadedFile {
        temp_file,
        byte_size: byte_size as i64,
        sha256_hex: format!("{:x}", hasher.finalize()),
        original_filename,
        content_type,
    })
}

pub(super) async fn field_bytes_limited(
    mut field: Field<'_>,
    max_bytes: usize,
    too_large_message: &str,
) -> Result<Vec<u8>, AppError> {
    let mut bytes = Vec::new();
    while let Some(chunk) = field
        .chunk()
        .await
        .map_err(|e| invalid(&format!("could not read field: {e}")))?
    {
        let new_len = bytes
            .len()
            .checked_add(chunk.len())
            .ok_or_else(|| AppError::PayloadTooLarge(too_large_message.to_string()))?;
        if new_len > max_bytes {
            return Err(AppError::PayloadTooLarge(too_large_message.to_string()));
        }
        bytes.extend_from_slice(&chunk);
    }
    Ok(bytes)
}

pub(super) async fn discard_field(field: Field<'_>, max_bytes: usize) -> Result<(), AppError> {
    field_bytes_limited(field, max_bytes, "field exceeds upload limit")
        .await
        .map(|_| ())
}

async fn text_value(field: Field<'_>) -> Result<Option<String>, AppError> {
    let bytes = field_bytes_limited(field, MAX_TEXT_BYTES, "text field exceeds limit").await?;
    let value = String::from_utf8(bytes).map_err(|_| invalid("text fields must be utf-8"))?;
    let trimmed = value.trim();
    Ok((!trimmed.is_empty()).then(|| trimmed.to_string()))
}

async fn bool_value(field: Field<'_>) -> Result<bool, AppError> {
    let value = text_value(field).await?.unwrap_or_default();
    match value.as_str() {
        "" | "0" | "false" | "off" => Ok(false),
        "1" | "true" | "on" => Ok(true),
        _ => Err(invalid("boolean fields must be true/false")),
    }
}

fn invalid(message: &str) -> AppError {
    AppError::InvalidRequest(message.to_string())
}
