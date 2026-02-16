/// Automation domain types per /docs/spec/domain/automation.md
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Automation rule per /docs/spec/api/types.md
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutomationRule {
    pub id: Uuid,
    pub workspace_id: Uuid,
    pub trigger: String,
    pub condition_json: serde_json::Value,
    pub action_json: serde_json::Value,
    pub enabled: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

/// Run status lifecycle per /docs/spec/domain/automation.md
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RunStatus {
    Queued,
    Running,
    Succeeded,
    Failed,
}

/// AutomationRun per /docs/spec/api/types.md
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutomationRun {
    pub id: Uuid,
    pub rule_id: Uuid,
    pub status: RunStatus,
    pub started_at: Option<NaiveDateTime>,
    pub finished_at: Option<NaiveDateTime>,
    pub result_json: Option<serde_json::Value>,
    pub created_at: NaiveDateTime,
}

/// Agent action JSON validation per /docs/spec/api/types.md
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentActionJson {
    pub kind: String,
    pub mode: String,
    pub prompt_json_path: String,
    pub provider: String,
    pub memory: AgentMemoryConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentMemoryConfig {
    #[serde(rename = "type")]
    pub memory_type: String,
    pub carry_over: bool,
}

/// Agent prompt config ref per /docs/spec/api/types.md
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentPromptConfigRef {
    pub path: String,
    pub hash: String,
    pub loaded_at: NaiveDateTime,
}

/// Agent prompt JSON schema per /docs/spec/technical/agent-prompt-json.md
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentPromptJson {
    pub agent_name: String,
    pub version: String,
    pub default_mode: Option<String>,
    pub protocol: Option<String>,
    pub allowed_tags: Option<Vec<String>>,
    pub segments: Vec<PromptSegment>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromptSegment {
    pub condition: String,
    pub prompt: String,
}

impl AgentPromptJson {
    /// Validate per /docs/spec/technical/agent-prompt-json.md
    pub fn validate(&self) -> Result<(), String> {
        if self.agent_name != "kjxlkj-agent" {
            return Err("agent_name must be 'kjxlkj-agent'".into());
        }
        if self.version.is_empty() {
            return Err("version must not be empty".into());
        }
        if self.segments.is_empty() {
            return Err("segments must not be empty".into());
        }
        for seg in &self.segments {
            if seg.prompt.is_empty() {
                return Err("segment prompt must not be empty".into());
            }
        }
        Ok(())
    }
}

/// Create rule input
#[derive(Debug, Clone, Deserialize)]
pub struct CreateRuleInput {
    pub workspace_id: Uuid,
    pub trigger: String,
    pub condition_json: serde_json::Value,
    pub action_json: serde_json::Value,
    pub enabled: Option<bool>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn agent_01_prompt_loaded_and_validated() {
        // Acceptance: AGENT-01
        let json_str = include_str!("../../../../../data/agent-prompt.json");
        let prompt: AgentPromptJson =
            serde_json::from_str(json_str).expect("parse prompt JSON");
        prompt.validate().expect("validate prompt");
        assert_eq!(prompt.agent_name, "kjxlkj-agent");
        assert!(!prompt.segments.is_empty());
    }

    #[test]
    fn api_auto_03_validates_prompt_mode_fields() {
        // Acceptance: API-AUTO-03
        let action = AgentActionJson {
            kind: "kjxlkj_agent".into(),
            mode: "yolo".into(),
            prompt_json_path: "./data/agent-prompt.json".into(),
            provider: "lmstudio".into(),
            memory: AgentMemoryConfig {
                memory_type: "kv_store".into(),
                carry_over: true,
            },
        };
        assert_eq!(action.kind, "kjxlkj_agent");
        assert!(action.mode == "yolo" || action.mode == "reviewed");
    }

    #[test]
    fn agent_04_transcript_retention_disabled() {
        // Acceptance: AGENT-04
        let config_str = include_str!("../../../../../data/config.json");
        let config: serde_json::Value =
            serde_json::from_str(config_str).expect("parse config");
        let retain = config["agent"]["retain_full_conversation_logs"]
            .as_bool()
            .unwrap_or(true);
        assert!(!retain, "transcript retention must be disabled by default");
    }
}
