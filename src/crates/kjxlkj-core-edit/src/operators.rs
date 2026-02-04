//! Operator implementations.

use kjxlkj_core_text::TextBuffer;
use kjxlkj_core_types::{Position, Range};
use kjxlkj_core_undo::{Edit, UndoHistory};

/// Delete text in a range.
pub fn delete_range(
    buf: &mut TextBuffer,
    range: Range,
    history: &mut UndoHistory,
    cursor: Position,
) -> String {
    let range = range.normalized();
    let deleted = buf.delete(range.start, range.end);
    let edit = Edit::delete(range.start, deleted.clone(), cursor);
    history.push(edit);
    deleted
}

/// Yank (copy) text in a range.
pub fn yank_range(buf: &TextBuffer, range: Range) -> String {
    let range = range.normalized();
    buf.slice(range.start, range.end)
}

/// Insert text at a position.
pub fn insert_text(
    buf: &mut TextBuffer,
    pos: Position,
    text: &str,
    history: &mut UndoHistory,
    cursor: Position,
) {
    buf.insert(pos, text);
    let edit = Edit::insert(pos, text.to_string(), cursor);
    history.push(edit);
}

/// Change (delete and prepare for insert) text in a range.
pub fn change_range(
    buf: &mut TextBuffer,
    range: Range,
    history: &mut UndoHistory,
    cursor: Position,
) -> String {
    delete_range(buf, range, history, cursor)
}

/// Indent lines in a range.
pub fn indent_lines(
    buf: &mut TextBuffer,
    start_line: usize,
    end_line: usize,
    history: &mut UndoHistory,
) {
    let indent = "    ";
    for line in start_line..=end_line.min(buf.line_count().saturating_sub(1)) {
        let pos = Position::new(line, 0);
        buf.insert(pos, indent);
        history.push(Edit::insert(pos, indent.to_string(), pos));
    }
}

/// Outdent lines in a range.
pub fn outdent_lines(
    buf: &mut TextBuffer,
    start_line: usize,
    end_line: usize,
    history: &mut UndoHistory,
) {
    for line in start_line..=end_line.min(buf.line_count().saturating_sub(1)) {
        if let Some(content) = buf.line(line) {
            let spaces: usize = content.chars().take(4).take_while(|c| *c == ' ').count();
            if spaces > 0 {
                let start = Position::new(line, 0);
                let end = Position::new(line, spaces);
                let deleted = buf.delete(start, end);
                history.push(Edit::delete(start, deleted, start));
            }
        }
    }
}

/// Join lines.
pub fn join_lines(
    buf: &mut TextBuffer,
    line: usize,
    add_space: bool,
    history: &mut UndoHistory,
) -> bool {
    if line + 1 >= buf.line_count() {
        return false;
    }

    let line_len = buf.line_len(line);
    let pos = Position::new(line, line_len);

    // Delete the newline
    let newline_end = Position::new(line + 1, 0);
    buf.delete(pos, newline_end);
    history.push(Edit::delete(pos, "\n".to_string(), pos));

    if add_space {
        buf.insert(pos, " ");
        history.push(Edit::insert(pos, " ".to_string(), pos));
    }

    true
}

/// Undo the last edit.
pub fn undo(buf: &mut TextBuffer, history: &mut UndoHistory) -> Option<Position> {
    let edit = history.undo()?;

    if !edit.inserted.is_empty() {
        let end_idx = buf.pos_to_char(edit.pos) + edit.inserted.chars().count();
        let end_pos = buf.char_to_pos(end_idx);
        buf.delete(edit.pos, end_pos);
    }
    if !edit.deleted.is_empty() {
        buf.insert(edit.pos, &edit.deleted);
    }

    Some(edit.cursor_before)
}

/// Redo the last undone edit.
pub fn redo(buf: &mut TextBuffer, history: &mut UndoHistory) -> Option<Position> {
    let edit = history.redo()?;

    if !edit.deleted.is_empty() {
        let end_idx = buf.pos_to_char(edit.pos) + edit.deleted.chars().count();
        let end_pos = buf.char_to_pos(end_idx);
        buf.delete(edit.pos, end_pos);
    }
    if !edit.inserted.is_empty() {
        buf.insert(edit.pos, &edit.inserted);
    }

    Some(edit.cursor_after)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_delete_range() {
        let mut buf = TextBuffer::from_text("hello world");
        let mut history = UndoHistory::new();
        let range = Range::new(Position::new(0, 5), Position::new(0, 11));
        let deleted = delete_range(&mut buf, range, &mut history, Position::new(0, 5));
        assert_eq!(deleted, " world");
        assert_eq!(buf.contents(), "hello");
    }

    #[test]
    fn test_undo() {
        let mut buf = TextBuffer::from_text("hello");
        let mut history = UndoHistory::new();
        insert_text(
            &mut buf,
            Position::new(0, 5),
            " world",
            &mut history,
            Position::new(0, 5),
        );
        assert_eq!(buf.contents(), "hello world");
        undo(&mut buf, &mut history);
        assert_eq!(buf.contents(), "hello");
    }
}
