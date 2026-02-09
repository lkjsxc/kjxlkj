use kjxlkj_core_types::{ServiceRequest, ServiceResponse};
use std::path::PathBuf;
use tokio::sync::{broadcast, mpsc};
use tracing::{debug, info};

/// Index service task.
pub struct IndexService;

impl IndexService {
    /// Run the index service loop.
    pub async fn run(
        mut request_rx: mpsc::Receiver<ServiceRequest>,
        response_tx: mpsc::Sender<ServiceResponse>,
        mut quit_rx: broadcast::Receiver<()>,
    ) -> anyhow::Result<()> {
        info!("index-service: started");
        loop {
            tokio::select! {
                _ = quit_rx.recv() => {
                    info!("index-service: quit signal received");
                    break;
                }
                req = request_rx.recv() => {
                    match req {
                        Some(ServiceRequest::IndexWorkspace { request_id, root }) => {
                            debug!("index-service: indexing {}", root.display());
                            let files = collect_files(&root).await;
                            let resp = ServiceResponse::IndexResult {
                                request_id,
                                files: Ok(files),
                            };
                            let _ = response_tx.send(resp).await;
                        }
                        Some(other) => {
                            debug!("index-service: ignoring unrelated request: {:?}", other);
                        }
                        None => break,
                    }
                }
            }
        }
        Ok(())
    }
}

async fn collect_files(root: &std::path::Path) -> Vec<PathBuf> {
    let mut files = Vec::new();
    let mut stack = vec![root.to_path_buf()];
    while let Some(dir) = stack.pop() {
        let mut entries = match tokio::fs::read_dir(&dir).await {
            Ok(e) => e,
            Err(_) => continue,
        };
        while let Ok(Some(entry)) = entries.next_entry().await {
            let path = entry.path();
            if path.is_dir() {
                // Skip hidden directories
                if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                    if !name.starts_with('.') {
                        stack.push(path);
                    }
                }
            } else {
                files.push(path);
            }
        }
    }
    files
}
