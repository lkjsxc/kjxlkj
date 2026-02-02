//! Service supervisor.

use crate::MessageBus;
use tokio::task::JoinHandle;

/// Service supervisor managing all services.
pub struct ServiceSupervisor {
    /// Message bus.
    bus: MessageBus,
    /// Service handles.
    handles: Vec<JoinHandle<()>>,
}

impl ServiceSupervisor {
    /// Creates a new supervisor.
    pub fn new(bus: MessageBus) -> Self {
        Self {
            bus,
            handles: Vec::new(),
        }
    }

    /// Starts all services.
    pub fn start(&mut self) {
        // Services are started lazily when needed
    }

    /// Stops all services.
    pub async fn stop(&mut self) {
        for handle in self.handles.drain(..) {
            handle.abort();
        }
    }

    /// Returns the message bus.
    pub fn bus(&self) -> &MessageBus {
        &self.bus
    }
}
