//! Cursor motions.

use kjxlkj_core_text::TextBuffer;
use kjxlkj_core_types::Position;

/// Motion types.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Motion {
    /// Move left by count characters.
    Left,
    /// Move right by count characters.
    Right,
    /// Move up by count lines.
    Up,
    /// Move down by count lines.
    Down,
    /// Move to line start (first non-blank).
    LineStart,
    /// Move to line end.
    LineEnd,
    /// Move to first column.
    FirstColumn,
    /// Move to next word start.
    WordForward,
    /// Move to previous word start.
    WordBackward,
    /// Move to end of word.
    WordEnd,
    /// Move to file start.
    FileStart,
    /// Move to file end.
    FileEnd,
    /// Move to specific line.
    GoToLine(usize),
}

/// Apply a motion to a position.
pub fn apply_motion(
    buf: &TextBuffer,
    pos: Position,
    motion: Motion,
    count: usize,
    end_inclusive: bool,
) -> Position {
    let count = count.max(1);
    match motion {
        Motion::Left => {
            let new_col = pos.column.saturating_sub(count);
            Position::new(pos.line, new_col)
        }
        Motion::Right => {
            let line_len = buf.line_len(pos.line);
            let max_col = if end_inclusive {
                line_len.saturating_sub(1)
            } else {
                line_len
            };
            let new_col = (pos.column + count).min(max_col);
            Position::new(pos.line, new_col)
        }
        Motion::Up => {
            let new_line = pos.line.saturating_sub(count);
            let line_len = buf.line_len(new_line);
            let max_col = if end_inclusive {
                line_len.saturating_sub(1).max(0)
            } else {
                line_len
            };
            let new_col = pos.column.min(max_col);
            Position::new(new_line, new_col)
        }
        Motion::Down => {
            let max_line = buf.line_count().saturating_sub(1);
            let new_line = (pos.line + count).min(max_line);
            let line_len = buf.line_len(new_line);
            let max_col = if end_inclusive {
                line_len.saturating_sub(1).max(0)
            } else {
                line_len
            };
            let new_col = pos.column.min(max_col);
            Position::new(new_line, new_col)
        }
        Motion::LineStart => Position::new(pos.line, 0),
        Motion::LineEnd => {
            let line_len = buf.line_len(pos.line);
            let col = if end_inclusive {
                line_len.saturating_sub(1)
            } else {
                line_len
            };
            Position::new(pos.line, col)
        }
        Motion::FirstColumn => Position::new(pos.line, 0),
        Motion::WordForward => {
            // Simplified word forward: move to next non-space after space
            if let Some(line) = buf.line(pos.line) {
                let chars: Vec<char> = line.chars().collect();
                let mut col = pos.column;
                // Skip current word
                while col < chars.len() && !chars[col].is_whitespace() {
                    col += 1;
                }
                // Skip whitespace
                while col < chars.len() && chars[col].is_whitespace() {
                    col += 1;
                }
                if col < chars.len() {
                    return Position::new(pos.line, col);
                }
                // Move to next line
                if pos.line + 1 < buf.line_count() {
                    return Position::new(pos.line + 1, 0);
                }
            }
            pos
        }
        Motion::WordBackward => {
            if let Some(line) = buf.line(pos.line) {
                let chars: Vec<char> = line.chars().collect();
                let mut col = pos.column.saturating_sub(1);
                // Skip whitespace
                while col > 0 && chars.get(col).is_some_and(|c| c.is_whitespace()) {
                    col -= 1;
                }
                // Skip to word start
                while col > 0 && chars.get(col - 1).is_some_and(|c| !c.is_whitespace()) {
                    col -= 1;
                }
                return Position::new(pos.line, col);
            }
            pos
        }
        Motion::WordEnd => {
            if let Some(line) = buf.line(pos.line) {
                let chars: Vec<char> = line.chars().collect();
                let mut col = pos.column + 1;
                // Skip whitespace
                while col < chars.len() && chars[col].is_whitespace() {
                    col += 1;
                }
                // Go to end of word
                while col < chars.len() && !chars[col].is_whitespace() {
                    col += 1;
                }
                let end_col = col.saturating_sub(1);
                return Position::new(pos.line, end_col.max(pos.column));
            }
            pos
        }
        Motion::FileStart => Position::origin(),
        Motion::FileEnd => {
            let last_line = buf.line_count().saturating_sub(1);
            Position::new(last_line, 0)
        }
        Motion::GoToLine(line) => {
            let target = line
                .saturating_sub(1)
                .min(buf.line_count().saturating_sub(1));
            Position::new(target, 0)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_motion_left() {
        let buf: TextBuffer = "hello".parse().unwrap();
        let pos = Position::new(0, 3);
        let new_pos = apply_motion(&buf, pos, Motion::Left, 1, true);
        assert_eq!(new_pos, Position::new(0, 2));
    }

    #[test]
    fn test_motion_right() {
        let buf: TextBuffer = "hello".parse().unwrap();
        let pos = Position::new(0, 0);
        let new_pos = apply_motion(&buf, pos, Motion::Right, 1, true);
        assert_eq!(new_pos, Position::new(0, 1));
    }
}
