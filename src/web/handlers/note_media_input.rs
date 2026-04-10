use crate::error::AppError;
use axum::extract::Multipart;

use super::media_input::{discard_field, field_bytes_limited, read_uploaded_file, UploadedFile};

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
    max_file_bytes: usize,
) -> Result<NoteMediaAttachmentInput, AppError> {
    let mut files = Vec::new();
    let mut body = None;
    let mut alias = None;
    let mut is_favorite = None;
    let mut is_private = None;
    let mut insert_start = None;
    let mut insert_end = None;
    while let Some(field) = payload
        .next_field()
        .await
        .map_err(|e| AppError::InvalidRequest(format!("invalid multipart payload: {e}")))?
    {
        match field.name() {
            Some("file") => files.push(read_uploaded_file(field, max_file_bytes).await?),
            Some("body") => body = Some(raw_text(field).await?),
            Some("alias") => alias = text_value(field).await?,
            Some("is_favorite") => is_favorite = Some(bool_value(field).await?),
            Some("is_private") => is_private = Some(bool_value(field).await?),
            Some("insert_start") => insert_start = Some(usize_value(field).await?),
            Some("insert_end") => insert_end = Some(usize_value(field).await?),
            _ => {
                discard_field(field, MAX_TEXT_BYTES).await?;
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

async fn required_text(field: axum::extract::multipart::Field<'_>) -> Result<String, AppError> {
    text_value(field)
        .await?
        .ok_or_else(|| invalid("text field is required"))
}

async fn text_value(
    field: axum::extract::multipart::Field<'_>,
) -> Result<Option<String>, AppError> {
    Ok(trimmed_text(raw_text(field).await?))
}

async fn raw_text(field: axum::extract::multipart::Field<'_>) -> Result<String, AppError> {
    decode_text(field_bytes_limited(field, MAX_TEXT_BYTES, "text field exceeds limit").await?)
}

fn decode_text(bytes: Vec<u8>) -> Result<String, AppError> {
    String::from_utf8(bytes).map_err(|_| invalid("text fields must be utf-8"))
}

fn trimmed_text(value: String) -> Option<String> {
    let trimmed = value.trim();
    (!trimmed.is_empty()).then(|| trimmed.to_string())
}

async fn bool_value(field: axum::extract::multipart::Field<'_>) -> Result<bool, AppError> {
    match required_text(field).await?.as_str() {
        "0" | "false" | "off" => Ok(false),
        "1" | "true" | "on" => Ok(true),
        _ => Err(invalid("boolean fields must be true/false")),
    }
}

async fn usize_value(field: axum::extract::multipart::Field<'_>) -> Result<usize, AppError> {
    required_text(field)
        .await?
        .parse::<usize>()
        .map_err(|_| invalid("insert positions must be non-negative integers"))
}

fn invalid(message: &str) -> AppError {
    AppError::InvalidRequest(message.to_string())
}

#[cfg(test)]
mod tests {
    use super::{decode_text, trimmed_text};

    #[test]
    fn decode_text_preserves_raw_body_whitespace() {
        assert_eq!(
            decode_text(b"# Title\n\n  ".to_vec()).unwrap(),
            "# Title\n\n  "
        );
    }

    #[test]
    fn trimmed_text_keeps_non_body_fields_trimmed() {
        assert_eq!(
            trimmed_text("  launch-note  \n".to_string()),
            Some("launch-note".to_string())
        );
        assert_eq!(trimmed_text(" \n\t ".to_string()), None);
    }
}
