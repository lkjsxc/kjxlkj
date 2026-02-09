use kjxlkj_core_types::ServiceRequest;
use tokio::sync::{broadcast, mpsc};
use tracing::info;

/// Terminal service task (stub — no PTY management yet).
pub struct TerminalService;

impl TerminalService {
    /// Run the terminal service loop.
    pub async fn run(
        mut request_rx: mpsc::Receiver<ServiceRequest>,
        mut quit_rx: broadcast::Receiver<()>,
    ) -> anyhow::Result<()> {
        info!("terminal-service: started (stub)");
        loop {
            tokio::select! {
                _ = quit_rx.recv() => {
                    info!("terminal-service: quit signal received");
                    break;
                }
                req = request_rx.recv() => {
                    match req {
                        Some(_) => {
                            // Stub: no terminal PTY handling yet
                        }
                        None => break,
                    }
                }
            }
        }
        Ok(())
    }
}
