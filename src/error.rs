use std::fmt::{Display, Formatter};
use std::io;

use crate::core::content::ContentValidationError;

#[derive(Debug)]
pub enum AppError {
    MissingEnv { key: &'static str },
    InvalidHost { value: String },
    InvalidPort { value: String },
    InvalidContentRoot,
    UnsupportedCommand { command: String },
    DatabaseConnect(sqlx::Error),
    DatabaseQuery(sqlx::Error),
    PasswordHash(argon2::password_hash::Error),
    ContentValidation(ContentValidationError),
    ContentIo { path: String, source: io::Error },
    ServerBind { addr: String, source: io::Error },
    ServerRun(io::Error),
}

impl AppError {
    pub fn unsupported_command(command: String) -> Self {
        Self::UnsupportedCommand { command }
    }

    pub fn database_connect(source: sqlx::Error) -> Self {
        Self::DatabaseConnect(source)
    }

    pub fn database_query(source: sqlx::Error) -> Self {
        Self::DatabaseQuery(source)
    }

    pub fn content_io(path: String, source: io::Error) -> Self {
        Self::ContentIo { path, source }
    }

    pub fn password_hash(source: argon2::password_hash::Error) -> Self {
        Self::PasswordHash(source)
    }

    pub fn code(&self) -> &'static str {
        match self {
            Self::MissingEnv { .. } => "E_CONFIG_MISSING_ENV",
            Self::InvalidHost { .. } => "E_CONFIG_INVALID_HOST",
            Self::InvalidPort { .. } => "E_CONFIG_INVALID_PORT",
            Self::InvalidContentRoot => "E_CONFIG_INVALID_CONTENT_ROOT",
            Self::UnsupportedCommand { .. } => "E_CLI_UNSUPPORTED_COMMAND",
            Self::DatabaseConnect(_) => "E_DATA_DB_CONNECT",
            Self::DatabaseQuery(_) => "E_DATA_DB_QUERY",
            Self::PasswordHash(_) => "E_AUTH_PASSWORD_HASH",
            Self::ContentValidation(error) => error.code(),
            Self::ContentIo { .. } => "E_CONTENT_IO",
            Self::ServerBind { .. } => "E_SERVER_BIND",
            Self::ServerRun(_) => "E_SERVER_RUNTIME",
        }
    }
}

impl Display for AppError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MissingEnv { key } => write!(f, "missing required environment variable: {key}"),
            Self::InvalidHost { value } => write!(f, "invalid BIND_HOST value: {value}"),
            Self::InvalidPort { value } => write!(f, "invalid BIND_PORT value: {value}"),
            Self::InvalidContentRoot => write!(f, "CONTENT_ROOT must not be empty"),
            Self::UnsupportedCommand { command } => write!(f, "unsupported command: {command}"),
            Self::DatabaseConnect(_) => write!(f, "failed to initialize database adapter"),
            Self::DatabaseQuery(_) => write!(f, "database query failed"),
            Self::PasswordHash(_) => write!(f, "password hashing failed"),
            Self::ContentValidation(error) => write!(f, "content validation failed: {error}"),
            Self::ContentIo { path, .. } => write!(f, "content I/O failed at path: {path}"),
            Self::ServerBind { addr, .. } => write!(f, "failed to bind HTTP server to {addr}"),
            Self::ServerRun(_) => write!(f, "HTTP server runtime failure"),
        }
    }
}

impl std::error::Error for AppError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::DatabaseConnect(source) => Some(source),
            Self::DatabaseQuery(source) => Some(source),
            Self::PasswordHash(_) => None,
            Self::ContentIo { source, .. } => Some(source),
            Self::ServerBind { source, .. } => Some(source),
            Self::ServerRun(source) => Some(source),
            _ => None,
        }
    }
}

impl From<ContentValidationError> for AppError {
    fn from(value: ContentValidationError) -> Self {
        Self::ContentValidation(value)
    }
}
