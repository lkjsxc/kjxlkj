//! Filesystem IO/watch service (placeholder).

/// Filesystem service for file operations and watching.
pub struct FsService {
    running: bool,
}

impl Default for FsService {
    fn default() -> Self {
        Self::new()
    }
}

impl FsService {
    /// Create new FS service.
    pub fn new() -> Self {
        Self { running: false }
    }

    /// Start the service.
    pub fn start(&mut self) {
        self.running = true;
    }

    /// Stop the service.
    pub fn stop(&mut self) {
        self.running = false;
    }

    /// Check if running.
    pub fn is_running(&self) -> bool {
        self.running
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fs_service() {
        let mut svc = FsService::new();
        assert!(!svc.is_running());
        svc.start();
        assert!(svc.is_running());
    }
}
