use thiserror::Error;

/// Domain-level errors following error contract in errors.md.
#[derive(Debug, Error)]
pub enum DomainError {
    #[error("not found: {entity}")]
    NotFound { entity: String },

    #[error("version conflict: expected {expected}, got {actual}")]
    VersionConflict { expected: i64, actual: i64 },

    #[error("setup already locked")]
    SetupLocked,

    #[error("invalid credentials")]
    InvalidCredentials,

    #[error("role forbidden")]
    RoleForbidden,

    #[error("bad request: {reason}")]
    BadRequest { reason: String },

    #[error("rule invalid: {reason}")]
    RuleInvalid { reason: String },

    #[error("rate limited")]
    RateLimited,

    #[error("attachment too large")]
    AttachmentTooLarge,

    #[error("internal: {0}")]
    Internal(String),
}
