//! Message bus for service communication.

use tokio::sync::mpsc;

/// Message from services to core.
#[derive(Debug, Clone)]
pub enum ServiceMessage {
    /// File system event.
    FileChanged { path: String },
    /// External command output.
    CommandOutput { output: String },
    /// Service error.
    Error { service: String, message: String },
}

/// Message from core to services.
#[derive(Debug, Clone)]
pub enum CoreMessage {
    /// Shutdown all services.
    Shutdown,
    /// Run external command.
    RunCommand { cmd: String },
}

/// Create a message bus channel pair.
pub fn create_bus() -> (BusSender, BusReceiver) {
    let (tx, rx) = mpsc::channel(256);
    (BusSender(tx), BusReceiver(rx))
}

/// Sender side of the message bus.
#[derive(Clone)]
pub struct BusSender(mpsc::Sender<ServiceMessage>);

impl BusSender {
    /// Send a message.
    pub async fn send(&self, msg: ServiceMessage) -> Result<(), mpsc::error::SendError<ServiceMessage>> {
        self.0.send(msg).await
    }
}

/// Receiver side of the message bus.
pub struct BusReceiver(mpsc::Receiver<ServiceMessage>);

impl BusReceiver {
    /// Receive a message.
    pub async fn recv(&mut self) -> Option<ServiceMessage> {
        self.0.recv().await
    }
}
