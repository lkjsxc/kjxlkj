//! Editor state aggregation and snapshot production.

use kjxlkj_core_edit::{execute_motion, execute_operator};
use kjxlkj_core_mode::{parse_key, ModeState, ParseResult};
use kjxlkj_core_text::TextBuffer;
use kjxlkj_core_types::{
    BufferId, BufferName, CursorPosition, CursorShape, CursorState, InputEvent, InsertPosition,
    Intent, KeyEvent, LineCol, Mode, Motion, Operator, RegisterContent, RegisterName, Selection,
    ViewportState, VisualMode, WindowId,
};
use kjxlkj_core_ui::{
    EditorSnapshot, LineSnapshot, SnapshotSeq, StatusLine, TerminalSize, WindowRect, WindowSnapshot,
};
use kjxlkj_core_undo::{EditKind, EditOp, UndoHistory};
use std::collections::HashMap;
use std::path::PathBuf;

/// The main editor state.
pub struct Editor {
    /// Buffers by ID.
    buffers: HashMap<BufferId, TextBuffer>,
    /// Next buffer ID.
    next_buffer_id: u64,
    /// Windows by ID.
    windows: HashMap<WindowId, WindowState>,
    /// Focused window ID.
    focused_window: WindowId,
    /// Mode state.
    mode_state: ModeState,
    /// Terminal size.
    terminal_size: TerminalSize,
    /// Snapshot sequence.
    snapshot_seq: SnapshotSeq,
    /// Registers.
    registers: HashMap<RegisterName, RegisterContent>,
    /// Undo history per buffer.
    undo_history: HashMap<BufferId, UndoHistory>,
    /// Last change for repeat.
    #[allow(dead_code)]
    last_change: Option<Vec<Intent>>,
    /// Message to display.
    message: Option<String>,
}

/// Window state.
#[derive(Debug, Clone)]
#[allow(dead_code)]
struct WindowState {
    id: WindowId,
    buffer_id: BufferId,
    cursor: CursorState,
    viewport: ViewportState,
    selection: Option<Selection>,
    number: bool,
}

impl Default for Editor {
    fn default() -> Self {
        Self::new()
    }
}

impl Editor {
    /// Create new editor with scratch buffer.
    pub fn new() -> Self {
        let buffer_id = BufferId::new(1);
        let buffer = TextBuffer::new(buffer_id, BufferName::new("[Scratch]"));

        let window_id = WindowId::new();
        let window = WindowState {
            id: window_id,
            buffer_id,
            cursor: CursorState::default(),
            viewport: ViewportState::default(),
            selection: None,
            number: true,
        };

        let mut buffers = HashMap::new();
        buffers.insert(buffer_id, buffer);

        let mut windows = HashMap::new();
        windows.insert(window_id, window);

        let mut undo_history = HashMap::new();
        undo_history.insert(buffer_id, UndoHistory::new());

        Self {
            buffers,
            next_buffer_id: 2,
            windows,
            focused_window: window_id,
            mode_state: ModeState::new(),
            terminal_size: TerminalSize::new(80, 24),
            snapshot_seq: SnapshotSeq::new(0),
            registers: HashMap::new(),
            undo_history,
            last_change: None,
            message: None,
        }
    }

    /// Handle input event.
    pub fn handle_input(&mut self, event: InputEvent) -> Vec<Intent> {
        match event {
            InputEvent::Key(key) => self.handle_key(key),
            InputEvent::Resize { cols, rows } => {
                self.terminal_size = TerminalSize::new(cols, rows);
                self.update_all_viewports();
                vec![]
            }
            InputEvent::Paste(text) => {
                if self.mode_state.mode == Mode::Insert {
                    vec![Intent::InsertText(text)]
                } else {
                    vec![]
                }
            }
            _ => vec![],
        }
    }

    /// Handle key event and apply intents.
    fn handle_key(&mut self, key: KeyEvent) -> Vec<Intent> {
        let result = parse_key(&mut self.mode_state, &key);
        match result {
            ParseResult::Intent(intents) => {
                for intent in &intents {
                    self.apply_intent(intent.clone());
                }
                intents
            }
            ParseResult::Pending => vec![],
            ParseResult::Unhandled => vec![],
        }
    }

    /// Apply an intent to the editor state.
    pub fn apply_intent(&mut self, intent: Intent) {
        match intent {
            Intent::Nop => {}
            Intent::InsertText(text) => self.insert_text(&text),
            Intent::DeleteChar => self.delete_char(),
            Intent::Backspace => self.backspace(),
            Intent::MoveCursor(motion) => self.move_cursor(&motion),
            Intent::ChangeMode(mode) => self.change_mode(mode),
            Intent::EnterInsert(pos) => self.enter_insert(pos),
            Intent::Operator(op, motion) => self.apply_operator(op, motion),
            Intent::OperatorSelection(op) => self.apply_operator_selection(op),
            Intent::StartVisual(mode) => self.start_visual(mode),
            Intent::ToggleVisualMode(mode) => self.toggle_visual_mode(mode),
            Intent::Undo => self.undo(),
            Intent::Redo => self.redo(),
            Intent::Repeat => self.repeat(),
            Intent::SearchForward(pattern) => self.search_forward(&pattern),
            Intent::SearchBackward(pattern) => self.search_backward(&pattern),
            Intent::NextMatch => self.next_match(),
            Intent::PrevMatch => self.prev_match(),
            Intent::ExCommand(cmd) => self.execute_command(&cmd),
            Intent::SaveFile => self.save_file(),
            Intent::Quit => self.quit(),
            Intent::ForceQuit => self.force_quit(),
            Intent::OpenFile(path) => self.open_file(&path),
            Intent::CenterViewport => self.center_viewport(),
            Intent::ToggleExplorer => self.show_message("Explorer not implemented"),
            Intent::ToggleTerminal => self.show_message("Terminal not implemented"),
            Intent::OpenFinder => self.show_message("Finder not implemented"),
            _ => {}
        }
        self.ensure_cursor_visible();
    }

    fn insert_text(&mut self, text: &str) {
        let window = self.windows.get_mut(&self.focused_window).unwrap();
        let buffer = self.buffers.get_mut(&window.buffer_id).unwrap();
        let history = self.undo_history.get_mut(&window.buffer_id).unwrap();

        let cursor_before = window.cursor.position;
        buffer.insert(window.cursor.position, text);

        let _new_col = if text.contains('\n') {
            let lines: Vec<&str> = text.split('\n').collect();
            let new_lines = lines.len() - 1;
            window.cursor.position.line += new_lines;
            window.cursor.position.column = lines.last().map(|s| s.len()).unwrap_or(0);
            window.cursor.position.column
        } else {
            window.cursor.position.column += text.chars().count();
            window.cursor.position.column
        };

        history.push(EditOp {
            kind: EditKind::Insert {
                pos: cursor_before,
                text: text.to_string(),
            },
            cursor_before,
            cursor_after: window.cursor.position,
            version_before: buffer.version(),
        });
    }

    fn delete_char(&mut self) {
        let window = self.windows.get_mut(&self.focused_window).unwrap();
        let buffer = self.buffers.get_mut(&window.buffer_id).unwrap();

        if window.cursor.position.column < buffer.line_len(window.cursor.position.line)
            || window.cursor.position.line < buffer.line_count() - 1
        {
            buffer.delete_char(window.cursor.position);
        }
    }

    fn backspace(&mut self) {
        let window = self.windows.get_mut(&self.focused_window).unwrap();
        let buffer = self.buffers.get_mut(&window.buffer_id).unwrap();

        if window.cursor.position.column > 0 {
            window.cursor.position.column -= 1;
            buffer.delete_char(window.cursor.position);
        } else if window.cursor.position.line > 0 {
            let prev_line = window.cursor.position.line - 1;
            let prev_len = buffer.line_len(prev_line);
            window.cursor.position.line = prev_line;
            window.cursor.position.column = prev_len;
            buffer.delete_char(window.cursor.position);
        }
    }

    fn move_cursor(&mut self, motion: &Motion) {
        let window = self.windows.get_mut(&self.focused_window).unwrap();
        let buffer = self.buffers.get(&window.buffer_id).unwrap();
        let count = self.mode_state.effective_count();

        let new_pos = execute_motion(buffer, window.cursor.position, motion, count);
        let clamped = buffer.clamp_cursor(new_pos, self.mode_state.mode.is_end_inclusive());
        window.cursor.move_to(clamped);

        if let Some(ref mut sel) = window.selection {
            sel.cursor = LineCol::new(clamped.line, clamped.column);
        }
    }

    fn change_mode(&mut self, mode: Mode) {
        self.mode_state.mode = mode;
        let window = self.windows.get_mut(&self.focused_window).unwrap();

        if !mode.is_visual() {
            window.selection = None;
        }

        if !mode.is_end_inclusive() {
            let buffer = self.buffers.get(&window.buffer_id).unwrap();
            window.cursor.position = buffer.clamp_cursor(window.cursor.position, false);
        }
    }

    fn enter_insert(&mut self, pos: InsertPosition) {
        self.mode_state.enter_insert();
        let window = self.windows.get_mut(&self.focused_window).unwrap();
        let buffer = self.buffers.get_mut(&window.buffer_id).unwrap();

        match pos {
            InsertPosition::Before => {}
            InsertPosition::After => {
                let line_len = buffer.line_len(window.cursor.position.line);
                window.cursor.position.column = (window.cursor.position.column + 1).min(line_len);
            }
            InsertPosition::LineStart => {
                window.cursor.position.column = 0;
                let line = buffer.line(window.cursor.position.line);
                if let Some(l) = line {
                    let first_non_blank = l.chars().position(|c| !c.is_whitespace()).unwrap_or(0);
                    window.cursor.position.column = first_non_blank;
                }
            }
            InsertPosition::LineEnd => {
                let line_len = buffer.line_len(window.cursor.position.line);
                window.cursor.position.column = line_len;
            }
            InsertPosition::NewLineBelow => {
                let line_len = buffer.line_len(window.cursor.position.line);
                window.cursor.position.column = line_len;
                buffer.insert(window.cursor.position, "\n");
                window.cursor.position.line += 1;
                window.cursor.position.column = 0;
            }
            InsertPosition::NewLineAbove => {
                window.cursor.position.column = 0;
                buffer.insert(window.cursor.position, "\n");
                window.cursor.position.column = 0;
            }
        }
    }

    fn apply_operator(&mut self, op: Operator, motion: Motion) {
        let window = self.windows.get_mut(&self.focused_window).unwrap();
        let buffer = self.buffers.get_mut(&window.buffer_id).unwrap();
        let count = self.mode_state.effective_count();

        let start = window.cursor.position;
        let end = execute_motion(buffer, start, &motion, count);

        let linewise = matches!(motion, Motion::Down | Motion::Up);
        let result = execute_operator(buffer, op, start, end, linewise);

        if let Some(content) = result.text {
            self.registers.insert(self.mode_state.register, content);
        }

        window.cursor.position = result.cursor;

        if result.enter_insert {
            self.mode_state.enter_insert();
        }
    }

    fn apply_operator_selection(&mut self, op: Operator) {
        let window = self.windows.get_mut(&self.focused_window).unwrap();
        let buffer = self.buffers.get_mut(&window.buffer_id).unwrap();

        if let Some(sel) = window.selection.take() {
            let start = CursorPosition::new(sel.start().line, sel.start().col);
            let end = CursorPosition::new(sel.end().line, sel.end().col);
            let linewise = self.mode_state.mode == Mode::VisualLine;

            let result = execute_operator(buffer, op, start, end, linewise);

            if let Some(content) = result.text {
                self.registers.insert(self.mode_state.register, content);
            }

            window.cursor.position = result.cursor;

            if result.enter_insert {
                self.mode_state.enter_insert();
            } else {
                self.mode_state.reset();
            }
        }
    }

    fn start_visual(&mut self, _mode: VisualMode) {
        let window = self.windows.get_mut(&self.focused_window).unwrap();
        let pos = LineCol::new(window.cursor.position.line, window.cursor.position.column);
        window.selection = Some(Selection::new(pos, pos));
    }

    fn toggle_visual_mode(&mut self, _mode: VisualMode) {
        let window = self.windows.get_mut(&self.focused_window).unwrap();
        if window.selection.is_none() {
            let pos = LineCol::new(window.cursor.position.line, window.cursor.position.column);
            window.selection = Some(Selection::new(pos, pos));
        }
    }

    fn undo(&mut self) {
        let window = self.windows.get(&self.focused_window).unwrap();
        let buffer_id = window.buffer_id;

        if let Some(history) = self.undo_history.get_mut(&buffer_id) {
            if let Some(op) = history.pop_undo() {
                let buffer = self.buffers.get_mut(&buffer_id).unwrap();
                let window = self.windows.get_mut(&self.focused_window).unwrap();

                match &op.kind {
                    EditKind::Insert { pos, text } => {
                        let end = buffer.char_to_pos(buffer.pos_to_char(*pos) + text.len());
                        buffer.delete_range(*pos, end);
                    }
                    EditKind::Delete { pos, text } => {
                        buffer.insert(*pos, text);
                    }
                    EditKind::Replace {
                        pos,
                        old_text,
                        new_text,
                    } => {
                        let end = buffer.char_to_pos(buffer.pos_to_char(*pos) + new_text.len());
                        buffer.delete_range(*pos, end);
                        buffer.insert(*pos, old_text);
                    }
                }

                window.cursor.position = op.cursor_before;
            }
        }
    }

    fn redo(&mut self) {
        let window = self.windows.get(&self.focused_window).unwrap();
        let buffer_id = window.buffer_id;

        if let Some(history) = self.undo_history.get_mut(&buffer_id) {
            if let Some(op) = history.pop_redo() {
                let buffer = self.buffers.get_mut(&buffer_id).unwrap();
                let window = self.windows.get_mut(&self.focused_window).unwrap();

                match &op.kind {
                    EditKind::Insert { pos, text } => {
                        buffer.insert(*pos, text);
                    }
                    EditKind::Delete { pos, text } => {
                        let end = buffer.char_to_pos(buffer.pos_to_char(*pos) + text.len());
                        buffer.delete_range(*pos, end);
                    }
                    EditKind::Replace {
                        pos,
                        old_text,
                        new_text,
                    } => {
                        let end = buffer.char_to_pos(buffer.pos_to_char(*pos) + old_text.len());
                        buffer.delete_range(*pos, end);
                        buffer.insert(*pos, new_text);
                    }
                }

                window.cursor.position = op.cursor_after;
            }
        }
    }

    fn repeat(&mut self) {
        self.show_message("Repeat not fully implemented");
    }

    fn search_forward(&mut self, pattern: &str) {
        self.mode_state.search_pattern = pattern.to_string();
        self.mode_state.search_forward = true;
        self.next_match();
    }

    fn search_backward(&mut self, pattern: &str) {
        self.mode_state.search_pattern = pattern.to_string();
        self.mode_state.search_forward = false;
        self.prev_match();
    }

    fn next_match(&mut self) {
        if self.mode_state.search_pattern.is_empty() {
            return;
        }

        let window = self.windows.get_mut(&self.focused_window).unwrap();
        let buffer = self.buffers.get(&window.buffer_id).unwrap();
        let pattern = &self.mode_state.search_pattern;

        let start_line = window.cursor.position.line;
        let start_col = window.cursor.position.column + 1;

        for line_idx in start_line..buffer.line_count() {
            if let Some(line) = buffer.line(line_idx) {
                let search_start = if line_idx == start_line { start_col } else { 0 };
                if let Some(pos) = line[search_start..].find(pattern) {
                    window.cursor.position = CursorPosition::new(line_idx, search_start + pos);
                    return;
                }
            }
        }

        for line_idx in 0..=start_line {
            if let Some(line) = buffer.line(line_idx) {
                if let Some(pos) = line.find(pattern) {
                    window.cursor.position = CursorPosition::new(line_idx, pos);
                    return;
                }
            }
        }
    }

    fn prev_match(&mut self) {
        if self.mode_state.search_pattern.is_empty() {
            return;
        }

        let window = self.windows.get_mut(&self.focused_window).unwrap();
        let buffer = self.buffers.get(&window.buffer_id).unwrap();
        let pattern = &self.mode_state.search_pattern;

        let start_line = window.cursor.position.line;
        let start_col = window.cursor.position.column;

        for line_idx in (0..=start_line).rev() {
            if let Some(line) = buffer.line(line_idx) {
                let search_end = if line_idx == start_line {
                    start_col
                } else {
                    line.len()
                };
                if let Some(pos) = line[..search_end].rfind(pattern) {
                    window.cursor.position = CursorPosition::new(line_idx, pos);
                    return;
                }
            }
        }

        for line_idx in (start_line..buffer.line_count()).rev() {
            if let Some(line) = buffer.line(line_idx) {
                if let Some(pos) = line.rfind(pattern) {
                    window.cursor.position = CursorPosition::new(line_idx, pos);
                    return;
                }
            }
        }
    }

    fn execute_command(&mut self, cmd: &str) {
        let cmd = cmd.trim();
        if cmd.is_empty() {
            return;
        }

        match cmd {
            "w" | "write" => self.save_file(),
            "q" | "quit" => self.quit(),
            "q!" | "quit!" => self.force_quit(),
            "wq" | "x" => {
                self.save_file();
                self.quit();
            }
            _ => {
                if cmd.starts_with("e ") || cmd.starts_with("edit ") {
                    let path = cmd.split_whitespace().nth(1).unwrap_or("");
                    self.open_file(path);
                } else if cmd.starts_with("w ") || cmd.starts_with("write ") {
                    let path = cmd.split_whitespace().nth(1).unwrap_or("");
                    self.save_file_as(path);
                } else {
                    self.show_message(&format!("Unknown command: {}", cmd));
                }
            }
        }
    }

    fn save_file(&mut self) {
        let window = self.windows.get(&self.focused_window).unwrap();
        let buffer = self.buffers.get_mut(&window.buffer_id).unwrap();

        if let Some(path) = buffer.path().cloned() {
            match std::fs::write(&path, buffer.text()) {
                Ok(_) => {
                    buffer.mark_saved();
                    self.show_message(&format!("Wrote {}", path.display()));
                }
                Err(e) => {
                    self.show_message(&format!("Error writing file: {}", e));
                }
            }
        } else {
            self.show_message("No file name");
        }
    }

    fn save_file_as(&mut self, path: &str) {
        let window = self.windows.get(&self.focused_window).unwrap();
        let buffer = self.buffers.get_mut(&window.buffer_id).unwrap();

        let path = PathBuf::from(path);
        match std::fs::write(&path, buffer.text()) {
            Ok(_) => {
                buffer.set_path(path.clone());
                buffer.mark_saved();
                self.show_message(&format!("Wrote {}", path.display()));
            }
            Err(e) => {
                self.show_message(&format!("Error writing file: {}", e));
            }
        }
    }

    fn quit(&mut self) {
        let window = self.windows.get(&self.focused_window).unwrap();
        let buffer = self.buffers.get(&window.buffer_id).unwrap();

        if buffer.is_modified() {
            self.show_message("No write since last change (use :q! to override)");
        } else {
            std::process::exit(0);
        }
    }

    fn force_quit(&mut self) {
        std::process::exit(0);
    }

    fn open_file(&mut self, path: &str) {
        let path = PathBuf::from(path);
        match std::fs::read_to_string(&path) {
            Ok(content) => {
                let buffer_id = BufferId::new(self.next_buffer_id);
                self.next_buffer_id += 1;

                let buffer = TextBuffer::from_file(buffer_id, path.clone(), &content);
                self.buffers.insert(buffer_id, buffer);
                self.undo_history.insert(buffer_id, UndoHistory::new());

                let window = self.windows.get_mut(&self.focused_window).unwrap();
                window.buffer_id = buffer_id;
                window.cursor = CursorState::default();
                window.viewport = ViewportState::default();
                window.selection = None;

                self.show_message(&format!("Opened {}", path.display()));
            }
            Err(e) => {
                self.show_message(&format!("Error opening file: {}", e));
            }
        }
    }

    fn center_viewport(&mut self) {
        let window = self.windows.get_mut(&self.focused_window).unwrap();
        let cursor_line = window.cursor.position.line;
        let half_height = (window.viewport.text_rows / 2) as usize;
        window.viewport.top_line = cursor_line.saturating_sub(half_height);
    }

    fn ensure_cursor_visible(&mut self) {
        let window = self.windows.get_mut(&self.focused_window).unwrap();
        let buffer = self.buffers.get(&window.buffer_id).unwrap();

        window
            .viewport
            .ensure_cursor_visible(window.cursor.position.line, buffer.line_count());
    }

    fn update_all_viewports(&mut self) {
        for window in self.windows.values_mut() {
            window.viewport.text_rows = self.terminal_size.rows.saturating_sub(2);
            window.viewport.text_cols = self.terminal_size.cols;
        }
    }

    fn show_message(&mut self, msg: &str) {
        self.message = Some(msg.to_string());
    }

    /// Get current mode.
    pub fn mode(&self) -> Mode {
        self.mode_state.mode
    }

    /// Get command line input.
    pub fn cmdline(&self) -> &str {
        &self.mode_state.cmdline
    }

    /// Produce a snapshot for rendering.
    pub fn snapshot(&mut self) -> EditorSnapshot {
        self.snapshot_seq = self.snapshot_seq.next();

        let mut window_snapshots = Vec::new();

        for (&id, window) in &self.windows {
            let buffer = self.buffers.get(&window.buffer_id).unwrap();
            let rect = WindowRect::new(
                0,
                0,
                self.terminal_size.cols,
                self.terminal_size.rows.saturating_sub(2),
            );

            let mut lines = Vec::new();
            let start_line = window.viewport.top_line;
            let end_line =
                (start_line + window.viewport.text_rows as usize).min(buffer.line_count());

            for line_idx in start_line..end_line {
                if let Some(text) = buffer.line(line_idx) {
                    lines.push(LineSnapshot {
                        line_number: line_idx + 1,
                        text: text.trim_end_matches('\n').to_string(),
                        highlights: vec![],
                        diagnostics: vec![],
                        git_status: None,
                    });
                }
            }

            let cursor_shape = match self.mode_state.mode {
                Mode::Normal => CursorShape::Block,
                Mode::Insert => CursorShape::Bar,
                Mode::Visual | Mode::VisualLine | Mode::VisualBlock => CursorShape::Hollow,
                Mode::Replace => CursorShape::Underline,
                Mode::Command => CursorShape::Block,
            };

            window_snapshots.push(WindowSnapshot {
                id,
                buffer: buffer.meta(),
                lines,
                cursor: window.cursor.position,
                cursor_shape,
                viewport: window.viewport.clone(),
                selection: window.selection,
                rect,
                show_numbers: window.number,
                focused: id == self.focused_window,
            });
        }

        let focused_window = self.windows.get(&self.focused_window).unwrap();
        let focused_buffer = self.buffers.get(&focused_window.buffer_id).unwrap();

        let statusline = StatusLine::from_editor_state(
            self.mode_state.mode,
            &focused_buffer.meta(),
            focused_window.cursor.position,
        );

        let cmdline = if self.mode_state.mode == Mode::Command {
            let prompt = if self.mode_state.search_forward {
                '/'
            } else if !self.mode_state.search_pattern.is_empty() {
                '?'
            } else {
                ':'
            };
            Some(kjxlkj_core_ui::CommandLineState::new(
                prompt,
                self.mode_state.cmdline.clone(),
            ))
        } else {
            None
        };

        EditorSnapshot {
            seq: self.snapshot_seq,
            mode: self.mode_state.mode,
            windows: window_snapshots,
            focused_window: self.focused_window,
            terminal_size: self.terminal_size,
            statusline,
            cmdline,
            message: self.message.take(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_editor() {
        let editor = Editor::new();
        assert_eq!(editor.mode(), Mode::Normal);
    }

    #[test]
    fn test_insert_text() {
        let mut editor = Editor::new();
        editor.apply_intent(Intent::EnterInsert(InsertPosition::Before));
        editor.apply_intent(Intent::InsertText("hello".to_string()));
        let snapshot = editor.snapshot();
        assert!(!snapshot.windows.is_empty());
    }

    #[test]
    fn test_mode_change() {
        let mut editor = Editor::new();
        assert_eq!(editor.mode(), Mode::Normal);
        editor.apply_intent(Intent::EnterInsert(InsertPosition::Before));
        assert_eq!(editor.mode(), Mode::Insert);
        editor.apply_intent(Intent::ChangeMode(Mode::Normal));
        assert_eq!(editor.mode(), Mode::Normal);
    }
}
