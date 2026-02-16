/// Domain error types mapped to /docs/spec/api/errors.md
use thiserror::Error;

#[derive(Debug, Error)]
pub enum DomainError {
    #[error("note not found")]
    NoteNotFound,
    #[error("workspace not found")]
    WorkspaceNotFound,
    #[error("project not found")]
    ProjectNotFound,
    #[error("version conflict: expected {expected}, got {actual}")]
    VersionConflict { expected: i64, actual: i64 },
    #[error("authentication required")]
    AuthRequired,
    #[error("invalid credentials")]
    InvalidCredentials,
    #[error("role forbidden")]
    RoleForbidden,
    #[error("workspace forbidden")]
    WorkspaceForbidden,
    #[error("setup already completed")]
    SetupAlreadyCompleted,
    #[error("bad request: {0}")]
    BadRequest(String),
    #[error("invalid patch")]
    InvalidPatch,
    #[error("rule invalid: {0}")]
    RuleInvalid(String),
    #[error("prompt json invalid: {0}")]
    PromptJsonInvalid(String),
    #[error("prompt schema invalid: {0}")]
    PromptSchemaInvalid(String),
    #[error("agent memory store error: {0}")]
    AgentMemoryStoreError(String),
    #[error("agent yolo policy violation: {0}")]
    AgentYoloPolicyViolation(String),
    #[error("search mode invalid: {0}")]
    SearchModeInvalid(String),
    #[error("search embedding degraded")]
    SearchEmbeddingDegraded,
    #[error("rate limited")]
    RateLimited,
    #[error("llm upstream error: {0}")]
    LlmUpstreamError(String),
    #[error("embedding provider error: {0}")]
    EmbeddingProviderError(String),
    #[error("payload too large")]
    PayloadTooLarge,
    #[error("stale cursor")]
    StaleCursor,
    #[error("internal error: {0}")]
    Internal(String),
}

impl DomainError {
    /// Stable machine code per /docs/spec/api/errors.md
    pub fn code(&self) -> &'static str {
        match self {
            Self::NoteNotFound => "NOTE_NOT_FOUND",
            Self::WorkspaceNotFound => "WORKSPACE_NOT_FOUND",
            Self::ProjectNotFound => "PROJECT_NOT_FOUND",
            Self::VersionConflict { .. } => "VERSION_CONFLICT",
            Self::AuthRequired => "AUTH_REQUIRED",
            Self::InvalidCredentials => "INVALID_CREDENTIALS",
            Self::RoleForbidden => "ROLE_FORBIDDEN",
            Self::WorkspaceForbidden => "WORKSPACE_FORBIDDEN",
            Self::SetupAlreadyCompleted => "SETUP_ALREADY_COMPLETED",
            Self::BadRequest(_) => "BAD_REQUEST",
            Self::InvalidPatch => "INVALID_PATCH",
            Self::RuleInvalid(_) => "RULE_INVALID",
            Self::PromptJsonInvalid(_) => "PROMPT_JSON_INVALID",
            Self::PromptSchemaInvalid(_) => "PROMPT_SCHEMA_INVALID",
            Self::AgentMemoryStoreError(_) => "AGENT_MEMORY_STORE_ERROR",
            Self::AgentYoloPolicyViolation(_) => "AGENT_YOLO_POLICY_VIOLATION",
            Self::SearchModeInvalid(_) => "SEARCH_MODE_INVALID",
            Self::SearchEmbeddingDegraded => "SEARCH_EMBEDDING_DEGRADED",
            Self::RateLimited => "RATE_LIMITED",
            Self::LlmUpstreamError(_) => "LLM_UPSTREAM_ERROR",
            Self::EmbeddingProviderError(_) => "EMBEDDING_PROVIDER_ERROR",
            Self::PayloadTooLarge => "PAYLOAD_TOO_LARGE",
            Self::StaleCursor => "STALE_CURSOR",
            Self::Internal(_) => "INTERNAL_ERROR",
        }
    }

    /// HTTP status code per /docs/spec/api/errors.md
    pub fn status_code(&self) -> u16 {
        match self {
            Self::BadRequest(_) | Self::InvalidPatch => 400,
            Self::AuthRequired | Self::InvalidCredentials => 401,
            Self::RoleForbidden | Self::WorkspaceForbidden => 403,
            Self::NoteNotFound | Self::WorkspaceNotFound | Self::ProjectNotFound => 404,
            Self::VersionConflict { .. } => 409,
            Self::PayloadTooLarge => 413,
            Self::RuleInvalid(_)
            | Self::PromptJsonInvalid(_)
            | Self::PromptSchemaInvalid(_)
            | Self::SearchModeInvalid(_)
            | Self::SetupAlreadyCompleted => 422,
            Self::RateLimited => 429,
            Self::AgentYoloPolicyViolation(_) => 403,
            Self::Internal(_)
            | Self::AgentMemoryStoreError(_) => 500,
            Self::LlmUpstreamError(_) | Self::EmbeddingProviderError(_) => 502,
            Self::SearchEmbeddingDegraded => 502,
            Self::StaleCursor => 409,
        }
    }
}
