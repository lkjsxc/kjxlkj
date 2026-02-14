use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Role {
    Owner,
    Admin,
    Editor,
    Viewer,
}

#[derive(Debug, Clone, Serialize)]
pub struct UserRecord {
    pub id: String,
    pub email: String,
    pub display_name: String,
    pub role: Role,
    pub status: String,
    pub created_at: String,
    pub password_hash: String,
}

#[derive(Debug, Clone)]
pub struct SessionRecord {
    pub id: String,
    pub user_id: String,
    pub csrf_token: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct WorkspaceRecord {
    pub id: String,
    pub slug: String,
    pub name: String,
    pub owner_user_id: String,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct ProjectRecord {
    pub id: String,
    pub workspace_id: String,
    pub name: String,
    pub description: String,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SavedViewRecord {
    pub id: String,
    pub workspace_id: String,
    pub query_json: Value,
    pub sort: String,
    pub filters: Value,
    pub owner_user_id: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct NoteEvent {
    pub event_seq: u64,
    pub version: u64,
    pub event_type: String,
    pub payload: Value,
}

#[derive(Debug, Clone, Serialize)]
pub struct NoteRecord {
    pub id: String,
    pub workspace_id: String,
    pub project_id: Option<String>,
    pub title: String,
    pub note_kind: String,
    pub access_scope: String,
    pub markdown: String,
    pub current_version: u64,
    pub deleted: bool,
    pub metadata_json: HashMap<String, Value>,
    pub tags: Vec<String>,
    pub history: Vec<NoteEvent>,
    pub idempotency: HashMap<String, (u64, u64)>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutomationRuleRecord {
    pub id: String,
    pub workspace_id: String,
    pub trigger: String,
    pub condition_json: Value,
    pub action_json: Value,
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutomationRunRecord {
    pub id: String,
    pub rule_id: String,
    pub status: String,
    pub started_at: String,
    pub finished_at: Option<String>,
    pub result_json: Value,
}

#[derive(Debug, Default)]
pub struct Store {
    pub users: HashMap<String, UserRecord>,
    pub sessions: HashMap<String, SessionRecord>,
    pub workspaces: HashMap<String, WorkspaceRecord>,
    pub workspace_members: HashMap<String, HashMap<String, Role>>,
    pub projects: HashMap<String, ProjectRecord>,
    pub views: HashMap<String, SavedViewRecord>,
    pub notes: HashMap<String, NoteRecord>,
    pub rules: HashMap<String, AutomationRuleRecord>,
    pub runs: HashMap<String, AutomationRunRecord>,
    pub stream_cursors: HashMap<String, u64>,
    pub stream_events: HashMap<String, Vec<Value>>,
}

impl Store {
    pub fn next_id() -> String {
        Uuid::now_v7().to_string()
    }

    pub fn owner_exists(&self) -> bool {
        self.users.values().any(|user| user.role == Role::Owner)
    }

    pub fn next_stream_seq(&mut self, stream_id: &str) -> u64 {
        let cursor = self.stream_cursors.entry(stream_id.to_string()).or_insert(0);
        *cursor += 1;
        *cursor
    }

    pub fn append_stream_event(&mut self, stream_id: &str, payload: Value) {
        self.stream_events
            .entry(stream_id.to_string())
            .or_default()
            .push(payload);
    }

    pub fn replay_after(&self, stream_id: &str, cursor: u64) -> Vec<Value> {
        self.stream_events
            .get(stream_id)
            .cloned()
            .unwrap_or_default()
            .into_iter()
            .filter(|event| {
                event
                    .get("event_seq")
                    .and_then(Value::as_u64)
                    .unwrap_or_default()
                    > cursor
            })
            .collect()
    }
}
