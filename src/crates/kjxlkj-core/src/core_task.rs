//! Core task implementation.

use kjxlkj_core_state::{BufferState, EditorState};
use kjxlkj_core_types::Cursor;
use kjxlkj_core_ui::EditorSnapshot;
use std::path::PathBuf;
use tokio::sync::{mpsc, watch};

use crate::intent_handler::process_intent;
use crate::{Action, ActionResult};

/// The core task manages all editor state.
pub struct CoreTask {
    /// Editor state.
    state: EditorState,
    /// Action receiver.
    action_rx: mpsc::Receiver<Action>,
    /// Result sender.
    result_tx: mpsc::Sender<ActionResult>,
    /// Snapshot sender.
    snapshot_tx: watch::Sender<EditorSnapshot>,
    /// Running flag.
    pub(crate) running: bool,
}

/// Handle for communicating with the core task.
pub struct CoreHandle {
    /// Action sender.
    action_tx: mpsc::Sender<Action>,
    /// Snapshot receiver.
    snapshot_rx: watch::Receiver<EditorSnapshot>,
}

impl CoreTask {
    /// Creates a new core task.
    pub fn new() -> (Self, CoreHandle) {
        let (action_tx, action_rx) = mpsc::channel(256);
        let (result_tx, _result_rx) = mpsc::channel::<ActionResult>(256);
        let (snapshot_tx, snapshot_rx) = watch::channel(EditorSnapshot::default());

        let task = Self {
            state: EditorState::new(),
            action_rx,
            result_tx,
            snapshot_tx,
            running: true,
        };

        let handle = CoreHandle {
            action_tx,
            snapshot_rx,
        };

        (task, handle)
    }

    /// Runs the core task.
    pub async fn run(mut self) {
        while self.running {
            if let Some(action) = self.action_rx.recv().await {
                let result = self.process_action(action);
                let _ = self.result_tx.send(result).await;
            } else {
                break;
            }
        }
    }

    /// Processes an action.
    fn process_action(&mut self, action: Action) -> ActionResult {
        match action {
            Action::Intent(intent) => {
                let result = process_intent(&mut self.state, intent);
                self.publish_snapshot();
                if matches!(result, ActionResult::Quit) {
                    self.running = false;
                }
                result
            }
            Action::Resize(dims) => {
                self.state.dimensions = dims;
                self.publish_snapshot();
                ActionResult::Ok
            }
            Action::OpenFile { path } => {
                self.open_file(&path);
                self.publish_snapshot();
                ActionResult::Ok
            }
            Action::Save => ActionResult::Ok,
            Action::SaveAs { path: _ } => ActionResult::Ok,
            Action::Quit => {
                self.running = false;
                ActionResult::Quit
            }
            Action::ForceQuit => {
                self.running = false;
                ActionResult::Quit
            }
            Action::RequestSnapshot => {
                self.publish_snapshot();
                ActionResult::Snapshot
            }
        }
    }

    /// Opens a file and loads it into the current buffer.
    fn open_file(&mut self, path: &str) {
        let path_buf = PathBuf::from(path);

        // Read file contents
        let content = match std::fs::read_to_string(&path_buf) {
            Ok(c) => c,
            Err(_) => String::new(),
        };

        // Get active window's buffer
        let Some(window) = self.state.windows.get(&self.state.layout.active) else {
            return;
        };
        let buffer_id = window.buffer_id;

        // Update buffer with file content
        let new_buffer = BufferState::from_content(buffer_id, &content)
            .with_path(path_buf.clone());

        self.state.buffers.insert(buffer_id, new_buffer);

        // Reset cursor to start
        if let Some(window) = self.state.windows.get_mut(&self.state.layout.active) {
            window.cursor = Cursor::origin();
        }
    }

    /// Publishes a snapshot.
    fn publish_snapshot(&self) {
        let snapshot = self.state.snapshot();
        let _ = self.snapshot_tx.send(snapshot);
    }
}

impl CoreHandle {
    /// Sends an action to the core.
    pub async fn send(&self, action: Action) -> Result<(), mpsc::error::SendError<Action>> {
        self.action_tx.send(action).await
    }

    /// Gets the current snapshot.
    pub fn snapshot(&self) -> EditorSnapshot {
        self.snapshot_rx.borrow().clone()
    }

    /// Watches for snapshot changes.
    pub fn watch_snapshots(&self) -> watch::Receiver<EditorSnapshot> {
        self.snapshot_rx.clone()
    }
}

impl Default for CoreTask {
    fn default() -> Self {
        Self::new().0
    }
}
