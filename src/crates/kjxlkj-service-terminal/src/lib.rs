//! Terminal/PTY service.
//!
//! This crate provides terminal emulation functionality.
//! Currently a placeholder for future implementation.

/// Terminal service state.
pub struct TerminalService {
    /// Whether the service is active.
    active: bool,
}

impl TerminalService {
    /// Create a new terminal service.
    pub fn new() -> Self {
        Self { active: false }
    }

    /// Start the service.
    pub fn start(&mut self) {
        self.active = true;
    }

    /// Stop the service.
    pub fn stop(&mut self) {
        self.active = false;
    }

    /// Check if active.
    pub fn is_active(&self) -> bool {
        self.active
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

    #[test]
    fn terminal_service_lifecycle() {
        let mut svc = TerminalService::new();
        assert!(!svc.is_active());
        svc.start();
        assert!(svc.is_active());
    }
}
