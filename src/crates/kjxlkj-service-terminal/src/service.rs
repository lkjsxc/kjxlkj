//! Terminal/PTY service implementation.

/// Terminal service for PTY operations.
pub struct TerminalService {
    running: bool,
}

impl TerminalService {
    /// Create a new terminal service.
    pub fn new() -> Self {
        Self { running: false }
    }

    /// Start the service.
    pub async fn start(&mut self) {
        self.running = true;
        tracing::info!("Terminal service started");
    }

    /// Stop the service.
    pub async fn stop(&mut self) {
        self.running = false;
        tracing::info!("Terminal service stopped");
    }

    /// Check if the service is running.
    pub fn is_running(&self) -> bool {
        self.running
    }
}

impl Default for TerminalService {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_terminal_service() {
        let mut svc = TerminalService::new();
        assert!(!svc.is_running());
        svc.start().await;
        assert!(svc.is_running());
    }
}
