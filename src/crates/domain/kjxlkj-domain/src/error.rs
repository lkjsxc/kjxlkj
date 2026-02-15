//! Domain error types per /docs/spec/api/errors.md.

use thiserror::Error;
use uuid::Uuid;

/// Canonical error codes per /docs/spec/api/errors.md.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ErrorCode {
    BadRequest,
    InvalidPatch,
    AuthRequired,
    InvalidCredentials,
    CsrfInvalid,
    RoleForbidden,
    NoteNotFound,
    WorkspaceNotFound,
    ProjectNotFound,
    UserNotFound,
    ViewNotFound,
    RuleNotFound,
    RunNotFound,
    AttachmentNotFound,
    VersionConflict,
    MembershipConflict,
    AttachmentTooLarge,
    RuleInvalid,
    ProjectScopeInvalid,
    RateLimited,
    LlmUpstreamError,
    LlmProviderUnreachable,
    LibrarianProtocolInvalid,
    LibrarianParseFailed,
    LibrarianOperationRejected,
    StaleCursor,
    SetupLocked,
    InternalError,
}

impl ErrorCode {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::BadRequest => "BAD_REQUEST",
            Self::InvalidPatch => "INVALID_PATCH",
            Self::AuthRequired => "AUTH_REQUIRED",
            Self::InvalidCredentials => "INVALID_CREDENTIALS",
            Self::CsrfInvalid => "CSRF_INVALID",
            Self::RoleForbidden => "ROLE_FORBIDDEN",
            Self::NoteNotFound => "NOTE_NOT_FOUND",
            Self::WorkspaceNotFound => "WORKSPACE_NOT_FOUND",
            Self::ProjectNotFound => "PROJECT_NOT_FOUND",
            Self::UserNotFound => "USER_NOT_FOUND",
            Self::ViewNotFound => "VIEW_NOT_FOUND",
            Self::RuleNotFound => "RULE_NOT_FOUND",
            Self::RunNotFound => "RUN_NOT_FOUND",
            Self::AttachmentNotFound => "ATTACHMENT_NOT_FOUND",
            Self::VersionConflict => "VERSION_CONFLICT",
            Self::MembershipConflict => "MEMBERSHIP_CONFLICT",
            Self::AttachmentTooLarge => "ATTACHMENT_TOO_LARGE",
            Self::RuleInvalid => "RULE_INVALID",
            Self::ProjectScopeInvalid => "PROJECT_SCOPE_INVALID",
            Self::RateLimited => "RATE_LIMITED",
            Self::LlmUpstreamError => "LLM_UPSTREAM_ERROR",
            Self::LlmProviderUnreachable => "LLM_PROVIDER_UNREACHABLE",
            Self::LibrarianProtocolInvalid => "LIBRARIAN_PROTOCOL_INVALID",
            Self::LibrarianParseFailed => "LIBRARIAN_PARSE_FAILED",
            Self::LibrarianOperationRejected => "LIBRARIAN_OPERATION_REJECTED",
            Self::StaleCursor => "STALE_CURSOR",
            Self::SetupLocked => "SETUP_LOCKED",
            Self::InternalError => "INTERNAL_ERROR",
        }
    }

    pub fn http_status(&self) -> u16 {
        match self {
            Self::BadRequest | Self::InvalidPatch => 400,
            Self::AuthRequired | Self::InvalidCredentials => 401,
            Self::CsrfInvalid | Self::RoleForbidden => 403,
            Self::NoteNotFound
            | Self::WorkspaceNotFound
            | Self::ProjectNotFound
            | Self::UserNotFound
            | Self::ViewNotFound
            | Self::RuleNotFound
            | Self::RunNotFound
            | Self::AttachmentNotFound => 404,
            Self::VersionConflict
            | Self::MembershipConflict
            | Self::SetupLocked => 409,
            Self::AttachmentTooLarge => 413,
            Self::RuleInvalid
            | Self::ProjectScopeInvalid
            | Self::LibrarianProtocolInvalid
            | Self::LibrarianParseFailed
            | Self::LibrarianOperationRejected => 422,
            Self::RateLimited => 429,
            Self::LlmUpstreamError | Self::LlmProviderUnreachable => 502,
            Self::InternalError | Self::StaleCursor => 500,
        }
    }
}

/// Domain-level error with code, message, and request context.
#[derive(Debug, Error)]
#[error("{message}")]
pub struct DomainError {
    pub code: ErrorCode,
    pub message: String,
    pub details: Option<serde_json::Value>,
    pub request_id: Uuid,
}

impl DomainError {
    pub fn new(code: ErrorCode, message: impl Into<String>, request_id: Uuid) -> Self {
        Self {
            code,
            message: message.into(),
            details: None,
            request_id,
        }
    }

    pub fn with_details(mut self, details: serde_json::Value) -> Self {
        self.details = Some(details);
        self
    }
}
