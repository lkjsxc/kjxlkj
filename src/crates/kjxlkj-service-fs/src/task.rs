//! FS service task.

use kjxlkj_core_types::{Action, ServiceResponse};
use std::path::PathBuf;
use tokio::sync::mpsc;

/// Filesystem service request.
#[derive(Debug)]
#[allow(dead_code)]
pub enum FsRequest {
    ReadFile(PathBuf),
    WriteFile(PathBuf, String),
}

/// Filesystem service.
pub struct FsService;

impl FsService {
    /// Process a read-file request.
    pub async fn read_file(path: PathBuf, response_tx: mpsc::Sender<Action>) {
        match tokio::fs::read_to_string(&path).await {
            Ok(content) => {
                let resp = Action::ServiceResponse(ServiceResponse::FileRead { path, content });
                let _ = response_tx.send(resp).await;
            }
            Err(e) => {
                tracing::error!("Failed to read {}: {e}", path.display());
            }
        }
    }

    /// Process a write-file request.
    pub async fn write_file(path: PathBuf, content: String, response_tx: mpsc::Sender<Action>) {
        match tokio::fs::write(&path, &content).await {
            Ok(()) => {
                let resp = Action::ServiceResponse(ServiceResponse::FileWritten { path });
                let _ = response_tx.send(resp).await;
            }
            Err(e) => {
                tracing::error!("Failed to write {}: {e}", path.display());
            }
        }
    }
}
