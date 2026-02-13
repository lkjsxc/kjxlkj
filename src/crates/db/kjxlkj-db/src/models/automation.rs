use kjxlkj_domain::types::RunStatus;
use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Clone, sqlx::FromRow, Serialize)]
pub struct AutomationRuleRow {
    pub id: Uuid,
    pub workspace_id: Uuid,
    pub name: String,
    pub trigger: String,
    pub condition_json: serde_json::Value,
    pub action_json: serde_json::Value,
    pub enabled: bool,
    pub created_at: time::OffsetDateTime,
    pub updated_at: time::OffsetDateTime,
}

#[derive(Debug, Clone, sqlx::FromRow, Serialize)]
pub struct AutomationRunRow {
    pub id: Uuid,
    pub rule_id: Uuid,
    pub workspace_id: Uuid,
    pub status: RunStatus,
    pub trigger_event_id: Option<Uuid>,
    pub result_json: Option<serde_json::Value>,
    pub error_message: Option<String>,
    pub raw_prompt: Option<String>,
    pub raw_response: Option<String>,
    pub created_at: time::OffsetDateTime,
    pub updated_at: time::OffsetDateTime,
}
