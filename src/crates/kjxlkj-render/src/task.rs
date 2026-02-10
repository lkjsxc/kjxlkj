//! Render task: consumes EditorSnapshot via watch channel.

use kjxlkj_core_ui::EditorSnapshot;
use tokio::sync::{broadcast, watch};

use crate::grid::CellGrid;
use crate::painter;

/// Spawn the render task.
pub async fn spawn_render_task(
    mut snapshot_rx: watch::Receiver<EditorSnapshot>,
    mut quit_rx: broadcast::Receiver<()>,
) {
    let mut prev_grid: Option<CellGrid> = None;

    loop {
        tokio::select! {
            _ = quit_rx.recv() => {
                tracing::info!("Render task: quit signal");
                break;
            }
            result = snapshot_rx.changed() => {
                match result {
                    Ok(()) => {
                        let snapshot = snapshot_rx
                            .borrow_and_update()
                            .clone();
                        if let Err(e) = painter::paint(
                            &snapshot,
                            &mut prev_grid,
                        ) {
                            tracing::error!(
                                "Render error: {e}"
                            );
                        }
                    }
                    Err(_) => break,
                }
            }
        }
    }
}
