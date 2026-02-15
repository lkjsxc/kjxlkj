use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

use crate::ids::{ProjectId, WorkspaceId};

/// Project entity per /docs/spec/domain/projects.md.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub id: ProjectId,
    pub workspace_id: WorkspaceId,
    pub name: String,
    pub description: Option<String>,
    pub archived: bool,
    #[serde(with = "time::serde::rfc3339")]
    pub created_at: OffsetDateTime,
}
