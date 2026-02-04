//! Service supervisor.

use tokio::sync::mpsc;
use tracing::info;

use crate::message::{ServiceMessage, ServiceRequest};

/// Service supervisor manages background services.
pub struct ServiceSupervisor {
    /// Sender for service requests.
    request_tx: mpsc::Sender<ServiceRequest>,
    /// Receiver for service messages.
    message_rx: mpsc::Receiver<ServiceMessage>,
    /// Whether the supervisor is running.
    running: bool,
}

impl ServiceSupervisor {
    /// Create a new service supervisor.
    pub fn new() -> (Self, mpsc::Sender<ServiceMessage>, mpsc::Receiver<ServiceRequest>) {
        let (request_tx, request_rx) = mpsc::channel(256);
        let (message_tx, message_rx) = mpsc::channel(256);

        let supervisor = Self {
            request_tx,
            message_rx,
            running: false,
        };

        (supervisor, message_tx, request_rx)
    }

    /// Start the supervisor.
    pub fn start(&mut self) {
        self.running = true;
        info!("Service supervisor started");
    }

    /// Stop the supervisor.
    pub fn stop(&mut self) {
        self.running = false;
        info!("Service supervisor stopped");
    }

    /// Send a request to services.
    pub async fn request(&self, req: ServiceRequest) -> Result<(), mpsc::error::SendError<ServiceRequest>> {
        self.request_tx.send(req).await
    }

    /// Try to receive a message from services.
    pub fn try_recv(&mut self) -> Option<ServiceMessage> {
        self.message_rx.try_recv().ok()
    }

    /// Check if supervisor is running.
    pub fn is_running(&self) -> bool {
        self.running
    }
}

impl Default for ServiceSupervisor {
    fn default() -> Self {
        Self::new().0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn supervisor_lifecycle() {
        let (mut supervisor, _msg_tx, _req_rx) = ServiceSupervisor::new();
        assert!(!supervisor.is_running());

        supervisor.start();
        assert!(supervisor.is_running());

        supervisor.stop();
        assert!(!supervisor.is_running());
    }
}
