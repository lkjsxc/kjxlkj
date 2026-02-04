//! Motion types and application.

use kjxlkj_core_text::TextBuffer;
use kjxlkj_core_types::{Cursor, MotionIntent, Position};

/// A motion that moves the cursor.
#[derive(Debug, Clone, PartialEq)]
pub struct Motion {
    /// The motion intent.
    pub intent: MotionIntent,
    /// Count multiplier.
    pub count: usize,
}

impl Motion {
    /// Create a new motion.
    pub fn new(intent: MotionIntent, count: usize) -> Self {
        Self {
            intent,
            count: count.max(1),
        }
    }
}

/// Apply a motion to a cursor, returning the new position.
pub fn apply_motion(
    motion: &Motion,
    cursor: &Cursor,
    buffer: &TextBuffer,
    viewport_height: usize,
) -> Position {
    let count = motion.count;
    let line = cursor.line();
    let col = cursor.col();

    match &motion.intent {
        MotionIntent::Left => {
            let new_col = col.saturating_sub(count);
            Position::new(line, new_col)
        }
        MotionIntent::Right => {
            let line_len = buffer.line_grapheme_len(line);
            let max_col = line_len.saturating_sub(1).max(0);
            let new_col = (col + count).min(max_col);
            Position::new(line, new_col)
        }
        MotionIntent::Up => {
            let new_line = line.saturating_sub(count);
            let line_len = buffer.line_grapheme_len(new_line);
            let target_col = cursor.preferred_col.unwrap_or(col);
            let new_col = target_col.min(line_len.saturating_sub(1).max(0));
            Position::new(new_line, new_col)
        }
        MotionIntent::Down => {
            let max_line = buffer.line_count().saturating_sub(1);
            let new_line = (line + count).min(max_line);
            let line_len = buffer.line_grapheme_len(new_line);
            let target_col = cursor.preferred_col.unwrap_or(col);
            let new_col = target_col.min(line_len.saturating_sub(1).max(0));
            Position::new(new_line, new_col)
        }
        MotionIntent::LineStart => Position::new(line, 0),
        MotionIntent::LineEnd => {
            let line_len = buffer.line_grapheme_len(line);
            Position::new(line, line_len.saturating_sub(1).max(0))
        }
        MotionIntent::FirstNonBlank => {
            let new_col = first_non_blank_col(buffer, line);
            Position::new(line, new_col)
        }
        MotionIntent::LastNonBlank => {
            let new_col = last_non_blank_col(buffer, line);
            Position::new(line, new_col)
        }
        MotionIntent::FileStart => Position::new(0, 0),
        MotionIntent::FileEnd => {
            let last_line = buffer.line_count().saturating_sub(1);
            Position::new(last_line, 0)
        }
        MotionIntent::GotoLine(target_line) => {
            let target = (*target_line).saturating_sub(1); // 1-indexed to 0-indexed
            let max_line = buffer.line_count().saturating_sub(1);
            let new_line = target.min(max_line);
            Position::new(new_line, first_non_blank_col(buffer, new_line))
        }
        MotionIntent::GotoPercent(percent) => {
            let percent = (*percent).min(100) as usize;
            let line_count = buffer.line_count();
            let target_line = (line_count * percent / 100).saturating_sub(1).max(0);
            Position::new(target_line, first_non_blank_col(buffer, target_line))
        }
        MotionIntent::GotoColumn(target_col) => {
            let target = (*target_col).saturating_sub(1); // 1-indexed
            let line_len = buffer.line_grapheme_len(line);
            let new_col = target.min(line_len.saturating_sub(1).max(0));
            Position::new(line, new_col)
        }
        MotionIntent::ScreenTop => {
            // Simplified: just go to top visible line (line 0 for now)
            Position::new(0, first_non_blank_col(buffer, 0))
        }
        MotionIntent::ScreenMiddle => {
            let mid = buffer.line_count() / 2;
            Position::new(mid, first_non_blank_col(buffer, mid))
        }
        MotionIntent::ScreenBottom => {
            let last = buffer.line_count().saturating_sub(1);
            Position::new(last, first_non_blank_col(buffer, last))
        }
        MotionIntent::WordStart => {
            let mut pos = Position::new(line, col);
            for _ in 0..count {
                pos = next_word_start(buffer, pos);
            }
            pos
        }
        MotionIntent::WORDStart => {
            let mut pos = Position::new(line, col);
            for _ in 0..count {
                pos = next_word_start(buffer, pos); // Same as word for now
            }
            pos
        }
        MotionIntent::WordStartBack => {
            let mut pos = Position::new(line, col);
            for _ in 0..count {
                pos = prev_word_start(buffer, pos);
            }
            pos
        }
        MotionIntent::WORDStartBack => {
            let mut pos = Position::new(line, col);
            for _ in 0..count {
                pos = prev_word_start(buffer, pos);
            }
            pos
        }
        MotionIntent::WordEnd => {
            let mut pos = Position::new(line, col);
            for _ in 0..count {
                pos = next_word_end(buffer, pos);
            }
            pos
        }
        MotionIntent::WORDEnd => {
            let mut pos = Position::new(line, col);
            for _ in 0..count {
                pos = next_word_end(buffer, pos);
            }
            pos
        }
        MotionIntent::WordEndBack => {
            let mut pos = Position::new(line, col);
            for _ in 0..count {
                pos = prev_word_end(buffer, pos);
            }
            pos
        }
        MotionIntent::WORDEndBack => {
            let mut pos = Position::new(line, col);
            for _ in 0..count {
                pos = prev_word_end(buffer, pos);
            }
            pos
        }
        MotionIntent::LineMiddle => {
            let line_len = buffer.line_grapheme_len(line);
            let mid = line_len / 2;
            Position::new(line, mid)
        }
        _ => Position::new(line, col), // Default: no movement
    }
}

/// Find the first non-blank column in a line.
fn first_non_blank_col(buffer: &TextBuffer, line: usize) -> usize {
    if let Some(slice) = buffer.line(line) {
        let s = slice.as_str().unwrap_or("");
        for (i, c) in s.chars().enumerate() {
            if !c.is_whitespace() {
                return i;
            }
        }
    }
    0
}

/// Find the last non-blank column in a line.
fn last_non_blank_col(buffer: &TextBuffer, line: usize) -> usize {
    if let Some(slice) = buffer.line(line) {
        let s = slice.as_str().unwrap_or("");
        let s = s.trim_end_matches('\n').trim_end_matches('\r');
        for (i, c) in s.chars().rev().enumerate() {
            if !c.is_whitespace() {
                return s.chars().count().saturating_sub(1).saturating_sub(i);
            }
        }
    }
    0
}

/// Find the next word start position.
fn next_word_start(buffer: &TextBuffer, pos: Position) -> Position {
    let line_count = buffer.line_count();
    let mut line = pos.line;
    let mut col = pos.col;

    while line < line_count {
        if let Some(slice) = buffer.line(line) {
            let s = slice.as_str().unwrap_or("");
            let s = s.trim_end_matches('\n').trim_end_matches('\r');
            let chars: Vec<char> = s.chars().collect();

            // Skip current word/non-whitespace
            while col < chars.len() && !chars[col].is_whitespace() {
                col += 1;
            }
            // Skip whitespace
            while col < chars.len() && chars[col].is_whitespace() {
                col += 1;
            }

            if col < chars.len() {
                return Position::new(line, col);
            }
        }

        // Move to next line
        line += 1;
        col = 0;

        // Find first non-blank on new line
        if line < line_count {
            if let Some(slice) = buffer.line(line) {
                let s = slice.as_str().unwrap_or("");
                for (i, c) in s.chars().enumerate() {
                    if !c.is_whitespace() && c != '\n' && c != '\r' {
                        return Position::new(line, i);
                    }
                }
            }
        }
    }

    Position::new(line_count.saturating_sub(1), 0)
}

/// Find the previous word start position.
fn prev_word_start(buffer: &TextBuffer, pos: Position) -> Position {
    let mut line = pos.line;
    let mut col = pos.col;

    loop {
        if let Some(slice) = buffer.line(line) {
            let s = slice.as_str().unwrap_or("");
            let s = s.trim_end_matches('\n').trim_end_matches('\r');
            let chars: Vec<char> = s.chars().collect();

            if col > 0 {
                col = col.saturating_sub(1);

                // Skip whitespace
                while col > 0 && chars.get(col).map_or(false, |c| c.is_whitespace()) {
                    col -= 1;
                }

                // Skip word
                while col > 0
                    && chars
                        .get(col.saturating_sub(1))
                        .map_or(false, |c| !c.is_whitespace())
                {
                    col -= 1;
                }

                return Position::new(line, col);
            }
        }

        if line == 0 {
            return Position::new(0, 0);
        }

        line -= 1;
        col = buffer.line_grapheme_len(line);
    }
}

/// Find the next word end position.
fn next_word_end(buffer: &TextBuffer, pos: Position) -> Position {
    let line_count = buffer.line_count();
    let mut line = pos.line;
    let mut col = pos.col + 1;

    while line < line_count {
        if let Some(slice) = buffer.line(line) {
            let s = slice.as_str().unwrap_or("");
            let s = s.trim_end_matches('\n').trim_end_matches('\r');
            let chars: Vec<char> = s.chars().collect();

            // Skip whitespace
            while col < chars.len() && chars[col].is_whitespace() {
                col += 1;
            }

            // Move to end of word
            while col < chars.len()
                && !chars[col].is_whitespace()
                && col + 1 < chars.len()
                && !chars[col + 1].is_whitespace()
            {
                col += 1;
            }

            if col < chars.len() && !chars[col].is_whitespace() {
                return Position::new(line, col);
            }
        }

        line += 1;
        col = 0;
    }

    Position::new(line_count.saturating_sub(1), 0)
}

/// Find the previous word end position.
fn prev_word_end(buffer: &TextBuffer, pos: Position) -> Position {
    let mut line = pos.line;
    let mut col = pos.col.saturating_sub(1);

    loop {
        if let Some(slice) = buffer.line(line) {
            let s = slice.as_str().unwrap_or("");
            let s = s.trim_end_matches('\n').trim_end_matches('\r');
            let chars: Vec<char> = s.chars().collect();

            // Skip whitespace
            while col > 0 && chars.get(col).map_or(false, |c| c.is_whitespace()) {
                col -= 1;
            }

            if col > 0 || chars.first().map_or(false, |c| !c.is_whitespace()) {
                return Position::new(line, col);
            }
        }

        if line == 0 {
            return Position::new(0, 0);
        }

        line -= 1;
        col = buffer.line_grapheme_len(line).saturating_sub(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use kjxlkj_core_types::BufferId;

    #[test]
    fn test_motion_left_right() {
        let buffer = TextBuffer::from_text(BufferId::new(1), "hello");
        let cursor = Cursor::new(0, 2);

        let left = apply_motion(
            &Motion::new(MotionIntent::Left, 1),
            &cursor,
            &buffer,
            24,
        );
        assert_eq!(left.col, 1);

        let right = apply_motion(
            &Motion::new(MotionIntent::Right, 1),
            &cursor,
            &buffer,
            24,
        );
        assert_eq!(right.col, 3);
    }

    #[test]
    fn test_motion_up_down() {
        let buffer = TextBuffer::from_text(BufferId::new(1), "line1\nline2\nline3");
        let cursor = Cursor::new(1, 2);

        let up = apply_motion(&Motion::new(MotionIntent::Up, 1), &cursor, &buffer, 24);
        assert_eq!(up.line, 0);

        let down = apply_motion(
            &Motion::new(MotionIntent::Down, 1),
            &cursor,
            &buffer,
            24,
        );
        assert_eq!(down.line, 2);
    }

    #[test]
    fn test_motion_line_start_end() {
        let buffer = TextBuffer::from_text(BufferId::new(1), "  hello");
        let cursor = Cursor::new(0, 3);

        let start = apply_motion(
            &Motion::new(MotionIntent::LineStart, 1),
            &cursor,
            &buffer,
            24,
        );
        assert_eq!(start.col, 0);

        let first_nb = apply_motion(
            &Motion::new(MotionIntent::FirstNonBlank, 1),
            &cursor,
            &buffer,
            24,
        );
        assert_eq!(first_nb.col, 2);
    }
}
