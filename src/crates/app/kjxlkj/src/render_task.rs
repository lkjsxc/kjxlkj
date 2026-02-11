//! Render task per /docs/spec/architecture/runtime.md.
//!
//! Watches the EditorSnapshot channel and renders each
//! new snapshot to the terminal. Uses latest-wins semantics
//! so rapid updates are coalesced automatically by the
//! watch channel.

use kjxlkj_core_types::EditorSnapshot;
use tokio::sync::{broadcast, watch};

/// Run the render task until quit signal.
pub async fn render_task(
    mut snap_rx: watch::Receiver<EditorSnapshot>,
    mut quit_rx: broadcast::Receiver<()>,
) {
    loop {
        tokio::select! {
            biased;
            _ = quit_rx.recv() => break,
            result = snap_rx.changed() => {
                match result {
                    Ok(()) => {
                        let snapshot =
                            snap_rx.borrow().clone();
                        if let Err(e) =
                            kjxlkj_render::render(&snapshot)
                        {
                            tracing::error!(
                                "render error: {}", e
                            );
                        }
                    }
                    Err(_) => break,
                }
            }
        }
    }
}
