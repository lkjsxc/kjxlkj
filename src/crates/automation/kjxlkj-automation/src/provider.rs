/// LLM provider adapter trait and implementations.
///
/// Spec: /docs/spec/technical/librarian-agent.md
/// Providers: OpenRouter and LMStudio per /docs/spec/domain/automation.md
use kjxlkj_domain::DomainError;

/// LLM completion request.
#[derive(Debug, Clone)]
pub struct CompletionRequest {
    pub model: String,
    pub messages: Vec<ChatMessage>,
    pub max_tokens: u32,
    pub temperature: f64,
}

/// Chat message for LLM.
#[derive(Debug, Clone)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
}

/// LLM completion response.
#[derive(Debug, Clone)]
pub struct CompletionResponse {
    pub content: String,
    pub model: String,
    pub usage_tokens: u32,
}

/// Provider trait for LLM completions.
pub trait LlmProvider: Send + Sync {
    fn complete(&self, req: &CompletionRequest) -> Result<CompletionResponse, DomainError>;
    fn provider_name(&self) -> &'static str;
}

/// OpenRouter LLM provider per /docs/spec/technical/librarian-agent.md
pub struct OpenRouterProvider {
    pub base_url: String,
    pub api_key: String,
    pub default_model: String,
}

impl OpenRouterProvider {
    pub fn new(api_key: String, model: String) -> Self {
        Self {
            base_url: "https://openrouter.ai/api/v1".into(),
            api_key,
            default_model: model,
        }
    }
}

impl LlmProvider for OpenRouterProvider {
    fn complete(&self, req: &CompletionRequest) -> Result<CompletionResponse, DomainError> {
        // Stub: actual HTTP call to OpenRouter API would go here.
        // For now, return a simulated response for testing.
        let _ = req;
        Err(DomainError::LlmUpstreamError(
            "OpenRouter: not connected (stub)".into(),
        ))
    }
    fn provider_name(&self) -> &'static str { "openrouter" }
}

/// LM Studio local provider per /docs/spec/technical/librarian-agent.md
pub struct LmStudioProvider {
    pub base_url: String,
    pub default_model: String,
}

impl LmStudioProvider {
    pub fn new(base_url: String, model: String) -> Self {
        Self { base_url, default_model: model }
    }
}

impl LlmProvider for LmStudioProvider {
    fn complete(&self, req: &CompletionRequest) -> Result<CompletionResponse, DomainError> {
        let _ = req;
        Err(DomainError::LlmUpstreamError(
            "LMStudio: not connected (stub)".into(),
        ))
    }
    fn provider_name(&self) -> &'static str { "lmstudio" }
}

/// Select provider by name from config.
pub fn create_provider(kind: &str, base_url: &str, model: &str) -> Box<dyn LlmProvider> {
    match kind {
        "openrouter" => {
            let api_key = std::env::var("OPENROUTER_API_KEY").unwrap_or_default();
            Box::new(OpenRouterProvider::new(api_key, model.to_string()))
        }
        "lmstudio" => Box::new(LmStudioProvider::new(base_url.to_string(), model.to_string())),
        _ => Box::new(LmStudioProvider::new(base_url.to_string(), model.to_string())),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_openrouter_stub_returns_error() {
        let provider = OpenRouterProvider::new("key".into(), "model".into());
        let req = CompletionRequest {
            model: "test".into(),
            messages: vec![ChatMessage { role: "user".into(), content: "hi".into() }],
            max_tokens: 100,
            temperature: 0.7,
        };
        assert!(provider.complete(&req).is_err());
        assert_eq!(provider.provider_name(), "openrouter");
    }

    #[test]
    fn test_lmstudio_stub_returns_error() {
        let provider = LmStudioProvider::new("http://localhost:1234".into(), "m".into());
        let req = CompletionRequest {
            model: "test".into(),
            messages: vec![],
            max_tokens: 50,
            temperature: 0.5,
        };
        assert!(provider.complete(&req).is_err());
        assert_eq!(provider.provider_name(), "lmstudio");
    }

    #[test]
    fn test_create_provider_factory() {
        let p = create_provider("openrouter", "", "gpt-4");
        assert_eq!(p.provider_name(), "openrouter");
        let p2 = create_provider("lmstudio", "http://localhost:1234", "local");
        assert_eq!(p2.provider_name(), "lmstudio");
    }
}
