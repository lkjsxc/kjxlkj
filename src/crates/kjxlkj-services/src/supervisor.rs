//! Service supervisor.

use tokio::sync::mpsc;

use crate::{BusSender, CoreMessage};

/// Service supervisor manages all background services.
pub struct Supervisor {
    core_tx: mpsc::Sender<CoreMessage>,
    core_rx: mpsc::Receiver<CoreMessage>,
}

impl Supervisor {
    /// Create a new supervisor.
    pub fn new() -> Self {
        let (tx, rx) = mpsc::channel(64);
        Self {
            core_tx: tx,
            core_rx: rx,
        }
    }

    /// Get a sender for core messages.
    pub fn core_sender(&self) -> mpsc::Sender<CoreMessage> {
        self.core_tx.clone()
    }

    /// Start all services.
    pub async fn start(&mut self, _bus: BusSender) {
        // Services would be spawned here
        // For now, this is a placeholder
    }

    /// Shutdown all services.
    pub async fn shutdown(&mut self) {
        let _ = self.core_tx.send(CoreMessage::Shutdown).await;
    }
}

impl Default for Supervisor {
    fn default() -> Self {
        Self::new()
    }
}
