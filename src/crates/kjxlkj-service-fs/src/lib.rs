//! Filesystem service (placeholder).
//!
//! Will provide file watching and async IO.

/// FS service state.
pub struct FsService {
    // Placeholder
}

impl FsService {
    /// Create a new FS service.
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for FsService {
    fn default() -> Self {
        Self::new()
    }
}
