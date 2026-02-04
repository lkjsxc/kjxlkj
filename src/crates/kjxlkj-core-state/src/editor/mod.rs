//! Main editor state.

mod snapshot;
#[cfg(test)]
mod tests;

use kjxlkj_core_mode::ModeState;
use kjxlkj_core_text::TextBuffer;
use kjxlkj_core_types::{BufferId, Cursor, Mode, Position, Selection, SelectionKind};
use kjxlkj_core_ui::Viewport;
use kjxlkj_core_undo::UndoHistory;

use crate::{MarkStore, RegisterStore};

/// The main editor state.
#[derive(Debug)]
pub struct EditorState {
    pub buffer: TextBuffer,
    pub cursor: Cursor,
    pub selection: Option<Selection>,
    pub mode_state: ModeState,
    pub undo: UndoHistory,
    pub registers: RegisterStore,
    pub marks: MarkStore,
    pub viewport: Viewport,
    pub status_message: Option<String>,
    pub should_quit: bool,
    jump_list: Vec<Position>,
    jump_index: usize,
    change_list: Vec<Position>,
    change_index: usize,
    #[allow(dead_code)]
    next_buffer_id: u64,
}

impl EditorState {
    pub fn new() -> Self {
        Self {
            buffer: TextBuffer::new(BufferId::new(0)),
            cursor: Cursor::origin(),
            selection: None,
            mode_state: ModeState::new(),
            undo: UndoHistory::new(),
            registers: RegisterStore::new(),
            marks: MarkStore::new(),
            viewport: Viewport::default(),
            status_message: None,
            should_quit: false,
            jump_list: Vec::new(),
            jump_index: 0,
            change_list: Vec::new(),
            change_index: 0,
            next_buffer_id: 1,
        }
    }

    pub fn mode(&self) -> Mode { self.mode_state.mode }

    pub fn set_mode(&mut self, mode: Mode) {
        self.mode_state.set_mode(mode);
        if mode.is_visual() {
            let kind = match mode {
                Mode::Visual => SelectionKind::Char,
                Mode::VisualLine => SelectionKind::Line,
                Mode::VisualBlock => SelectionKind::Block,
                _ => SelectionKind::Char,
            };
            self.selection = Some(Selection::new(self.cursor.position, self.cursor.position, kind));
        } else {
            self.selection = None;
        }
    }

    pub fn set_status(&mut self, msg: impl Into<String>) { self.status_message = Some(msg.into()); }
    pub fn clear_status(&mut self) { self.status_message = None; }

    pub fn push_jump(&mut self) {
        self.jump_list.truncate(self.jump_index);
        self.jump_list.push(self.cursor.position);
        self.jump_index = self.jump_list.len();
    }

    pub fn jump_backward(&mut self) -> Option<Position> {
        if self.jump_index > 0 {
            self.jump_index -= 1;
            Some(self.jump_list[self.jump_index])
        } else { None }
    }

    pub fn jump_forward(&mut self) -> Option<Position> {
        if self.jump_index < self.jump_list.len() {
            let pos = self.jump_list[self.jump_index];
            self.jump_index += 1;
            Some(pos)
        } else { None }
    }

    pub fn push_change(&mut self, pos: Position) {
        self.change_list.push(pos);
        self.change_index = self.change_list.len();
    }

    pub fn ensure_cursor_visible(&mut self) { self.viewport.ensure_visible(self.cursor.line(), 3); }

    pub fn clamp_cursor(&mut self) {
        let line_count = self.buffer.line_count();
        if line_count == 0 { self.cursor = Cursor::origin(); return; }
        let line = self.cursor.line().min(line_count.saturating_sub(1));
        let max_col = if self.mode().is_insert() {
            self.buffer.line_len(line)
        } else {
            self.buffer.line_len(line).saturating_sub(1)
        };
        let col = self.cursor.col().min(max_col);
        self.cursor.position = Position::new(line, col);
    }
}

impl Default for EditorState {
    fn default() -> Self { Self::new() }
}
