// LLM provider adapter per /docs/spec/technical/librarian-agent.md
// Supports openrouter and lmstudio through OpenAI-compatible APIs.
// Deterministic failure categories: auth_failed, rate_limited, timeout,
// unreachable, invalid_payload.

use serde::{Deserialize, Serialize};

/// Provider configuration per spec.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderConfig {
    pub provider_kind: ProviderKind,
    pub base_url: String,
    pub model: String,
    pub timeout_ms: u64,
    pub max_tokens: u32,
    pub temperature: f64,
    pub fallback_models: Vec<String>,
    /// Bearer token for openrouter; ignored for lmstudio.
    pub api_key: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProviderKind {
    Openrouter,
    Lmstudio,
}

/// OpenAI-compatible chat completion request.
#[derive(Debug, Serialize)]
pub struct ChatRequest {
    pub model: String,
    pub messages: Vec<ChatMessage>,
    pub max_tokens: u32,
    pub temperature: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
}

/// OpenAI-compatible chat completion response.
#[derive(Debug, Deserialize)]
pub struct ChatResponse {
    pub choices: Vec<ChatChoice>,
}

#[derive(Debug, Deserialize)]
pub struct ChatChoice {
    pub message: ChatMessage,
}

impl ProviderConfig {
    /// Default config for lmstudio local mode.
    pub fn lmstudio_default(model: &str) -> Self {
        Self {
            provider_kind: ProviderKind::Lmstudio,
            base_url: "http://127.0.0.1:1234/v1".into(),
            model: model.into(),
            timeout_ms: 30_000,
            max_tokens: 2048,
            temperature: 0.1,
            fallback_models: vec![],
            api_key: None,
        }
    }

    /// Build the chat completions URL.
    pub fn completions_url(&self) -> String {
        let base = self.base_url.trim_end_matches('/');
        format!("{base}/chat/completions")
    }

    /// Build authorization header value if needed.
    pub fn auth_header(&self) -> Option<String> {
        match self.provider_kind {
            ProviderKind::Openrouter => {
                self.api_key.as_ref().map(|k| format!("Bearer {k}"))
            }
            ProviderKind::Lmstudio => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lmstudio_url() {
        let cfg = ProviderConfig::lmstudio_default("test-model");
        assert_eq!(
            cfg.completions_url(),
            "http://127.0.0.1:1234/v1/chat/completions"
        );
        assert!(cfg.auth_header().is_none());
    }

    #[test]
    fn openrouter_auth() {
        let cfg = ProviderConfig {
            provider_kind: ProviderKind::Openrouter,
            base_url: "https://openrouter.ai/api/v1".into(),
            model: "gpt-4".into(),
            timeout_ms: 60_000,
            max_tokens: 4096,
            temperature: 0.2,
            fallback_models: vec!["gpt-3.5-turbo".into()],
            api_key: Some("sk-test".into()),
        };
        assert_eq!(cfg.auth_header(), Some("Bearer sk-test".into()));
    }
}
