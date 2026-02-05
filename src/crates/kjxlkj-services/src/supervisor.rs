//! Service supervisor and wiring.

use kjxlkj_service_fs::FsService;
use kjxlkj_service_git::GitService;
use kjxlkj_service_index::IndexService;
use kjxlkj_service_lsp::LspService;
use kjxlkj_service_terminal::TerminalService;

/// Service supervisor managing all background services.
pub struct ServiceSupervisor {
    /// LSP service.
    pub lsp: LspService,
    /// Git service.
    pub git: GitService,
    /// Index service.
    pub index: IndexService,
    /// Filesystem service.
    pub fs: FsService,
    /// Terminal service.
    pub terminal: TerminalService,
}

impl Default for ServiceSupervisor {
    fn default() -> Self {
        Self::new()
    }
}

impl ServiceSupervisor {
    /// Create new service supervisor.
    pub fn new() -> Self {
        Self {
            lsp: LspService::new(),
            git: GitService::new(),
            index: IndexService::new(),
            fs: FsService::new(),
            terminal: TerminalService::new(),
        }
    }

    /// Start all services.
    pub fn start_all(&mut self) {
        self.lsp.start();
        self.git.start();
        self.index.start();
        self.fs.start();
        self.terminal.start();
    }

    /// Stop all services.
    pub fn stop_all(&mut self) {
        self.lsp.stop();
        self.git.stop();
        self.index.stop();
        self.fs.stop();
        self.terminal.stop();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_supervisor() {
        let mut sup = ServiceSupervisor::new();
        sup.start_all();
        assert!(sup.lsp.is_running());
        assert!(sup.git.is_running());
        sup.stop_all();
        assert!(!sup.lsp.is_running());
    }
}
