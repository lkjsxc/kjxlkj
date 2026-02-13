use kjxlkj_domain::errors::DomainError;
use serde::{Deserialize, Serialize};

/// Provider configuration for LLM calls.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderConfig {
    pub provider_kind: String, // "openrouter" or "lmstudio"
    pub base_url: String,
    pub model: String,
    pub api_key: Option<String>,
    pub timeout_ms: u64,
    pub max_tokens: u32,
    pub temperature: f32,
}

/// A parsed librarian operation from the XML-like response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LibrarianOperation {
    pub operation_id: String,
    pub kind: String,
    pub target_note_id: Option<String>,
    pub target_path: Option<String>,
    pub title: String,
    pub body_markdown: String,
    pub reason: String,
    pub confidence: f64,
}

/// Parse a librarian XML-like response into operations.
pub fn parse_response(raw: &str) -> Result<Vec<LibrarianOperation>, DomainError> {
    let normalized = raw.replace("\r\n", "\n");
    let ops_start = normalized
        .find("<operations>")
        .ok_or(DomainError::Internal("missing <operations> tag".into()))?;
    let ops_end = normalized
        .find("</operations>")
        .ok_or(DomainError::Internal("missing </operations> tag".into()))?;
    let ops_block = &normalized[ops_start..ops_end];
    let mut operations = Vec::new();
    let mut remaining = ops_block;
    while let Some(start) = remaining.find("<operation>") {
        let after = &remaining[start..];
        let end = after
            .find("</operation>")
            .ok_or(DomainError::Internal("unclosed <operation> tag".into()))?;
        let block = &after[..end + 12];
        let op = parse_single_operation(block)?;
        operations.push(op);
        remaining = &after[end + 12..];
    }
    Ok(operations)
}

fn parse_single_operation(block: &str) -> Result<LibrarianOperation, DomainError> {
    Ok(LibrarianOperation {
        operation_id: extract_tag(block, "operation_id")?,
        kind: extract_tag(block, "kind")?,
        target_note_id: extract_tag_opt(block, "target_note_id"),
        target_path: extract_tag_opt(block, "target_path"),
        title: extract_tag(block, "title").unwrap_or_default(),
        body_markdown: extract_tag(block, "body_markdown").unwrap_or_default(),
        reason: extract_tag(block, "reason").unwrap_or_default(),
        confidence: extract_tag(block, "confidence")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(0.0),
    })
}

fn extract_tag(block: &str, tag: &str) -> Result<String, DomainError> {
    let open = format!("<{tag}>");
    let close = format!("</{tag}>");
    let start = block.find(&open).ok_or(DomainError::Internal(
        format!("missing <{tag}> tag"),
    ))?;
    let after = &block[start + open.len()..];
    let end = after.find(&close).ok_or(DomainError::Internal(
        format!("missing </{tag}> tag"),
    ))?;
    Ok(after[..end].trim().to_string())
}

fn extract_tag_opt(block: &str, tag: &str) -> Option<String> {
    extract_tag(block, tag).ok()
}

/// Validate provider kind is supported (openrouter or lmstudio).
pub fn validate_provider(kind: &str) -> Result<(), DomainError> {
    match kind {
        "openrouter" | "lmstudio" => Ok(()),
        _ => Err(DomainError::RuleInvalid {
            reason: format!("unknown provider: {kind}"),
        }),
    }
}

/// Call LLM provider with the given prompt, returning raw response text.
pub async fn call_provider(
    config: &ProviderConfig,
    prompt: &str,
) -> Result<String, DomainError> {
    validate_provider(&config.provider_kind)?;
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_millis(config.timeout_ms))
        .build()
        .map_err(|e| DomainError::Internal(e.to_string()))?;
    let mut req = client
        .post(format!("{}/chat/completions", config.base_url))
        .json(&serde_json::json!({
            "model": config.model,
            "messages": [{"role": "user", "content": prompt}],
            "max_tokens": config.max_tokens,
            "temperature": config.temperature,
        }));
    if let Some(ref key) = config.api_key {
        req = req.bearer_auth(key);
    }
    let resp = req
        .send()
        .await
        .map_err(|e| DomainError::Internal(format!("provider call failed: {e}")))?;
    if !resp.status().is_success() {
        return Err(DomainError::Internal(format!(
            "provider returned status {}",
            resp.status()
        )));
    }
    let body: serde_json::Value = resp
        .json()
        .await
        .map_err(|e| DomainError::Internal(e.to_string()))?;
    body["choices"][0]["message"]["content"]
        .as_str()
        .map(|s| s.to_string())
        .ok_or(DomainError::Internal(
            "provider response missing content".into(),
        ))
}
