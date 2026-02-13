use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Clone, sqlx::FromRow, Serialize)]
pub struct MetadataRow {
    pub note_id: Uuid,
    pub key: String,
    pub value: serde_json::Value,
    pub updated_at: time::OffsetDateTime,
}

#[derive(Debug, Clone, sqlx::FromRow, Serialize)]
pub struct BacklinkRow {
    pub source_note_id: Uuid,
    pub target_note_id: Uuid,
}
