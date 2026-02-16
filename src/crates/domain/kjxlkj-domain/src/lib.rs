/// kjxlkj-domain: Core entities, enums, and error types.
///
/// Canonical spec: /docs/spec/domain/README.md
pub mod error;
pub mod note;
pub mod workspace;
pub mod project;
pub mod permission;
pub mod event;
pub mod metadata;
pub mod search;
pub mod automation;
pub mod attachment;
pub mod config;
pub mod export;

pub use error::DomainError;
