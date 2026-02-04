//! Index/navigation service.
//!
//! This crate provides file indexing and search functionality.
//! Currently a placeholder for future implementation.

/// Index service state.
pub struct IndexService {
    /// Whether the service is active.
    active: bool,
}

impl IndexService {
    /// Create a new index service.
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

impl Default for IndexService {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn index_service_lifecycle() {
        let mut svc = IndexService::new();
        assert!(!svc.is_active());
        svc.start();
        assert!(svc.is_active());
    }
}
