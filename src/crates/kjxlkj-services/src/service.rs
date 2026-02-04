//! Service trait and handle.

use tokio::sync::mpsc;
use std::future::Future;
use std::pin::Pin;

/// Service status.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ServiceStatus {
    /// Service is starting.
    Starting,
    /// Service is running.
    Running,
    /// Service is stopping.
    Stopping,
    /// Service has stopped.
    Stopped,
    /// Service has failed.
    Failed,
}

/// Message that can be sent to a service.
#[derive(Debug, Clone)]
pub enum ServiceMessage {
    /// Request to shutdown.
    Shutdown,
    /// Custom message.
    Custom(String),
}

/// Handle to a running service.
pub struct ServiceHandle {
    /// Service name.
    pub name: String,
    /// Current status.
    pub status: ServiceStatus,
    /// Channel to send messages.
    sender: mpsc::Sender<ServiceMessage>,
}

impl ServiceHandle {
    /// Create a new service handle.
    pub fn new(name: String, sender: mpsc::Sender<ServiceMessage>) -> Self {
        Self {
            name,
            status: ServiceStatus::Starting,
            sender,
        }
    }

    /// Request service shutdown.
    pub async fn shutdown(&self) -> Result<(), mpsc::error::SendError<ServiceMessage>> {
        self.sender.send(ServiceMessage::Shutdown).await
    }

    /// Send a custom message.
    pub async fn send(&self, msg: String) -> Result<(), mpsc::error::SendError<ServiceMessage>> {
        self.sender.send(ServiceMessage::Custom(msg)).await
    }
}

/// Service trait for background services.
pub trait Service: Send + Sync + 'static {
    /// Service name.
    fn name(&self) -> &str;

    /// Run the service.
    fn run(
        self: Box<Self>,
        rx: mpsc::Receiver<ServiceMessage>,
    ) -> Pin<Box<dyn Future<Output = ()> + Send>>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_service_status() {
        let status = ServiceStatus::Running;
        assert_eq!(status, ServiceStatus::Running);
    }

    #[test]
    fn test_service_message() {
        let msg = ServiceMessage::Shutdown;
        assert!(matches!(msg, ServiceMessage::Shutdown));
    }
}
