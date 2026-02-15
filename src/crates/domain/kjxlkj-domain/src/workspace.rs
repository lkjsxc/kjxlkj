use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

use crate::ids::{UserId, WorkspaceId};

/// Workspace lifecycle state per /docs/spec/domain/workspaces.md.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum WorkspaceStatus {
    Active,
    Archived,
    Deleted,
}

/// Workspace entity per /docs/spec/api/types.md.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Workspace {
    pub id: WorkspaceId,
    pub slug: String,
    pub name: String,
    pub owner_user_id: UserId,
    pub status: WorkspaceStatus,
    #[serde(with = "time::serde::rfc3339")]
    pub created_at: OffsetDateTime,
}
