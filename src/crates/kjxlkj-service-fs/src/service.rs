//! Filesystem service implementation.

use tokio::sync::mpsc;

use kjxlkj_core_types::{ServiceEvent, ServiceRequest};

/// Async filesystem service.
pub struct FsService {
    request_rx: mpsc::Receiver<ServiceRequest>,
    event_tx: mpsc::Sender<ServiceEvent>,
}

impl FsService {
    /// Creates a new filesystem service.
    pub fn new(
        request_rx: mpsc::Receiver<ServiceRequest>,
        event_tx: mpsc::Sender<ServiceEvent>,
    ) -> Self {
        Self {
            request_rx,
            event_tx,
        }
    }

    /// Runs the service loop.
    pub async fn run(mut self) {
        while let Some(request) = self.request_rx.recv().await {
            let event = self.handle_request(request).await;
            if self.event_tx.send(event).await.is_err() {
                break;
            }
        }
    }

    async fn handle_request(&self, request: ServiceRequest) -> ServiceEvent {
        match request {
            ServiceRequest::ReadFile { path } => {
                match tokio::fs::read_to_string(&path).await {
                    Ok(content) => ServiceEvent::FileRead { path, content },
                    Err(e) => ServiceEvent::Error {
                        message: format!("Failed to read {}: {}", path, e),
                    },
                }
            }
            ServiceRequest::WriteFile { path, content } => {
                match tokio::fs::write(&path, &content).await {
                    Ok(()) => ServiceEvent::FileWritten { path },
                    Err(e) => ServiceEvent::Error {
                        message: format!("Failed to write {}: {}", path, e),
                    },
                }
            }
            ServiceRequest::ExecuteCommand { .. } => ServiceEvent::Error {
                message: "FS service cannot execute commands".to_string(),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[tokio::test]
    async fn read_file() {
        let mut file = NamedTempFile::new().unwrap();
        writeln!(file, "test content").unwrap();
        let path = file.path().to_str().unwrap().to_string();

        let (req_tx, req_rx) = mpsc::channel(10);
        let (evt_tx, mut evt_rx) = mpsc::channel(10);

        let service = FsService::new(req_rx, evt_tx);
        let handle = tokio::spawn(service.run());

        req_tx
            .send(ServiceRequest::ReadFile { path: path.clone() })
            .await
            .unwrap();

        drop(req_tx);

        if let Some(ServiceEvent::FileRead { content, .. }) = evt_rx.recv().await {
            assert!(content.contains("test content"));
        }

        handle.await.unwrap();
    }
}
