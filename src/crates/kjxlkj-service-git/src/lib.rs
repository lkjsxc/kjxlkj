//! Git integration service (placeholder).
//!
//! Will provide git status, diff, blame integration.

/// Git service state.
pub struct GitService {
    // Placeholder
}

impl GitService {
    /// Create a new Git service.
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for GitService {
    fn default() -> Self {
        Self::new()
    }
}
