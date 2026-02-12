use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceScoped {
    pub workspace_id: Uuid,
    pub updated_at: DateTime<Utc>,
}
