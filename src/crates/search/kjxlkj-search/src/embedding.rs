//! Embedding provider abstraction

use serde::{Deserialize, Serialize};

/// Embedding provider configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmbeddingConfig {
    pub provider: String,
    pub base_url: String,
    pub model: String,
    pub dimensions: usize,
    pub timeout_ms: u64,
}

impl Default for EmbeddingConfig {
    fn default() -> Self {
        Self {
            provider: "lmstudio".to_string(),
            base_url: "http://127.0.0.1:1234/v1".to_string(),
            model: "text-embedding-nomic-embed-text-v1.5".to_string(),
            dimensions: 768,
            timeout_ms: 30000,
        }
    }
}

/// Embedding provider trait
#[async_trait::async_trait]
pub trait EmbeddingProvider: Send + Sync {
    async fn embed(&self, text: &str) -> Result<Vec<f64>, EmbeddingError>;
    fn provider_name(&self) -> &str;
    fn is_available(&self) -> bool;
}

/// Embedding error
#[derive(Debug, thiserror::Error)]
pub enum EmbeddingError {
    #[error("Provider unavailable: {0}")]
    ProviderUnavailable(String),

    #[error("Timeout: {0}")]
    Timeout(String),

    #[error("Invalid response: {0}")]
    InvalidResponse(String),

    #[error("Dimension mismatch: expected {expected}, got {actual}")]
    DimensionMismatch { expected: usize, actual: usize },
}

/// LMStudio embedding provider
pub struct LmStudioProvider {
    config: EmbeddingConfig,
}

impl LmStudioProvider {
    pub fn new(config: EmbeddingConfig) -> Self {
        Self { config }
    }
}

#[async_trait::async_trait]
impl EmbeddingProvider for LmStudioProvider {
    async fn embed(&self, _text: &str) -> Result<Vec<f64>, EmbeddingError> {
        // Stub implementation
        Ok(vec![0.0; self.config.dimensions])
    }

    fn provider_name(&self) -> &str {
        &self.config.provider
    }

    fn is_available(&self) -> bool {
        // In production, health check via HTTP
        true
    }
}
