//! Services supervisor implementation.

use kjxlkj_service_fs::FsService;
use kjxlkj_service_git::GitService;
use kjxlkj_service_index::IndexService;
use kjxlkj_service_lsp::LspService;
use kjxlkj_service_terminal::TerminalService;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::info;

/// Services supervisor.
pub struct Services {
    /// File system service.
    pub fs: Arc<FsService>,
    /// Git service.
    pub git: Arc<GitService>,
    /// Index service.
    pub index: Arc<RwLock<IndexService>>,
    /// LSP service.
    pub lsp: Arc<RwLock<LspService>>,
    /// Terminal service.
    pub terminal: Arc<RwLock<TerminalService>>,
    /// Workspace root.
    pub workspace_root: PathBuf,
}

impl Services {
    /// Create a new services supervisor.
    pub fn new() -> Self {
        Self::with_root(std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")))
    }

    /// Create services with a specific workspace root.
    pub fn with_root(root: PathBuf) -> Self {
        info!("Initializing services with root {:?}", root);
        Self {
            fs: Arc::new(FsService::new()),
            git: Arc::new(GitService::new()),
            index: Arc::new(RwLock::new(IndexService::new())),
            lsp: Arc::new(RwLock::new(LspService::new())),
            terminal: Arc::new(RwLock::new(TerminalService::new())),
            workspace_root: root,
        }
    }

    /// Initialize all services.
    pub async fn init(&self) {
        info!("Starting services");
        self.index.write().await.init(self.workspace_root.clone());
        self.lsp.write().await.init();
    }

    /// Shutdown all services.
    pub async fn shutdown(&self) {
        info!("Shutting down services");
    }
}

impl Default for Services {
    fn default() -> Self {
        Self::new()
    }
}
