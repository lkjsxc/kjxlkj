//! Workspace and Project entities

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Workspace aggregate root
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Workspace {
    pub workspace_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub owner_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Workspace {
    pub fn new(name: String, owner_id: Uuid) -> Self {
        let now = Utc::now();
        Self {
            workspace_id: Uuid::new_v4(),
            name,
            description: None,
            owner_id,
            created_at: now,
            updated_at: now,
        }
    }
}

/// Project for scoping notes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub project_id: Uuid,
    pub workspace_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Project {
    pub fn new(name: String, workspace_id: Uuid) -> Self {
        let now = Utc::now();
        Self {
            project_id: Uuid::new_v4(),
            workspace_id,
            name,
            description: None,
            created_at: now,
            updated_at: now,
        }
    }
}
