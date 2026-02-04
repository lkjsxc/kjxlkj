//! Filesystem service (placeholder).
//!
//! Filesystem IO and watch service.

/// Filesystem service placeholder.
pub struct FsService;

impl FsService {
    /// Create a new FS service.
    pub fn new() -> Self {
        Self
    }
}

impl Default for FsService {
    fn default() -> Self {
        Self::new()
    }
}
