//! Service supervisor implementation.

use kjxlkj_service_fs::FsService;
use kjxlkj_service_git::GitService;
use kjxlkj_service_index::IndexService;
use kjxlkj_service_lsp::LspService;
use kjxlkj_service_terminal::TerminalService;

/// Supervisor for all background services.
pub struct ServiceSupervisor {
    pub fs: FsService,
    pub git: GitService,
    pub index: IndexService,
    pub lsp: LspService,
    pub terminal: TerminalService,
}

impl ServiceSupervisor {
    /// Create a new service supervisor.
    pub fn new() -> Self {
        Self {
            fs: FsService::new(),
            git: GitService::new(),
            index: IndexService::new(),
            lsp: LspService::new(),
            terminal: TerminalService::new(),
        }
    }

    /// Start all services.
    pub async fn start_all(&mut self) {
        self.fs.start().await;
        self.git.start().await;
        self.index.start().await;
        self.lsp.start().await;
        self.terminal.start().await;
        tracing::info!("All services started");
    }

    /// Stop all services.
    pub async fn stop_all(&mut self) {
        self.fs.stop().await;
        self.git.stop().await;
        self.index.stop().await;
        self.lsp.stop().await;
        self.terminal.stop().await;
        tracing::info!("All services stopped");
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

    #[tokio::test]
    async fn test_supervisor() {
        let mut supervisor = ServiceSupervisor::new();
        supervisor.start_all().await;
        assert!(supervisor.fs.is_running());
        assert!(supervisor.git.is_running());
        supervisor.stop_all().await;
        assert!(!supervisor.fs.is_running());
    }
}
