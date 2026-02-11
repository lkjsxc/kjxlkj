//! Git integration service.
//!
//! Provides diff hunks, branch info, and gutter signs asynchronously.

pub mod gitsigns;

/// Placeholder for git service state.
pub struct GitService;

impl GitService {
    pub fn new() -> Self {
        Self
    }
}
