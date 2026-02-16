use serde::{Deserialize, Serialize};
use std::path::Path;
use tracing::info;

/// Agent prompt configuration per docs/spec/technical/agent-prompt-json.md.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentPrompt {
    pub agent_name: String,
    pub version: String,
    pub default_mode: String,
    #[serde(default)]
    pub protocol: Option<String>,
    #[serde(default)]
    pub allowed_tags: Vec<String>,
    pub segments: Vec<PromptSegment>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromptSegment {
    pub condition: String,
    pub prompt: String,
}

/// Load and validate agent prompt from a JSON file.
/// Hard-fail on invalid JSON or missing required fields.
pub fn load_prompt(path: &Path) -> Result<AgentPrompt, PromptLoadError> {
    let data = std::fs::read_to_string(path)
        .map_err(|e| PromptLoadError::Io(format!("{path:?}: {e}")))?;

    let prompt: AgentPrompt = serde_json::from_str(&data)
        .map_err(|e| PromptLoadError::Schema(format!("invalid prompt JSON: {e}")))?;

    // Validate required fields
    if prompt.agent_name != "kjxlkj-agent" {
        return Err(PromptLoadError::Schema(
            "agent_name must be 'kjxlkj-agent'".into(),
        ));
    }
    if prompt.segments.is_empty() {
        return Err(PromptLoadError::Schema(
            "segments must not be empty".into(),
        ));
    }
    for seg in &prompt.segments {
        if seg.prompt.is_empty() {
            return Err(PromptLoadError::Schema(
                "segment prompt must not be empty".into(),
            ));
        }
    }

    info!(
        "loaded agent prompt: name={}, version={}, segments={}",
        prompt.agent_name,
        prompt.version,
        prompt.segments.len(),
    );
    Ok(prompt)
}

/// Compute SHA-256 hash of prompt file for audit.
pub fn prompt_hash(path: &Path) -> Result<String, std::io::Error> {
    use sha2::{Digest, Sha256};
    let data = std::fs::read(path)?;
    let hash = Sha256::digest(&data);
    Ok(hex::encode(hash))
}

#[derive(Debug, thiserror::Error)]
pub enum PromptLoadError {
    #[error("io: {0}")]
    Io(String),
    #[error("schema: {0}")]
    Schema(String),
}
