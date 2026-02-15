use thiserror::Error;

/// Domain error types per /docs/spec/api/errors.md.
#[derive(Debug, Error)]
pub enum DomainError {
    #[error("authentication required")]
    AuthRequired,

    #[error("invalid credentials")]
    InvalidCredentials,

    #[error("CSRF token invalid")]
    CsrfInvalid,

    #[error("role forbidden: {0}")]
    RoleForbidden(String),

    #[error("not found: {0}")]
    NotFound(String),

    #[error("workspace not found: {0}")]
    WorkspaceNotFound(String),

    #[error("version conflict: expected {expected}, found {found}")]
    VersionConflict { expected: i64, found: i64 },

    #[error("stale cursor: stream {stream_id}, attempted {attempted}, current {current}")]
    StaleCursor {
        stream_id: String,
        attempted: i64,
        current: i64,
    },

    #[error("bad request: {0}")]
    BadRequest(String),

    #[error("invalid patch")]
    InvalidPatch,

    #[error("attachment too large")]
    AttachmentTooLarge,

    #[error("rate limited")]
    RateLimited,

    #[error("rule invalid: {0}")]
    RuleInvalid(String),

    #[error("project scope invalid")]
    ProjectScopeInvalid,

    #[error("setup already locked")]
    SetupLocked,

    #[error("membership conflict")]
    MembershipConflict,

    #[error("upstream provider error: {0}")]
    LlmUpstreamError(String),

    #[error("internal error: {0}")]
    Internal(String),
}

impl DomainError {
    /// Machine error code per /docs/spec/api/errors.md.
    pub fn code(&self) -> &'static str {
        match self {
            Self::AuthRequired => "AUTH_REQUIRED",
            Self::InvalidCredentials => "INVALID_CREDENTIALS",
            Self::CsrfInvalid => "CSRF_INVALID",
            Self::RoleForbidden(_) => "ROLE_FORBIDDEN",
            Self::NotFound(_) => "NOTE_NOT_FOUND",
            Self::WorkspaceNotFound(_) => "WORKSPACE_NOT_FOUND",
            Self::VersionConflict { .. } => "VERSION_CONFLICT",
            Self::StaleCursor { .. } => "STALE_CURSOR",
            Self::BadRequest(_) => "BAD_REQUEST",
            Self::InvalidPatch => "INVALID_PATCH",
            Self::AttachmentTooLarge => "ATTACHMENT_TOO_LARGE",
            Self::RateLimited => "RATE_LIMITED",
            Self::RuleInvalid(_) => "RULE_INVALID",
            Self::ProjectScopeInvalid => "PROJECT_SCOPE_INVALID",
            Self::SetupLocked => "SETUP_LOCKED",
            Self::MembershipConflict => "MEMBERSHIP_CONFLICT",
            Self::LlmUpstreamError(_) => "LLM_UPSTREAM_ERROR",
            Self::Internal(_) => "INTERNAL_ERROR",
        }
    }

    /// HTTP status code per /docs/spec/api/errors.md.
    pub fn status_code(&self) -> u16 {
        match self {
            Self::AuthRequired | Self::InvalidCredentials => 401,
            Self::CsrfInvalid | Self::RoleForbidden(_) => 403,
            Self::NotFound(_) | Self::WorkspaceNotFound(_) => 404,
            Self::VersionConflict { .. } | Self::MembershipConflict => 409,
            Self::StaleCursor { .. } => 409,
            Self::BadRequest(_) | Self::InvalidPatch => 400,
            Self::AttachmentTooLarge => 413,
            Self::RateLimited => 429,
            Self::RuleInvalid(_) | Self::ProjectScopeInvalid => 422,
            Self::SetupLocked => 422,
            Self::LlmUpstreamError(_) => 502,
            Self::Internal(_) => 500,
        }
    }
}
