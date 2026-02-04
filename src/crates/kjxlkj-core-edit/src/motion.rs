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

    #[test]
    fn test_motion_left_boundary() {
        let buffer = TextBuffer::from_text(BufferId::new(1), "hello");
        let cursor = Cursor::new(0, 0);
        
        // Can't go left from column 0
        let pos = apply_motion(
            &Motion::new(MotionIntent::Left, 1),
            &cursor,
            &buffer,
            24,
        );
        assert_eq!(pos.col, 0);
    }

    #[test]
    fn test_motion_right_boundary() {
        let buffer = TextBuffer::from_text(BufferId::new(1), "hello");
        let cursor = Cursor::new(0, 4); // Last char
        
        // Can't go right past end
        let pos = apply_motion(
            &Motion::new(MotionIntent::Right, 1),
            &cursor,
            &buffer,
            24,
        );
        assert_eq!(pos.col, 4); // Stays at end
    }

    #[test]
    fn test_motion_up_boundary() {
        let buffer = TextBuffer::from_text(BufferId::new(1), "line1\nline2");
        let cursor = Cursor::new(0, 2);
        
        // Can't go up from line 0
        let pos = apply_motion(
            &Motion::new(MotionIntent::Up, 1),
            &cursor,
            &buffer,
            24,
        );
        assert_eq!(pos.line, 0);
    }

    #[test]
    fn test_motion_down_boundary() {
        let buffer = TextBuffer::from_text(BufferId::new(1), "line1\nline2");
        let cursor = Cursor::new(1, 2);
        
        // Can't go down past last line
        let pos = apply_motion(
            &Motion::new(MotionIntent::Down, 1),
            &cursor,
            &buffer,
            24,
        );
        assert_eq!(pos.line, 1);
    }

    #[test]
    fn test_motion_with_count() {
        let buffer = TextBuffer::from_text(BufferId::new(1), "hello world");
        let cursor = Cursor::new(0, 5);
        
        // Move left 3
        let pos = apply_motion(
            &Motion::new(MotionIntent::Left, 3),
            &cursor,
            &buffer,
            24,
        );
        assert_eq!(pos.col, 2);
    }

    #[test]
    fn test_motion_file_start_end() {
        let buffer = TextBuffer::from_text(BufferId::new(1), "line1\nline2\nline3");
        let cursor = Cursor::new(1, 2);
        
        let start = apply_motion(
            &Motion::new(MotionIntent::FileStart, 1),
            &cursor,
            &buffer,
            24,
        );
        assert_eq!(start.line, 0);
        assert_eq!(start.col, 0);
        
        let end = apply_motion(
            &Motion::new(MotionIntent::FileEnd, 1),
            &cursor,
            &buffer,
            24,
        );
        assert_eq!(end.line, 2);
    }

    #[test]
    fn test_motion_word_start() {
        let buffer = TextBuffer::from_text(BufferId::new(1), "hello world test");
        let cursor = Cursor::new(0, 0);
        
        let pos = apply_motion(
            &Motion::new(MotionIntent::WordStart, 1),
            &cursor,
            &buffer,
            24,
        );
        assert_eq!(pos.col, 6); // Start of "world"
    }

    #[test]
    fn test_motion_word_start_back() {
        let buffer = TextBuffer::from_text(BufferId::new(1), "hello world test");
        let cursor = Cursor::new(0, 12);
        
        let pos = apply_motion(
            &Motion::new(MotionIntent::WordStartBack, 1),
            &cursor,
            &buffer,
            24,
        );
        assert_eq!(pos.col, 6); // Start of "world"
    }

    #[test]
    fn test_motion_word_end() {
        let buffer = TextBuffer::from_text(BufferId::new(1), "hello world");
        let cursor = Cursor::new(0, 0);
        
        let pos = apply_motion(
            &Motion::new(MotionIntent::WordEnd, 1),
            &cursor,
            &buffer,
            24,
        );
        assert_eq!(pos.col, 4); // End of "hello"
    }

    #[test]
    fn test_motion_goto_line() {
        let buffer = TextBuffer::from_text(BufferId::new(1), "line1\nline2\nline3");
        let cursor = Cursor::new(0, 0);
        
        let pos = apply_motion(
            &Motion::new(MotionIntent::GotoLine(2), 1),
            &cursor,
            &buffer,
            24,
        );
        assert_eq!(pos.line, 1); // 0-indexed, so line 2 is index 1
    }

    #[test]
    fn test_motion_goto_column() {
        let buffer = TextBuffer::from_text(BufferId::new(1), "hello world");
        let cursor = Cursor::new(0, 0);
        
        let pos = apply_motion(
            &Motion::new(MotionIntent::GotoColumn(6), 1),
            &cursor,
            &buffer,
            24,
        );
        assert_eq!(pos.col, 5); // 0-indexed, so column 6 is index 5
    }

    #[test]
    fn test_motion_line_end() {
        let buffer = TextBuffer::from_text(BufferId::new(1), "hello");
        let cursor = Cursor::new(0, 0);
        
        let pos = apply_motion(
            &Motion::new(MotionIntent::LineEnd, 1),
            &cursor,
            &buffer,
            24,
        );
        assert_eq!(pos.col, 4); // Last char of "hello"
    }

    #[test]
    fn test_motion_screen_positions() {
        let buffer = TextBuffer::from_text(BufferId::new(1), "line1\nline2\nline3\nline4\nline5");
        let cursor = Cursor::new(2, 0);
        
        let top = apply_motion(
            &Motion::new(MotionIntent::ScreenTop, 1),
            &cursor,
            &buffer,
            24,
        );
        assert_eq!(top.line, 0);
        
        let mid = apply_motion(
            &Motion::new(MotionIntent::ScreenMiddle, 1),
            &cursor,
            &buffer,
            24,
        );
        assert_eq!(mid.line, 2); // 5 lines / 2 = 2
        
        let bot = apply_motion(
            &Motion::new(MotionIntent::ScreenBottom, 1),
            &cursor,
            &buffer,
            24,
        );
        assert_eq!(bot.line, 4);
    }

    #[test]
    fn test_motion_line_middle() {
        let buffer = TextBuffer::from_text(BufferId::new(1), "hello world");
        let cursor = Cursor::new(0, 0);
        
        let pos = apply_motion(
            &Motion::new(MotionIntent::LineMiddle, 1),
            &cursor,
            &buffer,
            24,
        );
        assert_eq!(pos.col, 5); // 11 chars / 2 = 5
    }

    #[test]
    fn test_motion_preferred_column() {
        let buffer = TextBuffer::from_text(BufferId::new(1), "long line here\nshort\nlong line here");
        let mut cursor = Cursor::new(0, 10);
        cursor.preferred_col = Some(10);
        
        // Go down to short line
        let pos = apply_motion(
            &Motion::new(MotionIntent::Down, 1),
            &cursor,
            &buffer,
            24,
        );
        assert_eq!(pos.line, 1);
        assert!(pos.col < 10); // Clamped to line length
    }

    #[test]
    fn test_motion_empty_buffer() {
        let buffer = TextBuffer::from_text(BufferId::new(1), "");
        let cursor = Cursor::new(0, 0);
        
        // These should not panic
        let _ = apply_motion(&Motion::new(MotionIntent::Left, 1), &cursor, &buffer, 24);
        let _ = apply_motion(&Motion::new(MotionIntent::Right, 1), &cursor, &buffer, 24);
        let _ = apply_motion(&Motion::new(MotionIntent::Up, 1), &cursor, &buffer, 24);
        let _ = apply_motion(&Motion::new(MotionIntent::Down, 1), &cursor, &buffer, 24);
    }
}
