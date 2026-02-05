//! Main editor state machine.

use kjxlkj_core_edit::{apply_motion, Motion};
use kjxlkj_core_mode::ModeState;
use kjxlkj_core_text::TextBuffer;
use kjxlkj_core_types::{
    BufferId, BufferName, BufferVersion, Cursor, EditorEvent, Intent, Mode, Position, Range,
    WindowId,
};
use kjxlkj_core_ui::{BufferSnapshot, EditorSnapshot, Viewport, WindowSnapshot};
use kjxlkj_core_undo::{Edit, UndoHistory};
use std::collections::HashMap;
use std::path::PathBuf;

use crate::{BufferState, WindowState};

/// The main editor state machine.
pub struct Editor {
    buffers: HashMap<BufferId, BufferState>,
    windows: HashMap<WindowId, WindowState>,
    active_window: WindowId,
    mode_state: ModeState,
    next_buffer_id: u64,
    next_window_id: u64,
    snapshot_sequence: u64,
    terminal_width: u16,
    terminal_height: u16,
    message: Option<String>,
    register: String,
    quit_requested: bool,
}

impl Editor {
    /// Create a new editor.
    pub fn new(width: u16, height: u16) -> Self {
        let buffer_id = BufferId::new(1);
        let window_id = WindowId::new(1);

        let mut buffers = HashMap::new();
        buffers.insert(
            buffer_id,
            BufferState::new(buffer_id, BufferName::unnamed()),
        );

        let mut windows = HashMap::new();
        let content_height = height.saturating_sub(2) as usize;
        windows.insert(
            window_id,
            WindowState::new(window_id, buffer_id, width as usize, content_height),
        );

        Self {
            buffers,
            windows,
            active_window: window_id,
            mode_state: ModeState::new(),
            next_buffer_id: 2,
            next_window_id: 2,
            snapshot_sequence: 0,
            terminal_width: width,
            terminal_height: height,
            message: None,
            register: String::new(),
            quit_requested: false,
        }
    }

    /// Check if quit was requested.
    pub fn quit_requested(&self) -> bool {
        self.quit_requested
    }

    /// Get the current mode.
    pub fn mode(&self) -> Mode {
        self.mode_state.mode()
    }

    /// Process an editor event.
    pub fn process_event(&mut self, event: EditorEvent) {
        match event {
            EditorEvent::Key(key) => {
                if let Some(intent) = self.mode_state.process_key(&key) {
                    self.apply_intent(intent);
                }
            }
            EditorEvent::Resize { width, height } => {
                self.terminal_width = width;
                self.terminal_height = height;
                let content_height = height.saturating_sub(2) as usize;
                for win in self.windows.values_mut() {
                    win.resize(width as usize, content_height);
                }
            }
            EditorEvent::Focus(_) => {}
            EditorEvent::Quit => {
                self.quit_requested = true;
            }
        }
    }

    /// Apply an intent to the editor state.
    pub fn apply_intent(&mut self, intent: Intent) {
        self.message = None;

        match intent {
            Intent::EnterMode(mode) => {
                self.mode_state.set_mode(mode);
                if mode == Mode::Normal {
                    // Clamp cursor for end-exclusive mode
                    if let Some(win) = self.windows.get_mut(&self.active_window) {
                        if let Some(buf) = self.buffers.get(&win.buffer_id) {
                            let line_len = buf.line_len(win.cursor.line);
                            win.cursor = win.cursor.clamp_end_exclusive(line_len);
                        }
                    }
                }
            }

            // Cursor movement
            Intent::MoveUp(n) => self.apply_motion(Motion::Up(n)),
            Intent::MoveDown(n) => self.apply_motion(Motion::Down(n)),
            Intent::MoveLeft(n) => self.apply_motion(Motion::Left(n)),
            Intent::MoveRight(n) => self.apply_motion(Motion::Right(n)),
            Intent::MoveToLineStart => self.apply_motion(Motion::LineStart),
            Intent::MoveToFirstNonBlank => self.apply_motion(Motion::FirstNonBlank),
            Intent::MoveToLineEnd => self.apply_motion(Motion::LineEnd),
            Intent::MoveToDocumentStart => self.apply_motion(Motion::DocumentStart),
            Intent::MoveToDocumentEnd => self.apply_motion(Motion::DocumentEnd),
            Intent::MoveToLine(n) => self.apply_motion(Motion::Line(n)),
            Intent::MoveWordForward(n) => self.apply_motion(Motion::WordForward(n)),
            Intent::MoveWordBackward(n) => self.apply_motion(Motion::WordBackward(n)),
            Intent::MoveWordEnd(n) => self.apply_motion(Motion::WordEnd(n)),
            Intent::MoveBigWordForward(n) => self.apply_motion(Motion::BigWordForward(n)),
            Intent::MoveBigWordBackward(n) => self.apply_motion(Motion::BigWordBackward(n)),
            Intent::MoveBigWordEnd(n) => self.apply_motion(Motion::BigWordEnd(n)),

            // Scrolling
            Intent::ScrollHalfPageUp => {
                let total = self.total_lines();
                if let Some(win) = self.windows.get_mut(&self.active_window) {
                    let half = win.viewport.height / 2;
                    win.viewport.scroll(-(half as isize), total);
                    win.cursor.line = win.cursor.line.saturating_sub(half);
                }
            }
            Intent::ScrollHalfPageDown => {
                let total = self.total_lines();
                if let Some(win) = self.windows.get_mut(&self.active_window) {
                    let half = win.viewport.height / 2;
                    win.viewport.scroll(half as isize, total);
                    win.cursor.line = (win.cursor.line + half).min(total.saturating_sub(1));
                }
            }
            Intent::CenterCursor => {
                let total = self.total_lines();
                if let Some(win) = self.windows.get_mut(&self.active_window) {
                    win.viewport.center_on(win.cursor.line, total);
                }
            }

            // Text insertion
            Intent::InsertChar(c) => self.insert_char(c),
            Intent::InsertNewline => self.insert_newline(),
            Intent::InsertNewlineBelow => self.open_line_below(),
            Intent::InsertNewlineAbove => self.open_line_above(),

            // Text deletion
            Intent::DeleteChar => self.delete_char_forward(),
            Intent::DeleteCharBackward => self.delete_char_backward(),
            Intent::DeleteLine(n) => self.delete_lines(n),
            Intent::DeleteToLineEnd => self.delete_to_line_end(),

            // Undo/Redo
            Intent::Undo => self.undo(),
            Intent::Redo => self.redo(),

            // Paste
            Intent::Paste { after } => self.paste(after),

            // Yank
            Intent::YankLine(n) => self.yank_lines(n),

            // Commands
            Intent::ExecuteCommand(cmd) => self.execute_command(&cmd),
            Intent::WriteBuffer(path) => self.write_buffer(path),

            // Quit
            Intent::Quit => self.try_quit(false),
            Intent::QuitForce => self.try_quit(true),
            Intent::WriteQuit => {
                self.write_buffer(None);
                self.try_quit(false);
            }

            Intent::Noop => {}

            _ => {
                tracing::debug!("Unhandled intent: {:?}", intent);
            }
        }
    }

    fn apply_motion(&mut self, motion: Motion) {
        let end_inclusive = self.mode().is_end_inclusive();

        if let Some(win) = self.windows.get_mut(&self.active_window) {
            if let Some(buf) = self.buffers.get(&win.buffer_id) {
                let result = apply_motion(&motion, win.cursor, &buf.text, end_inclusive);
                win.cursor = result.cursor;
                win.viewport.follow_cursor(&win.cursor, buf.line_count());
            }
        }
    }

    fn insert_char(&mut self, c: char) {
        if let Some(win) = self.windows.get(&self.active_window) {
            let buffer_id = win.buffer_id;
            let pos = Position::new(win.cursor.line, win.cursor.column);

            if let Some(buf) = self.buffers.get_mut(&buffer_id) {
                let version_before = buf.version();
                if let Ok(version_after) = buf.text.insert_char(pos, c) {
                    let edit = Edit::insert(
                        pos,
                        c.to_string(),
                        version_before,
                        version_after,
                        Position::new(pos.line, pos.column + 1),
                    );
                    buf.history.record(edit);
                    buf.set_modified(true);
                }
            }

            if let Some(win) = self.windows.get_mut(&self.active_window) {
                win.cursor.column += 1;
            }
        }
    }

    fn insert_newline(&mut self) {
        if let Some(win) = self.windows.get(&self.active_window) {
            let buffer_id = win.buffer_id;
            let pos = Position::new(win.cursor.line, win.cursor.column);

            if let Some(buf) = self.buffers.get_mut(&buffer_id) {
                let version_before = buf.version();
                if let Ok(version_after) = buf.text.insert_char(pos, '\n') {
                    let edit = Edit::insert(
                        pos,
                        "\n".to_string(),
                        version_before,
                        version_after,
                        Position::new(pos.line + 1, 0),
                    );
                    buf.history.record(edit);
                    buf.set_modified(true);
                }
            }

            if let Some(win) = self.windows.get_mut(&self.active_window) {
                win.cursor.line += 1;
                win.cursor.column = 0;
                if let Some(buf) = self.buffers.get(&buffer_id) {
                    win.viewport.follow_cursor(&win.cursor, buf.line_count());
                }
            }
        }
    }

    fn open_line_below(&mut self) {
        if let Some(win) = self.windows.get(&self.active_window) {
            let buffer_id = win.buffer_id;
            let line = win.cursor.line;

            if let Some(buf) = self.buffers.get_mut(&buffer_id) {
                let line_len = buf.line_len(line);
                let pos = Position::new(line, line_len);
                let version_before = buf.version();
                if let Ok(version_after) = buf.text.insert_char(pos, '\n') {
                    let edit = Edit::insert(
                        pos,
                        "\n".to_string(),
                        version_before,
                        version_after,
                        Position::new(line + 1, 0),
                    );
                    buf.history.record(edit);
                    buf.set_modified(true);
                }
            }

            if let Some(win) = self.windows.get_mut(&self.active_window) {
                win.cursor.line += 1;
                win.cursor.column = 0;
            }
        }
        self.mode_state.set_mode(Mode::Insert);
    }

    fn open_line_above(&mut self) {
        if let Some(win) = self.windows.get(&self.active_window) {
            let buffer_id = win.buffer_id;
            let line = win.cursor.line;

            if let Some(buf) = self.buffers.get_mut(&buffer_id) {
                let pos = Position::new(line, 0);
                let version_before = buf.version();
                if let Ok(version_after) = buf.text.insert(pos, "\n") {
                    let edit = Edit::insert(
                        pos,
                        "\n".to_string(),
                        version_before,
                        version_after,
                        Position::new(line, 0),
                    );
                    buf.history.record(edit);
                    buf.set_modified(true);
                }
            }

            if let Some(win) = self.windows.get_mut(&self.active_window) {
                win.cursor.column = 0;
            }
        }
        self.mode_state.set_mode(Mode::Insert);
    }

    fn delete_char_forward(&mut self) {
        if let Some(win) = self.windows.get(&self.active_window) {
            let buffer_id = win.buffer_id;
            let pos = Position::new(win.cursor.line, win.cursor.column);

            if let Some(buf) = self.buffers.get_mut(&buffer_id) {
                if buf.text.delete_char(pos).is_ok() {
                    buf.set_modified(true);
                }
            }
        }
    }

    fn delete_char_backward(&mut self) {
        if let Some(win) = self.windows.get(&self.active_window) {
            if win.cursor.column == 0 && win.cursor.line == 0 {
                return;
            }

            let buffer_id = win.buffer_id;

            if win.cursor.column > 0 {
                let pos = Position::new(win.cursor.line, win.cursor.column - 1);
                if let Some(buf) = self.buffers.get_mut(&buffer_id) {
                    if buf.text.delete_char(pos).is_ok() {
                        buf.set_modified(true);
                    }
                }
                if let Some(win) = self.windows.get_mut(&self.active_window) {
                    win.cursor.column -= 1;
                }
            } else if win.cursor.line > 0 {
                // Delete newline at end of previous line
                if let Some(buf) = self.buffers.get_mut(&buffer_id) {
                    let prev_line = win.cursor.line - 1;
                    let prev_len = buf.line_len(prev_line);
                    let pos = Position::new(prev_line, prev_len);
                    if buf.text.delete_char(pos).is_ok() {
                        buf.set_modified(true);
                    }

                    if let Some(win) = self.windows.get_mut(&self.active_window) {
                        win.cursor.line -= 1;
                        win.cursor.column = prev_len;
                    }
                }
            }
        }
    }

    fn delete_lines(&mut self, count: usize) {
        if let Some(win) = self.windows.get(&self.active_window) {
            let buffer_id = win.buffer_id;
            let start_line = win.cursor.line;

            if let Some(buf) = self.buffers.get_mut(&buffer_id) {
                let end_line = (start_line + count).min(buf.line_count());
                let range = Range::from_coords(start_line, 0, end_line, 0);

                if let Ok((deleted, _)) = buf.text.delete(range) {
                    self.register = deleted;
                    buf.set_modified(true);
                }
            }

            // Clamp cursor
            if let Some(win) = self.windows.get_mut(&self.active_window) {
                if let Some(buf) = self.buffers.get(&buffer_id) {
                    if win.cursor.line >= buf.line_count() {
                        win.cursor.line = buf.line_count().saturating_sub(1);
                    }
                    win.cursor.column = 0;
                }
            }
        }
    }

    fn delete_to_line_end(&mut self) {
        if let Some(win) = self.windows.get(&self.active_window) {
            let buffer_id = win.buffer_id;
            let line = win.cursor.line;
            let col = win.cursor.column;

            if let Some(buf) = self.buffers.get_mut(&buffer_id) {
                let line_len = buf.line_len(line);
                if col < line_len {
                    let range = Range::from_coords(line, col, line, line_len);
                    if let Ok((deleted, _)) = buf.text.delete(range) {
                        self.register = deleted;
                        buf.set_modified(true);
                    }
                }
            }
        }
    }

    fn yank_lines(&mut self, count: usize) {
        if let Some(win) = self.windows.get(&self.active_window) {
            if let Some(buf) = self.buffers.get(&win.buffer_id) {
                let start_line = win.cursor.line;
                let end_line = (start_line + count).min(buf.line_count());
                let range = Range::from_coords(start_line, 0, end_line, 0);

                if let Ok(text) = buf.text.slice(range) {
                    self.register = text;
                    self.message = Some(format!("{} lines yanked", count));
                }
            }
        }
    }

    fn paste(&mut self, after: bool) {
        if self.register.is_empty() {
            return;
        }

        if let Some(win) = self.windows.get(&self.active_window) {
            let buffer_id = win.buffer_id;
            let mut pos = Position::new(win.cursor.line, win.cursor.column);

            // If register ends with newline, paste as lines
            let is_linewise = self.register.ends_with('\n');

            if is_linewise {
                if after {
                    pos = Position::new(win.cursor.line + 1, 0);
                } else {
                    pos = Position::new(win.cursor.line, 0);
                }
            } else if after {
                pos.column += 1;
            }

            if let Some(buf) = self.buffers.get_mut(&buffer_id) {
                if buf.text.insert(pos, &self.register).is_ok() {
                    buf.set_modified(true);
                }
            }
        }
    }

    fn undo(&mut self) {
        if let Some(win) = self.windows.get(&self.active_window) {
            let buffer_id = win.buffer_id;

            if let Some(buf) = self.buffers.get_mut(&buffer_id) {
                if let Some(edit) = buf.history.undo() {
                    // Apply inverse edit
                    match &edit.kind {
                        kjxlkj_core_undo::EditKind::Insert { pos, text } => {
                            let range = Range::from_coords(
                                pos.line,
                                pos.column,
                                pos.line,
                                pos.column + text.chars().count(),
                            );
                            let _ = buf.text.delete(range);
                        }
                        kjxlkj_core_undo::EditKind::Delete { range, text } => {
                            let _ = buf.text.insert(range.start, text);
                        }
                        kjxlkj_core_undo::EditKind::Replace { range, old, .. } => {
                            let _ = buf.text.replace(*range, old);
                        }
                    }

                    // Restore cursor
                    if let Some(win) = self.windows.get_mut(&self.active_window) {
                        win.cursor = Cursor::from(edit.cursor_before);
                    }

                    buf.set_modified(buf.history.can_undo());
                    self.message = Some("Undo".to_string());
                }
            }
        }
    }

    fn redo(&mut self) {
        if let Some(win) = self.windows.get(&self.active_window) {
            let buffer_id = win.buffer_id;

            if let Some(buf) = self.buffers.get_mut(&buffer_id) {
                if let Some(edit) = buf.history.redo() {
                    // Re-apply edit
                    match &edit.kind {
                        kjxlkj_core_undo::EditKind::Insert { pos, text } => {
                            let _ = buf.text.insert(*pos, text);
                        }
                        kjxlkj_core_undo::EditKind::Delete { range, .. } => {
                            let _ = buf.text.delete(*range);
                        }
                        kjxlkj_core_undo::EditKind::Replace { range, new, .. } => {
                            let _ = buf.text.replace(*range, new);
                        }
                    }

                    // Restore cursor
                    if let Some(win) = self.windows.get_mut(&self.active_window) {
                        win.cursor = Cursor::from(edit.cursor_after);
                    }

                    buf.set_modified(true);
                    self.message = Some("Redo".to_string());
                }
            }
        }
    }

    fn execute_command(&mut self, cmd: &str) {
        let cmd = cmd.trim();
        match cmd {
            "q" => self.try_quit(false),
            "q!" => self.try_quit(true),
            "w" => self.write_buffer(None),
            "wq" | "x" => {
                self.write_buffer(None);
                self.try_quit(false);
            }
            "qa" => self.try_quit(false),
            "qa!" => self.try_quit(true),
            _ if cmd.starts_with("w ") => {
                let path = cmd.strip_prefix("w ").unwrap().trim();
                self.write_buffer(Some(path.to_string()));
            }
            _ if cmd.starts_with("e ") => {
                let path = cmd.strip_prefix("e ").unwrap().trim();
                self.edit_file(path);
            }
            _ => {
                self.message = Some(format!("Unknown command: {}", cmd));
            }
        }
    }

    fn write_buffer(&mut self, path: Option<String>) {
        if let Some(win) = self.windows.get(&self.active_window) {
            if let Some(buf) = self.buffers.get_mut(&win.buffer_id) {
                let target_path = path
                    .map(PathBuf::from)
                    .or_else(|| buf.path.clone());

                if let Some(target) = target_path {
                    let content = buf.text.to_string();
                    match std::fs::write(&target, &content) {
                        Ok(_) => {
                            buf.path = Some(target.clone());
                            buf.set_modified(false);
                            let name = target
                                .file_name()
                                .and_then(|s| s.to_str())
                                .unwrap_or("[No Name]");
                            buf.name = BufferName::new(name);
                            self.message = Some(format!(
                                "\"{}\" written, {} bytes",
                                target.display(),
                                content.len()
                            ));
                        }
                        Err(e) => {
                            self.message = Some(format!("Error writing: {}", e));
                        }
                    }
                } else {
                    self.message = Some("No file name".to_string());
                }
            }
        }
    }

    fn edit_file(&mut self, path: &str) {
        let path = PathBuf::from(path);
        let content = std::fs::read_to_string(&path).unwrap_or_default();

        let buffer_id = BufferId::new(self.next_buffer_id);
        self.next_buffer_id += 1;

        let buffer = BufferState::from_file(buffer_id, path, &content);
        self.buffers.insert(buffer_id, buffer);

        if let Some(win) = self.windows.get_mut(&self.active_window) {
            win.set_buffer(buffer_id);
        }
    }

    fn try_quit(&mut self, force: bool) {
        let has_modified = self.buffers.values().any(|b| b.modified);

        if has_modified && !force {
            self.message = Some("Unsaved changes. Use :q! to force quit".to_string());
        } else {
            self.quit_requested = true;
        }
    }

    fn total_lines(&self) -> usize {
        if let Some(win) = self.windows.get(&self.active_window) {
            if let Some(buf) = self.buffers.get(&win.buffer_id) {
                return buf.line_count();
            }
        }
        1
    }

    /// Create a snapshot for rendering.
    pub fn snapshot(&mut self) -> EditorSnapshot {
        self.snapshot_sequence += 1;

        let window_snapshots: Vec<WindowSnapshot> = self
            .windows
            .values()
            .map(|win| {
                let buf = self.buffers.get(&win.buffer_id).unwrap();
                let start = win.viewport.top_line;
                let end = start + win.viewport.height;
                let lines = buf.text.lines_in_range(start, end);

                let buffer_snapshot = BufferSnapshot::new(
                    buf.id,
                    buf.name.clone(),
                    buf.version(),
                    lines,
                    start,
                    buf.line_count(),
                    buf.modified,
                );

                WindowSnapshot::new(
                    win.id,
                    buffer_snapshot,
                    win.cursor,
                    win.viewport,
                    win.id == self.active_window,
                )
            })
            .collect();

        EditorSnapshot::new(
            self.snapshot_sequence,
            window_snapshots,
            self.active_window,
            self.mode_state.mode(),
            self.mode_state.command_line().to_string(),
            self.message.clone(),
            self.terminal_width,
            self.terminal_height,
        )
    }

    /// Open a file in the editor.
    pub fn open_file(&mut self, path: &PathBuf) -> std::io::Result<()> {
        let content = std::fs::read_to_string(path)?;

        if let Some(win) = self.windows.get(&self.active_window) {
            if let Some(buf) = self.buffers.get_mut(&win.buffer_id) {
                buf.text = TextBuffer::from_str(&content);
                buf.path = Some(path.clone());
                buf.name = BufferName::new(
                    path.file_name()
                        .and_then(|s| s.to_str())
                        .unwrap_or("[No Name]"),
                );
                buf.set_modified(false);
                buf.history.clear();
            }
        }

        // Reset cursor
        if let Some(win) = self.windows.get_mut(&self.active_window) {
            win.cursor = Cursor::origin();
            win.viewport.top_line = 0;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use kjxlkj_core_types::KeyEvent;

    #[test]
    fn test_editor_new() {
        let editor = Editor::new(80, 24);
        assert_eq!(editor.mode(), Mode::Normal);
        assert!(!editor.quit_requested());
    }

    #[test]
    fn test_editor_insert_mode() {
        let mut editor = Editor::new(80, 24);
        editor.process_event(EditorEvent::Key(KeyEvent::char('i')));
        assert_eq!(editor.mode(), Mode::Insert);

        editor.process_event(EditorEvent::Key(KeyEvent::char('a')));
        editor.process_event(EditorEvent::Key(KeyEvent::char('b')));
        editor.process_event(EditorEvent::Key(KeyEvent::char('c')));

        let snapshot = editor.snapshot();
        assert!(snapshot.windows[0].buffer.lines[0].contains("abc"));
    }

    #[test]
    fn test_editor_escape_to_normal() {
        let mut editor = Editor::new(80, 24);
        editor.process_event(EditorEvent::Key(KeyEvent::char('i')));
        assert_eq!(editor.mode(), Mode::Insert);

        editor.process_event(EditorEvent::Key(KeyEvent::Escape));
        assert_eq!(editor.mode(), Mode::Normal);
    }

    #[test]
    fn test_editor_undo_redo() {
        let mut editor = Editor::new(80, 24);
        editor.process_event(EditorEvent::Key(KeyEvent::char('i')));
        editor.process_event(EditorEvent::Key(KeyEvent::char('x')));
        editor.process_event(EditorEvent::Key(KeyEvent::Escape));

        // Undo
        editor.process_event(EditorEvent::Key(KeyEvent::char('u')));
        let snapshot = editor.snapshot();
        assert!(!snapshot.windows[0].buffer.lines[0].contains('x'));

        // Redo
        editor.process_event(EditorEvent::Key(KeyEvent::ctrl('r')));
        let snapshot = editor.snapshot();
        assert!(snapshot.windows[0].buffer.lines[0].contains('x'));
    }
}
