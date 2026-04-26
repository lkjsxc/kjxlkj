use crate::error::AppError;
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
struct SnapshotCursor {
    snapshot_number: i32,
}

pub(super) fn decode_snapshot_cursor(cursor: Option<&str>) -> Result<Option<i32>, AppError> {
    let Some(cursor) = cursor else {
        return Ok(None);
    };
    let raw = URL_SAFE_NO_PAD
        .decode(cursor)
        .map_err(|_| AppError::InvalidRequest("invalid cursor".to_string()))?;
    let text = String::from_utf8(raw)
        .map_err(|_| AppError::InvalidRequest("invalid cursor".to_string()))?;
    let cursor: SnapshotCursor = serde_json::from_str(&text)
        .map_err(|_| AppError::InvalidRequest("invalid cursor".to_string()))?;
    Ok(Some(cursor.snapshot_number))
}

pub(super) fn encode_snapshot_cursor(snapshot_number: i32) -> String {
    URL_SAFE_NO_PAD.encode(serde_json::to_string(&SnapshotCursor { snapshot_number }).unwrap())
}
