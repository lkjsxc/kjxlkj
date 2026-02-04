//! Service supervisor and wiring.
//!
//! This crate manages all background services.

pub use kjxlkj_service_fs::FsService;
pub use kjxlkj_service_git::GitService;
pub use kjxlkj_service_index::IndexService;
pub use kjxlkj_service_lsp::LspService;
pub use kjxlkj_service_terminal::TerminalService;

/// Service supervisor.
pub struct Services {
    pub lsp: LspService,
    pub git: GitService,
    pub index: IndexService,
    pub fs: FsService,
    pub terminal: TerminalService,
}

impl Services {
    /// Create and initialize all services.
    pub fn new() -> Self {
        Self {
            lsp: LspService::new(),
            git: GitService::new(),
            index: IndexService::new(),
            fs: FsService::new(),
            terminal: TerminalService::new(),
        }
    }
}

impl Default for Services {
    fn default() -> Self {
        Self::new()
    }
}
