//! Git integration service for kjxlkj editor.
//!
//! This crate provides git functionality.

mod hunk;
mod repo;
mod service;

#[cfg(test)]
mod tests;

pub use hunk::{Hunk, HunkKind};
pub use repo::GitRepo;
pub use service::GitService;
