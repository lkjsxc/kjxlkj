//! Typed channel wiring for the runtime.
//!
//! Channels follow the topology described in runtime.md:
//! - action_bus: mpsc 256 (core ← input/services)
//! - key_bus:    mpsc 256 (core ← input)
//! - snapshot:   watch    (core → render, latest-wins)
//! - quit:       broadcast (core → all tasks)

use kjxlkj_core_types::{Action, Key};
use kjxlkj_core_ui::EditorSnapshot;
use tokio::sync::{broadcast, mpsc, watch};

/// All channels used by the editor runtime.
pub struct Channels {
    pub action_tx: mpsc::Sender<Action>,
    pub action_rx: mpsc::Receiver<Action>,
    pub key_tx: mpsc::Sender<Key>,
    pub key_rx: mpsc::Receiver<Key>,
    pub snapshot_tx: watch::Sender<EditorSnapshot>,
    pub snapshot_rx: watch::Receiver<EditorSnapshot>,
    pub quit_tx: broadcast::Sender<()>,
}

impl Channels {
    /// Create all channels with an initial snapshot.
    pub fn new(initial_snapshot: EditorSnapshot) -> Self {
        let (action_tx, action_rx) = mpsc::channel(256);
        let (key_tx, key_rx) = mpsc::channel(256);
        let (snapshot_tx, snapshot_rx) = watch::channel(initial_snapshot);
        let (quit_tx, _) = broadcast::channel(16);

        Self {
            action_tx,
            action_rx,
            key_tx,
            key_rx,
            snapshot_tx,
            snapshot_rx,
            quit_tx,
        }
    }
}
