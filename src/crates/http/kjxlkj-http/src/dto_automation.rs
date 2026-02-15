//! Automation and export DTOs.
//! Split from dto.rs per 200-line policy.
use serde::{Deserialize, Serialize};

/// Create automation rule request per /docs/spec/domain/automation.md.
#[derive(Debug, Deserialize)]
pub struct CreateRuleRequest {
    pub workspace_id: uuid::Uuid,
    pub name: String,
    pub trigger_type: String,
    pub condition_json: serde_json::Value,
    pub action_json: serde_json::Value,
}

/// Update automation rule request.
#[derive(Debug, Deserialize)]
pub struct UpdateRuleRequest {
    pub name: Option<String>,
    pub trigger_type: Option<String>,
    pub condition_json: Option<serde_json::Value>,
    pub action_json: Option<serde_json::Value>,
    pub enabled: Option<bool>,
}

/// Automation rule response.
#[derive(Debug, Serialize)]
pub struct AutomationRuleResponse {
    pub id: uuid::Uuid,
    pub workspace_id: uuid::Uuid,
    pub name: String,
    pub trigger_type: String,
    pub condition_json: serde_json::Value,
    pub action_json: serde_json::Value,
    pub enabled: bool,
}

/// Launch automation run request.
#[derive(Debug, Deserialize)]
pub struct LaunchRunRequest {
    pub triggering_event_id: Option<uuid::Uuid>,
}

/// Automation run response.
#[derive(Debug, Serialize)]
pub struct AutomationRunResponse {
    pub id: uuid::Uuid,
    pub rule_id: uuid::Uuid,
    pub status: String,
    pub result_json: serde_json::Value,
}

/// Review decision for librarian operations.
#[derive(Debug, Deserialize)]
pub struct ReviewDecision {
    pub operation_id: uuid::Uuid,
    pub decision: String,
    pub reject_reason: Option<String>,
}

/// Review request for automation run.
#[derive(Debug, Deserialize)]
pub struct ReviewRunRequest {
    pub decisions: Vec<ReviewDecision>,
}

/// Export job request.
#[derive(Debug, Deserialize)]
pub struct LaunchExportRequest {
    pub workspace_id: Option<uuid::Uuid>,
}

/// Export job response.
#[derive(Debug, Serialize)]
pub struct ExportJobResponse {
    pub id: uuid::Uuid,
    pub job_type: String,
    pub status: String,
    pub artifact_path: Option<String>,
}
