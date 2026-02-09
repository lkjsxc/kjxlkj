/// Core event loop extracted from main for file-size compliance.
use kjxlkj_core_state::EditorState;
use kjxlkj_core_types::{Action, Key, ServiceResponse};

use kjxlkj_host::{HostTerminal, TerminalGuard};
use kjxlkj_input::InputTask;
use kjxlkj_render::RenderTask;
use kjxlkj_services::ServiceSupervisor;

pub async fn run_inner(files: Vec<String>) -> anyhow::Result<()> {
    // Step 1: Detect terminal capabilities
    let (cols, rows) = HostTerminal::size()?;
    tracing::info!("Terminal size: {cols}x{rows}");

    // Step 2: Initialize editor state
    let mut state = EditorState::new(cols, rows);

    // Step 3: Open CLI files or keep scratch buffer
    for path in &files {
        match std::fs::read_to_string(path) {
            Ok(content) => state.open_file(path, &content),
            Err(e) => {
                tracing::warn!("Could not read {path}: {e}");
            }
        }
    }

    // Step 4: Create bounded channels
    let (action_tx, mut action_rx) = tokio::sync::mpsc::channel::<Action>(256);
    let (key_tx, mut key_rx) = tokio::sync::mpsc::channel::<Key>(256);
    let (service_resp_tx, mut service_resp_rx) = tokio::sync::mpsc::channel::<ServiceResponse>(256);
    let (quit_tx, _) = tokio::sync::broadcast::channel::<()>(1);

    // Snapshot watch channel
    let initial_snapshot = state.snapshot();
    let (snapshot_tx, snapshot_rx) = tokio::sync::watch::channel(initial_snapshot);

    // Step 5: Enter raw mode and alternate screen
    let _guard = TerminalGuard::enter()?;

    // Step 6: Signal watchers
    spawn_signal_handler(quit_tx.clone());

    // Step 7: Spawn render task
    let render_quit = quit_tx.subscribe();
    let render_handle =
        tokio::spawn(async move { RenderTask::run(snapshot_rx, render_quit).await });

    // Step 8: Spawn service supervisor
    let (_service_channels, _service_handles) = ServiceSupervisor::spawn(service_resp_tx, &quit_tx);

    // Step 9: Spawn input task
    let input_quit = quit_tx.subscribe();
    let input_handle =
        tokio::spawn(async move { InputTask::run(action_tx, key_tx, input_quit).await });

    // Step 10: Publish initial snapshot
    let _ = snapshot_tx.send(state.snapshot());

    // Step 11: Core select loop
    core_loop(
        &mut state,
        &mut action_rx,
        &mut key_rx,
        &mut service_resp_rx,
        &snapshot_tx,
    )
    .await;

    // --- Shutdown Sequence ---
    let _ = quit_tx.send(());
    let timeout = std::time::Duration::from_secs(2);
    let _ = tokio::time::timeout(timeout, async {
        let _ = input_handle.await;
        let _ = render_handle.await;
    })
    .await;

    Ok(())
}

fn spawn_signal_handler(quit_tx: tokio::sync::broadcast::Sender<()>) {
    tokio::spawn(async move {
        #[cfg(unix)]
        {
            use tokio::signal::unix::{signal, SignalKind};
            let mut sigterm = signal(SignalKind::terminate()).unwrap();
            let mut sighup = signal(SignalKind::hangup()).unwrap();
            tokio::select! {
                _ = sigterm.recv() => {},
                _ = sighup.recv() => {},
                _ = tokio::signal::ctrl_c() => {},
            }
            let _ = quit_tx.send(());
        }
        #[cfg(not(unix))]
        {
            let _ = tokio::signal::ctrl_c().await;
            let _ = quit_tx.send(());
        }
    });
}

async fn core_loop(
    state: &mut EditorState,
    action_rx: &mut tokio::sync::mpsc::Receiver<Action>,
    key_rx: &mut tokio::sync::mpsc::Receiver<Key>,
    service_resp_rx: &mut tokio::sync::mpsc::Receiver<ServiceResponse>,
    snapshot_tx: &tokio::sync::watch::Sender<kjxlkj_core_ui::EditorSnapshot>,
) {
    loop {
        tokio::select! {
            Some(action) = action_rx.recv() => {
                state.handle_action(action);
                if state.quit_requested { break; }
                let _ = snapshot_tx.send(state.snapshot());
            }
            Some(key) = key_rx.recv() => {
                state.handle_key(key);
                if state.quit_requested { break; }
                let _ = snapshot_tx.send(state.snapshot());
            }
            Some(resp) = service_resp_rx.recv() => {
                handle_service_response(resp);
                let _ = snapshot_tx.send(state.snapshot());
            }
        }
    }
}

fn handle_service_response(resp: ServiceResponse) {
    match resp {
        ServiceResponse::FileRead {
            request_id,
            content,
        } => {
            if let Ok(_text) = content {
                tracing::info!("File read complete (req {})", request_id);
            }
        }
        ServiceResponse::FileWritten { request_id, result } => {
            if let Err(e) = result {
                tracing::error!("Write failed (req {}): {}", request_id, e);
            }
        }
        _ => {
            tracing::debug!("Unhandled service response");
        }
    }
}
