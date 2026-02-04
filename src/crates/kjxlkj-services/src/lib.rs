//! Service supervisor and wiring.
//!
//! Coordinates all background services.

pub use kjxlkj_service_fs as fs;
pub use kjxlkj_service_git as git;
pub use kjxlkj_service_index as index;
pub use kjxlkj_service_lsp as lsp;
pub use kjxlkj_service_terminal as terminal;

use tokio::sync::mpsc;

/// Message for service communication.
#[derive(Debug)]
pub enum ServiceMessage {
    /// Shutdown request.
    Shutdown,
}

/// Service supervisor.
pub struct ServiceSupervisor {
    _tx: mpsc::Sender<ServiceMessage>,
}

impl ServiceSupervisor {
    /// Create a new service supervisor.
    pub fn new() -> Self {
        let (tx, _rx) = mpsc::channel(32);
        Self { _tx: tx }
    }

    /// Start all services.
    pub async fn start(&self) {
        // Services are placeholders for now.
    }

    /// Shutdown all services.
    pub async fn shutdown(&self) {
        // Placeholder.
    }
}

impl Default for ServiceSupervisor {
    fn default() -> Self {
        Self::new()
    }
}
