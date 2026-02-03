//! Service supervisor.

use tokio::sync::mpsc;

use kjxlkj_core_types::{ServiceEvent, ServiceRequest};

/// Supervises all async services.
#[derive(Debug)]
pub struct ServiceSupervisor {
    request_tx: mpsc::Sender<ServiceRequest>,
    event_rx: mpsc::Receiver<ServiceEvent>,
    shutdown: bool,
}

impl ServiceSupervisor {
    /// Creates a new supervisor.
    pub fn new(
        request_tx: mpsc::Sender<ServiceRequest>,
        event_rx: mpsc::Receiver<ServiceEvent>,
    ) -> Self {
        Self {
            request_tx,
            event_rx,
            shutdown: false,
        }
    }

    /// Sends a request to services.
    pub async fn send_request(
        &self,
        request: ServiceRequest,
    ) -> Result<(), mpsc::error::SendError<ServiceRequest>> {
        self.request_tx.send(request).await
    }

    /// Receives an event from services (non-blocking).
    pub fn try_recv_event(&mut self) -> Option<ServiceEvent> {
        self.event_rx.try_recv().ok()
    }

    /// Receives an event from services (blocking).
    pub async fn recv_event(&mut self) -> Option<ServiceEvent> {
        self.event_rx.recv().await
    }

    /// Initiates shutdown.
    pub fn shutdown(&mut self) {
        self.shutdown = true;
    }

    /// Returns true if shutdown was requested.
    pub fn is_shutdown(&self) -> bool {
        self.shutdown
    }
}
