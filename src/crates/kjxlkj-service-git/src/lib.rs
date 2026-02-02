//! kjxlkj-service-git - Git integration service.
//!
//! This crate provides Git repository interaction.

mod diff;
mod repo;

pub use diff::{DiffHunk, HunkType};
pub use repo::{GitRepo, GitStatus};
