//! Runtime construction and the core event loop.

use kjxlkj_core_state::EditorState;
use kjxlkj_core_types::Action;
use kjxlkj_host::{enter_raw_mode, leave_raw_mode, terminal_size};
use kjxlkj_input::spawn_input_task;
use kjxlkj_render::spawn_render_task;
use kjxlkj_service_terminal::TerminalService;

use crate::channels::Channels;
use crate::cli::CliArgs;
use crate::services;
use crate::signals;

/// Run the editor.
pub async fn run(args: CliArgs) -> anyhow::Result<()> {
    enter_raw_mode()?;
    let (cols, rows) = terminal_size()?;
    let mut state = EditorState::new(cols, rows);
    let mut terminal_svc = TerminalService::new();

    // Auto-load session if no files provided
    if args.files.is_empty() {
        auto_load_session(&mut state);
    }

    // Load files from command line
    for path in &args.files {
        if path.exists() {
            let content = tokio::fs::read_to_string(path).await?;
            state.open_file(path.clone(), content);
        }
    }

    let initial_snapshot = state.snapshot();
    let mut ch = Channels::new(initial_snapshot);
    let _supervisor = services::spawn_services(ch.action_tx.clone(), ch.quit_tx.subscribe());
    let input_handle = tokio::spawn(spawn_input_task(
        ch.action_tx.clone(),
        ch.key_tx.clone(),
        ch.quit_tx.subscribe(),
    ));
    let render_handle = tokio::spawn(spawn_render_task(
        ch.snapshot_rx.clone(),
        ch.quit_tx.subscribe(),
    ));
    let quit_tx_sig = ch.quit_tx.clone();
    let _signal_handle = tokio::spawn(async move {
        signals::wait_for_signal(quit_tx_sig).await;
    });

    core_loop(
        &mut state,
        &mut terminal_svc,
        &mut ch.action_rx,
        &mut ch.key_rx,
        &ch.snapshot_tx,
        &ch.quit_tx,
    )
    .await;

    // Auto-save session on exit
    auto_save_session(&state);

    let _ = ch.quit_tx.send(());
    let _ = tokio::time::timeout(std::time::Duration::from_secs(2), async {
        let _ = input_handle.await;
        let _ = render_handle.await;
    })
    .await;
    leave_raw_mode()?;
    tracing::info!("kjxlkj exited cleanly");
    Ok(())
}

/// The core event loop.
async fn core_loop(
    state: &mut EditorState,
    terminal_svc: &mut TerminalService,
    action_rx: &mut tokio::sync::mpsc::Receiver<Action>,
    key_rx: &mut tokio::sync::mpsc::Receiver<kjxlkj_core_types::Key>,
    snapshot_tx: &tokio::sync::watch::Sender<kjxlkj_core_ui::EditorSnapshot>,
    quit_tx: &tokio::sync::broadcast::Sender<()>,
) {
    loop {
        tokio::select! {
            Some(action) = action_rx.recv() => {
                // Propagate resize to terminal instances
                if let Action::Resize(cols, rows) = &action {
                    for inst in terminal_svc.terminals.values_mut() {
                        inst.resize(*cols as usize, *rows as usize);
                    }
                }
                state.process_action(action);
            }
            Some(key) = key_rx.recv() => {
                state.process_key(key);
            }
        }
        let snap = state.snapshot();
        let _ = snapshot_tx.send(snap);
        if state.quit_requested {
            let _ = quit_tx.send(());
            break;
        }
    }
}

/// Auto-session file path for current working directory.
fn auto_session_path() -> Option<std::path::PathBuf> {
    let cwd = std::env::current_dir().ok()?;
    let hash = {
        let s = cwd.to_string_lossy();
        let mut h: u64 = 5381;
        for b in s.bytes() {
            h = h.wrapping_mul(33).wrapping_add(b as u64);
        }
        format!("{h:016x}")
    };
    let dir = dirs_path();
    Some(dir.join(format!("auto_{hash}.json")))
}

fn dirs_path() -> std::path::PathBuf {
    let home = std::env::var("HOME").unwrap_or_else(|_| "/tmp".to_string());
    std::path::PathBuf::from(home).join(".local/share/kjxlkj/sessions")
}

fn auto_save_session(state: &EditorState) {
    if let Some(path) = auto_session_path() {
        let session = state.session_save();
        if let Ok(json) = session.to_json() {
            let _ = std::fs::create_dir_all(path.parent().unwrap_or(std::path::Path::new(".")));
            let _ = std::fs::write(&path, json);
            tracing::info!("Auto-saved session to {}", path.display());
        }
    }
}

fn auto_load_session(state: &mut EditorState) {
    if let Some(path) = auto_session_path() {
        if path.exists() {
            if let Ok(json) = std::fs::read_to_string(&path) {
                match kjxlkj_core_state::session::SessionData::from_json(&json) {
                    Ok(session) => {
                        state.session_load(&session);
                        tracing::info!("Auto-loaded session from {}", path.display());
                    }
                    Err(e) => tracing::warn!("Failed to load session: {e}"),
                }
            }
        }
    }
}
