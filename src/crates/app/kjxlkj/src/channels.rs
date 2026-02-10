//! Channel setup.

use kjxlkj_core_types::InputAction;
use kjxlkj_core_ui::EditorSnapshot;
use tokio::sync::{broadcast, mpsc, watch};

/// Channel capacity for action queue.
pub const ACTION_CAPACITY: usize = 256;

/// All application channels.
pub struct Channels {
    /// Action sender.
    pub action_tx: mpsc::Sender<InputAction>,
    /// Action receiver (Option to allow taking).
    pub action_rx: Option<mpsc::Receiver<InputAction>>,
    /// Snapshot sender.
    pub snapshot_tx: watch::Sender<EditorSnapshot>,
    /// Snapshot receiver.
    pub snapshot_rx: watch::Receiver<EditorSnapshot>,
    /// Quit signal sender.
    pub quit_tx: broadcast::Sender<()>,
}

impl Channels {
    /// Create all channels.
    pub fn new() -> Self {
        let (action_tx, action_rx) = mpsc::channel(ACTION_CAPACITY);
        let (snapshot_tx, snapshot_rx) = watch::channel(EditorSnapshot::default());
        let (quit_tx, _) = broadcast::channel(1);

        Self {
            action_tx,
            action_rx: Some(action_rx),
            snapshot_tx,
            snapshot_rx,
            quit_tx,
        }
    }

    /// Take the action receiver.
    pub fn take_action_rx(&mut self) -> Option<mpsc::Receiver<InputAction>> {
        self.action_rx.take()
    }

    /// Get a quit signal receiver.
    pub fn quit_rx(&self) -> broadcast::Receiver<()> {
        self.quit_tx.subscribe()
    }
}

impl Default for Channels {
    fn default() -> Self {
        Self::new()
    }
}
