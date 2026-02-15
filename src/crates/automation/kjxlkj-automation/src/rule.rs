use serde::{Deserialize, Serialize};

/// Provider kind per /docs/spec/api/types.md.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ProviderKind {
    Openrouter,
    Lmstudio,
}

/// Validate that a provider kind string is known.
/// Per /docs/spec/api/http.md: unknown providers return 422.
pub fn validate_provider_kind(kind: &str) -> Result<ProviderKind, String> {
    match kind {
        "openrouter" => Ok(ProviderKind::Openrouter),
        "lmstudio" => Ok(ProviderKind::Lmstudio),
        other => Err(format!("unknown provider kind: {}", other)),
    }
}
