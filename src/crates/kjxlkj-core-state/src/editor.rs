//! Main editor state.

use kjxlkj_core_edit::{apply_motion, Operator};
use kjxlkj_core_mode::{Intent, ModeState};
use kjxlkj_core_text::TextBuffer;
use kjxlkj_core_types::{BufferId, BufferName, Cursor, EditorEvent, Mode, Position, Range};
use kjxlkj_core_ui::{BufferSnapshot, EditorSnapshot, Viewport, WindowSnapshot};
use kjxlkj_core_undo::{Edit, UndoHistory};
use std::path::PathBuf;

/// Buffer state.
struct Buffer {
    id: BufferId,
    name: BufferName,
    text: TextBuffer,
    path: Option<PathBuf>,
    modified: bool,
    undo: UndoHistory,
}

/// Window state.
struct Window {
    buffer_id: BufferId,
    cursor: Cursor,
    viewport: Viewport,
    selection_anchor: Option<Position>,
}

/// Main editor state.
pub struct Editor {
    buffers: Vec<Buffer>,
    windows: Vec<Window>,
    active_window: usize,
    mode_state: ModeState,
    next_buffer_id: u64,
    width: u16,
    height: u16,
    status: String,
    should_quit: bool,
}

impl Editor {
    /// Create a new editor.
    pub fn new(width: u16, height: u16) -> Self {
        let buffer = Buffer {
            id: BufferId::new(1),
            name: BufferName::unnamed(),
            text: TextBuffer::new(),
            path: None,
            modified: false,
            undo: UndoHistory::new(),
        };

        let window = Window {
            buffer_id: BufferId::new(1),
            cursor: Cursor::origin(),
            viewport: Viewport::new(width as usize, height.saturating_sub(2) as usize),
            selection_anchor: None,
        };

        Self {
            buffers: vec![buffer],
            windows: vec![window],
            active_window: 0,
            mode_state: ModeState::new(),
            next_buffer_id: 2,
            width,
            height,
            status: String::new(),
            should_quit: false,
        }
    }

    /// Check if the editor should quit.
    pub fn should_quit(&self) -> bool {
        self.should_quit
    }

    /// Get the current mode.
    pub fn mode(&self) -> Mode {
        self.mode_state.mode()
    }

    /// Open a file.
    pub fn open_file(&mut self, path: &std::path::Path) -> std::io::Result<()> {
        let content = std::fs::read_to_string(path)?;
        let name = path
            .file_name()
            .map(|s| s.to_string_lossy().to_string())
            .unwrap_or_else(|| path.display().to_string());

        let id = BufferId::new(self.next_buffer_id);
        self.next_buffer_id += 1;

        let buffer = Buffer {
            id,
            name: BufferName::new(name),
            text: content.parse().unwrap(),
            path: Some(path.to_path_buf()),
            modified: false,
            undo: UndoHistory::new(),
        };
        self.buffers.push(buffer);

        if let Some(window) = self.windows.get_mut(self.active_window) {
            window.buffer_id = id;
            window.cursor = Cursor::origin();
            window.viewport.top_line = 0;
        }

        self.status = format!("\"{}\" opened", path.display());
        Ok(())
    }

    /// Process an event.
    pub fn process_event(&mut self, event: EditorEvent) {
        match event {
            EditorEvent::Key(key) => {
                if let Some(intent) = self.mode_state.process_key(key) {
                    self.execute_intent(intent);
                }
            }
            EditorEvent::Resize(w, h) => {
                self.width = w;
                self.height = h;
                for window in &mut self.windows {
                    window.viewport.width = w as usize;
                    window.viewport.height = h.saturating_sub(2) as usize;
                }
            }
            EditorEvent::Quit => {
                self.should_quit = true;
            }
        }
    }

    fn execute_intent(&mut self, intent: Intent) {
        let Some(window) = self.windows.get_mut(self.active_window) else {
            return;
        };
        let Some(buffer) = self.buffers.iter_mut().find(|b| b.id == window.buffer_id) else {
            return;
        };

        match intent {
            Intent::Noop => {}
            Intent::InsertChar(c) => {
                let pos = window.cursor.position;
                let old_pos = pos;
                buffer.text.insert(pos, &c.to_string());
                window.cursor.position.column += 1;
                buffer.modified = true;
                buffer.undo.record(Edit {
                    range: Range::new(pos, pos),
                    old_text: String::new(),
                    new_text: c.to_string(),
                    cursor_before: old_pos,
                    cursor_after: window.cursor.position,
                });
            }
            Intent::Backspace => {
                let pos = window.cursor.position;
                if pos.column > 0 {
                    let new_col = pos.column - 1;
                    let start = Position::new(pos.line, new_col);
                    let deleted = buffer.text.delete(Range::new(start, pos));
                    window.cursor.position.column = new_col;
                    buffer.modified = true;
                    buffer.undo.record(Edit {
                        range: Range::new(start, pos),
                        old_text: deleted,
                        new_text: String::new(),
                        cursor_before: pos,
                        cursor_after: window.cursor.position,
                    });
                } else if pos.line > 0 {
                    // Join with previous line
                    let prev_line_len = buffer.text.line_len(pos.line - 1);
                    let range = Range::new(
                        Position::new(pos.line - 1, prev_line_len),
                        Position::new(pos.line, 0),
                    );
                    let deleted = buffer.text.delete(range);
                    window.cursor.position = Position::new(pos.line - 1, prev_line_len);
                    buffer.modified = true;
                    buffer.undo.record(Edit {
                        range,
                        old_text: deleted,
                        new_text: String::new(),
                        cursor_before: pos,
                        cursor_after: window.cursor.position,
                    });
                }
            }
            Intent::Newline => {
                let pos = window.cursor.position;
                buffer.text.insert(pos, "\n");
                window.cursor.position = Position::new(pos.line + 1, 0);
                buffer.modified = true;
                buffer.undo.record(Edit {
                    range: Range::new(pos, pos),
                    old_text: String::new(),
                    new_text: "\n".to_string(),
                    cursor_before: pos,
                    cursor_after: window.cursor.position,
                });
            }
            Intent::DeleteChar => {
                let pos = window.cursor.position;
                let line_len = buffer.text.line_len(pos.line);
                if pos.column < line_len {
                    let end = Position::new(pos.line, pos.column + 1);
                    let deleted = buffer.text.delete(Range::new(pos, end));
                    buffer.modified = true;
                    buffer.undo.record(Edit {
                        range: Range::new(pos, end),
                        old_text: deleted,
                        new_text: String::new(),
                        cursor_before: pos,
                        cursor_after: pos,
                    });
                }
            }
            Intent::Motion(motion, count) => {
                let end_inclusive = self.mode_state.mode().is_end_inclusive();
                window.cursor.position = apply_motion(
                    &buffer.text,
                    window.cursor.position,
                    motion,
                    count,
                    end_inclusive,
                );
                self.ensure_cursor_visible();
            }
            Intent::EnterMode(mode) => {
                if mode.is_visual() {
                    window.selection_anchor = Some(window.cursor.position);
                } else {
                    window.selection_anchor = None;
                }
            }
            Intent::Append => {
                let line_len = buffer.text.line_len(window.cursor.position.line);
                window.cursor.position.column = (window.cursor.position.column + 1).min(line_len);
            }
            Intent::OpenBelow => {
                let line = window.cursor.position.line;
                let line_len = buffer.text.line_len(line);
                let pos = Position::new(line, line_len);
                buffer.text.insert(pos, "\n");
                window.cursor.position = Position::new(line + 1, 0);
                buffer.modified = true;
            }
            Intent::OpenAbove => {
                let line = window.cursor.position.line;
                let pos = Position::new(line, 0);
                buffer.text.insert(pos, "\n");
                window.cursor.position = Position::new(line, 0);
                buffer.modified = true;
            }
            Intent::Undo => {
                if let Some(edit) = buffer.undo.undo() {
                    // Reverse the edit
                    if !edit.new_text.is_empty() {
                        let end = buffer.text.char_to_pos(
                            buffer.text.pos_to_char(edit.range.start) + edit.new_text.len(),
                        );
                        buffer.text.delete(Range::new(edit.range.start, end));
                    }
                    if !edit.old_text.is_empty() {
                        buffer.text.insert(edit.range.start, &edit.old_text);
                    }
                    window.cursor.position = edit.cursor_before;
                    buffer.modified = true;
                }
            }
            Intent::Redo => {
                if let Some(edit) = buffer.undo.redo() {
                    // Reapply the edit
                    if !edit.old_text.is_empty() {
                        buffer.text.delete(edit.range);
                    }
                    if !edit.new_text.is_empty() {
                        buffer.text.insert(edit.range.start, &edit.new_text);
                    }
                    window.cursor.position = edit.cursor_after;
                    buffer.modified = true;
                }
            }
            Intent::ExecuteCommand(cmd) => {
                self.execute_command(&cmd);
            }
            Intent::OperatorMotion(op, motion, count) => {
                let start = window.cursor.position;
                let end = apply_motion(&buffer.text, start, motion, count, false);
                let range = if start <= end {
                    Range::new(start, end)
                } else {
                    Range::new(end, start)
                };
                match op {
                    Operator::Delete | Operator::Change => {
                        let deleted = buffer.text.delete(range);
                        buffer.modified = true;
                        buffer.undo.record(Edit {
                            range,
                            old_text: deleted,
                            new_text: String::new(),
                            cursor_before: start,
                            cursor_after: range.start,
                        });
                        window.cursor.position = range.start;
                    }
                    Operator::Yank => {
                        // Just copy, do nothing for now
                    }
                    _ => {}
                }
            }
            Intent::OperatorLine(op, count) => {
                let line = window.cursor.position.line;
                let start = Position::new(line, 0);
                let end_line = (line + count).min(buffer.text.line_count());
                let end = Position::new(end_line, 0);
                let range = Range::new(start, end);
                match op {
                    Operator::Delete | Operator::Change => {
                        let deleted = buffer.text.delete(range);
                        buffer.modified = true;
                        buffer.undo.record(Edit {
                            range,
                            old_text: deleted,
                            new_text: String::new(),
                            cursor_before: window.cursor.position,
                            cursor_after: start,
                        });
                        window.cursor.position = start;
                    }
                    Operator::Yank => {}
                    _ => {}
                }
            }
            Intent::Cancel | Intent::CommandLineAppend(_) | Intent::CommandLineBackspace => {}
            Intent::Quit => {
                self.should_quit = true;
            }
        }

        self.ensure_cursor_visible();
    }

    fn execute_command(&mut self, cmd: &str) {
        let cmd = cmd.trim();
        match cmd {
            "q" | "quit" => {
                if self.has_unsaved_changes() {
                    self.status = "Unsaved changes. Use :q! to force.".to_string();
                } else {
                    self.should_quit = true;
                }
            }
            "q!" | "quit!" => {
                self.should_quit = true;
            }
            "w" | "write" => {
                self.save_current_buffer();
            }
            "wq" | "x" => {
                self.save_current_buffer();
                self.should_quit = true;
            }
            _ => {
                if let Ok(line_num) = cmd.parse::<usize>() {
                    if let Some(window) = self.windows.get_mut(self.active_window) {
                        if let Some(buffer) = self.buffers.iter().find(|b| b.id == window.buffer_id)
                        {
                            let target = line_num
                                .saturating_sub(1)
                                .min(buffer.text.line_count().saturating_sub(1));
                            window.cursor.position = Position::new(target, 0);
                            self.ensure_cursor_visible();
                        }
                    }
                } else {
                    self.status = format!("Unknown command: {}", cmd);
                }
            }
        }
    }

    fn save_current_buffer(&mut self) {
        if let Some(window) = self.windows.get(self.active_window) {
            if let Some(buffer) = self.buffers.iter_mut().find(|b| b.id == window.buffer_id) {
                if let Some(path) = &buffer.path {
                    match std::fs::write(path, buffer.text.to_string()) {
                        Ok(_) => {
                            buffer.modified = false;
                            self.status = format!("\"{}\" written", path.display());
                        }
                        Err(e) => {
                            self.status = format!("Error writing file: {}", e);
                        }
                    }
                } else {
                    self.status = "No file name".to_string();
                }
            }
        }
    }

    fn has_unsaved_changes(&self) -> bool {
        self.buffers.iter().any(|b| b.modified)
    }

    fn ensure_cursor_visible(&mut self) {
        if let Some(window) = self.windows.get_mut(self.active_window) {
            window
                .viewport
                .ensure_visible(window.cursor.position.line, 3);
        }
    }

    /// Generate a snapshot for rendering.
    pub fn snapshot(&self) -> EditorSnapshot {
        let window = &self.windows[self.active_window];
        let buffer = self
            .buffers
            .iter()
            .find(|b| b.id == window.buffer_id)
            .unwrap();

        let visible_range = window.viewport.visible_lines();
        let lines: Vec<String> = (visible_range.start..visible_range.end)
            .filter_map(|i| buffer.text.line(i))
            .collect();

        let buffer_snapshot = BufferSnapshot {
            id: buffer.id,
            name: buffer.name.clone(),
            lines,
            first_line: window.viewport.top_line,
            total_lines: buffer.text.line_count(),
            modified: buffer.modified,
        };

        let window_snapshot = WindowSnapshot {
            cursor: window.cursor,
            selection_anchor: window.selection_anchor,
            buffer: buffer_snapshot,
            top_line: window.viewport.top_line,
            left_col: window.viewport.left_col,
        };

        EditorSnapshot::new(
            self.mode_state.mode(),
            window_snapshot,
            self.mode_state.command_line().to_string(),
            self.status.clone(),
            self.width,
            self.height,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use kjxlkj_core_types::KeyEvent;

    #[test]
    fn test_new_editor() {
        let editor = Editor::new(80, 24);
        assert_eq!(editor.mode(), Mode::Normal);
        assert!(!editor.should_quit());
    }

    #[test]
    fn test_enter_insert_mode() {
        let mut editor = Editor::new(80, 24);
        editor.process_event(EditorEvent::Key(KeyEvent::Char('i', Default::default())));
        assert_eq!(editor.mode(), Mode::Insert);
    }

    #[test]
    fn test_insert_text() {
        let mut editor = Editor::new(80, 24);
        editor.process_event(EditorEvent::Key(KeyEvent::Char('i', Default::default())));
        editor.process_event(EditorEvent::Key(KeyEvent::Char('h', Default::default())));
        editor.process_event(EditorEvent::Key(KeyEvent::Char('i', Default::default())));
        let snapshot = editor.snapshot();
        assert!(snapshot.window.buffer.lines[0].contains("hi"));
    }
}
