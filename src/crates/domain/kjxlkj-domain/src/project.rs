use serde::{Deserialize, Serialize};
use uuid::Uuid;
use time::OffsetDateTime;

/// Project entity per docs/spec/domain/projects.md.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub id: Uuid,
    pub workspace_id: Uuid,
    pub name: String,
    pub is_archived: bool,
    #[serde(with = "time::serde::rfc3339")]
    pub created_at: OffsetDateTime,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateProjectRequest {
    pub workspace_id: Uuid,
    pub name: String,
}
