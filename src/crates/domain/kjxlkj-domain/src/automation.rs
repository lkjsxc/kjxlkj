//! Automation and agent entities

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Automation rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutomationRule {
    pub rule_id: Uuid,
    pub workspace_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub trigger: RuleTrigger,
    pub action: RuleAction,
    pub enabled: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Rule trigger types
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum RuleTrigger {
    Manual,
    Scheduled { cron: String },
    OnNoteCreated { pattern: Option<String> },
    OnNoteUpdated { pattern: Option<String> },
}

/// Rule action types
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum RuleAction {
    KjxlkjAgent {
        prompt_path: String,
        mode: AgentMode,
    },
    CreateNote {
        template: String,
    },
    TagNote {
        tags: Vec<String>,
    },
}

/// Agent operation mode
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AgentMode {
    Reviewed,
    Yolo,
}

/// Agent run status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RunStatus {
    Pending,
    Running,
    Completed,
    Failed,
    Cancelled,
}

/// Agent run record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentRun {
    pub run_id: Uuid,
    pub rule_id: Uuid,
    pub workspace_id: Uuid,
    pub status: RunStatus,
    pub prompt_hash: String,
    pub loop_count: u32,
    pub operation_count: u32,
    pub error_code: Option<String>,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
}

impl AgentRun {
    pub fn new(rule_id: Uuid, workspace_id: Uuid, prompt_hash: String) -> Self {
        Self {
            run_id: Uuid::new_v4(),
            rule_id,
            workspace_id,
            status: RunStatus::Pending,
            prompt_hash,
            loop_count: 0,
            operation_count: 0,
            error_code: None,
            started_at: None,
            completed_at: None,
        }
    }
}
