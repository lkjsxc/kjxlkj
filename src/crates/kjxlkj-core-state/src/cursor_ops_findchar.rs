//! f/t/F/T motions, toggle-case (~), case/rot13 helpers.

use kjxlkj_core_edit::{resolve_motion, Motion};
use kjxlkj_core_types::CursorPosition;

use crate::editor::EditorState;

impl EditorState {
    #[rustfmt::skip]
    pub(crate) fn find_char_forward(&mut self, ch: char) { self.last_ft = Some(('f', ch)); self.apply_findchar_motion(Motion::FindCharForward(ch)); }
    #[rustfmt::skip]
    pub(crate) fn find_char_backward(&mut self, ch: char) { self.last_ft = Some(('F', ch)); self.apply_findchar_motion(Motion::FindCharBackward(ch)); }
    #[rustfmt::skip]
    pub(crate) fn till_char_forward(&mut self, ch: char) { self.last_ft = Some(('t', ch)); self.apply_findchar_motion(Motion::TillCharForward(ch)); }
    #[rustfmt::skip]
    pub(crate) fn till_char_backward(&mut self, ch: char) { self.last_ft = Some(('T', ch)); self.apply_findchar_motion(Motion::TillCharBackward(ch)); }

    /// ; repeat last f/t/F/T.
    #[rustfmt::skip]
    pub(crate) fn repeat_find_char(&mut self) {
        if let Some((kind, ch)) = self.last_ft {
            let m = match kind { 'f'=>Motion::FindCharForward(ch), 'F'=>Motion::FindCharBackward(ch), 't'=>Motion::TillCharForward(ch), 'T'=>Motion::TillCharBackward(ch), _=>return };
            self.apply_findchar_motion(m);
        }
    }

    /// , repeat last f/t/F/T in reverse direction.
    #[rustfmt::skip]
    pub(crate) fn repeat_find_char_reverse(&mut self) {
        if let Some((kind, ch)) = self.last_ft {
            let m = match kind { 'f'=>Motion::FindCharBackward(ch), 'F'=>Motion::FindCharForward(ch), 't'=>Motion::TillCharBackward(ch), 'T'=>Motion::TillCharForward(ch), _=>return };
            self.apply_findchar_motion(m);
        }
    }

    fn apply_findchar_motion(&mut self, motion: Motion) {
        let buf_id = self.current_buffer_id();
        if let Some(buf) = self.buffers.get(buf_id) {
            let pos = self.windows.focused().cursor;
            let (new_pos, _) = resolve_motion(&motion, pos, &buf.content, self.viewport_height());
            self.windows.focused_mut().cursor = new_pos;
        }
    }

    /// ~ toggle case of char under cursor and advance.
    #[rustfmt::skip]
    pub(crate) fn toggle_case(&mut self) {
        let buf_id = self.current_buffer_id();
        let cursor = self.windows.focused().cursor;
        if let Some(buf) = self.buffers.get_mut(buf_id) {
            let line = cursor.line;
            if line >= buf.content.len_lines() { return; }
            let chars: Vec<char> = buf.content.line(line).chars().collect();
            let g = cursor.grapheme;
            if g >= chars.len() || chars[g] == '\n' { return; }
            buf.save_undo_checkpoint(cursor);
            let toggled: String = if chars[g].is_uppercase() { chars[g].to_lowercase().collect() } else { chars[g].to_uppercase().collect() };
            let byte_start = buf.content.line_to_byte(line) + chars[..g].iter().map(|c| c.len_utf8()).sum::<usize>();
            let (cs, ce) = (buf.content.byte_to_char(byte_start), buf.content.byte_to_char(byte_start + chars[g].len_utf8()));
            buf.content.remove(cs..ce); buf.content.insert(cs, &toggled); buf.increment_version();
        }
        self.move_cursor_right(1);
    }

    /// Lowercase lines range (for gu operator linewise).
    pub(crate) fn lowercase_lines(&mut self, start: usize, end: usize) {
        self.transform_lines(start, end, |s| s.to_lowercase());
    }

    /// Uppercase lines range (for gU operator linewise).
    pub(crate) fn uppercase_lines(&mut self, start: usize, end: usize) {
        self.transform_lines(start, end, |s| s.to_uppercase());
    }

    /// case_range for charwise gu/gU operators.
    pub(crate) fn case_range(
        &mut self,
        buf_id: kjxlkj_core_types::BufferId,
        start: CursorPosition,
        end: CursorPosition,
        inclusive: bool,
        upper: bool,
    ) {
        let text = self.extract_range(buf_id, start, end, inclusive);
        let new_text = if upper { text.to_uppercase() } else { text.to_lowercase() };
        self.replace_range_text(buf_id, start, end, inclusive, &text, &new_text);
    }

    /// Toggle case for charwise range.
    pub(crate) fn toggle_case_range(
        &mut self, buf_id: kjxlkj_core_types::BufferId,
        start: CursorPosition, end: CursorPosition, inclusive: bool,
    ) {
        let text = self.extract_range(buf_id, start, end, inclusive);
        let new_text: String = text.chars().map(|c| {
            if c.is_uppercase() { c.to_lowercase().next().unwrap_or(c) }
            else { c.to_uppercase().next().unwrap_or(c) }
        }).collect();
        self.replace_range_text(buf_id, start, end, inclusive, &text, &new_text);
    }

    /// ROT13 for charwise range.
    pub(crate) fn rot13_range(
        &mut self, buf_id: kjxlkj_core_types::BufferId,
        start: CursorPosition, end: CursorPosition, inclusive: bool,
    ) {
        let text = self.extract_range(buf_id, start, end, inclusive);
        let new_text: String = text.chars().map(rot13_char).collect();
        self.replace_range_text(buf_id, start, end, inclusive, &text, &new_text);
    }

    #[rustfmt::skip]
    fn replace_range_text(
        &mut self, buf_id: kjxlkj_core_types::BufferId,
        start: CursorPosition, end: CursorPosition, inclusive: bool,
        old: &str, new: &str,
    ) {
        if old == new { return; }
        let cursor = self.windows.focused().cursor;
        let end_g = if inclusive { end.grapheme + 1 } else { end.grapheme };
        if let Some(buf) = self.buffers.get_mut(buf_id) {
            buf.save_undo_checkpoint(cursor);
            use kjxlkj_core_text::RopeExt;
            let sb = buf.content.grapheme_pos_to_byte(start.line, start.grapheme);
            let eb = buf.content.grapheme_pos_to_byte(end.line, end_g);
            let (sc, ec) = (buf.content.byte_to_char(sb), buf.content.byte_to_char(eb));
            buf.content.remove(sc..ec);
            buf.content.insert(sc, new);
            buf.increment_version();
        }
        self.windows.focused_mut().cursor = start;
    }

    /// Toggle case for linewise range.
    pub(crate) fn toggle_case_lines(&mut self, start: usize, end: usize) {
        self.transform_lines(start, end, |s| {
            s.chars().map(|c| {
                if c.is_uppercase() { c.to_lowercase().next().unwrap_or(c) }
                else { c.to_uppercase().next().unwrap_or(c) }
            }).collect()
        });
    }

    /// ROT13 for linewise range.
    pub(crate) fn rot13_lines(&mut self, start: usize, end: usize) {
        self.transform_lines(start, end, |s| s.chars().map(rot13_char).collect());
    }

    fn transform_lines(&mut self, start: usize, end: usize, f: impl Fn(&str) -> String) {
        let buf_id = self.current_buffer_id();
        let cursor = self.windows.focused().cursor;
        if let Some(buf) = self.buffers.get_mut(buf_id) {
            buf.save_undo_checkpoint(cursor);
            for li in start..=end.min(buf.content.len_lines().saturating_sub(1)) {
                let s: String = buf.content.line(li).chars().collect();
                let new_s = f(&s);
                if new_s != s {
                    let sb = buf.content.line_to_byte(li);
                    let eb = if li + 1 < buf.content.len_lines() { buf.content.line_to_byte(li + 1) } else { buf.content.len_bytes() };
                    let (sc, ec) = (buf.content.byte_to_char(sb), buf.content.byte_to_char(eb));
                    buf.content.remove(sc..ec);
                    buf.content.insert(sc, &new_s);
                }
            }
            buf.increment_version();
        }
    }
}

fn rot13_char(c: char) -> char {
    match c {
        'a'..='m' | 'A'..='M' => (c as u8 + 13) as char,
        'n'..='z' | 'N'..='Z' => (c as u8 - 13) as char,
        _ => c,
    }
}
