//! Text editing operations on the focused buffer.
//! See /docs/spec/editing/README.md.

use kjxlkj_core_types::{ContentKind, RangeType};
use crate::editor::EditorState;
use crate::window_state::WindowState;

impl EditorState {
    pub(crate) fn focused_window_mut(&mut self) -> &mut WindowState {
        self.windows.get_mut(&self.focus.focused).expect("focused window must exist")
    }

    pub(crate) fn insert_char(&mut self, c: char) {
        let win = self.windows.get(&self.focus.focused).unwrap();
        if let ContentKind::Buffer(buf_id) = win.content {
            let (line, col) = (win.cursor.line, win.cursor.col);
            if let Some(buf) = self.buffers.get_mut(&buf_id) {
                if c == '\n' {
                    let _ = buf.insert(line, col, "\n");
                    let w = self.focused_window_mut();
                    w.cursor.line += 1;
                    w.cursor.col = 0;
                } else {
                    let _ = buf.insert(line, col, &c.to_string());
                    self.focused_window_mut().cursor.col += 1;
                }
            }
        }
    }

    pub(crate) fn delete_char_forward(&mut self) {
        let win = self.windows.get(&self.focus.focused).unwrap();
        if let ContentKind::Buffer(buf_id) = win.content {
            let (line, col) = (win.cursor.line, win.cursor.col);
            if let Some(buf) = self.buffers.get_mut(&buf_id) {
                let _ = buf.delete(line, col, line, col + 1);
            }
        }
    }

    pub(crate) fn delete_char_backward(&mut self) {
        let win = self.windows.get(&self.focus.focused).unwrap();
        if let ContentKind::Buffer(buf_id) = win.content {
            let (line, col) = (win.cursor.line, win.cursor.col);
            if col > 0 {
                if let Some(buf) = self.buffers.get_mut(&buf_id) {
                    let _ = buf.delete(line, col - 1, line, col);
                    self.focused_window_mut().cursor.col -= 1;
                }
            }
        }
    }

    pub(crate) fn cursor_to_eol(&mut self) {
        let win = self.windows.get(&self.focus.focused).unwrap();
        if let ContentKind::Buffer(buf_id) = win.content {
            if let Some(buf) = self.buffers.get(&buf_id) {
                let text = buf.line(win.cursor.line).unwrap_or_default();
                let vc = text.trim_end_matches('\n').chars().count();
                let w = self.focused_window_mut();
                w.cursor.col = vc;
                w.cursor.desired_col = vc;
            }
        }
    }

    pub(crate) fn cursor_to_first_nonblank(&mut self) {
        let win = self.windows.get(&self.focus.focused).unwrap();
        if let ContentKind::Buffer(buf_id) = win.content {
            if let Some(buf) = self.buffers.get(&buf_id) {
                let text = buf.line(win.cursor.line).unwrap_or_default();
                let col = text.chars().position(|c| !c.is_whitespace()).unwrap_or(0);
                let w = self.focused_window_mut();
                w.cursor.col = col;
                w.cursor.desired_col = col;
            }
        }
    }

    pub(crate) fn open_line_below(&mut self) {
        let win = self.windows.get(&self.focus.focused).unwrap();
        if let ContentKind::Buffer(buf_id) = win.content {
            let line = win.cursor.line;
            if let Some(buf) = self.buffers.get_mut(&buf_id) {
                let gc = buf.line_grapheme_count(line);
                let _ = buf.insert(line, gc, "\n");
                let w = self.focused_window_mut();
                w.cursor.line = line + 1;
                w.cursor.col = 0;
            }
        }
    }

    pub(crate) fn open_line_above(&mut self) {
        let win = self.windows.get(&self.focus.focused).unwrap();
        if let ContentKind::Buffer(buf_id) = win.content {
            let line = win.cursor.line;
            if let Some(buf) = self.buffers.get_mut(&buf_id) {
                let _ = buf.insert(line, 0, "\n");
                self.focused_window_mut().cursor.col = 0;
            }
        }
    }

    /// Put from effective register after cursor.
    pub(crate) fn put_after(&mut self) {
        let entry = match self.get_put_entry() { Some(e) => e, None => return };
        let win = self.windows.get(&self.focus.focused).unwrap();
        if let ContentKind::Buffer(buf_id) = win.content {
            let (line, col) = (win.cursor.line, win.cursor.col);
            if entry.scope == RangeType::Linewise {
                if let Some(buf) = self.buffers.get_mut(&buf_id) {
                    let gc = buf.line_grapheme_count(line);
                    let clean = entry.text.trim_end_matches('\n');
                    let _ = buf.insert(line, gc, &format!("\n{clean}"));
                    let w = self.focused_window_mut();
                    w.cursor.line = line + 1;
                    w.cursor.col = 0;
                }
            } else if let Some(buf) = self.buffers.get_mut(&buf_id) {
                let _ = buf.insert(line, col + 1, &entry.text);
                self.focused_window_mut().cursor.col = col + entry.text.chars().count();
            }
        }
    }

    /// Put from effective register before cursor.
    pub(crate) fn put_before(&mut self) {
        let entry = match self.get_put_entry() { Some(e) => e, None => return };
        let win = self.windows.get(&self.focus.focused).unwrap();
        if let ContentKind::Buffer(buf_id) = win.content {
            let (line, col) = (win.cursor.line, win.cursor.col);
            if entry.scope == RangeType::Linewise {
                if let Some(buf) = self.buffers.get_mut(&buf_id) {
                    let text = if entry.text.ends_with('\n') {
                        entry.text.clone()
                    } else {
                        format!("{}\n", entry.text)
                    };
                    let _ = buf.insert(line, 0, &text);
                    self.focused_window_mut().cursor.col = 0;
                }
            } else if let Some(buf) = self.buffers.get_mut(&buf_id) {
                let _ = buf.insert(line, col, &entry.text);
                let nc = col + entry.text.chars().count().saturating_sub(1);
                self.focused_window_mut().cursor.col = nc;
            }
        }
    }

    /// Get the register entry for put, falling back to unnamed.
    fn get_put_entry(&mut self) -> Option<crate::register::RegisterEntry> {
        let reg = self.registers.effective();
        let entry = self.registers.get(reg)
            .or_else(|| self.registers.get('"'))
            .cloned();
        self.registers.clear_selection();
        entry
    }

    /// Clamp cursor to valid buffer bounds.
    pub(crate) fn clamp_cursor(&mut self) {
        let win = self.windows.get(&self.focus.focused).unwrap();
        if let ContentKind::Buffer(buf_id) = win.content {
            let cur_line = win.cursor.line;
            let cur_col = win.cursor.col;
            if let Some(buf) = self.buffers.get(&buf_id) {
                let lc = buf.line_count().max(1);
                let clamped_line = if cur_line >= lc { lc - 1 } else { cur_line };
                let gc = buf.line_grapheme_count(clamped_line);
                let max_col = if gc > 0 { gc - 1 } else { 0 };
                let clamped_col = if cur_col > max_col { max_col } else { cur_col };
                let w = self.focused_window_mut();
                w.cursor.line = clamped_line;
                w.cursor.col = clamped_col;
            }
        }
    }
}
