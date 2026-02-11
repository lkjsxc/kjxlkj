//! Core task per /docs/spec/architecture/runtime.md.
//!
//! Single-writer owner of EditorState. Receives keys and
//! actions from bounded mpsc channels, publishes snapshots
//! via watch channel.

use crate::channels::CoreReceivers;
use kjxlkj_core_state::EditorState;
use kjxlkj_core_types::EditorSnapshot;
use tokio::sync::{broadcast, watch};

/// Run the core select loop until quit.
///
/// After each state mutation, publishes a new snapshot.
/// On quit, broadcasts the quit signal to all tasks.
pub async fn core_task(
    mut state: EditorState,
    mut receivers: CoreReceivers,
    snap_tx: watch::Sender<EditorSnapshot>,
    quit_tx: broadcast::Sender<()>,
) {
    // Publish initial snapshot.
    let _ = snap_tx.send(state.snapshot());

    loop {
        tokio::select! {
            biased;
            // Process key events (mode dispatch pipeline).
            key = receivers.key_rx.recv() => {
                match key {
                    Some((k, m)) => {
                        state.handle_key(&k, &m);
                    }
                    None => break,
                }
            }
            // Process direct actions (resize, paste, etc.).
            action = receivers.action_rx.recv() => {
                match action {
                    Some(a) => {
                        state.apply_action(a);
                    }
                    None => break,
                }
            }
        }

        // Check quit after processing.
        if state.quit_requested {
            let _ = quit_tx.send(());
            break;
        }

        // Publish new snapshot for render task.
        let _ = snap_tx.send(state.snapshot());
    }
}
