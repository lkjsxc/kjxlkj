//! Automation domain types for rules, runs, and librarian operations.

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use time::OffsetDateTime;

use crate::types::{ProviderMode, RuleState, RunState};

/// Automation rule entity.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutomationRule {
    pub id: Uuid,
    pub workspace_id: Uuid,
    pub name: String,
    pub trigger: AutomationTrigger,
    pub condition: Option<AutomationCondition>,
    pub action: AutomationAction,
    pub state: RuleState,
    pub created_by: Uuid,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

impl AutomationRule {
    /// Create a new automation rule.
    pub fn new(workspace_id: Uuid, name: String, trigger: AutomationTrigger, action: AutomationAction, created_by: Uuid) -> Self {
        let now = OffsetDateTime::now_utc();
        Self {
            id: Uuid::new_v4(),
            workspace_id,
            name,
            trigger,
            condition: None,
            action,
            state: RuleState::Enabled,
            created_by,
            created_at: now,
            updated_at: now,
        }
    }

    /// Check if rule is enabled.
    pub fn is_enabled(&self) -> bool {
        self.state == RuleState::Enabled
    }
}

/// Automation trigger types.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum AutomationTrigger {
    NoteCreated { workspace_id: Uuid },
    NoteUpdated { workspace_id: Uuid },
    Manual,
    Scheduled { cron: String },
}

/// Automation condition.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutomationCondition {
    pub expression: String,
}

/// Automation action types.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum AutomationAction {
    LibrarianStructure {
        provider: ProviderMode,
        model: String,
        prompt_template: String,
        strict_mode: bool,
    },
    Webhook {
        url: String,
        method: String,
        headers: Option<serde_json::Value>,
    },
}

/// Automation run entity.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutomationRun {
    pub id: Uuid,
    pub rule_id: Uuid,
    pub workspace_id: Uuid,
    pub trigger_event: Option<serde_json::Value>,
    pub state: RunState,
    pub operations: Vec<LibrarianOperation>,
    pub raw_model_output: Option<String>,
    pub parse_diagnostics: Option<String>,
    pub provider_metadata: Option<serde_json::Value>,
    pub started_at: Option<OffsetDateTime>,
    pub completed_at: Option<OffsetDateTime>,
    pub created_at: OffsetDateTime,
}

impl AutomationRun {
    /// Create a new automation run.
    pub fn new(rule_id: Uuid, workspace_id: Uuid, trigger_event: Option<serde_json::Value>) -> Self {
        Self {
            id: Uuid::new_v4(),
            rule_id,
            workspace_id,
            trigger_event,
            state: RunState::Queued,
            operations: Vec::new(),
            raw_model_output: None,
            parse_diagnostics: None,
            provider_metadata: None,
            started_at: None,
            completed_at: None,
            created_at: OffsetDateTime::now_utc(),
        }
    }

    /// Mark run as running.
    pub fn start(&mut self) {
        self.state = RunState::Running;
        self.started_at = Some(OffsetDateTime::now_utc());
    }

    /// Mark run as succeeded.
    pub fn succeed(&mut self) {
        self.state = RunState::Succeeded;
        self.completed_at = Some(OffsetDateTime::now_utc());
    }

    /// Mark run as failed.
    pub fn fail(&mut self, diagnostics: String) {
        self.state = RunState::Failed;
        self.parse_diagnostics = Some(diagnostics);
        self.completed_at = Some(OffsetDateTime::now_utc());
    }
}

/// Librarian operation from model output.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LibrarianOperation {
    pub op_type: OperationType,
    pub target_note_id: Option<Uuid>,
    pub title: Option<String>,
    pub content: Option<String>,
    pub decision: OperationDecision,
}

/// Operation types for librarian actions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OperationType {
    Create,
    Rewrite,
    Retitle,
    Relink,
}

impl Default for OperationType {
    fn default() -> Self {
        Self::Create
    }
}

/// Decision state for operations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OperationDecision {
    Pending,
    Accepted,
    Rejected,
}

impl Default for OperationDecision {
    fn default() -> Self {
        Self::Pending
    }
}

/// Provider adapter error types.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderError {
    pub error_type: ProviderErrorType,
    pub message: String,
    pub is_retryable: bool,
}

/// Provider error types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ProviderErrorType {
    Timeout,
    RateLimited,
    UpstreamError,
    InvalidResponse,
    LibrarianProtocolInvalid,
    LibrarianParseFailed,
}

/// XML-like attribute-less protocol parsing result.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct XmlParseResult {
    pub operations: Vec<LibrarianOperation>,
    pub raw_output: String,
    pub parse_errors: Vec<String>,
}
