//! Channel topology per /docs/spec/architecture/runtime.md.
//!
//! Bounded channels connect tasks:
//! - Action bus (mpsc, 256): Input task → Core task
//! - Key bus (mpsc, 256): Input task → Core task
//! - Snapshot bus (watch, latest-wins): Core task → Render task
//! - Quit signal (broadcast, 1): Core task → all tasks

use kjxlkj_core_types::{
    Action, EditorSnapshot, Key, KeyModifiers,
};
use tokio::sync::{broadcast, mpsc, watch};

/// Senders handed to the input task.
pub struct InputSenders {
    pub action_tx: mpsc::Sender<Action>,
    pub key_tx: mpsc::Sender<(Key, KeyModifiers)>,
}

/// Receivers consumed by the core task.
pub struct CoreReceivers {
    pub action_rx: mpsc::Receiver<Action>,
    pub key_rx: mpsc::Receiver<(Key, KeyModifiers)>,
}

/// Create the full channel set for the runtime.
///
/// Returns (input-side senders, core-side receivers,
///          snapshot sender, snapshot receiver,
///          quit sender).
pub fn create_channels() -> (
    InputSenders,
    CoreReceivers,
    watch::Sender<EditorSnapshot>,
    watch::Receiver<EditorSnapshot>,
    broadcast::Sender<()>,
) {
    let (action_tx, action_rx) = mpsc::channel(256);
    let (key_tx, key_rx) = mpsc::channel(256);
    let (snap_tx, snap_rx) =
        watch::channel(EditorSnapshot::default());
    let (quit_tx, _) = broadcast::channel(1);

    (
        InputSenders { action_tx, key_tx },
        CoreReceivers { action_rx, key_rx },
        snap_tx,
        snap_rx,
        quit_tx,
    )
}
