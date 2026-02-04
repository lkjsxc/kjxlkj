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

    #[test]
    fn supervisor_default() {
        let _sup = ServiceSupervisor::default();
    }

    #[test]
    fn fs_service_reexport() {
        // Verify fs service is re-exported
        let _ = std::any::type_name::<fs::FsService>();
    }

    #[test]
    fn git_service_reexport() {
        // Verify git service is re-exported
        let _ = std::any::type_name::<git::GitService>();
    }

    #[test]
    fn lsp_service_reexport() {
        // Verify lsp service is re-exported
        let _ = std::any::type_name::<lsp::LspService>();
    }

    #[test]
    fn terminal_service_reexport() {
        // Verify terminal service is re-exported
        let _ = std::any::type_name::<terminal::TerminalService>();
    }

    #[test]
    fn index_service_reexport() {
        // Verify index service is re-exported
        let _ = std::any::type_name::<index::IndexService>();
    }

    #[test]
    fn supervisor_multiple_instances() {
        let sup1 = ServiceSupervisor::new();
        let sup2 = ServiceSupervisor::new();
        let _ = (&sup1, &sup2);
    }
}
