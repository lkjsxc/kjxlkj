//! Visual block replace: r{char} replaces every character in block.

use kjxlkj_core_text::RopeExt;
use kjxlkj_core_types::{CursorPosition, VisualKind};

use crate::editor::EditorState;

impl EditorState {
    /// Replace every character in the visual selection with `ch`.
    pub(crate) fn visual_replace(&mut self, ch: char, kind: VisualKind) {
        let anchor = match self.visual_anchor.take() {
            Some(a) => a,
            None => {
                self.mode = kjxlkj_core_types::Mode::Normal;
                return;
            }
        };
        let cursor = self.windows.focused().cursor;
        let (start, end) = order(anchor, cursor);
        let buf_id = self.current_buffer_id();
        if let Some(buf) = self.buffers.get_mut(buf_id) {
            buf.save_undo_checkpoint(cursor);
            match kind {
                VisualKind::Block => {
                    let c0 = start.grapheme.min(end.grapheme);
                    let c1 = start.grapheme.max(end.grapheme);
                    for line in (start.line..=end.line).rev() {
                        let len = buf.content.line_grapheme_count(line);
                        for col in (c0..=c1.min(len.saturating_sub(1))).rev() {
                            replace_one(&mut buf.content, line, col, ch);
                        }
                    }
                }
                VisualKind::Line => {
                    for line in (start.line..=end.line).rev() {
                        let len = buf.content.line_grapheme_count(line);
                        // Skip newline at end of line.
                        let end_col = if len > 0 { len - 1 } else { 0 };
                        for col in (0..end_col).rev() {
                            replace_one(&mut buf.content, line, col, ch);
                        }
                    }
                }
                VisualKind::Char => {
                    replace_char_range(&mut buf.content, start, end, ch);
                }
            }
            buf.increment_version();
        }
        self.windows.focused_mut().cursor = start;
        self.mode = kjxlkj_core_types::Mode::Normal;
        self.clamp_cursor();
        self.ensure_cursor_visible();
    }
}

/// Replace a single grapheme at (line, col) with `ch`.
fn replace_one(rope: &mut kjxlkj_core_text::Rope, line: usize, col: usize, ch: char) {
    rope.delete_grapheme_range(line, col, line, col + 1);
    rope.insert_at_grapheme(line, col, &ch.to_string());
}

/// Replace characters in a char-wise range.
fn replace_char_range(
    rope: &mut kjxlkj_core_text::Rope,
    start: CursorPosition,
    end: CursorPosition,
    ch: char,
) {
    if start.line == end.line {
        for col in (start.grapheme..=end.grapheme).rev() {
            replace_one(rope, start.line, col, ch);
        }
    } else {
        // Last line first (reverse order to keep positions stable).
        for col in (0..=end.grapheme).rev() {
            replace_one(rope, end.line, col, ch);
        }
        for line in (start.line + 1..end.line).rev() {
            let len = rope.line_grapheme_count(line);
            let end_col = if len > 0 { len - 1 } else { 0 };
            for col in (0..end_col).rev() {
                replace_one(rope, line, col, ch);
            }
        }
        let len0 = rope.line_grapheme_count(start.line);
        let end_col = if len0 > 0 { len0 - 1 } else { 0 };
        for col in (start.grapheme..end_col).rev() {
            replace_one(rope, start.line, col, ch);
        }
    }
}

fn order(a: CursorPosition, b: CursorPosition) -> (CursorPosition, CursorPosition) {
    if (a.line, a.grapheme) <= (b.line, b.grapheme) {
        (a, b)
    } else {
        (b, a)
    }
}

impl EditorState {
    /// * and # in visual mode: search for selected text.
    pub(crate) fn visual_star_search(&mut self, forward: bool, kind: VisualKind) {
        let anchor = match self.visual_anchor.take() {
            Some(a) => a,
            None => {
                self.mode = kjxlkj_core_types::Mode::Normal;
                return;
            }
        };
        let cursor = self.windows.focused().cursor;
        let (start, end) = order(anchor, cursor);
        let buf_id = self.current_buffer_id();
        let text = match kind {
            VisualKind::Line => {
                if let Some(buf) = self.buffers.get(buf_id) {
                    let mut s = String::new();
                    for l in start.line..=end.line {
                        let slice = buf.content.line(l);
                        s.push_str(&slice.to_string());
                    }
                    s.trim_end_matches('\n').to_string()
                } else {
                    String::new()
                }
            }
            _ => self.extract_range(buf_id, start, end, true),
        };
        if !text.is_empty() {
            let escaped = regex::escape(&text);
            self.search.pattern = Some(escaped);
            self.search.forward = forward;
            self.search.active = true;
            if forward {
                self.search_next();
            } else {
                self.search_prev();
            }
        }
        self.mode = kjxlkj_core_types::Mode::Normal;
    }

    /// Restore last visual selection (gv).
    pub(crate) fn restore_last_visual(&mut self) {
        if let Some((anchor, cursor, kind)) = self.last_visual {
            self.visual_anchor = Some(anchor);
            self.windows.focused_mut().cursor = cursor;
            self.mode = kjxlkj_core_types::Mode::Visual(kind);
        }
    }
}
