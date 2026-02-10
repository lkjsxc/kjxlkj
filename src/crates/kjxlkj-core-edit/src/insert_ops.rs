//! Insert-mode editing operations.

use kjxlkj_core_text::grapheme::line_graphemes;
use kjxlkj_core_text::TextBuffer;

use crate::cursor::CursorPosition;

/// Insert a character at the cursor position.
pub fn insert_char_at(buffer: &mut TextBuffer, cursor: &mut CursorPosition, ch: char) {
    let line_start = buffer.line_to_char(cursor.line);
    let line_str = buffer.line(cursor.line).unwrap_or_default();
    let graphemes = line_graphemes(&line_str);
    // byte position within line from grapheme offset
    let char_offset: usize = graphemes
        .iter()
        .take(cursor.grapheme_offset)
        .map(|g: &&str| g.chars().count())
        .sum();
    let insert_pos = line_start + char_offset;
    let mut s = String::new();
    s.push(ch);
    buffer.insert_at_char(insert_pos, &s);
    cursor.grapheme_offset += 1;
}

/// Delete character before cursor (backspace).
pub fn delete_char_backward(buffer: &mut TextBuffer, cursor: &mut CursorPosition) {
    if cursor.grapheme_offset == 0 {
        if cursor.line == 0 {
            return;
        }
        // Join with previous line
        let prev_line = cursor.line - 1;
        let prev_str = buffer.line(prev_line).unwrap_or_default();
        let prev_g_count = line_graphemes(&prev_str).len();
        // Remove the newline at end of previous line
        let prev_line_start = buffer.line_to_char(prev_line);
        let prev_line_len: usize = prev_str.chars().count();
        let newline_pos = prev_line_start + prev_line_len - 1;
        buffer.remove_char_range(newline_pos, newline_pos + 1);
        cursor.line = prev_line;
        cursor.grapheme_offset = prev_g_count;
        return;
    }
    let line_start = buffer.line_to_char(cursor.line);
    let line_str = buffer.line(cursor.line).unwrap_or_default();
    let graphemes = line_graphemes(&line_str);
    let before: usize = graphemes
        .iter()
        .take(cursor.grapheme_offset - 1)
        .map(|g: &&str| g.chars().count())
        .sum();
    let at: usize = graphemes
        .get(cursor.grapheme_offset - 1)
        .map(|g: &&str| g.chars().count())
        .unwrap_or(0);
    let del_start = line_start + before;
    let del_end = del_start + at;
    buffer.remove_char_range(del_start, del_end);
    cursor.grapheme_offset -= 1;
}

/// Delete character under cursor (forward delete / `x`).
pub fn delete_char_forward(buffer: &mut TextBuffer, cursor: &mut CursorPosition) {
    let line_str = buffer.line(cursor.line).unwrap_or_default();
    let graphemes = line_graphemes(&line_str);
    if cursor.grapheme_offset >= graphemes.len() {
        return;
    }
    let line_start = buffer.line_to_char(cursor.line);
    let before: usize = graphemes
        .iter()
        .take(cursor.grapheme_offset)
        .map(|g: &&str| g.chars().count())
        .sum();
    let at: usize = graphemes
        .get(cursor.grapheme_offset)
        .map(|g: &&str| g.chars().count())
        .unwrap_or(0);
    let del_start = line_start + before;
    let del_end = del_start + at;
    buffer.remove_char_range(del_start, del_end);
}

/// Insert a new line below cursor and move cursor there.
pub fn insert_newline_below(buffer: &mut TextBuffer, cursor: &mut CursorPosition) {
    let line_str = buffer.line(cursor.line).unwrap_or_default();
    let line_start = buffer.line_to_char(cursor.line);
    let line_len = line_str.chars().count();
    let insert_pos = line_start + line_len;
    buffer.insert_at_char(insert_pos, "\n");
    cursor.line += 1;
    cursor.grapheme_offset = 0;
}

/// Insert a new line above cursor and move cursor there.
pub fn insert_newline_above(buffer: &mut TextBuffer, cursor: &mut CursorPosition) {
    let line_start = buffer.line_to_char(cursor.line);
    buffer.insert_at_char(line_start, "\n");
    cursor.grapheme_offset = 0;
}

/// Join current line with next line (Normal mode `J`).
pub fn join_lines(buffer: &mut TextBuffer, cursor: &mut CursorPosition) {
    if cursor.line >= buffer.line_count() - 1 {
        return;
    }
    let line_str = buffer.line(cursor.line).unwrap_or_default();
    let g_count = line_graphemes(&line_str).len();
    let line_start = buffer.line_to_char(cursor.line);
    let line_char_len = line_str.chars().count();
    // Remove newline at end of current line
    let newline_pos = line_start + line_char_len - 1;
    buffer.remove_char_range(newline_pos, newline_pos + 1);
    // Insert a space at join point
    buffer.insert_at_char(newline_pos, " ");
    cursor.grapheme_offset = g_count;
}

/// Replace (overwrite) the character at cursor and advance.
/// Returns the original character for backspace restore, or None
/// if at end of line (appends like Insert mode).
pub fn replace_char_at(
    buffer: &mut TextBuffer,
    cursor: &mut CursorPosition,
    ch: char,
) -> Option<char> {
    let line_str = buffer.line(cursor.line).unwrap_or_default();
    let graphemes = line_graphemes(&line_str);
    if cursor.grapheme_offset >= graphemes.len() {
        // At or past end of line: append like insert
        insert_char_at(buffer, cursor, ch);
        return None;
    }
    let line_start = buffer.line_to_char(cursor.line);
    let before: usize = graphemes
        .iter()
        .take(cursor.grapheme_offset)
        .map(|g: &&str| g.chars().count())
        .sum();
    let old_g = graphemes[cursor.grapheme_offset];
    let old_char = old_g.chars().next();
    let old_len = old_g.chars().count();
    let del_start = line_start + before;
    let del_end = del_start + old_len;
    buffer.remove_char_range(del_start, del_end);
    let mut s = String::new();
    s.push(ch);
    buffer.insert_at_char(del_start, &s);
    cursor.grapheme_offset += 1;
    old_char
}

#[cfg(test)]
mod tests {
    use super::*;
    use kjxlkj_core_text::TextBuffer;
    use kjxlkj_core_types::BufferId;

    #[test]
    fn test_insert_char() {
        let mut buf = TextBuffer::new_scratch(BufferId(1));
        buf.insert_at_char(0, "hllo\n");
        let mut cur = CursorPosition::new(0, 1);
        insert_char_at(&mut buf, &mut cur, 'e');
        assert_eq!(buf.line(0).unwrap(), "hello\n");
        assert_eq!(cur.grapheme_offset, 2);
    }

    #[test]
    fn test_delete_char_forward() {
        let mut buf = TextBuffer::new_scratch(BufferId(1));
        buf.insert_at_char(0, "hello\n");
        let mut cur = CursorPosition::new(0, 0);
        delete_char_forward(&mut buf, &mut cur);
        assert_eq!(buf.line(0).unwrap(), "ello\n");
    }

    #[test]
    fn test_delete_char_backward() {
        let mut buf = TextBuffer::new_scratch(BufferId(1));
        buf.insert_at_char(0, "hello\n");
        let mut cur = CursorPosition::new(0, 3);
        delete_char_backward(&mut buf, &mut cur);
        assert_eq!(buf.line(0).unwrap(), "helo\n");
        assert_eq!(cur.grapheme_offset, 2);
    }

    #[test]
    fn test_insert_newline_below() {
        let mut buf = TextBuffer::new_scratch(BufferId(1));
        buf.insert_at_char(0, "hello\n");
        let mut cur = CursorPosition::new(0, 2);
        insert_newline_below(&mut buf, &mut cur);
        assert_eq!(cur.line, 1);
        assert_eq!(cur.grapheme_offset, 0);
    }
}
