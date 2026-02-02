//! kjxlkj-service-git - Git integration service.
//!
//! This crate provides Git repository interaction.

mod repo;
mod diff;

pub use repo::{GitRepo, GitStatus};
pub use diff::{DiffHunk, HunkType};
