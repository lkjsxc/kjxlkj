//! Index/navigation service (placeholder).

/// Index service for file navigation and search.
pub struct IndexService {
    running: bool,
}

impl Default for IndexService {
    fn default() -> Self {
        Self::new()
    }
}

impl IndexService {
    /// Create new index service.
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
    fn test_index_service() {
        let mut svc = IndexService::new();
        assert!(!svc.is_running());
        svc.start();
        assert!(svc.is_running());
    }
}
