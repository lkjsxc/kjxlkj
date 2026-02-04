//! Service supervisor and wiring.

pub use kjxlkj_service_fs as fs;
pub use kjxlkj_service_git as git;
pub use kjxlkj_service_index as index;
pub use kjxlkj_service_lsp as lsp;
pub use kjxlkj_service_terminal as terminal;

/// Service supervisor (placeholder for future implementation).
pub struct ServiceSupervisor {
    // Future: manage service lifecycles
}

impl ServiceSupervisor {
    /// Create a new supervisor.
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for ServiceSupervisor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn supervisor_creation() {
        let _sup = ServiceSupervisor::new();
    }
}
