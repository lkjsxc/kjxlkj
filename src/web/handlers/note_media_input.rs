use crate::error::AppError;
use actix_multipart::Multipart;
use futures_util::TryStreamExt;

use super::media_input::UploadedFile;

const MAX_FILE_BYTES: usize = 128 * 1024 * 1024;
const MAX_TEXT_BYTES: usize = 2 * 1024 * 1024;

pub struct NoteMediaAttachmentInput {
    pub files: Vec<UploadedFile>,
    pub body: String,
    pub alias: Option<String>,
    pub is_favorite: bool,
    pub is_private: bool,
    pub insert_start: usize,
    pub insert_end: usize,
}

pub async fn parse_note_media_form(
    mut payload: Multipart,
) -> Result<NoteMediaAttachmentInput, AppError> {
    let mut files = Vec::new();
    let mut body = None;
    let mut alias = None;
    let mut is_favorite = None;
    let mut is_private = None;
    let mut insert_start = None;
    let mut insert_end = None;
    while let Some(mut field) = payload
        .try_next()
        .await
        .map_err(|e| AppError::InvalidRequest(format!("invalid multipart payload: {e}")))?
    {
        match field.name() {
            Some("file") => files.push(read_file(&mut field).await?),
            Some("body") => body = Some(required_text(&mut field).await?),
            Some("alias") => alias = text_value(&mut field).await?,
            Some("is_favorite") => is_favorite = Some(bool_value(&mut field).await?),
            Some("is_private") => is_private = Some(bool_value(&mut field).await?),
            Some("insert_start") => insert_start = Some(usize_value(&mut field).await?),
            Some("insert_end") => insert_end = Some(usize_value(&mut field).await?),
            _ => {
                let _ = field
                    .bytes(MAX_TEXT_BYTES)
                    .await
                    .map_err(|_| invalid("unexpected field exceeds limit"))?;
            }
        }
    }
    if files.is_empty() {
        return Err(invalid("at least one file is required"));
    }
    Ok(NoteMediaAttachmentInput {
        files,
        body: body.ok_or_else(|| invalid("body is required"))?,
        alias,
        is_favorite: is_favorite.ok_or_else(|| invalid("is_favorite is required"))?,
        is_private: is_private.ok_or_else(|| invalid("is_private is required"))?,
        insert_start: insert_start.ok_or_else(|| invalid("insert_start is required"))?,
        insert_end: insert_end.ok_or_else(|| invalid("insert_end is required"))?,
    })
}

async fn read_file(field: &mut actix_multipart::Field) -> Result<UploadedFile, AppError> {
    let bytes = field
        .bytes(MAX_FILE_BYTES)
        .await
        .map_err(|_| invalid("file exceeds upload limit"))?
        .map_err(|e| invalid(&format!("could not read upload: {e}")))?;
    if bytes.is_empty() {
        return Err(invalid("file is required"));
    }
    Ok(UploadedFile {
        bytes: bytes.to_vec(),
        original_filename: field
            .content_disposition()
            .and_then(|item| item.get_filename())
            .map(str::to_string)
            .filter(|value| !value.trim().is_empty())
            .unwrap_or_else(|| "upload.bin".to_string()),
        content_type: field
            .content_type()
            .map(|value| value.to_string())
            .unwrap_or_else(|| "application/octet-stream".to_string()),
    })
}

async fn required_text(field: &mut actix_multipart::Field) -> Result<String, AppError> {
    text_value(field)
        .await?
        .ok_or_else(|| invalid("text field is required"))
}

async fn text_value(field: &mut actix_multipart::Field) -> Result<Option<String>, AppError> {
    let bytes = field
        .bytes(MAX_TEXT_BYTES)
        .await
        .map_err(|_| invalid("text field exceeds limit"))?
        .map_err(|e| invalid(&format!("could not read field: {e}")))?;
    let value =
        String::from_utf8(bytes.to_vec()).map_err(|_| invalid("text fields must be utf-8"))?;
    let trimmed = value.trim();
    Ok((!trimmed.is_empty()).then(|| trimmed.to_string()))
}

async fn bool_value(field: &mut actix_multipart::Field) -> Result<bool, AppError> {
    match required_text(field).await?.as_str() {
        "0" | "false" | "off" => Ok(false),
        "1" | "true" | "on" => Ok(true),
        _ => Err(invalid("boolean fields must be true/false")),
    }
}

async fn usize_value(field: &mut actix_multipart::Field) -> Result<usize, AppError> {
    required_text(field)
        .await?
        .parse::<usize>()
        .map_err(|_| invalid("insert positions must be non-negative integers"))
}

fn invalid(message: &str) -> AppError {
    AppError::InvalidRequest(message.to_string())
}
