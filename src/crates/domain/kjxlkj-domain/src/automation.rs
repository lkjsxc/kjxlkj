use serde::{Deserialize, Serialize};
use uuid::Uuid;
use time::OffsetDateTime;

/// Automation rule per docs/spec/domain/automation.md.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutomationRule {
    pub id: Uuid,
    pub workspace_id: Uuid,
    pub trigger: String,
    pub condition_json: serde_json::Value,
    pub action_json: serde_json::Value,
    pub enabled: bool,
    #[serde(with = "time::serde::rfc3339")]
    pub created_at: OffsetDateTime,
}

/// Run status per docs/spec/domain/automation.md.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RunStatus {
    Queued,
    Running,
    Succeeded,
    Failed,
}

/// Automation run per docs/spec/api/types.md.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutomationRun {
    pub id: Uuid,
    pub rule_id: Uuid,
    pub status: RunStatus,
    #[serde(with = "time::serde::rfc3339")]
    pub started_at: OffsetDateTime,
    pub finished_at: Option<OffsetDateTime>,
    pub result_json: Option<serde_json::Value>,
}
