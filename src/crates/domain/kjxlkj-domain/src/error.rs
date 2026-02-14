// Domain error types per /docs/spec/api/errors.md
use thiserror::Error;

/// Application error codes matching /docs/spec/api/errors.md
#[derive(Debug, Error)]
pub enum DomainError {
    #[error("resource not found: {0}")]
    NotFound(String),

    #[error("version conflict: expected {expected}, got {actual}")]
    VersionConflict { expected: i64, actual: i64 },

    #[error("authentication required")]
    AuthRequired,

    #[error("invalid credentials")]
    InvalidCredentials,

    #[error("forbidden: {0}")]
    Forbidden(String),

    #[error("bad request: {0}")]
    BadRequest(String),

    #[error("setup already locked")]
    SetupLocked,

    #[error("invalid patch: {0}")]
    InvalidPatch(String),

    #[error("attachment too large: {size} bytes")]
    AttachmentTooLarge { size: i64 },

    #[error("rule invalid: {0}")]
    RuleInvalid(String),

    #[error("csrf invalid")]
    CsrfInvalid,

    #[error("rate limited")]
    RateLimited,

    #[error("internal error: {0}")]
    Internal(String),
}

/// Error code string for API responses
impl DomainError {
    pub fn code(&self) -> &'static str {
        match self {
            Self::NotFound(_) => "NOT_FOUND",
            Self::VersionConflict { .. } => "VERSION_CONFLICT",
            Self::AuthRequired => "AUTH_REQUIRED",
            Self::InvalidCredentials => "INVALID_CREDENTIALS",
            Self::Forbidden(_) => "ROLE_FORBIDDEN",
            Self::BadRequest(_) => "BAD_REQUEST",
            Self::SetupLocked => "SETUP_LOCKED",
            Self::InvalidPatch(_) => "INVALID_PATCH",
            Self::AttachmentTooLarge { .. } => "ATTACHMENT_TOO_LARGE",
            Self::RuleInvalid(_) => "RULE_INVALID",
            Self::CsrfInvalid => "CSRF_INVALID",
            Self::RateLimited => "RATE_LIMITED",
            Self::Internal(_) => "INTERNAL_ERROR",
        }
    }

    pub fn status_code(&self) -> u16 {
        match self {
            Self::NotFound(_) => 404,
            Self::VersionConflict { .. } => 409,
            Self::AuthRequired => 401,
            Self::InvalidCredentials => 401,
            Self::Forbidden(_) => 403,
            Self::BadRequest(_) => 400,
            Self::SetupLocked => 409,
            Self::InvalidPatch(_) => 400,
            Self::AttachmentTooLarge { .. } => 413,
            Self::RuleInvalid(_) => 422,
            Self::CsrfInvalid => 403,
            Self::RateLimited => 429,
            Self::Internal(_) => 500,
        }
    }
}
