//! Service spawn and shutdown hooks.

use kjxlkj_core_types::Action;
use kjxlkj_services::ServiceSupervisor;
use tokio::sync::{broadcast, mpsc};

/// Spawn all background service tasks.
pub fn spawn_services(
    action_tx: mpsc::Sender<Action>,
    quit_rx: broadcast::Receiver<()>,
) -> ServiceSupervisor {
    ServiceSupervisor::spawn(action_tx, quit_rx)
}
