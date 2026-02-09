//! LSP service: coordinates language server lifecycle.

use tokio::sync::{broadcast, mpsc};

use kjxlkj_core_types::ServiceResponse;

/// LSP service managing language server connections.
pub struct LspService {
    response_tx: mpsc::Sender<ServiceResponse>,
}

impl LspService {
    pub fn new(response_tx: mpsc::Sender<ServiceResponse>) -> Self {
        Self { response_tx }
    }

    /// Run the LSP service loop.
    pub async fn run(self, mut quit_rx: broadcast::Receiver<()>) {
        loop {
            tokio::select! {
                _ = quit_rx.recv() => break,
                _ = tokio::time::sleep(
                    std::time::Duration::from_secs(3600)
                ) => {}
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_lsp_service() {
        let (tx, _rx) = mpsc::channel(256);
        let _svc = LspService::new(tx);
    }
}
