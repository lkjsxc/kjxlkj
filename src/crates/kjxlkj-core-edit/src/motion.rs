//! Motion implementations.

use kjxlkj_core_text::TextBuffer;
use kjxlkj_core_types::{Cursor, Position};
use unicode_segmentation::UnicodeSegmentation;

/// A motion describes cursor movement.
#[derive(Debug, Clone, PartialEq)]
pub enum Motion {
    Left(usize),
    Right(usize),
    Up(usize),
    Down(usize),
    LineStart,
    FirstNonBlank,
    LineEnd,
    DocumentStart,
    DocumentEnd,
    Line(usize),
    WordForward(usize),
    WordBackward(usize),
    WordEnd(usize),
    BigWordForward(usize),
    BigWordBackward(usize),
    BigWordEnd(usize),
    FindChar { char: char, forward: bool, till: bool },
    MatchBracket,
}

/// Result of applying a motion.
#[derive(Debug, Clone)]
pub struct MotionResult {
    pub cursor: Cursor,
    pub linewise: bool,
}

/// Apply a motion to a cursor position.
pub fn apply_motion(
    motion: &Motion,
    cursor: Cursor,
    buffer: &TextBuffer,
    end_inclusive: bool,
) -> MotionResult {
    let line_count = buffer.line_count();
    if line_count == 0 {
        return MotionResult {
            cursor: Cursor::origin(),
            linewise: false,
        };
    }

    let (new_cursor, linewise) = match motion {
        Motion::Left(n) => {
            let new_col = cursor.column.saturating_sub(*n);
            (Cursor::new(cursor.line, new_col), false)
        }

        Motion::Right(n) => {
            let line_len = buffer.line_len(cursor.line).unwrap_or(0);
            let max_col = if end_inclusive {
                line_len
            } else if line_len > 0 {
                line_len - 1
            } else {
                0
            };
            let new_col = (cursor.column + n).min(max_col);
            (Cursor::new(cursor.line, new_col), false)
        }

        Motion::Up(n) => {
            let new_line = cursor.line.saturating_sub(*n);
            let line_len = buffer.line_len(new_line).unwrap_or(0);
            let target = cursor.effective_target();
            let new_col = clamp_column(target, line_len, end_inclusive);
            (
                Cursor::new(new_line, new_col).with_target_column(target),
                false,
            )
        }

        Motion::Down(n) => {
            let new_line = (cursor.line + n).min(line_count - 1);
            let line_len = buffer.line_len(new_line).unwrap_or(0);
            let target = cursor.effective_target();
            let new_col = clamp_column(target, line_len, end_inclusive);
            (
                Cursor::new(new_line, new_col).with_target_column(target),
                false,
            )
        }

        Motion::LineStart => (Cursor::new(cursor.line, 0), false),

        Motion::FirstNonBlank => {
            let line = buffer.line(cursor.line).unwrap_or_default();
            let col = line
                .graphemes(true)
                .enumerate()
                .find(|(_, g)| !g.chars().all(char::is_whitespace))
                .map(|(i, _)| i)
                .unwrap_or(0);
            (Cursor::new(cursor.line, col), false)
        }

        Motion::LineEnd => {
            let line_len = buffer.line_len(cursor.line).unwrap_or(0);
            let col = if end_inclusive {
                line_len
            } else if line_len > 0 {
                line_len - 1
            } else {
                0
            };
            (Cursor::new(cursor.line, col), false)
        }

        Motion::DocumentStart => (Cursor::origin(), true),

        Motion::DocumentEnd => {
            let last_line = line_count.saturating_sub(1);
            let line_len = buffer.line_len(last_line).unwrap_or(0);
            let col = if line_len > 0 && !end_inclusive {
                line_len - 1
            } else {
                0
            };
            (Cursor::new(last_line, col), true)
        }

        Motion::Line(n) => {
            let target_line = (*n).min(line_count.saturating_sub(1));
            let line_len = buffer.line_len(target_line).unwrap_or(0);
            let col = clamp_column(cursor.column, line_len, end_inclusive);
            (Cursor::new(target_line, col), true)
        }

        Motion::WordForward(n) => {
            let mut pos = cursor;
            for _ in 0..*n {
                pos = move_word_forward(pos, buffer, false, end_inclusive);
            }
            (pos, false)
        }

        Motion::WordBackward(n) => {
            let mut pos = cursor;
            for _ in 0..*n {
                pos = move_word_backward(pos, buffer, false);
            }
            (pos, false)
        }

        Motion::WordEnd(n) => {
            let mut pos = cursor;
            for _ in 0..*n {
                pos = move_word_end(pos, buffer, false, end_inclusive);
            }
            (pos, false)
        }

        Motion::BigWordForward(n) => {
            let mut pos = cursor;
            for _ in 0..*n {
                pos = move_word_forward(pos, buffer, true, end_inclusive);
            }
            (pos, false)
        }

        Motion::BigWordBackward(n) => {
            let mut pos = cursor;
            for _ in 0..*n {
                pos = move_word_backward(pos, buffer, true);
            }
            (pos, false)
        }

        Motion::BigWordEnd(n) => {
            let mut pos = cursor;
            for _ in 0..*n {
                pos = move_word_end(pos, buffer, true, end_inclusive);
            }
            (pos, false)
        }

        Motion::FindChar { char, forward, till } => {
            if let Some(pos) = find_char(cursor, buffer, *char, *forward, *till) {
                (pos, false)
            } else {
                (cursor, false)
            }
        }

        Motion::MatchBracket => {
            if let Some(pos) = find_matching_bracket(cursor, buffer) {
                (pos, false)
            } else {
                (cursor, false)
            }
        }
    };

    MotionResult {
        cursor: new_cursor,
        linewise,
    }
}

fn clamp_column(col: usize, line_len: usize, end_inclusive: bool) -> usize {
    if line_len == 0 {
        0
    } else if end_inclusive {
        col.min(line_len)
    } else {
        col.min(line_len - 1)
    }
}

fn is_word_char(c: char, big_word: bool) -> bool {
    if big_word {
        !c.is_whitespace()
    } else {
        c.is_alphanumeric() || c == '_'
    }
}

fn move_word_forward(cursor: Cursor, buffer: &TextBuffer, big: bool, _end_incl: bool) -> Cursor {
    let line_count = buffer.line_count();
    let mut line = cursor.line;
    let mut col = cursor.column;

    loop {
        let text = buffer.line(line).unwrap_or_default();
        let graphemes: Vec<&str> = text.graphemes(true).collect();

        // Skip current word
        while col < graphemes.len() {
            let c = graphemes[col].chars().next().unwrap_or(' ');
            if is_word_char(c, big) {
                col += 1;
            } else {
                break;
            }
        }

        // Skip non-word chars
        while col < graphemes.len() {
            let c = graphemes[col].chars().next().unwrap_or(' ');
            if !is_word_char(c, big) && !c.is_whitespace() {
                col += 1;
            } else {
                break;
            }
        }

        // Skip whitespace
        while col < graphemes.len() {
            let c = graphemes[col].chars().next().unwrap_or(' ');
            if c.is_whitespace() {
                col += 1;
            } else {
                return Cursor::new(line, col);
            }
        }

        // Move to next line
        if line + 1 < line_count {
            line += 1;
            col = 0;
        } else {
            let len = graphemes.len();
            return Cursor::new(line, if len > 0 { len - 1 } else { 0 });
        }
    }
}

fn move_word_backward(cursor: Cursor, buffer: &TextBuffer, big: bool) -> Cursor {
    let mut line = cursor.line;
    let mut col = cursor.column;

    loop {
        let text = buffer.line(line).unwrap_or_default();
        let graphemes: Vec<&str> = text.graphemes(true).collect();

        if col > 0 {
            col -= 1;
        } else if line > 0 {
            line -= 1;
            let prev_text = buffer.line(line).unwrap_or_default();
            let prev_len = prev_text.graphemes(true).count();
            col = prev_len.saturating_sub(1);
            continue;
        } else {
            return Cursor::new(0, 0);
        }

        // Skip whitespace backward
        while col > 0 {
            let c = graphemes.get(col).and_then(|g| g.chars().next()).unwrap_or(' ');
            if c.is_whitespace() {
                col -= 1;
            } else {
                break;
            }
        }

        // Find start of word
        let c = graphemes.get(col).and_then(|g| g.chars().next()).unwrap_or(' ');
        let in_word = is_word_char(c, big);

        while col > 0 {
            let prev_c = graphemes
                .get(col - 1)
                .and_then(|g| g.chars().next())
                .unwrap_or(' ');
            if in_word && is_word_char(prev_c, big) {
                col -= 1;
            } else if !in_word && !prev_c.is_whitespace() && !is_word_char(prev_c, big) {
                col -= 1;
            } else {
                break;
            }
        }

        return Cursor::new(line, col);
    }
}

fn move_word_end(cursor: Cursor, buffer: &TextBuffer, big: bool, _end_incl: bool) -> Cursor {
    let line_count = buffer.line_count();
    let mut line = cursor.line;
    let mut col = cursor.column + 1;

    loop {
        let text = buffer.line(line).unwrap_or_default();
        let graphemes: Vec<&str> = text.graphemes(true).collect();

        // Skip whitespace
        while col < graphemes.len() {
            let c = graphemes[col].chars().next().unwrap_or(' ');
            if c.is_whitespace() {
                col += 1;
            } else {
                break;
            }
        }

        if col >= graphemes.len() {
            if line + 1 < line_count {
                line += 1;
                col = 0;
                continue;
            } else {
                return Cursor::new(line, graphemes.len().saturating_sub(1));
            }
        }

        // Find end of word
        let c = graphemes[col].chars().next().unwrap_or(' ');
        let in_word = is_word_char(c, big);

        while col + 1 < graphemes.len() {
            let next_c = graphemes[col + 1].chars().next().unwrap_or(' ');
            if in_word && is_word_char(next_c, big) {
                col += 1;
            } else if !in_word && !next_c.is_whitespace() && !is_word_char(next_c, big) {
                col += 1;
            } else {
                break;
            }
        }

        return Cursor::new(line, col);
    }
}

fn find_char(cursor: Cursor, buffer: &TextBuffer, ch: char, forward: bool, till: bool) -> Option<Cursor> {
    let line = buffer.line(cursor.line).ok()?;
    let graphemes: Vec<&str> = line.graphemes(true).collect();

    if forward {
        for i in (cursor.column + 1)..graphemes.len() {
            if graphemes[i].chars().next() == Some(ch) {
                let col = if till { i - 1 } else { i };
                return Some(Cursor::new(cursor.line, col));
            }
        }
    } else {
        for i in (0..cursor.column).rev() {
            if graphemes[i].chars().next() == Some(ch) {
                let col = if till { i + 1 } else { i };
                return Some(Cursor::new(cursor.line, col));
            }
        }
    }

    None
}

fn find_matching_bracket(cursor: Cursor, buffer: &TextBuffer) -> Option<Cursor> {
    let line = buffer.line(cursor.line).ok()?;
    let graphemes: Vec<&str> = line.graphemes(true).collect();
    let ch = graphemes.get(cursor.column)?.chars().next()?;

    let (open, close, forward) = match ch {
        '(' => ('(', ')', true),
        ')' => ('(', ')', false),
        '[' => ('[', ']', true),
        ']' => ('[', ']', false),
        '{' => ('{', '}', true),
        '}' => ('{', '}', false),
        '<' => ('<', '>', true),
        '>' => ('<', '>', false),
        _ => return None,
    };

    let mut depth = 1;
    let mut line_idx = cursor.line;
    let mut col = cursor.column;

    loop {
        let text = buffer.line(line_idx).ok()?;
        let graphemes: Vec<&str> = text.graphemes(true).collect();

        if forward {
            col += 1;
            while col < graphemes.len() {
                let c = graphemes[col].chars().next()?;
                if c == open {
                    depth += 1;
                } else if c == close {
                    depth -= 1;
                    if depth == 0 {
                        return Some(Cursor::new(line_idx, col));
                    }
                }
                col += 1;
            }
            if line_idx + 1 < buffer.line_count() {
                line_idx += 1;
                col = 0;
            } else {
                return None;
            }
        } else {
            if col == 0 {
                if line_idx == 0 {
                    return None;
                }
                line_idx -= 1;
                col = buffer.line_len(line_idx).ok()?.saturating_sub(1);
            } else {
                col -= 1;
            }
            let text = buffer.line(line_idx).ok()?;
            let graphemes: Vec<&str> = text.graphemes(true).collect();
            if col < graphemes.len() {
                let c = graphemes[col].chars().next()?;
                if c == close {
                    depth += 1;
                } else if c == open {
                    depth -= 1;
                    if depth == 0 {
                        return Some(Cursor::new(line_idx, col));
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_motion_left() {
        let buf = TextBuffer::from_str("hello");
        let cursor = Cursor::new(0, 3);
        let result = apply_motion(&Motion::Left(2), cursor, &buf, false);
        assert_eq!(result.cursor.column, 1);
    }

    #[test]
    fn test_motion_right() {
        let buf = TextBuffer::from_str("hello");
        let cursor = Cursor::new(0, 1);
        let result = apply_motion(&Motion::Right(2), cursor, &buf, false);
        assert_eq!(result.cursor.column, 3);
    }

    #[test]
    fn test_motion_right_clamp() {
        let buf = TextBuffer::from_str("hi");
        let cursor = Cursor::new(0, 0);
        let result = apply_motion(&Motion::Right(10), cursor, &buf, false);
        assert_eq!(result.cursor.column, 1);
    }

    #[test]
    fn test_motion_down() {
        let buf = TextBuffer::from_str("hello\nworld");
        let cursor = Cursor::new(0, 2);
        let result = apply_motion(&Motion::Down(1), cursor, &buf, false);
        assert_eq!(result.cursor.line, 1);
        assert_eq!(result.cursor.column, 2);
    }

    #[test]
    fn test_motion_word_forward() {
        let buf = TextBuffer::from_str("hello world");
        let cursor = Cursor::new(0, 0);
        let result = apply_motion(&Motion::WordForward(1), cursor, &buf, false);
        assert_eq!(result.cursor.column, 6);
    }

    #[test]
    fn test_motion_line_end() {
        let buf = TextBuffer::from_str("hello");
        let cursor = Cursor::new(0, 0);
        let result = apply_motion(&Motion::LineEnd, cursor, &buf, false);
        assert_eq!(result.cursor.column, 4);
    }

    #[test]
    fn test_motion_document_start() {
        let buf = TextBuffer::from_str("hello\nworld");
        let cursor = Cursor::new(1, 3);
        let result = apply_motion(&Motion::DocumentStart, cursor, &buf, false);
        assert_eq!(result.cursor.line, 0);
        assert_eq!(result.cursor.column, 0);
        assert!(result.linewise);
    }
}
