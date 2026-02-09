//! Character-level editing: toggle case, replace char,
//! join lines, increment/decrement numbers.

use crate::EditorState;

impl EditorState {
    pub(crate) fn do_join(&mut self) {
        let line = self.cursor_pos().0;
        if let Some(buf) = self.active_buffer_mut() {
            if line + 1 < buf.line_count() {
                let end = buf.content.line_end_offset(line);
                let next_start = end + 1;
                if next_start <= buf.content.len_chars() {
                    buf.content.delete_range(end, next_start);
                    buf.content.insert_char(end, ' ');
                    buf.modified = true;
                }
            }
        }
    }

    pub(crate) fn do_toggle_case(&mut self) {
        let (line, col) = self.cursor_pos();
        if let Some(buf) = self.active_buffer_mut() {
            let off = buf.content.line_grapheme_to_offset(line, col);
            if let Some(ch) = buf.content.char_at(off) {
                let toggled = if ch.is_uppercase() {
                    ch.to_lowercase().next().unwrap_or(ch)
                } else {
                    ch.to_uppercase().next().unwrap_or(ch)
                };
                buf.content.delete_range(off, off + 1);
                buf.content.insert_char(off, toggled);
                buf.modified = true;
            }
        }
        if let Some(w) = self.focused_window_mut() {
            w.cursor.grapheme_offset += 1;
        }
    }

    pub(crate) fn do_replace_char(&mut self, ch: char) {
        let (line, col) = self.cursor_pos();
        if let Some(buf) = self.active_buffer_mut() {
            let off = buf.content.line_grapheme_to_offset(line, col);
            if off < buf.content.len_chars() {
                buf.content.delete_range(off, off + 1);
                buf.content.insert_char(off, ch);
                buf.modified = true;
            }
        }
    }

    pub(crate) fn do_increment(&mut self, n: i64) {
        let (line, col) = self.cursor_pos();
        if let Some(buf) = self.active_buffer_mut() {
            let text = buf.content.line_content(line);
            if let Some((start, end, val)) = find_number_at(&text, col) {
                let new_val = val + n;
                let new_str = new_val.to_string();
                let off = buf.content.line_start_offset(line);
                buf.content.delete_range(off + start, off + end);
                for (i, ch) in new_str.chars().enumerate() {
                    buf.content.insert_char(off + start + i, ch);
                }
                buf.modified = true;
            }
        }
    }
}

/// Find a number at or near the given column.
fn find_number_at(line: &str, col: usize) -> Option<(usize, usize, i64)> {
    let bytes = line.as_bytes();
    let mut start = col;
    while start < bytes.len() && !bytes[start].is_ascii_digit() {
        start += 1;
    }
    if start >= bytes.len() {
        start = col;
        while start > 0 && !bytes[start - 1].is_ascii_digit() {
            start -= 1;
        }
        if start == 0 {
            return None;
        }
        start -= 1;
    }
    let mut end = start + 1;
    while end < bytes.len() && bytes[end].is_ascii_digit() {
        end += 1;
    }
    while start > 0 && (bytes[start - 1].is_ascii_digit() || bytes[start - 1] == b'-') {
        start -= 1;
    }
    line[start..end]
        .parse::<i64>()
        .ok()
        .map(|v| (start, end, v))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_number_simple() {
        let r = find_number_at("abc 42 def", 4);
        assert!(r.is_some());
        let (s, e, v) = r.unwrap();
        assert_eq!(v, 42);
        assert_eq!(&"abc 42 def"[s..e], "42");
    }

    #[test]
    fn find_number_negative() {
        let r = find_number_at("x -5 y", 2);
        assert!(r.is_some());
        assert_eq!(r.unwrap().2, -5);
    }
}
