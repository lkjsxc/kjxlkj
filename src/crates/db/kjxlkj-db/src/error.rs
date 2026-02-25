//! Database error types

use thiserror::Error;

#[derive(Debug, Error)]
pub enum DbError {
    #[error("Database connection failed: {0}")]
    ConnectionFailed(String),

    #[error("Query failed: {0}")]
    QueryFailed(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Concurrency conflict: expected {expected}, got {current}")]
    ConcurrencyConflict {
        expected: u64,
        current: u64,
    },

    #[error("Unique constraint violation: {0}")]
    UniqueViolation(String),

    #[error("Foreign key violation: {0}")]
    ForeignKeyViolation(String),

    #[error("Serialization error: {0}")]
    SerializationError(String),

    #[error("Migration error: {0}")]
    MigrationError(String),
}

pub type Result<T> = std::result::Result<T, DbError>;
