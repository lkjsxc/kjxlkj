/// Embedding provider trait and implementations for semantic search.
///
/// Spec: /docs/spec/domain/search.md
/// - Semantic retrieval via embedding vectors
/// - Embeddings MUST be regenerated whenever title or body changes
/// - If embedding service unavailable, lexical search MUST continue
/// - Semantic failure MUST set machine-readable diagnostics
///
/// Providers follow the OpenAI-compatible /v1/embeddings API shape
/// used by LMStudio, OpenRouter, and similar services.
use kjxlkj_domain::DomainError;

/// Embedding vector type.
pub type EmbeddingVec = Vec<f64>;

/// Embedding provider trait for semantic search.
pub trait EmbeddingProvider: Send + Sync {
    /// Embed a single text string into a vector.
    fn embed(&self, text: &str) -> Result<EmbeddingVec, DomainError>;
    /// Embed a batch of texts into vectors.
    fn embed_batch(&self, texts: &[String]) -> Result<Vec<EmbeddingVec>, DomainError>;
    /// Return the number of dimensions in the embedding vectors.
    fn dimensions(&self) -> u32;
    /// Provider name for diagnostics.
    fn provider_name(&self) -> &'static str;
    /// Whether the provider is currently available.
    fn is_available(&self) -> bool;
}

/// Stub provider that generates deterministic pseudo-embeddings for testing.
/// Uses a simple hash-based approach to produce consistent vectors.
pub struct StubEmbeddingProvider {
    dims: u32,
}

impl StubEmbeddingProvider {
    pub fn new(dims: u32) -> Self {
        Self { dims }
    }

    /// Generate a deterministic embedding from text using simple hashing.
    fn hash_embed(&self, text: &str) -> EmbeddingVec {
        let mut vec = vec![0.0f64; self.dims as usize];
        for (i, byte) in text.bytes().enumerate() {
            let idx = i % (self.dims as usize);
            vec[idx] += (byte as f64) / 255.0;
        }
        let norm: f64 = vec.iter().map(|v| v * v).sum::<f64>().sqrt();
        if norm > 0.0 {
            for v in &mut vec {
                *v /= norm;
            }
        }
        vec
    }
}

impl EmbeddingProvider for StubEmbeddingProvider {
    fn embed(&self, text: &str) -> Result<EmbeddingVec, DomainError> {
        Ok(self.hash_embed(text))
    }

    fn embed_batch(&self, texts: &[String]) -> Result<Vec<EmbeddingVec>, DomainError> {
        Ok(texts.iter().map(|t| self.hash_embed(t)).collect())
    }

    fn dimensions(&self) -> u32 {
        self.dims
    }

    fn provider_name(&self) -> &'static str {
        "stub"
    }

    fn is_available(&self) -> bool {
        true
    }
}

/// Null provider — always returns EmbeddingProviderError.
/// Used when semantic search is disabled in configuration.
pub struct NullEmbeddingProvider;

impl EmbeddingProvider for NullEmbeddingProvider {
    fn embed(&self, _text: &str) -> Result<EmbeddingVec, DomainError> {
        Err(DomainError::EmbeddingProviderError(
            "semantic search disabled".into(),
        ))
    }

    fn embed_batch(&self, _texts: &[String]) -> Result<Vec<EmbeddingVec>, DomainError> {
        Err(DomainError::EmbeddingProviderError(
            "semantic search disabled".into(),
        ))
    }

    fn dimensions(&self) -> u32 {
        0
    }

    fn provider_name(&self) -> &'static str {
        "null"
    }

    fn is_available(&self) -> bool {
        false
    }
}

/// HTTP-based embedding provider for OpenAI-compatible /v1/embeddings API.
/// Used with LMStudio, OpenRouter, and similar services.
///
/// API shape (POST /v1/embeddings):
/// ```json
/// { "model": "...", "input": ["text1", "text2"] }
/// ```
/// Response: `{ "data": [{ "embedding": [...] }] }`
///
/// Currently a configuration stub — actual HTTP calls require reqwest
/// or similar async HTTP client to be added as a dependency.
pub struct HttpEmbeddingProvider {
    pub base_url: String,
    pub model: String,
    pub dims: u32,
}

impl HttpEmbeddingProvider {
    pub fn new(base_url: String, model: String, dims: u32) -> Self {
        Self { base_url, model, dims }
    }
}

impl EmbeddingProvider for HttpEmbeddingProvider {
    fn embed(&self, _text: &str) -> Result<EmbeddingVec, DomainError> {
        Err(DomainError::EmbeddingProviderError(
            format!("HTTP embedding provider not connected ({})", self.base_url),
        ))
    }

    fn embed_batch(&self, _texts: &[String]) -> Result<Vec<EmbeddingVec>, DomainError> {
        Err(DomainError::EmbeddingProviderError(
            format!("HTTP embedding provider not connected ({})", self.base_url),
        ))
    }

    fn dimensions(&self) -> u32 {
        self.dims
    }

    fn provider_name(&self) -> &'static str {
        "http"
    }

    fn is_available(&self) -> bool {
        false
    }
}

/// Factory: create embedding provider from search config.
pub fn create_embedding_provider(
    semantic_enabled: bool,
    provider: &str,
    base_url: &str,
    model: &str,
    dims: u32,
) -> Box<dyn EmbeddingProvider> {
    if !semantic_enabled {
        return Box::new(NullEmbeddingProvider);
    }
    match provider {
        "stub" | "test" => Box::new(StubEmbeddingProvider::new(dims)),
        "lmstudio" | "openrouter" | "http" => {
            Box::new(HttpEmbeddingProvider::new(
                base_url.to_string(),
                model.to_string(),
                dims,
            ))
        }
        _ => Box::new(NullEmbeddingProvider),
    }
}
