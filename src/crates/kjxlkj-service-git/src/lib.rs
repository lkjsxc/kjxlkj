//! Git service for repository status and diff.
//!
//! Provides hunk detection, gutter signs, blame annotations,
//! and staging operations via git subprocess invocation.

mod service;
mod types;

#[cfg(test)]
mod git_tests;

pub use service::GitService;
pub use types::{
    BlameEntry, DiffHunk, FileStatus, GitCommand, GitNotification, GitSign, HunkAction, StatusEntry,
};
