//! Async entry point.
//!
//! See /docs/spec/architecture/startup.md "Async Initialization"
//! and /docs/spec/architecture/runtime.md "Event Loop".
//!
//! Startup order (normative):
//! 1. Detect terminal capabilities
//! 2. Initialize editor state
//! 3. Open CLI files or create scratch buffer
//! 4. Create bounded channels
//! 5. Enter raw mode and alternate screen
//! 6. Start signal watchers
//! 7. Spawn render task
//! 8. Spawn service tasks (stub)
//! 9. Spawn input task
//! 10. Publish initial snapshot and render
//! 11. Enter core select loop

use crate::channels;
use crate::core_task;
use crate::input_task;
use crate::render_task;
use crate::signal;
use anyhow::Result;
use kjxlkj_core_state::EditorState;
use std::time::Duration;

/// Async initialization and core event loop.
pub async fn run(args: Vec<String>) -> Result<()> {
    // Step 1: Detect terminal capabilities.
    let (cols, rows) = kjxlkj_host::terminal_size()?;

    // Step 2-3: Initialize editor state with scratch buffer.
    let state = EditorState::new(cols, rows);

    // Step 4: Create bounded channels.
    let (input_senders, core_receivers, snap_tx, snap_rx, quit_tx) =
        channels::create_channels();

    // Step 5: Enter raw mode and alternate screen.
    kjxlkj_host::setup_terminal()?;

    // Step 6: Start signal watchers.
    let _signal_handle = signal::spawn_signal_watchers(
        input_senders.action_tx.clone(),
    );

    // Step 7: Spawn render task.
    let quit_rx_render = quit_tx.subscribe();
    let render_handle = tokio::spawn(
        render_task::render_task(snap_rx, quit_rx_render),
    );

    // Step 8: Service tasks (stub — not yet implemented).

    // Step 9: Spawn input task.
    let quit_rx_input = quit_tx.subscribe();
    let input_handle = tokio::spawn(
        input_task::input_task(input_senders, quit_rx_input),
    );

    // Steps 10-11: Core task runs on this thread (owns state).
    core_task::core_task(
        state, core_receivers, snap_tx, quit_tx,
    )
    .await;

    // Shutdown: wait for tasks with bounded timeout.
    let timeout = Duration::from_secs(2);
    let _ = tokio::time::timeout(timeout, input_handle).await;
    let _ = tokio::time::timeout(timeout, render_handle).await;

    // Restore terminal.
    kjxlkj_host::restore_terminal()?;
    Ok(())
}

