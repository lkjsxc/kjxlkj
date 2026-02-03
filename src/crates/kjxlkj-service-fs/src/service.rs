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
    use tempfile::{NamedTempFile, TempDir};

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

    #[tokio::test]
    async fn write_file() {
        let dir = TempDir::new().unwrap();
        let path = dir.path().join("new_file.txt");
        let path_str = path.to_str().unwrap().to_string();

        let (req_tx, req_rx) = mpsc::channel(10);
        let (evt_tx, mut evt_rx) = mpsc::channel(10);

        let service = FsService::new(req_rx, evt_tx);
        let handle = tokio::spawn(service.run());

        req_tx
            .send(ServiceRequest::WriteFile {
                path: path_str.clone(),
                content: "written content".to_string(),
            })
            .await
            .unwrap();

        drop(req_tx);

        let evt = evt_rx.recv().await.unwrap();
        assert!(matches!(evt, ServiceEvent::FileWritten { .. }));

        // Verify file was written
        let content = std::fs::read_to_string(&path).unwrap();
        assert_eq!(content, "written content");

        handle.await.unwrap();
    }

    #[tokio::test]
    async fn read_nonexistent_file_returns_error() {
        let (req_tx, req_rx) = mpsc::channel(10);
        let (evt_tx, mut evt_rx) = mpsc::channel(10);

        let service = FsService::new(req_rx, evt_tx);
        let handle = tokio::spawn(service.run());

        req_tx
            .send(ServiceRequest::ReadFile {
                path: "/nonexistent/path/file.txt".to_string(),
            })
            .await
            .unwrap();

        drop(req_tx);

        let evt = evt_rx.recv().await.unwrap();
        match evt {
            ServiceEvent::Error { message } => {
                assert!(message.contains("Failed to read"));
            }
            _ => panic!("Expected error event"),
        }

        handle.await.unwrap();
    }

    #[tokio::test]
    async fn multiple_requests_processed_in_order() {
        let dir = TempDir::new().unwrap();
        let path1 = dir.path().join("file1.txt").to_str().unwrap().to_string();
        let path2 = dir.path().join("file2.txt").to_str().unwrap().to_string();
        let path3 = dir.path().join("file3.txt").to_str().unwrap().to_string();

        let (req_tx, req_rx) = mpsc::channel(10);
        let (evt_tx, mut evt_rx) = mpsc::channel(10);

        let service = FsService::new(req_rx, evt_tx);
        let handle = tokio::spawn(service.run());

        // Send multiple writes in order
        req_tx
            .send(ServiceRequest::WriteFile {
                path: path1.clone(),
                content: "content1".to_string(),
            })
            .await
            .unwrap();
        req_tx
            .send(ServiceRequest::WriteFile {
                path: path2.clone(),
                content: "content2".to_string(),
            })
            .await
            .unwrap();
        req_tx
            .send(ServiceRequest::WriteFile {
                path: path3.clone(),
                content: "content3".to_string(),
            })
            .await
            .unwrap();

        drop(req_tx);

        // Receive events in order
        let evt1 = evt_rx.recv().await.unwrap();
        let evt2 = evt_rx.recv().await.unwrap();
        let evt3 = evt_rx.recv().await.unwrap();

        match (evt1, evt2, evt3) {
            (
                ServiceEvent::FileWritten { path: p1 },
                ServiceEvent::FileWritten { path: p2 },
                ServiceEvent::FileWritten { path: p3 },
            ) => {
                assert_eq!(p1, path1);
                assert_eq!(p2, path2);
                assert_eq!(p3, path3);
            }
            _ => panic!("Expected three FileWritten events"),
        }

        handle.await.unwrap();
    }

    #[tokio::test]
    async fn service_continues_after_error() {
        let dir = TempDir::new().unwrap();
        let valid_path = dir.path().join("valid.txt").to_str().unwrap().to_string();

        let (req_tx, req_rx) = mpsc::channel(10);
        let (evt_tx, mut evt_rx) = mpsc::channel(10);

        let service = FsService::new(req_rx, evt_tx);
        let handle = tokio::spawn(service.run());

        // Request that will fail
        req_tx
            .send(ServiceRequest::ReadFile {
                path: "/nonexistent/file.txt".to_string(),
            })
            .await
            .unwrap();

        // Request that should succeed
        req_tx
            .send(ServiceRequest::WriteFile {
                path: valid_path.clone(),
                content: "after error".to_string(),
            })
            .await
            .unwrap();

        drop(req_tx);

        // First should be error
        let evt1 = evt_rx.recv().await.unwrap();
        assert!(matches!(evt1, ServiceEvent::Error { .. }));

        // Second should succeed (service recovered)
        let evt2 = evt_rx.recv().await.unwrap();
        assert!(matches!(evt2, ServiceEvent::FileWritten { .. }));

        handle.await.unwrap();
    }
}
