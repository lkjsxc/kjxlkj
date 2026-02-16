/// Prompt loader per /docs/spec/technical/agent-prompt-json.md
use kjxlkj_domain::automation::{AgentPromptJson, AgentPromptConfigRef};
use kjxlkj_domain::DomainError;
use sha2::{Digest, Sha256};

/// Load and validate prompt JSON from file.
pub fn load_prompt(path: &str) -> Result<(AgentPromptJson, AgentPromptConfigRef), DomainError> {
    let content = std::fs::read_to_string(path)
        .map_err(|e| DomainError::PromptJsonInvalid(format!("read: {e}")))?;
    let prompt: AgentPromptJson = serde_json::from_str(&content)
        .map_err(|e| DomainError::PromptSchemaInvalid(format!("parse: {e}")))?;
    prompt
        .validate()
        .map_err(|e| DomainError::PromptSchemaInvalid(e))?;
    let hash = format!("{:x}", Sha256::digest(content.as_bytes()));
    let config_ref = AgentPromptConfigRef {
        path: path.to_string(),
        hash,
        loaded_at: chrono::Utc::now().naive_utc(),
    };
    Ok((prompt, config_ref))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn agent_01_prompt_load_validate() {
        // Acceptance: AGENT-01
        // Use CARGO_MANIFEST_DIR to find workspace root
        let manifest_dir = env!("CARGO_MANIFEST_DIR");
        let workspace_root = std::path::Path::new(manifest_dir)
            .ancestors()
            .nth(4)
            .expect("workspace root");
        let path = workspace_root.join("data/agent-prompt.json");
        let (prompt, config_ref) = load_prompt(path.to_str().unwrap())
            .expect("prompt should load");
        assert_eq!(prompt.agent_name, "kjxlkj-agent");
        assert!(!config_ref.hash.is_empty());
    }

    #[test]
    fn test_invalid_prompt_fails() {
        let result = load_prompt("/nonexistent/prompt.json");
        assert!(result.is_err());
    }
}
