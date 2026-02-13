//! Project domain types.

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use time::OffsetDateTime;

/// Project entity.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub id: Uuid,
    pub workspace_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub is_active: bool,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

impl Project {
    /// Create a new project.
    pub fn new(workspace_id: Uuid, name: String) -> Self {
        let now = OffsetDateTime::now_utc();
        Self {
            id: Uuid::new_v4(),
            workspace_id,
            name,
            description: None,
            is_active: true,
            created_at: now,
            updated_at: now,
        }
    }
}
