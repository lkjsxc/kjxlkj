use kjxlkj_core_types::{ServiceRequest, ServiceResponse};
use kjxlkj_service_fs::FsService;
use kjxlkj_service_git::GitService;
use kjxlkj_service_index::IndexService;
use kjxlkj_service_lsp::LspService;
use kjxlkj_service_terminal::TerminalService;
use tokio::sync::{broadcast, mpsc};
use tokio::task::JoinHandle;
use tracing::info;

/// Handles for all per-service request channels.
pub struct ServiceChannels {
    pub fs_tx: mpsc::Sender<ServiceRequest>,
    pub git_tx: mpsc::Sender<ServiceRequest>,
    pub index_tx: mpsc::Sender<ServiceRequest>,
    pub lsp_tx: mpsc::Sender<ServiceRequest>,
    pub terminal_tx: mpsc::Sender<ServiceRequest>,
}

/// Service supervisor: creates channels and spawns all service tasks.
pub struct ServiceSupervisor;

impl ServiceSupervisor {
    /// Spawn all service tasks. Returns:
    /// - `ServiceChannels` for sending requests
    /// - Vec of join handles for shutdown
    pub fn spawn(
        response_tx: mpsc::Sender<ServiceResponse>,
        quit_tx: &broadcast::Sender<()>,
    ) -> (ServiceChannels, Vec<JoinHandle<anyhow::Result<()>>>) {
        let mut handles = Vec::with_capacity(5);

        // FS service
        let (fs_tx, fs_rx) = mpsc::channel(64);
        let fs_resp = response_tx.clone();
        let fs_quit = quit_tx.subscribe();
        handles.push(tokio::spawn(async move {
            FsService::run(fs_rx, fs_resp, fs_quit).await
        }));

        // Git service
        let (git_tx, git_rx) = mpsc::channel(64);
        let git_resp = response_tx.clone();
        let git_quit = quit_tx.subscribe();
        handles.push(tokio::spawn(async move {
            GitService::run(git_rx, git_resp, git_quit).await
        }));

        // Index service
        let (index_tx, index_rx) = mpsc::channel(64);
        let index_resp = response_tx.clone();
        let index_quit = quit_tx.subscribe();
        handles.push(tokio::spawn(async move {
            IndexService::run(index_rx, index_resp, index_quit).await
        }));

        // LSP service
        let (lsp_tx, lsp_rx) = mpsc::channel(64);
        let lsp_quit = quit_tx.subscribe();
        handles.push(tokio::spawn(async move {
            LspService::run(lsp_rx, lsp_quit).await
        }));

        // Terminal service
        let (terminal_tx, terminal_rx) = mpsc::channel(64);
        let terminal_quit = quit_tx.subscribe();
        handles.push(tokio::spawn(async move {
            TerminalService::run(terminal_rx, terminal_quit).await
        }));

        info!("service-supervisor: all 5 services spawned");

        let channels = ServiceChannels {
            fs_tx,
            git_tx,
            index_tx,
            lsp_tx,
            terminal_tx,
        };

        (channels, handles)
    }
}
