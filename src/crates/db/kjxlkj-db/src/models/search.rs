use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Clone, sqlx::FromRow, Serialize)]
pub struct SearchResultRow {
    pub note_id: Uuid,
    pub title: String,
    pub snippet: String,
    pub rank: f32,
}
