use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Clone, sqlx::FromRow, Serialize)]
pub struct TagRow {
    pub note_id: Uuid,
    pub tag: String,
}
