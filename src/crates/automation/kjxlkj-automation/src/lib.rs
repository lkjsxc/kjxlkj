//! Automation and librarian services for kjxlkj.
//!
//! This crate contains rule execution, provider adapters, and XML parsing.

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use thiserror::Error;

use kjxlkj_domain::{
    AutomationRule, AutomationRun, AutomationAction, ProviderMode,
    LibrarianOperation, OperationType, OperationDecision, ProviderError, ProviderErrorType,
};

/// Automation errors.
#[derive(Debug, Error)]
pub enum AutomationError {
    #[error("provider error: {0}")]
    ProviderError(String),
    #[error("parse error: {0}")]
    ParseError(String),
    #[error("invalid provider mode: {0}")]
    InvalidProviderMode(String),
    #[error("timeout")]
    Timeout,
    #[error("rate limited")]
    RateLimited,
}

/// Provider adapter configuration.
#[derive(Debug, Clone)]
pub struct ProviderConfig {
    pub mode: ProviderMode,
    pub api_key: Option<String>,
    pub base_url: Option<String>,
    pub model: String,
    pub timeout_seconds: u64,
    pub max_retries: u32,
}

impl Default for ProviderConfig {
    fn default() -> Self {
        Self {
            mode: ProviderMode::OpenRouter,
            api_key: None,
            base_url: None,
            model: "default".to_string(),
            timeout_seconds: 60,
            max_retries: 3,
        }
    }
}

/// Provider adapter for LLM calls.
pub struct ProviderAdapter {
    config: ProviderConfig,
    client: reqwest::Client,
}

impl ProviderAdapter {
    pub fn new(config: ProviderConfig) -> Self {
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(config.timeout_seconds))
            .build()
            .unwrap();
        Self { config, client }
    }

    /// Get the base URL for the provider.
    pub fn base_url(&self) -> &str {
        match &self.config.base_url {
            Some(url) => url,
            None => match self.config.mode {
                ProviderMode::OpenRouter => "https://openrouter.ai/api/v1",
                ProviderMode::LmStudio => "http://localhost:1234/v1",
            },
        }
    }

    /// Validate provider mode.
    pub fn validate_mode(mode: &str) -> Result<ProviderMode, AutomationError> {
        match mode.to_lowercase().as_str() {
            "openrouter" => Ok(ProviderMode::OpenRouter),
            "lmstudio" => Ok(ProviderMode::LmStudio),
            _ => Err(AutomationError::InvalidProviderMode(mode.to_string())),
        }
    }

    /// Send a prompt to the provider.
    pub async fn send_prompt(&self, prompt: &str) -> Result<String, AutomationError> {
        // Placeholder implementation - in real code this would make HTTP requests
        Ok(format!("Response to: {}", prompt))
    }
}

/// Attribute-less XML parser for librarian protocol.
pub struct XmlAttrlessParser;

impl XmlAttrlessParser {
    /// Parse operations from model output.
    pub fn parse(content: &str) -> Result<Vec<LibrarianOperation>, AutomationError> {
        let mut operations = Vec::new();
        let mut current_op: Option<LibrarianOperation> = None;
        let mut current_content = String::new();

        for line in content.lines() {
            let trimmed = line.trim();

            if trimmed.starts_with("<create>") {
                current_op = Some(LibrarianOperation {
                    op_type: OperationType::Create,
                    target_note_id: None,
                    title: None,
                    content: None,
                    decision: OperationDecision::Pending,
                });
                current_content.clear();
            } else if trimmed.starts_with("<rewrite>") {
                current_op = Some(LibrarianOperation {
                    op_type: OperationType::Rewrite,
                    target_note_id: None,
                    title: None,
                    content: None,
                    decision: OperationDecision::Pending,
                });
                current_content.clear();
            } else if trimmed.starts_with("<retitle>") {
                current_op = Some(LibrarianOperation {
                    op_type: OperationType::Retitle,
                    target_note_id: None,
                    title: None,
                    content: None,
                    decision: OperationDecision::Pending,
                });
                current_content.clear();
            } else if trimmed.starts_with("<relink>") {
                current_op = Some(LibrarianOperation {
                    op_type: OperationType::Relink,
                    target_note_id: None,
                    title: None,
                    content: None,
                    decision: OperationDecision::Pending,
                });
                current_content.clear();
            } else if trimmed.starts_with("</create>")
                || trimmed.starts_with("</rewrite>")
                || trimmed.starts_with("</retitle>")
                || trimmed.starts_with("</relink>")
            {
                if let Some(mut op) = current_op.take() {
                    op.content = Some(current_content.clone());
                    operations.push(op);
                }
                current_content.clear();
            } else if trimmed.starts_with("<title>") {
                // Extract title
                if let Some(end) = trimmed.find("</title>") {
                    let title = trimmed[7..end].to_string();
                    if let Some(op) = &mut current_op {
                        op.title = Some(title);
                    }
                }
            } else if current_op.is_some() {
                if !current_content.is_empty() {
                    current_content.push('\n');
                }
                current_content.push_str(trimmed);
            }
        }

        Ok(operations)
    }

    /// Validate required tags are present.
    pub fn validate_required_tags(content: &str) -> Result<(), AutomationError> {
        let required_tags = ["<operations>", "</operations>"];
        for tag in required_tags {
            if !content.contains(tag) {
                return Err(AutomationError::ParseError(format!(
                    "missing required tag: {}",
                    tag
                )));
            }
        }
        Ok(())
    }
}

/// Run executor for automation.
pub struct RunExecutor {
    provider: ProviderAdapter,
}

impl RunExecutor {
    pub fn new(provider: ProviderAdapter) -> Self {
        Self { provider }
    }

    /// Execute an automation run.
    pub async fn execute(&self, rule: &AutomationRule, run: &mut AutomationRun) -> Result<(), AutomationError> {
        run.start();

        // Get prompt template from action
        let prompt = match &rule.action {
            AutomationAction::LibrarianStructure { prompt_template, .. } => prompt_template.clone(),
            _ => return Err(AutomationError::ProviderError("unsupported action type".to_string())),
        };

        // Send prompt to provider
        let response = self.provider.send_prompt(&prompt).await?;
        run.raw_model_output = Some(response.clone());

        // Parse operations
        let operations = XmlAttrlessParser::parse(&response)?;
        run.operations = operations;

        run.succeed();
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_mode() {
        assert!(matches!(ProviderAdapter::validate_mode("openrouter"), Ok(ProviderMode::OpenRouter)));
        assert!(matches!(ProviderAdapter::validate_mode("lmstudio"), Ok(ProviderMode::LmStudio)));
        assert!(ProviderAdapter::validate_mode("unknown").is_err());
    }

    #[test]
    fn test_parse_operations() {
        let content = r#"
<operations>
<create>
<title>New Note</title>
This is the content of the new note.
</create>
<retitle>
<title>Updated Title</title>
</retitle>
</operations>
"#;
        let operations = XmlAttrlessParser::parse(content).unwrap();
        assert_eq!(operations.len(), 2);
        assert_eq!(operations[0].op_type, OperationType::Create);
        assert_eq!(operations[0].title, Some("New Note".to_string()));
    }
}
