//! Text editing operations on the focused buffer.
//!
//! Insert, delete, cursor movement helpers.
//! See /docs/spec/editing/README.md.

use kjxlkj_core_types::ContentKind;

use crate::editor::EditorState;
use crate::window_state::WindowState;

impl EditorState {
    pub(crate) fn focused_window_mut(
        &mut self,
    ) -> &mut WindowState {
        self.windows
            .get_mut(&self.focus.focused)
            .expect("focused window must exist")
    }

    pub(crate) fn insert_char(&mut self, c: char) {
        let win =
            self.windows.get(&self.focus.focused).unwrap();
        if let ContentKind::Buffer(buf_id) = win.content {
            let line = win.cursor.line;
            let col = win.cursor.col;
            if let Some(buf) =
                self.buffers.get_mut(&buf_id)
            {
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

    pub(crate) fn delete_char_forward(&mut self) {
        let win =
            self.windows.get(&self.focus.focused).unwrap();
        if let ContentKind::Buffer(buf_id) = win.content {
            let line = win.cursor.line;
            let col = win.cursor.col;
            if let Some(buf) =
                self.buffers.get_mut(&buf_id)
            {
                let _ =
                    buf.delete(line, col, line, col + 1);
            }
        }
    }

    pub(crate) fn delete_char_backward(&mut self) {
        let win =
            self.windows.get(&self.focus.focused).unwrap();
        if let ContentKind::Buffer(buf_id) = win.content {
            let line = win.cursor.line;
            let col = win.cursor.col;
            if col > 0 {
                if let Some(buf) =
                    self.buffers.get_mut(&buf_id)
                {
                    let _ = buf.delete(
                        line,
                        col - 1,
                        line,
                        col,
                    );
                    let win = self.focused_window_mut();
                    win.cursor.col -= 1;
                }
            }
        }
    }

    pub(crate) fn cursor_to_eol(&mut self) {
        let win =
            self.windows.get(&self.focus.focused).unwrap();
        if let ContentKind::Buffer(buf_id) = win.content {
            if let Some(buf) = self.buffers.get(&buf_id) {
                let text = buf
                    .line(win.cursor.line)
                    .unwrap_or_default();
                let trimmed =
                    text.trim_end_matches('\n');
                let visible_count: usize =
                    trimmed.chars().count();
                let win = self.focused_window_mut();
                win.cursor.col = visible_count;
                win.cursor.desired_col = visible_count;
            }
        }
    }

    pub(crate) fn cursor_to_first_nonblank(&mut self) {
        let win =
            self.windows.get(&self.focus.focused).unwrap();
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

    pub(crate) fn open_line_below(&mut self) {
        let win =
            self.windows.get(&self.focus.focused).unwrap();
        if let ContentKind::Buffer(buf_id) = win.content {
            let line = win.cursor.line;
            if let Some(buf) =
                self.buffers.get_mut(&buf_id)
            {
                let gc = buf.line_grapheme_count(line);
                let _ = buf.insert(line, gc, "\n");
                let win = self.focused_window_mut();
                win.cursor.line = line + 1;
                win.cursor.col = 0;
            }
        }
    }

    pub(crate) fn open_line_above(&mut self) {
        let win =
            self.windows.get(&self.focus.focused).unwrap();
        if let ContentKind::Buffer(buf_id) = win.content {
            let line = win.cursor.line;
            if let Some(buf) =
                self.buffers.get_mut(&buf_id)
            {
                let _ = buf.insert(line, 0, "\n");
                let win = self.focused_window_mut();
                win.cursor.col = 0;
            }
        }
    }
}
