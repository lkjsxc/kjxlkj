//! Terminal/PTY service (placeholder).

/// Terminal service for embedded PTY support.
pub struct TerminalService {
    running: bool,
}

impl Default for TerminalService {
    fn default() -> Self {
        Self::new()
    }
}

impl TerminalService {
    /// Create new terminal service.
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
    fn test_terminal_service() {
        let mut svc = TerminalService::new();
        assert!(!svc.is_running());
        svc.start();
        assert!(svc.is_running());
    }
}
