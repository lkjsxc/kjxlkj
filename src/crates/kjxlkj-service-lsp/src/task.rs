use kjxlkj_core_types::ServiceRequest;
use tokio::sync::{broadcast, mpsc};
use tracing::info;

/// LSP service task (stub — no language servers connected yet).
pub struct LspService;

impl LspService {
    /// Run the LSP service loop.
    pub async fn run(
        mut request_rx: mpsc::Receiver<ServiceRequest>,
        mut quit_rx: broadcast::Receiver<()>,
    ) -> anyhow::Result<()> {
        info!("lsp-service: started (stub)");
        loop {
            tokio::select! {
                _ = quit_rx.recv() => {
                    info!("lsp-service: quit signal received");
                    break;
                }
                req = request_rx.recv() => {
                    match req {
                        Some(_) => {
                            // Stub: no LSP handling yet
                        }
                        None => break,
                    }
                }
            }
        }
        Ok(())
    }
}
