//! Provider adapter trait per /docs/spec/technical/librarian-agent.md.
//! Abstracts LLM communication for openrouter and lmstudio modes.

use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Provider failure categories per spec.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProviderFailure {
    AuthFailed,
    RateLimited,
    Timeout,
    Unreachable,
    InvalidPayload(String),
}

impl std::fmt::Display for ProviderFailure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AuthFailed => write!(f, "auth_failed"),
            Self::RateLimited => write!(f, "rate_limited"),
            Self::Timeout => write!(f, "timeout"),
            Self::Unreachable => write!(f, "unreachable"),
            Self::InvalidPayload(m) => write!(f, "invalid_payload: {m}"),
        }
    }
}

/// Provider configuration per /docs/spec/api/types.md.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderConfig {
    pub provider_kind: String,
    pub base_url: String,
    pub model: String,
    pub timeout_ms: u64,
    pub max_tokens: u32,
    pub temperature: f32,
    pub fallback_models: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_key: Option<String>,
}

/// Chat message for OpenAI-compatible API.
#[derive(Debug, Serialize, Deserialize)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
}

/// Chat completion request.
#[derive(Debug, Serialize)]
pub struct ChatRequest {
    pub model: String,
    pub messages: Vec<ChatMessage>,
    pub max_tokens: u32,
    pub temperature: f32,
}

/// Chat completion response (subset).
#[derive(Debug, Deserialize)]
pub struct ChatResponse {
    pub choices: Vec<ChatChoice>,
}

#[derive(Debug, Deserialize)]
pub struct ChatChoice {
    pub message: ChatMessage,
}

/// Send a completion request to an OpenAI-compatible provider.
pub async fn chat_completion(
    config: &ProviderConfig,
    system_prompt: &str,
    user_prompt: &str,
) -> Result<String, ProviderFailure> {
    let client = reqwest::Client::builder()
        .timeout(Duration::from_millis(config.timeout_ms))
        .build()
        .map_err(|e| ProviderFailure::InvalidPayload(e.to_string()))?;

    let url = format!("{}/chat/completions", config.base_url.trim_end_matches('/'));

    let body = ChatRequest {
        model: config.model.clone(),
        messages: vec![
            ChatMessage {
                role: "system".into(),
                content: system_prompt.into(),
            },
            ChatMessage {
                role: "user".into(),
                content: user_prompt.into(),
            },
        ],
        max_tokens: config.max_tokens,
        temperature: config.temperature,
    };

    let mut req = client.post(&url).json(&body);

    // OpenRouter requires bearer token auth.
    if let Some(ref key) = config.api_key {
        req = req.bearer_auth(key);
    }

    let resp = req.send().await.map_err(|e| {
        if e.is_timeout() {
            ProviderFailure::Timeout
        } else if e.is_connect() {
            ProviderFailure::Unreachable
        } else {
            ProviderFailure::InvalidPayload(e.to_string())
        }
    })?;

    let status = resp.status();
    if status == reqwest::StatusCode::UNAUTHORIZED
        || status == reqwest::StatusCode::FORBIDDEN
    {
        return Err(ProviderFailure::AuthFailed);
    }
    if status == reqwest::StatusCode::TOO_MANY_REQUESTS {
        return Err(ProviderFailure::RateLimited);
    }
    if !status.is_success() {
        let text = resp.text().await.unwrap_or_default();
        return Err(ProviderFailure::InvalidPayload(format!(
            "status {status}: {text}"
        )));
    }

    let chat_resp: ChatResponse = resp
        .json()
        .await
        .map_err(|e| ProviderFailure::InvalidPayload(e.to_string()))?;

    chat_resp
        .choices
        .first()
        .map(|c| c.message.content.clone())
        .ok_or(ProviderFailure::InvalidPayload(
            "no choices returned".into(),
        ))
}

/// Attempt completion with fallback model chain per spec.
pub async fn chat_with_fallback(
    config: &ProviderConfig,
    system_prompt: &str,
    user_prompt: &str,
) -> Result<String, ProviderFailure> {
    match chat_completion(config, system_prompt, user_prompt).await {
        Ok(text) => Ok(text),
        Err(primary_err) => {
            tracing::warn!(
                model = %config.model,
                error = %primary_err,
                "primary model failed, trying fallbacks"
            );
            for fallback in &config.fallback_models {
                let mut fb_config = config.clone();
                fb_config.model = fallback.clone();
                match chat_completion(&fb_config, system_prompt, user_prompt).await {
                    Ok(text) => return Ok(text),
                    Err(e) => {
                        tracing::warn!(model = %fallback, error = %e, "fallback failed");
                    }
                }
            }
            Err(primary_err)
        }
    }
}
