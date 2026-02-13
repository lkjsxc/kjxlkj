use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Clone, sqlx::FromRow, Serialize)]
pub struct ViewRow {
    pub id: Uuid,
    pub workspace_id: Uuid,
    pub name: String,
    pub filter_json: serde_json::Value,
    pub sort_json: serde_json::Value,
    pub created_by: Uuid,
    pub created_at: time::OffsetDateTime,
    pub updated_at: time::OffsetDateTime,
}
