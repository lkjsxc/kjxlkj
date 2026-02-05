//! Git service implementation.

/// Git service for repository operations.
pub struct GitService {
    running: bool,
}

impl GitService {
    /// Create a new git service.
    pub fn new() -> Self {
        Self { running: false }
    }

    /// Start the service.
    pub async fn start(&mut self) {
        self.running = true;
        tracing::info!("Git service started");
    }

    /// Stop the service.
    pub async fn stop(&mut self) {
        self.running = false;
        tracing::info!("Git service stopped");
    }

    /// Check if the service is running.
    pub fn is_running(&self) -> bool {
        self.running
    }
}

impl Default for GitService {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_git_service() {
        let mut svc = GitService::new();
        assert!(!svc.is_running());
        svc.start().await;
        assert!(svc.is_running());
    }
}
