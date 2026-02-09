use kjxlkj_core_types::{ServiceRequest, ServiceResponse};
use tokio::sync::{broadcast, mpsc};
use tracing::{debug, info};

/// File system service task.
pub struct FsService;

impl FsService {
    /// Run the FS service loop.
    pub async fn run(
        mut request_rx: mpsc::Receiver<ServiceRequest>,
        response_tx: mpsc::Sender<ServiceResponse>,
        mut quit_rx: broadcast::Receiver<()>,
    ) -> anyhow::Result<()> {
        info!("fs-service: started");
        loop {
            tokio::select! {
                _ = quit_rx.recv() => {
                    info!("fs-service: quit signal received");
                    break;
                }
                req = request_rx.recv() => {
                    match req {
                        Some(ServiceRequest::ReadFile { request_id, path }) => {
                            debug!("fs-service: read {}", path.display());
                            let result = tokio::fs::read_to_string(&path).await;
                            let resp = ServiceResponse::FileRead {
                                request_id,
                                content: result.map_err(|e| e.to_string()),
                            };
                            let _ = response_tx.send(resp).await;
                        }
                        Some(ServiceRequest::WriteFile { request_id, path, content }) => {
                            debug!("fs-service: write {}", path.display());
                            let result = tokio::fs::write(&path, &content).await;
                            let resp = ServiceResponse::FileWritten {
                                request_id,
                                result: result.map_err(|e| e.to_string()),
                            };
                            let _ = response_tx.send(resp).await;
                        }
                        Some(other) => {
                            debug!("fs-service: ignoring unrelated request: {:?}", other);
                        }
                        None => break,
                    }
                }
            }
        }
        Ok(())
    }
}
