use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum AppError {
    Config(String),
    UnsupportedCommand(String),
    InvalidRequest(String),
    Unauthorized,
    NotFound(String),
    Database(String),
    Io(std::io::Error),
    Serde(serde_json::Error),
}

impl AppError {
    pub fn code(&self) -> &'static str {
        match self {
            Self::Config(_) => "config_error",
            Self::UnsupportedCommand(_) => "unsupported_command",
            Self::InvalidRequest(_) => "invalid_request",
            Self::Unauthorized => "unauthorized",
            Self::NotFound(_) => "not_found",
            Self::Database(_) => "database_error",
            Self::Io(_) => "storage_error",
            Self::Serde(_) => "storage_error",
        }
    }
}

impl Display for AppError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Config(message) => write!(f, "{message}"),
            Self::UnsupportedCommand(command) => write!(f, "unsupported command: {command}"),
            Self::InvalidRequest(message) => write!(f, "{message}"),
            Self::Unauthorized => write!(f, "x-admin-token is missing or invalid"),
            Self::NotFound(message) => write!(f, "{message}"),
            Self::Database(message) => write!(f, "{message}"),
            Self::Io(error) => write!(f, "{error}"),
            Self::Serde(error) => write!(f, "{error}"),
        }
    }
}

impl std::error::Error for AppError {}

impl From<std::io::Error> for AppError {
    fn from(value: std::io::Error) -> Self {
        Self::Io(value)
    }
}

impl From<serde_json::Error> for AppError {
    fn from(value: serde_json::Error) -> Self {
        Self::Serde(value)
    }
}

impl From<sqlx::Error> for AppError {
    fn from(value: sqlx::Error) -> Self {
        Self::Database(value.to_string())
    }
}
