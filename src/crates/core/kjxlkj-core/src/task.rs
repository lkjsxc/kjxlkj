//! Core task implementation.

use crate::EditorState;
use kjxlkj_core_mode::{dispatch_key, HandleResult, ModeAction, InsertPosition};
use kjxlkj_core_types::{KeyEvent, CursorPosition, InputAction};
use kjxlkj_core_ui::EditorSnapshot;
use tokio::sync::{mpsc, watch, broadcast};
use tracing::{info, debug, warn};

/// Core task handle for sending actions.
pub struct CoreHandle {
    action_tx: mpsc::Sender<InputAction>,
}

impl CoreHandle {
    /// Send an action to the core.
    pub async fn send(&self, action: InputAction) -> Result<(), mpsc::error::SendError<InputAction>> {
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

        // Publish initial snapshot.
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

            // Publish snapshot after state change.
            let _ = self.snapshot_tx.send(self.state.snapshot());

            // Check quit flag.
            if self.state.should_quit {
                info!("Quit requested");
                let _ = self.quit_tx.send(());
                break;
            }
        }

        info!("Core task ended");
    }

    /// Handle a key event.
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

    /// Handle paste.
    fn handle_paste(&mut self, text: &str) {
        // In insert mode, insert the text.
        if self.state.mode.mode.is_insert() {
            if let Some(window) = self.state.windows.focused() {
                if let kjxlkj_core_types::WindowContent::Buffer(buffer_id) = &window.content {
                    let cursor = window.cursor;
                    if let Some(buffer) = self.state.buffers.get_mut(*buffer_id) {
                        buffer.insert(cursor, text);
                    }
                }
            }
        }
    }

    /// Apply a mode action.
    fn apply_mode_action(&mut self, action: ModeAction) {
        match action {
            ModeAction::EnterInsert(pos) => {
                self.enter_insert(pos);
            }
            ModeAction::ReturnNormal => {
                // Clamp cursor if needed.
                self.clamp_cursor();
            }
            ModeAction::InsertText(text) => {
                self.insert_text(&text);
            }
            ModeAction::DeleteAtCursor(direction) => {
                self.delete_at_cursor(direction);
            }
            ModeAction::MoveCursor(motion, count) => {
                self.move_cursor(motion, count);
            }
            ModeAction::ExecuteCommand(cmd) => {
                self.execute_command(&cmd);
            }
            ModeAction::Undo => {
                self.undo();
            }
            ModeAction::Redo => {
                self.redo();
            }
            ModeAction::Quit { force: _ } => {
                self.state.quit();
            }
            _ => {
                warn!(?action, "Unhandled mode action");
            }
        }
    }

    /// Enter insert mode at position.
    fn enter_insert(&mut self, pos: InsertPosition) {
        if let Some(window) = self.state.windows.focused_mut() {
            if let kjxlkj_core_types::WindowContent::Buffer(buffer_id) = &window.content {
                if let Some(buffer) = self.state.buffers.get(*buffer_id) {
                    let line = window.cursor.line;
                    let max_grapheme = buffer.line_grapheme_count(line);

                    match pos {
                        InsertPosition::Before => {
                            // Stay at current position.
                        }
                        InsertPosition::After => {
                            // Move one right (or stay at end).
                            window.cursor.grapheme = (window.cursor.grapheme + 1).min(max_grapheme);
                        }
                        InsertPosition::EndOfLine => {
                            window.cursor.grapheme = max_grapheme;
                        }
                        InsertPosition::FirstNonBlank => {
                            // For simplicity, just go to start.
                            window.cursor.grapheme = 0;
                        }
                        InsertPosition::NewLineBelow => {
                            // Insert newline and position cursor.
                            let cursor = CursorPosition::new(line, max_grapheme);
                            if let Some(buffer) = self.state.buffers.get_mut(*buffer_id) {
                                buffer.insert(cursor, "\n");
                            }
                            window.cursor.line += 1;
                            window.cursor.grapheme = 0;
                        }
                        InsertPosition::NewLineAbove => {
                            let cursor = CursorPosition::new(line, 0);
                            if let Some(buffer) = self.state.buffers.get_mut(*buffer_id) {
                                buffer.insert(cursor, "\n");
                            }
                            window.cursor.grapheme = 0;
                        }
                    }
                }
            }
        }
    }

    /// Insert text at cursor.
    fn insert_text(&mut self, text: &str) {
        if let Some(window) = self.state.windows.focused_mut() {
            if let kjxlkj_core_types::WindowContent::Buffer(buffer_id) = window.content {
                let cursor = window.cursor;
                if let Some(buffer) = self.state.buffers.get_mut(buffer_id) {
                    buffer.insert(cursor, text);
                    // Advance cursor.
                    if text == "\n" {
                        window.cursor.line += 1;
                        window.cursor.grapheme = 0;
                    } else {
                        window.cursor.grapheme += kjxlkj_core_text::grapheme_count(text);
                    }
                }
            }
        }
    }

    /// Delete at cursor.
    fn delete_at_cursor(&mut self, direction: kjxlkj_core_edit::Direction) {
        if let Some(window) = self.state.windows.focused_mut() {
            if let kjxlkj_core_types::WindowContent::Buffer(buffer_id) = window.content {
                let cursor = window.cursor;
                if let Some(buffer) = self.state.buffers.get_mut(buffer_id) {
                    match direction {
                        kjxlkj_core_edit::Direction::Backward => {
                            if cursor.grapheme > 0 {
                                let start = CursorPosition::new(cursor.line, cursor.grapheme - 1);
                                buffer.delete(start, cursor);
                                window.cursor.grapheme -= 1;
                            } else if cursor.line > 0 {
                                // Join with previous line.
                                let prev_line = cursor.line - 1;
                                let prev_len = buffer.line_grapheme_count(prev_line);
                                let end = CursorPosition::new(cursor.line, 0);
                                let start = CursorPosition::new(prev_line, prev_len);
                                buffer.delete(start, end);
                                window.cursor.line = prev_line;
                                window.cursor.grapheme = prev_len;
                            }
                        }
                        kjxlkj_core_edit::Direction::Forward => {
                            let line_len = buffer.line_grapheme_count(cursor.line);
                            if cursor.grapheme < line_len {
                                let end = CursorPosition::new(cursor.line, cursor.grapheme + 1);
                                buffer.delete(cursor, end);
                            }
                        }
                    }
                }
            }
        }
    }

    /// Move cursor.
    fn move_cursor(&mut self, motion: kjxlkj_core_edit::Motion, count: usize) {
        if let Some(window) = self.state.windows.focused_mut() {
            if let kjxlkj_core_types::WindowContent::Buffer(buffer_id) = window.content {
                if let Some(buffer) = self.state.buffers.get(buffer_id) {
                    let mut cursor = window.cursor;

                    for _ in 0..count {
                        cursor = apply_motion(buffer, cursor, &motion);
                    }

                    window.cursor = cursor;
                }
            }
        }
    }

    /// Clamp cursor to valid range.
    fn clamp_cursor(&mut self) {
        if let Some(window) = self.state.windows.focused_mut() {
            if let kjxlkj_core_types::WindowContent::Buffer(buffer_id) = window.content {
                if let Some(buffer) = self.state.buffers.get(buffer_id) {
                    let max_line = buffer.line_count().saturating_sub(1);
                    window.cursor.line = window.cursor.line.min(max_line);

                    let line_len = buffer.line_grapheme_count(window.cursor.line);
                    let max_grapheme = line_len.saturating_sub(1);
                    window.cursor.grapheme = window.cursor.grapheme.min(max_grapheme);
                }
            }
        }
    }

    /// Execute a command.
    fn execute_command(&mut self, cmd: &str) {
        let cmd = cmd.trim();
        match cmd {
            "q" | "quit" => {
                self.state.quit();
            }
            "w" | "write" => {
                info!("Write command (not implemented)");
            }
            "wq" => {
                info!("Write and quit (not implemented)");
                self.state.quit();
            }
            _ => {
                warn!(?cmd, "Unknown command");
            }
        }
    }

    /// Undo.
    fn undo(&mut self) {
        info!("Undo (not fully implemented)");
    }

    /// Redo.
    fn redo(&mut self) {
        info!("Redo (not fully implemented)");
    }
}

/// Apply a motion to a cursor position.
fn apply_motion(
    buffer: &kjxlkj_core_state::Buffer,
    cursor: CursorPosition,
    motion: &kjxlkj_core_edit::Motion,
) -> CursorPosition {
    let line_count = buffer.line_count();
    let line_len = buffer.line_grapheme_count(cursor.line);

    match motion {
        kjxlkj_core_edit::Motion::Left => {
            if cursor.grapheme > 0 {
                CursorPosition::new(cursor.line, cursor.grapheme - 1)
            } else {
                cursor
            }
        }
        kjxlkj_core_edit::Motion::Right => {
            let max = if line_len > 0 { line_len - 1 } else { 0 };
            if cursor.grapheme < max {
                CursorPosition::new(cursor.line, cursor.grapheme + 1)
            } else {
                cursor
            }
        }
        kjxlkj_core_edit::Motion::Up => {
            if cursor.line > 0 {
                let new_line = cursor.line - 1;
                let new_len = buffer.line_grapheme_count(new_line);
                let new_grapheme = cursor.grapheme.min(new_len.saturating_sub(1));
                CursorPosition::new(new_line, new_grapheme)
            } else {
                cursor
            }
        }
        kjxlkj_core_edit::Motion::Down => {
            if cursor.line + 1 < line_count {
                let new_line = cursor.line + 1;
                let new_len = buffer.line_grapheme_count(new_line);
                let new_grapheme = cursor.grapheme.min(new_len.saturating_sub(1));
                CursorPosition::new(new_line, new_grapheme)
            } else {
                cursor
            }
        }
        kjxlkj_core_edit::Motion::LineStart => {
            CursorPosition::new(cursor.line, 0)
        }
        kjxlkj_core_edit::Motion::LineEnd => {
            let max = if line_len > 0 { line_len - 1 } else { 0 };
            CursorPosition::new(cursor.line, max)
        }
        kjxlkj_core_edit::Motion::DocumentStart => {
            CursorPosition::new(0, 0)
        }
        kjxlkj_core_edit::Motion::DocumentEnd => {
            let last_line = line_count.saturating_sub(1);
            CursorPosition::new(last_line, 0)
        }
        _ => cursor,
    }
}
