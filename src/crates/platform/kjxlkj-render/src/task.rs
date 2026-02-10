//! Render task.

use crate::paint;
use kjxlkj_core_ui::EditorSnapshot;
use tokio::sync::{watch, broadcast};
use tracing::{info, debug, error};

/// Render task that paints editor snapshots.
pub struct RenderTask {
    snapshot_rx: watch::Receiver<EditorSnapshot>,
    quit_rx: broadcast::Receiver<()>,
}

impl RenderTask {
    /// Create a new render task.
    pub fn new(
        snapshot_rx: watch::Receiver<EditorSnapshot>,
        quit_rx: broadcast::Receiver<()>,
    ) -> Self {
        Self { snapshot_rx, quit_rx }
    }

    /// Run the render loop.
    pub async fn run(mut self) {
        info!("Render task started");

        loop {
            tokio::select! {
                biased;

                _ = self.quit_rx.recv() => {
                    info!("Render task received quit signal");
                    break;
                }

                result = self.snapshot_rx.changed() => {
                    match result {
                        Ok(()) => {
                            let snapshot = self.snapshot_rx.borrow().clone();
                            debug!(seq = snapshot.sequence, "Rendering snapshot");

                            if let Err(e) = paint(&snapshot) {
                                error!(?e, "Failed to paint");
                            }
                        }
                        Err(_) => {
                            info!("Snapshot channel closed");
                            break;
                        }
                    }
                }
            }
        }

        info!("Render task ended");
    }
}
