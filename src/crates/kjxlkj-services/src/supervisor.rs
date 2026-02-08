//! Service supervisor: spawns and monitors all services.

use tokio::sync::{broadcast, mpsc};

use kjxlkj_core_types::{ServiceRequest, ServiceResponse};

/// Channels for a single service.
pub struct ServiceChannels {
    pub request_tx: mpsc::Sender<ServiceRequest>,
    pub request_rx: mpsc::Receiver<ServiceRequest>,
    pub response_tx: mpsc::Sender<ServiceResponse>,
}

/// The supervisor owns all service tasks and their channels.
pub struct ServiceSupervisor {
    /// Channel for services to send responses back to core.
    response_tx: mpsc::Sender<ServiceResponse>,
    /// Quit signal sender.
    quit_tx: broadcast::Sender<()>,
    /// Task handles.
    handles: Vec<tokio::task::JoinHandle<()>>,
}

impl ServiceSupervisor {
    /// Create a new supervisor.
    pub fn new(
        response_tx: mpsc::Sender<ServiceResponse>,
        quit_tx: broadcast::Sender<()>,
    ) -> Self {
        Self {
            response_tx,
            quit_tx,
            handles: Vec::new(),
        }
    }

    /// Create a new channel pair for a service.
    pub fn create_channels(
        &self,
    ) -> (
        mpsc::Sender<ServiceRequest>,
        mpsc::Receiver<ServiceRequest>,
        mpsc::Sender<ServiceResponse>,
    ) {
        let (req_tx, req_rx) = mpsc::channel(64);
        (req_tx, req_rx, self.response_tx.clone())
    }

    /// Get a quit receiver.
    pub fn quit_rx(&self) -> broadcast::Receiver<()> {
        self.quit_tx.subscribe()
    }

    /// Spawn a service task.
    pub fn spawn<F>(&mut self, name: &str, fut: F)
    where
        F: std::future::Future<Output = ()> + Send + 'static,
    {
        let handle = tokio::spawn(fut);
        self.handles.push(handle);
        let _ = name; // Used for logging in production.
    }

    /// Start all services.
    pub fn start_all(&mut self) {
        // FS service.
        let quit_rx = self.quit_rx();
        let resp_tx = self.response_tx.clone();
        self.spawn("fs", async move {
            kjxlkj_service_fs::FsService::new(resp_tx)
                .run(quit_rx)
                .await;
        });

        // Git service.
        let quit_rx = self.quit_rx();
        let resp_tx = self.response_tx.clone();
        self.spawn("git", async move {
            kjxlkj_service_git::GitService::new(resp_tx)
                .run(quit_rx)
                .await;
        });

        // Index service.
        let quit_rx = self.quit_rx();
        let resp_tx = self.response_tx.clone();
        self.spawn("index", async move {
            kjxlkj_service_index::IndexService::new(resp_tx)
                .run(quit_rx)
                .await;
        });
    }

    /// Wait for all services to shut down.
    pub async fn shutdown(self) {
        // Send quit signal.
        let _ = self.quit_tx.send(());

        // Wait for all tasks with a timeout.
        for handle in self.handles {
            let _ = tokio::time::timeout(
                std::time::Duration::from_secs(5),
                handle,
            )
            .await;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_supervisor() {
        let (resp_tx, _resp_rx) = mpsc::channel(256);
        let (quit_tx, _quit_rx) = broadcast::channel(1);
        let sup = ServiceSupervisor::new(resp_tx, quit_tx);
        assert!(sup.handles.is_empty());
    }
}
