//! Index service implementation.

/// Index service for file/symbol searching.
pub struct IndexService {
    running: bool,
}

impl IndexService {
    /// Create a new index service.
    pub fn new() -> Self {
        Self { running: false }
    }

    /// Start the service.
    pub async fn start(&mut self) {
        self.running = true;
        tracing::info!("Index service started");
    }

    /// Stop the service.
    pub async fn stop(&mut self) {
        self.running = false;
        tracing::info!("Index service stopped");
    }

    /// Check if the service is running.
    pub fn is_running(&self) -> bool {
        self.running
    }
}

impl Default for IndexService {
    fn default() -> Self {
        Self::new()
    }
}
