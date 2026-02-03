//! Message bus for service communication.

use tokio::sync::mpsc;

use kjxlkj_core_types::{ServiceEvent, ServiceRequest};

/// Message bus for communication between core and services.
#[derive(Debug)]
pub struct MessageBus {
    request_tx: mpsc::Sender<ServiceRequest>,
    request_rx: mpsc::Receiver<ServiceRequest>,
    event_tx: mpsc::Sender<ServiceEvent>,
    event_rx: mpsc::Receiver<ServiceEvent>,
}

impl MessageBus {
    /// Creates a new message bus with the given capacity.
    pub fn new(capacity: usize) -> Self {
        let (request_tx, request_rx) = mpsc::channel(capacity);
        let (event_tx, event_rx) = mpsc::channel(capacity);
        Self {
            request_tx,
            request_rx,
            event_tx,
            event_rx,
        }
    }

    /// Returns a sender for service requests.
    pub fn request_sender(&self) -> mpsc::Sender<ServiceRequest> {
        self.request_tx.clone()
    }

    /// Returns a receiver for service requests.
    pub fn request_receiver(&mut self) -> &mut mpsc::Receiver<ServiceRequest> {
        &mut self.request_rx
    }

    /// Returns a sender for service events.
    pub fn event_sender(&self) -> mpsc::Sender<ServiceEvent> {
        self.event_tx.clone()
    }

    /// Returns a receiver for service events.
    pub fn event_receiver(&mut self) -> &mut mpsc::Receiver<ServiceEvent> {
        &mut self.event_rx
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn bus_roundtrip() {
        let mut bus = MessageBus::new(10);
        let tx = bus.request_sender();
        tx.send(ServiceRequest::ReadFile {
            path: "test.txt".to_string(),
        })
        .await
        .unwrap();
        let req = bus.request_receiver().recv().await.unwrap();
        assert!(matches!(req, ServiceRequest::ReadFile { .. }));
    }
}
