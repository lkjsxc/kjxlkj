//! Motion implementations.

use kjxlkj_core_text::TextBuffer;
use kjxlkj_core_types::{CursorPosition, Motion};

/// Execute a motion and return the new cursor position.
pub fn execute_motion(
    buffer: &TextBuffer,
    cursor: CursorPosition,
    motion: &Motion,
    count: usize,
) -> CursorPosition {
    let count = count.max(1);
    let mut pos = cursor;

    for _ in 0..count {
        pos = execute_motion_once(buffer, pos, motion);
    }

    pos
}

fn execute_motion_once(
    buffer: &TextBuffer,
    cursor: CursorPosition,
    motion: &Motion,
) -> CursorPosition {
    match motion {
        Motion::Left => move_left(buffer, cursor),
        Motion::Right => move_right(buffer, cursor),
        Motion::Up => move_up(buffer, cursor),
        Motion::Down => move_down(buffer, cursor),
        Motion::LineStart => line_start(cursor),
        Motion::FirstNonBlank => first_non_blank(buffer, cursor),
        Motion::LineEnd => line_end(buffer, cursor),
        Motion::WordForward => word_forward(buffer, cursor),
        Motion::WordBackward => word_backward(buffer, cursor),
        Motion::WordEnd => word_end(buffer, cursor),
        Motion::BigWordForward => big_word_forward(buffer, cursor),
        Motion::BigWordBackward => big_word_backward(buffer, cursor),
        Motion::BigWordEnd => big_word_end(buffer, cursor),
        Motion::FileStart => CursorPosition::new(0, 0),
        Motion::FileEnd => file_end(buffer),
        Motion::GoToLine(line) => go_to_line(buffer, *line),
        Motion::PageDown => page_down(buffer, cursor, 20),
        Motion::PageUp => page_up(buffer, cursor, 20),
        Motion::HalfPageDown => page_down(buffer, cursor, 10),
        Motion::HalfPageUp => page_up(buffer, cursor, 10),
        Motion::ParagraphForward => paragraph_forward(buffer, cursor),
        Motion::ParagraphBackward => paragraph_backward(buffer, cursor),
        Motion::FindChar(c) => find_char(buffer, cursor, *c),
        Motion::TillChar(c) => till_char(buffer, cursor, *c),
        Motion::FindCharBack(c) => find_char_back(buffer, cursor, *c),
        Motion::TillCharBack(c) => till_char_back(buffer, cursor, *c),
        Motion::MatchingBracket => matching_bracket(buffer, cursor),
        Motion::NextSearchMatch | Motion::PrevSearchMatch => cursor,
    }
}

fn move_left(_buffer: &TextBuffer, cursor: CursorPosition) -> CursorPosition {
    if cursor.column > 0 {
        CursorPosition::new(cursor.line, cursor.column - 1)
    } else {
        cursor
    }
}

fn move_right(buffer: &TextBuffer, cursor: CursorPosition) -> CursorPosition {
    let line_len = buffer.line_len(cursor.line);
    if cursor.column < line_len.saturating_sub(1) {
        CursorPosition::new(cursor.line, cursor.column + 1)
    } else {
        cursor
    }
}

fn move_up(buffer: &TextBuffer, cursor: CursorPosition) -> CursorPosition {
    if cursor.line > 0 {
        let new_line = cursor.line - 1;
        let line_len = buffer.line_len(new_line);
        let new_col = cursor.column.min(line_len.saturating_sub(1).max(0));
        CursorPosition::new(new_line, new_col)
    } else {
        cursor
    }
}

fn move_down(buffer: &TextBuffer, cursor: CursorPosition) -> CursorPosition {
    if cursor.line < buffer.line_count() - 1 {
        let new_line = cursor.line + 1;
        let line_len = buffer.line_len(new_line);
        let new_col = cursor.column.min(line_len.saturating_sub(1).max(0));
        CursorPosition::new(new_line, new_col)
    } else {
        cursor
    }
}

fn line_start(cursor: CursorPosition) -> CursorPosition {
    CursorPosition::new(cursor.line, 0)
}

fn first_non_blank(buffer: &TextBuffer, cursor: CursorPosition) -> CursorPosition {
    if let Some(line) = buffer.line(cursor.line) {
        let line = line.trim_end_matches('\n');
        let col = line.chars().position(|c| !c.is_whitespace()).unwrap_or(0);
        CursorPosition::new(cursor.line, col)
    } else {
        cursor
    }
}

fn line_end(buffer: &TextBuffer, cursor: CursorPosition) -> CursorPosition {
    let line_len = buffer.line_len(cursor.line);
    CursorPosition::new(cursor.line, line_len.saturating_sub(1).max(0))
}

fn word_forward(buffer: &TextBuffer, cursor: CursorPosition) -> CursorPosition {
    if let Some(line) = buffer.line(cursor.line) {
        let line = line.trim_end_matches('\n');
        let chars: Vec<char> = line.chars().collect();
        let mut col = cursor.column;

        // Skip current word
        while col < chars.len() && is_word_char(chars[col]) {
            col += 1;
        }
        // Skip non-word chars
        while col < chars.len() && !is_word_char(chars[col]) {
            col += 1;
        }

        if col < chars.len() {
            CursorPosition::new(cursor.line, col)
        } else if cursor.line < buffer.line_count() - 1 {
            first_non_blank(buffer, CursorPosition::new(cursor.line + 1, 0))
        } else {
            line_end(buffer, cursor)
        }
    } else {
        cursor
    }
}

fn word_backward(buffer: &TextBuffer, cursor: CursorPosition) -> CursorPosition {
    if cursor.column == 0 {
        if cursor.line > 0 {
            return line_end(buffer, CursorPosition::new(cursor.line - 1, 0));
        }
        return cursor;
    }

    if let Some(line) = buffer.line(cursor.line) {
        let line = line.trim_end_matches('\n');
        let chars: Vec<char> = line.chars().collect();
        let mut col = cursor.column.saturating_sub(1);

        while col > 0 && !is_word_char(chars[col]) {
            col -= 1;
        }
        while col > 0 && is_word_char(chars[col - 1]) {
            col -= 1;
        }

        CursorPosition::new(cursor.line, col)
    } else {
        cursor
    }
}

fn word_end(buffer: &TextBuffer, cursor: CursorPosition) -> CursorPosition {
    if let Some(line) = buffer.line(cursor.line) {
        let line = line.trim_end_matches('\n');
        let chars: Vec<char> = line.chars().collect();
        let mut col = cursor.column + 1;

        while col < chars.len() && !is_word_char(chars[col]) {
            col += 1;
        }
        while col < chars.len() - 1 && is_word_char(chars[col + 1]) {
            col += 1;
        }

        if col < chars.len() {
            CursorPosition::new(cursor.line, col)
        } else {
            line_end(buffer, cursor)
        }
    } else {
        cursor
    }
}

fn big_word_forward(buffer: &TextBuffer, cursor: CursorPosition) -> CursorPosition {
    if let Some(line) = buffer.line(cursor.line) {
        let line = line.trim_end_matches('\n');
        let chars: Vec<char> = line.chars().collect();
        let mut col = cursor.column;

        while col < chars.len() && !chars[col].is_whitespace() {
            col += 1;
        }
        while col < chars.len() && chars[col].is_whitespace() {
            col += 1;
        }

        if col < chars.len() {
            CursorPosition::new(cursor.line, col)
        } else if cursor.line < buffer.line_count() - 1 {
            first_non_blank(buffer, CursorPosition::new(cursor.line + 1, 0))
        } else {
            line_end(buffer, cursor)
        }
    } else {
        cursor
    }
}

fn big_word_backward(buffer: &TextBuffer, cursor: CursorPosition) -> CursorPosition {
    if cursor.column == 0 {
        if cursor.line > 0 {
            return line_end(buffer, CursorPosition::new(cursor.line - 1, 0));
        }
        return cursor;
    }

    if let Some(line) = buffer.line(cursor.line) {
        let line = line.trim_end_matches('\n');
        let chars: Vec<char> = line.chars().collect();
        let mut col = cursor.column.saturating_sub(1);

        while col > 0 && chars[col].is_whitespace() {
            col -= 1;
        }
        while col > 0 && !chars[col - 1].is_whitespace() {
            col -= 1;
        }

        CursorPosition::new(cursor.line, col)
    } else {
        cursor
    }
}

fn big_word_end(buffer: &TextBuffer, cursor: CursorPosition) -> CursorPosition {
    if let Some(line) = buffer.line(cursor.line) {
        let line = line.trim_end_matches('\n');
        let chars: Vec<char> = line.chars().collect();
        let mut col = cursor.column + 1;

        while col < chars.len() && chars[col].is_whitespace() {
            col += 1;
        }
        while col < chars.len() - 1 && !chars[col + 1].is_whitespace() {
            col += 1;
        }

        if col < chars.len() {
            CursorPosition::new(cursor.line, col)
        } else {
            line_end(buffer, cursor)
        }
    } else {
        cursor
    }
}

fn file_end(buffer: &TextBuffer) -> CursorPosition {
    let last_line = buffer.line_count().saturating_sub(1);
    CursorPosition::new(last_line, 0)
}

fn go_to_line(buffer: &TextBuffer, line: usize) -> CursorPosition {
    let line = line
        .saturating_sub(1)
        .min(buffer.line_count().saturating_sub(1));
    first_non_blank(buffer, CursorPosition::new(line, 0))
}

fn page_down(buffer: &TextBuffer, cursor: CursorPosition, lines: usize) -> CursorPosition {
    let new_line = (cursor.line + lines).min(buffer.line_count().saturating_sub(1));
    let line_len = buffer.line_len(new_line);
    let new_col = cursor.column.min(line_len.saturating_sub(1).max(0));
    CursorPosition::new(new_line, new_col)
}

fn page_up(buffer: &TextBuffer, cursor: CursorPosition, lines: usize) -> CursorPosition {
    let new_line = cursor.line.saturating_sub(lines);
    let line_len = buffer.line_len(new_line);
    let new_col = cursor.column.min(line_len.saturating_sub(1).max(0));
    CursorPosition::new(new_line, new_col)
}

fn paragraph_forward(buffer: &TextBuffer, cursor: CursorPosition) -> CursorPosition {
    let mut line = cursor.line;
    while line < buffer.line_count() && !is_blank_line(buffer, line) {
        line += 1;
    }
    while line < buffer.line_count() && is_blank_line(buffer, line) {
        line += 1;
    }
    if line >= buffer.line_count() {
        line = buffer.line_count().saturating_sub(1);
    }
    CursorPosition::new(line, 0)
}

fn paragraph_backward(buffer: &TextBuffer, cursor: CursorPosition) -> CursorPosition {
    let mut line = cursor.line.saturating_sub(1);
    while line > 0 && is_blank_line(buffer, line) {
        line -= 1;
    }
    while line > 0 && !is_blank_line(buffer, line - 1) {
        line -= 1;
    }
    CursorPosition::new(line, 0)
}

fn is_blank_line(buffer: &TextBuffer, line: usize) -> bool {
    buffer
        .line(line)
        .map(|s| s.trim().is_empty())
        .unwrap_or(true)
}

fn find_char(buffer: &TextBuffer, cursor: CursorPosition, c: char) -> CursorPosition {
    if let Some(line) = buffer.line(cursor.line) {
        let line = line.trim_end_matches('\n');
        if let Some(pos) = line[cursor.column + 1..].chars().position(|ch| ch == c) {
            return CursorPosition::new(cursor.line, cursor.column + 1 + pos);
        }
    }
    cursor
}

fn till_char(buffer: &TextBuffer, cursor: CursorPosition, c: char) -> CursorPosition {
    let pos = find_char(buffer, cursor, c);
    if pos != cursor && pos.column > 0 {
        CursorPosition::new(pos.line, pos.column - 1)
    } else {
        cursor
    }
}

fn find_char_back(buffer: &TextBuffer, cursor: CursorPosition, c: char) -> CursorPosition {
    if cursor.column == 0 {
        return cursor;
    }
    if let Some(line) = buffer.line(cursor.line) {
        let line = line.trim_end_matches('\n');
        let search_area = &line[..cursor.column];
        if let Some(pos) = search_area.chars().rev().position(|ch| ch == c) {
            return CursorPosition::new(cursor.line, cursor.column - 1 - pos);
        }
    }
    cursor
}

fn till_char_back(buffer: &TextBuffer, cursor: CursorPosition, c: char) -> CursorPosition {
    let pos = find_char_back(buffer, cursor, c);
    if pos != cursor {
        CursorPosition::new(pos.line, pos.column + 1)
    } else {
        cursor
    }
}

fn matching_bracket(buffer: &TextBuffer, cursor: CursorPosition) -> CursorPosition {
    if let Some(line) = buffer.line(cursor.line) {
        let chars: Vec<char> = line.chars().collect();
        if cursor.column < chars.len() {
            let c = chars[cursor.column];
            if let Some((open, close, forward)) = bracket_pair(c) {
                return find_matching(buffer, cursor, open, close, forward);
            }
        }
    }
    cursor
}

fn bracket_pair(c: char) -> Option<(char, char, bool)> {
    match c {
        '(' => Some(('(', ')', true)),
        ')' => Some(('(', ')', false)),
        '[' => Some(('[', ']', true)),
        ']' => Some(('[', ']', false)),
        '{' => Some(('{', '}', true)),
        '}' => Some(('{', '}', false)),
        _ => None,
    }
}

fn find_matching(
    buffer: &TextBuffer,
    cursor: CursorPosition,
    open: char,
    close: char,
    forward: bool,
) -> CursorPosition {
    let text = buffer.text();
    let start_idx = buffer.pos_to_char(cursor);
    let chars: Vec<char> = text.chars().collect();
    let mut depth = 1;

    if forward {
        for (i, &c) in chars.iter().enumerate().skip(start_idx + 1) {
            if c == open {
                depth += 1;
            } else if c == close {
                depth -= 1;
                if depth == 0 {
                    return buffer.char_to_pos(i);
                }
            }
        }
    } else {
        for i in (0..start_idx).rev() {
            if chars[i] == close {
                depth += 1;
            } else if chars[i] == open {
                depth -= 1;
                if depth == 0 {
                    return buffer.char_to_pos(i);
                }
            }
        }
    }

    cursor
}

fn is_word_char(c: char) -> bool {
    c.is_alphanumeric() || c == '_'
}

#[cfg(test)]
mod tests {
    use super::*;
    use kjxlkj_core_types::{BufferId, BufferName};

    fn test_buffer() -> TextBuffer {
        TextBuffer::from_text(
            BufferId::new(1),
            BufferName::new("test"),
            "hello world\nfoo bar\n",
        )
    }

    #[test]
    fn test_move_left() {
        let buf = test_buffer();
        let pos = execute_motion(&buf, CursorPosition::new(0, 5), &Motion::Left, 1);
        assert_eq!(pos, CursorPosition::new(0, 4));
    }

    #[test]
    fn test_move_right() {
        let buf = test_buffer();
        let pos = execute_motion(&buf, CursorPosition::new(0, 0), &Motion::Right, 1);
        assert_eq!(pos, CursorPosition::new(0, 1));
    }

    #[test]
    fn test_move_down() {
        let buf = test_buffer();
        let pos = execute_motion(&buf, CursorPosition::new(0, 0), &Motion::Down, 1);
        assert_eq!(pos, CursorPosition::new(1, 0));
    }

    #[test]
    fn test_file_start_end() {
        let buf = test_buffer();
        let pos = execute_motion(&buf, CursorPosition::new(1, 5), &Motion::FileStart, 1);
        assert_eq!(pos, CursorPosition::new(0, 0));
    }
}
