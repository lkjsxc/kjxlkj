use kjxlkj_core_types::{ServiceRequest, ServiceResponse};
use tokio::sync::{broadcast, mpsc};
use tracing::{debug, info};

/// Git service task (stub implementation).
pub struct GitService;

impl GitService {
    /// Run the git service loop.
    pub async fn run(
        mut request_rx: mpsc::Receiver<ServiceRequest>,
        response_tx: mpsc::Sender<ServiceResponse>,
        mut quit_rx: broadcast::Receiver<()>,
    ) -> anyhow::Result<()> {
        info!("git-service: started");
        loop {
            tokio::select! {
                _ = quit_rx.recv() => {
                    info!("git-service: quit signal received");
                    break;
                }
                req = request_rx.recv() => {
                    match req {
                        Some(ServiceRequest::GitStatus { request_id }) => {
                            debug!("git-service: status query");
                            // Stub: return empty status
                            let resp = ServiceResponse::GitStatusResult {
                                request_id,
                                status: Ok(String::new()),
                            };
                            let _ = response_tx.send(resp).await;
                        }
                        Some(other) => {
                            debug!("git-service: ignoring unrelated request: {:?}", other);
                        }
                        None => break,
                    }
                }
            }
        }
        Ok(())
    }
}
