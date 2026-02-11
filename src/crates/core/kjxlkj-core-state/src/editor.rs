//! Central editor state owned by the core task.
//!
//! See /docs/spec/architecture/runtime.md for ownership model.
//! See /docs/spec/editor/README.md for state overview.

use std::collections::HashMap;

use kjxlkj_core_edit::{apply_motion, Cursor};
use kjxlkj_core_mode::{dispatch_key, resolve_mode_transition};
use kjxlkj_core_text::Buffer;
use kjxlkj_core_types::{
    Action, BufferId, CmdlineState, ContentKind, EditorSnapshot,
    Key, KeyModifiers, Mode, Rect, WindowContent, WindowId,
};
use kjxlkj_core_ui::{FocusState, LayoutTree};

use crate::window_state::WindowState;

/// The single mutable editor state.
pub struct EditorState {
    pub mode: Mode,
    pub buffers: HashMap<BufferId, Buffer>,
    pub layout: LayoutTree,
    pub focus: FocusState,
    pub windows: HashMap<WindowId, WindowState>,
    pub terminal_size: (u16, u16),
    pub cmdline: CmdlineState,
    pub quit_requested: bool,
    pub sequence: u64,
    id_counter: u64,
}

impl EditorState {
    /// Create initial editor state with one scratch buffer.
    pub fn new(cols: u16, rows: u16) -> Self {
        let buf_id = BufferId(0);
        let win_id = WindowId(1);
        let buf = Buffer::new_scratch(buf_id);
        let content = ContentKind::Buffer(buf_id);

        let mut buffers = HashMap::new();
        buffers.insert(buf_id, buf);

        let layout = LayoutTree::single(win_id, content);
        let focus = FocusState::new(win_id);

        let mut windows = HashMap::new();
        windows.insert(
            win_id,
            WindowState::new(win_id, content),
        );

        Self {
            mode: Mode::Normal,
            buffers,
            layout,
            focus,
            windows,
            terminal_size: (cols, rows),
            cmdline: CmdlineState::default(),
            quit_requested: false,
            sequence: 0,
            id_counter: 2,
        }
    }

    /// Process a key event through the mode dispatch pipeline.
    pub fn handle_key(
        &mut self,
        key: &Key,
        mods: &KeyModifiers,
    ) {
        let (action, new_mode) =
            dispatch_key(self.mode, key, mods);
        self.mode =
            resolve_mode_transition(self.mode, new_mode);
        self.apply_action(action);
        self.sequence += 1;
    }

    /// Apply a typed action to editor state.
    pub fn apply_action(&mut self, action: Action) {
        match action {
            Action::InsertChar(c) => self.insert_char(c),
            Action::DeleteCharForward => self.delete_char_forward(),
            Action::DeleteCharBackward => {
                self.delete_char_backward()
            }
            Action::Motion(motion) => {
                let wid = self.focus.focused;
                let win = self.windows.get(&wid).unwrap();
                if let ContentKind::Buffer(buf_id) = win.content {
                    if let Some(buf) = self.buffers.get(&buf_id) {
                        let cur = win.cursor;
                        let new_cur = apply_motion(
                            &cur, &motion, buf,
                        );
                        self.windows
                            .get_mut(&wid)
                            .unwrap()
                            .cursor = new_cur;
                    }
                }
            }
            Action::Quit => self.quit_requested = true,
            Action::ForceQuit => self.quit_requested = true,
            Action::WriteQuit => self.quit_requested = true,
            Action::Resize(cols, rows) => {
                self.terminal_size = (cols, rows);
            }
            Action::AppendEndOfLine => {
                self.cursor_to_eol();
            }
            Action::InsertFirstNonBlank => {
                self.cursor_to_first_nonblank();
            }
            Action::OpenLineBelow => {
                self.open_line_below();
            }
            Action::OpenLineAbove => {
                self.open_line_above();
            }
            Action::SplitVertical => {
                self.split_vertical();
            }
            Action::SplitHorizontal => {
                self.split_horizontal();
            }
            Action::CloseWindow => {
                self.close_window();
            }
            Action::ExitToNormal => {
                self.mode = Mode::Normal;
                // Adjust cursor left when leaving insert.
                let win = self.focused_window_mut();
                if win.cursor.col > 0 {
                    win.cursor.col -= 1;
                }
            }
            _ => {}
        }
    }

    /// Build an immutable snapshot for rendering.
    pub fn snapshot(&self) -> EditorSnapshot {
        let area = Rect::new(
            0,
            0,
            self.terminal_size.0,
            self.terminal_size.1,
        );
        let rects = self.layout.compute_rects(area);
        let mut window_contents = Vec::new();

        for (wid, content, rect) in &rects {
            let ws = self.windows.get(wid);
            let (lines, cursor_row, cursor_col, wtype, status) =
                match content {
                    ContentKind::Buffer(bid) => {
                        let buf = self
                            .buffers
                            .get(bid)
                            .map(|b| {
                                self.buffer_lines(
                                    b, ws, *rect,
                                )
                            })
                            .unwrap_or_default();
                        let cr = ws
                            .map(|w| {
                                w.cursor.line
                                    .saturating_sub(
                                        w.top_line,
                                    )
                            })
                            .unwrap_or(0);
                        let cc = ws
                            .map(|w| w.cursor.col)
                            .unwrap_or(0);
                        let name = self
                            .buffers
                            .get(bid)
                            .map(|b| b.name.clone())
                            .unwrap_or_default();
                        let st = format!(
                            " {} | {} ",
                            self.mode.display_name(),
                            name
                        );
                        (buf, cr, cc, "buffer", st)
                    }
                    ContentKind::Explorer(_) => {
                        (vec!["[Explorer]".into()], 0, 0, "explorer", " EXPLORER ".into())
                    }
                    ContentKind::Terminal(_) => {
                        (vec!["[Terminal]".into()], 0, 0, "terminal", " TERMINAL ".into())
                    }
                };

            window_contents.push(WindowContent {
                window_id: *wid,
                rect: *rect,
                lines,
                cursor_row,
                cursor_col,
                window_type: wtype.to_string(),
                statusline: status,
            });
        }

        let layout_summary = format!(
            "windows={} focused={}",
            rects.len(),
            self.focus.focused.0
        );

        EditorSnapshot {
            sequence: self.sequence,
            mode: self.mode,
            terminal_size: self.terminal_size,
            cmdline: self.cmdline.clone(),
            notifications: Vec::new(),
            layout_summary,
            focused_window: self.focus.focused,
            window_contents,
        }
    }

    fn buffer_lines(
        &self,
        buf: &Buffer,
        ws: Option<&WindowState>,
        rect: Rect,
    ) -> Vec<String> {
        let top = ws.map(|w| w.top_line).unwrap_or(0);
        let height = rect.height.saturating_sub(1) as usize;
        let mut lines = Vec::with_capacity(height);
        for i in 0..height {
            let line_idx = top + i;
            if let Some(content) = buf.line(line_idx) {
                lines.push(
                    content.trim_end_matches('\n').to_string(),
                );
            } else {
                lines.push("~".to_string());
            }
        }
        lines
    }

    fn focused_window_mut(&mut self) -> &mut WindowState {
        self.windows
            .get_mut(&self.focus.focused)
            .expect("focused window must exist")
    }

    fn next_id(&mut self) -> u64 {
        let id = self.id_counter;
        self.id_counter += 1;
        id
    }

    fn insert_char(&mut self, c: char) {
        let win = self.windows.get(&self.focus.focused).unwrap();
        if let ContentKind::Buffer(buf_id) = win.content {
            let line = win.cursor.line;
            let col = win.cursor.col;
            if let Some(buf) = self.buffers.get_mut(&buf_id) {
                if c == '\n' {
                    let _ = buf.insert(line, col, "\n");
                    let win = self.focused_window_mut();
                    win.cursor.line += 1;
                    win.cursor.col = 0;
                } else {
                    let s = c.to_string();
                    let _ = buf.insert(line, col, &s);
                    let win = self.focused_window_mut();
                    win.cursor.col += 1;
                }
            }
        }
    }

    fn delete_char_forward(&mut self) {
        let win = self.windows.get(&self.focus.focused).unwrap();
        if let ContentKind::Buffer(buf_id) = win.content {
            let line = win.cursor.line;
            let col = win.cursor.col;
            if let Some(buf) = self.buffers.get_mut(&buf_id) {
                let _ = buf.delete(line, col, line, col + 1);
            }
        }
    }

    fn delete_char_backward(&mut self) {
        let win = self.windows.get(&self.focus.focused).unwrap();
        if let ContentKind::Buffer(buf_id) = win.content {
            let line = win.cursor.line;
            let col = win.cursor.col;
            if col > 0 {
                if let Some(buf) =
                    self.buffers.get_mut(&buf_id)
                {
                    let _ =
                        buf.delete(line, col - 1, line, col);
                    let win = self.focused_window_mut();
                    win.cursor.col -= 1;
                }
            }
        }
    }

    fn cursor_to_eol(&mut self) {
        let win = self.windows.get(&self.focus.focused).unwrap();
        if let ContentKind::Buffer(buf_id) = win.content {
            if let Some(buf) = self.buffers.get(&buf_id) {
                let gc = buf.line_grapheme_count(win.cursor.line);
                let text = buf
                    .line(win.cursor.line)
                    .unwrap_or_default();
                let trimmed =
                    text.trim_end_matches('\n');
                let visible_count: usize = trimmed
                    .chars()
                    .count();
                let win = self.focused_window_mut();
                win.cursor.col = visible_count;
                win.cursor.desired_col = visible_count;
            }
        }
    }

    fn cursor_to_first_nonblank(&mut self) {
        let win = self.windows.get(&self.focus.focused).unwrap();
        if let ContentKind::Buffer(buf_id) = win.content {
            if let Some(buf) = self.buffers.get(&buf_id) {
                let text = buf
                    .line(win.cursor.line)
                    .unwrap_or_default();
                let col = text
                    .chars()
                    .position(|c| !c.is_whitespace())
                    .unwrap_or(0);
                let win = self.focused_window_mut();
                win.cursor.col = col;
                win.cursor.desired_col = col;
            }
        }
    }

    fn open_line_below(&mut self) {
        let win = self.windows.get(&self.focus.focused).unwrap();
        if let ContentKind::Buffer(buf_id) = win.content {
            let line = win.cursor.line;
            if let Some(buf) = self.buffers.get_mut(&buf_id) {
                let gc = buf.line_grapheme_count(line);
                let _ = buf.insert(line, gc, "\n");
                let win = self.focused_window_mut();
                win.cursor.line = line + 1;
                win.cursor.col = 0;
            }
        }
    }

    fn open_line_above(&mut self) {
        let win = self.windows.get(&self.focus.focused).unwrap();
        if let ContentKind::Buffer(buf_id) = win.content {
            let line = win.cursor.line;
            if let Some(buf) = self.buffers.get_mut(&buf_id) {
                let _ = buf.insert(line, 0, "\n");
                let win = self.focused_window_mut();
                win.cursor.col = 0;
            }
        }
    }

    fn split_vertical(&mut self) {
        let focused = self.focus.focused;
        let win = self.windows.get(&focused).unwrap();
        let content = win.content;
        let new_wid = WindowId(self.next_id());
        self.layout.split_vertical(
            focused, new_wid, content,
        );
        self.windows.insert(
            new_wid,
            WindowState::new(new_wid, content),
        );
        self.focus.set_focus(new_wid);
    }

    fn split_horizontal(&mut self) {
        let focused = self.focus.focused;
        let win = self.windows.get(&focused).unwrap();
        let content = win.content;
        let new_wid = WindowId(self.next_id());
        self.layout.split_horizontal(
            focused, new_wid, content,
        );
        self.windows.insert(
            new_wid,
            WindowState::new(new_wid, content),
        );
        self.focus.set_focus(new_wid);
    }

    fn close_window(&mut self) {
        let focused = self.focus.focused;
        let ids = self.layout.window_ids();
        if ids.len() <= 1 {
            return;
        }
        if self.layout.close_window(focused) {
            self.windows.remove(&focused);
            let remaining = self.layout.window_ids();
            let fallback = remaining
                .first()
                .copied()
                .unwrap_or(WindowId(0));
            self.focus.on_window_closed(focused, fallback);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initial_state() {
        let s = EditorState::new(80, 24);
        assert_eq!(s.mode, Mode::Normal);
        assert_eq!(s.buffers.len(), 1);
        assert_eq!(s.windows.len(), 1);
        assert!(!s.quit_requested);
    }

    #[test]
    fn insert_and_exit() {
        let mut s = EditorState::new(80, 24);
        // Enter insert mode.
        s.handle_key(
            &Key::Char('i'),
            &KeyModifiers::default(),
        );
        assert_eq!(s.mode, Mode::Insert);

        // Type a character.
        s.handle_key(
            &Key::Char('x'),
            &KeyModifiers::default(),
        );

        // Exit to normal.
        s.handle_key(
            &Key::Escape,
            &KeyModifiers::default(),
        );
        assert_eq!(s.mode, Mode::Normal);
    }

    #[test]
    fn shift_a_appends_at_eol() {
        let mut s = EditorState::new(80, 24);
        // Put some text in the buffer.
        let buf_id = BufferId(0);
        s.buffers.get_mut(&buf_id).unwrap().insert(
            0, 0, "hello",
        ).unwrap();

        // Press A (Shift+a normalized).
        s.handle_key(
            &Key::Char('A'),
            &KeyModifiers::default(),
        );
        assert_eq!(s.mode, Mode::Insert);
        // Cursor should be at end of line.
        let win =
            s.windows.get(&s.focus.focused).unwrap();
        assert_eq!(win.cursor.col, 5);
    }

    #[test]
    fn snapshot_works() {
        let s = EditorState::new(80, 24);
        let snap = s.snapshot();
        assert_eq!(snap.terminal_size, (80, 24));
        assert_eq!(snap.window_contents.len(), 1);
    }

    #[test]
    fn split_and_close() {
        let mut s = EditorState::new(80, 24);
        s.apply_action(Action::SplitVertical);
        assert_eq!(s.windows.len(), 2);
        s.apply_action(Action::CloseWindow);
        assert_eq!(s.windows.len(), 1);
    }
}
