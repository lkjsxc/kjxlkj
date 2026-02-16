use serde::Serialize;

/// Stable error codes for machine-readable API responses.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum ErrorCode {
    BadRequest,
    InvalidPatch,
    AuthRequired,
    InvalidCredentials,
    RoleForbidden,
    WorkspaceForbidden,
    NoteNotFound,
    WorkspaceNotFound,
    VersionConflict,
    RuleInvalid,
    PromptJsonInvalid,
    PromptSchemaInvalid,
    SearchModeInvalid,
    AgentMemoryStoreError,
    AgentYoloPolicyViolation,
    SearchEmbeddingDegraded,
    RateLimited,
    LlmUpstreamError,
    EmbeddingProviderError,
    InternalError,
    SetupLocked,
}

impl ErrorCode {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::BadRequest => "BAD_REQUEST",
            Self::InvalidPatch => "INVALID_PATCH",
            Self::AuthRequired => "AUTH_REQUIRED",
            Self::InvalidCredentials => "INVALID_CREDENTIALS",
            Self::RoleForbidden => "ROLE_FORBIDDEN",
            Self::WorkspaceForbidden => "WORKSPACE_FORBIDDEN",
            Self::NoteNotFound => "NOTE_NOT_FOUND",
            Self::WorkspaceNotFound => "WORKSPACE_NOT_FOUND",
            Self::VersionConflict => "VERSION_CONFLICT",
            Self::RuleInvalid => "RULE_INVALID",
            Self::PromptJsonInvalid => "PROMPT_JSON_INVALID",
            Self::PromptSchemaInvalid => "PROMPT_SCHEMA_INVALID",
            Self::SearchModeInvalid => "SEARCH_MODE_INVALID",
            Self::AgentMemoryStoreError => "AGENT_MEMORY_STORE_ERROR",
            Self::AgentYoloPolicyViolation => "AGENT_YOLO_POLICY_VIOLATION",
            Self::SearchEmbeddingDegraded => "SEARCH_EMBEDDING_DEGRADED",
            Self::RateLimited => "RATE_LIMITED",
            Self::LlmUpstreamError => "LLM_UPSTREAM_ERROR",
            Self::EmbeddingProviderError => "EMBEDDING_PROVIDER_ERROR",
            Self::InternalError => "INTERNAL_ERROR",
            Self::SetupLocked => "SETUP_LOCKED",
        }
    }

    pub fn http_status(&self) -> u16 {
        match self {
            Self::BadRequest | Self::InvalidPatch => 400,
            Self::AuthRequired | Self::InvalidCredentials => 401,
            Self::RoleForbidden | Self::WorkspaceForbidden => 403,
            Self::NoteNotFound | Self::WorkspaceNotFound => 404,
            Self::VersionConflict => 409,
            Self::RuleInvalid
            | Self::PromptJsonInvalid
            | Self::PromptSchemaInvalid
            | Self::SearchModeInvalid => 422,
            Self::AgentMemoryStoreError
            | Self::AgentYoloPolicyViolation => 422,
            Self::SearchEmbeddingDegraded => 200,
            Self::RateLimited => 429,
            Self::LlmUpstreamError
            | Self::EmbeddingProviderError => 502,
            Self::InternalError => 500,
            Self::SetupLocked => 403,
        }
    }
}

/// Canonical API error response envelope.
#[derive(Debug, Clone, Serialize)]
pub struct ApiError {
    pub code: String,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<serde_json::Value>,
    pub request_id: String,
}

impl ApiError {
    pub fn new(
        code: ErrorCode,
        message: impl Into<String>,
        request_id: impl Into<String>,
    ) -> Self {
        Self {
            code: code.as_str().to_string(),
            message: message.into(),
            details: None,
            request_id: request_id.into(),
        }
    }

    pub fn with_details(mut self, details: serde_json::Value) -> Self {
        self.details = Some(details);
        self
    }
}

/// Domain-level error type used across crates.
#[derive(Debug, thiserror::Error)]
pub enum DomainError {
    #[error("not found: {0}")]
    NotFound(String),
    #[error("version conflict: expected {expected}, got {actual}")]
    VersionConflict { expected: i64, actual: i64 },
    #[error("forbidden: {0}")]
    Forbidden(String),
    #[error("bad request: {0}")]
    BadRequest(String),
    #[error("internal: {0}")]
    Internal(String),
}
