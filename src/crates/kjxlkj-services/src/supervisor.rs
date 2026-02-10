//! Service supervisor implementation.

use kjxlkj_core_types::Action;
use tokio::sync::{broadcast, mpsc};

/// Coordinates all service tasks.
pub struct ServiceSupervisor;

impl ServiceSupervisor {
    /// Spawn all service tasks. Returns join handles.
    pub fn spawn(_action_tx: mpsc::Sender<Action>, _quit_rx: broadcast::Receiver<()>) -> Self {
        tracing::info!("Service supervisor started");
        Self
    }
}
