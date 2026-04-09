use crate::error::AppError;
use actix_multipart::Multipart;
use futures_util::TryStreamExt;

const MAX_FILE_BYTES: usize = 128 * 1024 * 1024;
const MAX_TEXT_BYTES: usize = 16 * 1024;

pub struct UploadedFile {
    pub bytes: Vec<u8>,
    pub original_filename: String,
    pub content_type: String,
}

pub struct MediaFormInput {
    pub file: UploadedFile,
    pub alias: Option<String>,
    pub is_favorite: Option<bool>,
    pub is_private: Option<bool>,
}

pub async fn parse_media_form(mut payload: Multipart) -> Result<MediaFormInput, AppError> {
    let mut file = None;
    let mut alias = None;
    let mut is_favorite = None;
    let mut is_private = None;
    while let Some(mut field) = payload
        .try_next()
        .await
        .map_err(|e| AppError::InvalidRequest(format!("invalid multipart payload: {e}")))?
    {
        match field.name() {
            Some("file") => {
                if file.is_some() {
                    return Err(invalid("file may only be provided once"));
                }
                let bytes = field
                    .bytes(MAX_FILE_BYTES)
                    .await
                    .map_err(|_| invalid("file exceeds upload limit"))?
                    .map_err(|e| invalid(&format!("could not read upload: {e}")))?;
                if bytes.is_empty() {
                    return Err(invalid("file is required"));
                }
                file = Some(UploadedFile {
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
                });
            }
            Some("alias") => alias = text_value(&mut field).await?,
            Some("is_favorite") => is_favorite = Some(bool_value(&mut field).await?),
            Some("is_private") => is_private = Some(bool_value(&mut field).await?),
            _ => {
                let _ = field
                    .bytes(MAX_TEXT_BYTES)
                    .await
                    .map_err(|_| invalid("unexpected field exceeds limit"))?;
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

async fn text_value(
    field: &mut actix_multipart::Field,
) -> Result<Option<String>, AppError> {
    let bytes = field
        .bytes(MAX_TEXT_BYTES)
        .await
        .map_err(|_| invalid("text field exceeds limit"))?
        .map_err(|e| invalid(&format!("could not read field: {e}")))?;
    let value = String::from_utf8(bytes.to_vec())
        .map_err(|_| invalid("text fields must be utf-8"))?;
    let trimmed = value.trim();
    Ok((!trimmed.is_empty()).then(|| trimmed.to_string()))
}

async fn bool_value(field: &mut actix_multipart::Field) -> Result<bool, AppError> {
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
