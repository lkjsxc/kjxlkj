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

    #[test]
    fn test_service_status_starting() {
        let status = ServiceStatus::Starting;
        assert_eq!(status, ServiceStatus::Starting);
    }

    #[test]
    fn test_service_status_stopping() {
        let status = ServiceStatus::Stopping;
        assert_eq!(status, ServiceStatus::Stopping);
    }

    #[test]
    fn test_service_status_stopped() {
        let status = ServiceStatus::Stopped;
        assert_eq!(status, ServiceStatus::Stopped);
    }

    #[test]
    fn test_service_status_failed() {
        let status = ServiceStatus::Failed;
        assert_eq!(status, ServiceStatus::Failed);
    }

    #[test]
    fn test_service_message_custom() {
        let msg = ServiceMessage::Custom("test".to_string());
        assert!(matches!(msg, ServiceMessage::Custom(_)));
    }

    #[test]
    fn test_service_handle_new() {
        let (tx, _rx) = mpsc::channel(1);
        let handle = ServiceHandle::new("test".to_string(), tx);
        assert_eq!(handle.name, "test");
        assert_eq!(handle.status, ServiceStatus::Starting);
    }

    #[tokio::test]
    async fn test_service_handle_shutdown() {
        let (tx, mut rx) = mpsc::channel(1);
        let handle = ServiceHandle::new("test".to_string(), tx);
        handle.shutdown().await.unwrap();
        let msg = rx.recv().await.unwrap();
        assert!(matches!(msg, ServiceMessage::Shutdown));
    }

    #[tokio::test]
    async fn test_service_handle_send() {
        let (tx, mut rx) = mpsc::channel(1);
        let handle = ServiceHandle::new("test".to_string(), tx);
        handle.send("hello".to_string()).await.unwrap();
        let msg = rx.recv().await.unwrap();
        assert!(matches!(msg, ServiceMessage::Custom(s) if s == "hello"));
    }

    #[test]
    fn test_service_status_clone() {
        let status = ServiceStatus::Running;
        let cloned = status.clone();
        assert_eq!(status, cloned);
    }

    #[test]
    fn test_service_status_debug() {
        let status = ServiceStatus::Running;
        let debug = format!("{:?}", status);
        assert!(debug.contains("Running"));
    }

    #[test]
    fn test_service_message_clone() {
        let msg = ServiceMessage::Custom("test".to_string());
        let cloned = msg.clone();
        assert!(matches!(cloned, ServiceMessage::Custom(s) if s == "test"));
    }

    #[test]
    fn test_service_message_debug() {
        let msg = ServiceMessage::Shutdown;
        let debug = format!("{:?}", msg);
        assert!(debug.contains("Shutdown"));
    }

    #[test]
    fn test_service_status_all_variants_eq() {
        let variants = [
            ServiceStatus::Starting,
            ServiceStatus::Running,
            ServiceStatus::Stopping,
            ServiceStatus::Stopped,
            ServiceStatus::Failed,
        ];
        for (i, a) in variants.iter().enumerate() {
            for (j, b) in variants.iter().enumerate() {
                if i == j {
                    assert_eq!(a, b);
                } else {
                    assert_ne!(a, b);
                }
            }
        }
    }

    #[test]
    fn test_service_handle_name() {
        let (tx, _rx) = mpsc::channel(1);
        let handle = ServiceHandle::new("my-service".to_string(), tx);
        assert_eq!(handle.name, "my-service");
    }

    #[test]
    fn test_service_handle_initial_status() {
        let (tx, _rx) = mpsc::channel(1);
        let handle = ServiceHandle::new("svc".to_string(), tx);
        assert!(matches!(handle.status, ServiceStatus::Starting));
    }

    #[test]
    fn test_service_message_custom_content() {
        let msg = ServiceMessage::Custom("payload".to_string());
        if let ServiceMessage::Custom(s) = msg {
            assert_eq!(s, "payload");
        } else {
            panic!("Expected Custom variant");
        }
    }

    #[test]
    fn test_service_status_copy() {
        let status = ServiceStatus::Running;
        let copied = status;
        assert_eq!(status, copied);
    }

    #[test]
    fn test_service_message_shutdown_clone() {
        let msg = ServiceMessage::Shutdown;
        let cloned = msg.clone();
        assert!(matches!(cloned, ServiceMessage::Shutdown));
    }

    #[test]
    fn test_service_status_not_eq_different() {
        assert_ne!(ServiceStatus::Running, ServiceStatus::Stopped);
        assert_ne!(ServiceStatus::Starting, ServiceStatus::Failed);
    }

    #[test]
    fn test_service_message_custom_empty() {
        let msg = ServiceMessage::Custom(String::new());
        if let ServiceMessage::Custom(s) = msg {
            assert!(s.is_empty());
        }
    }

    #[test]
    fn test_service_status_all_debug() {
        for status in [
            ServiceStatus::Starting,
            ServiceStatus::Running,
            ServiceStatus::Stopping,
            ServiceStatus::Stopped,
            ServiceStatus::Failed,
        ] {
            let debug = format!("{:?}", status);
            assert!(!debug.is_empty());
        }
    }

    #[test]
    fn test_service_handle_name_empty() {
        let (tx, _rx) = mpsc::channel(1);
        let handle = ServiceHandle::new(String::new(), tx);
        assert!(handle.name.is_empty());
    }
}
