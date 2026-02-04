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

    #[test]
    fn supervisor_is_send() {
        fn assert_send<T: Send>() {}
        assert_send::<ServiceSupervisor>();
    }

    #[test]
    fn supervisor_is_sync() {
        fn assert_sync<T: Sync>() {}
        assert_sync::<ServiceSupervisor>();
    }

    #[test]
    fn git_service_via_reexport() {
        let _ = std::any::type_name::<git::GitService>();
    }

    #[test]
    fn fs_service_via_reexport() {
        let _ = std::any::type_name::<fs::FsService>();
    }

    #[test]
    fn lsp_service_via_reexport() {
        let _ = std::any::type_name::<lsp::LspService>();
    }

    #[test]
    fn supervisor_default_new() {
        let sup = ServiceSupervisor::default();
        let _ = sup;
    }

    #[test]
    fn terminal_service_type_name() {
        let name = std::any::type_name::<terminal::TerminalService>();
        assert!(name.contains("TerminalService"));
    }

    #[test]
    fn index_service_type_via() {
        let name = std::any::type_name::<index::IndexService>();
        assert!(name.contains("IndexService"));
    }

    #[test]
    fn supervisor_type_size() {
        assert_eq!(std::mem::size_of::<ServiceSupervisor>(), 0);
    }

    #[test]
    fn supervisor_type_align() {
        assert_eq!(std::mem::align_of::<ServiceSupervisor>(), 1);
    }

    #[test]
    fn supervisor_new_create() {
        let sup = ServiceSupervisor::new();
        drop(sup);
    }

    #[test]
    fn supervisor_box_pattern() {
        let sup = Box::new(ServiceSupervisor::new());
        drop(sup);
    }

    #[test]
    fn supervisor_ref_pattern() {
        let sup = &ServiceSupervisor::new();
        let _ = sup;
    }

    #[test]
    fn supervisor_multiple_new() {
        let _ = ServiceSupervisor::new();
        let _ = ServiceSupervisor::new();
        let _ = ServiceSupervisor::new();
    }

    #[test]
    fn git_type_via_services() {
        let name = std::any::type_name::<git::GitService>();
        assert!(name.contains("Git"));
    }

    #[test]
    fn lsp_type_via_services() {
        let name = std::any::type_name::<lsp::LspService>();
        assert!(name.contains("Lsp"));
    }

    #[test]
    fn fs_type_via_services() {
        let name = std::any::type_name::<fs::FsService>();
        assert!(name.contains("Fs"));
    }

    #[test]
    fn supervisor_option_some() {
        let svc = Some(ServiceSupervisor::new());
        assert!(svc.is_some());
    }

    #[test]
    fn supervisor_option_none() {
        let svc: Option<ServiceSupervisor> = None;
        assert!(svc.is_none());
    }
}
