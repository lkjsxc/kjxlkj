//! Core task implementation.

use crate::command;
use crate::editing;
use crate::EditorState;
use kjxlkj_core_mode::{dispatch_key, HandleResult, ModeAction};
use kjxlkj_core_types::{InputAction, KeyEvent, WindowContent};
use kjxlkj_core_ui::EditorSnapshot;
use tokio::sync::{broadcast, mpsc, watch};
use tracing::{debug, info, warn};

/// Core task handle for sending actions.
pub struct CoreHandle {
    action_tx: mpsc::Sender<InputAction>,
}

impl CoreHandle {
    /// Send an action to the core.
    pub async fn send(
        &self,
        action: InputAction,
    ) -> Result<(), mpsc::error::SendError<InputAction>> {
        self.action_tx.send(action).await
    }
}

/// Core task that owns editor state.
pub struct CoreTask {
    state: EditorState,
    action_rx: mpsc::Receiver<InputAction>,
    snapshot_tx: watch::Sender<EditorSnapshot>,
    quit_tx: broadcast::Sender<()>,
}

impl CoreTask {
    /// Create a new core task.
    pub fn new(
        action_rx: mpsc::Receiver<InputAction>,
        snapshot_tx: watch::Sender<EditorSnapshot>,
        quit_tx: broadcast::Sender<()>,
    ) -> Self {
        Self {
            state: EditorState::new(),
            action_rx,
            snapshot_tx,
            quit_tx,
        }
    }

    /// Run the core loop.
    pub async fn run(mut self) {
        info!("Core task started");

        let _ = self.snapshot_tx.send(self.state.snapshot());

        while let Some(action) = self.action_rx.recv().await {
            debug!(?action, "Received action");

            match action {
                InputAction::Key(key) => self.handle_key(key),
                InputAction::Resize(cols, rows) => {
                    self.state.set_terminal_size(cols, rows);
                }
                InputAction::Paste(text) => {
                    self.handle_paste(&text);
                }
                InputAction::FocusGained => {
                    debug!("Focus gained");
                }
                InputAction::FocusLost => {
                    debug!("Focus lost");
                }
            }

            let _ = self.snapshot_tx.send(self.state.snapshot());

            if self.state.should_quit {
                info!("Quit requested");
                let _ = self.quit_tx.send(());
                break;
            }
        }

        info!("Core task ended");
    }

    fn handle_key(&mut self, key: KeyEvent) {
        let result = dispatch_key(&mut self.state.mode, &key);

        match result {
            HandleResult::Consumed(actions) => {
                for action in actions {
                    self.apply_mode_action(action);
                }
            }
            HandleResult::Ignored => {
                debug!(?key, "Key ignored");
            }
            HandleResult::Pending => {
                debug!("Key pending more input");
            }
        }
    }

    fn handle_paste(&mut self, text: &str) {
        if self.state.mode.mode.is_insert() {
            if let Some(window) = self.state.windows.focused() {
                if let WindowContent::Buffer(buffer_id) = &window.content {
                    let cursor = window.cursor;
                    if let Some(buffer) = self.state.buffers.get_mut(*buffer_id) {
                        buffer.insert(cursor, text);
                    }
                }
            }
        }
    }

    fn apply_mode_action(&mut self, action: ModeAction) {
        match action {
            ModeAction::EnterInsert(pos) => {
                editing::enter_insert(&mut self.state, pos);
            }
            ModeAction::ReturnNormal => {
                editing::clamp_cursor(&mut self.state);
            }
            ModeAction::InsertText(text) => {
                editing::insert_text(&mut self.state, &text);
            }
            ModeAction::DeleteAtCursor(direction) => {
                editing::delete_at_cursor(&mut self.state, direction);
            }
            ModeAction::MoveCursor(motion, count) => {
                editing::move_cursor(&mut self.state, motion, count);
            }
            ModeAction::ExecuteCommand(cmd) => {
                command::execute_command(&mut self.state, &cmd);
            }
            ModeAction::Undo => {
                command::undo(&mut self.state);
            }
            ModeAction::Redo => {
                command::redo(&mut self.state);
            }
            ModeAction::Quit { force: _ } => {
                self.state.quit();
            }
            _ => {
                warn!(?action, "Unhandled mode action");
            }
        }
    }
}
