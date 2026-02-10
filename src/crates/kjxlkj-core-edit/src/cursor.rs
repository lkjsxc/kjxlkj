//! Cursor position and clamping.

use kjxlkj_core_text::grapheme::line_graphemes;
use kjxlkj_core_text::TextBuffer;
use kjxlkj_core_types::Mode;

/// A cursor position as (line, grapheme_offset).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CursorPosition {
    pub line: usize,
    pub grapheme_offset: usize,
}

impl CursorPosition {
    pub fn new(line: usize, grapheme_offset: usize) -> Self {
        Self {
            line,
            grapheme_offset,
        }
    }

    pub fn zero() -> Self {
        Self {
            line: 0,
            grapheme_offset: 0,
        }
    }
}

impl Default for CursorPosition {
    fn default() -> Self {
        Self::zero()
    }
}

/// Clamp cursor to valid range for the given mode.
///
/// In end-exclusive modes (Normal, Visual, Replace):
///   valid range is 0..G-1 where G = grapheme count
/// In end-inclusive modes (Insert):
///   valid range is 0..G
pub fn clamp_cursor(cursor: &mut CursorPosition, buffer: &TextBuffer, mode: &Mode) {
    let line_count = buffer.line_count();
    if line_count == 0 {
        cursor.line = 0;
        cursor.grapheme_offset = 0;
        return;
    }
    // Clamp line
    if cursor.line >= line_count {
        cursor.line = line_count - 1;
    }
    // Get line content
    let line_str = buffer.line(cursor.line).unwrap_or_default();
    let g_count = line_graphemes(&line_str).len();

    let max_offset = match mode {
        Mode::Insert | Mode::InsertNormal => g_count,
        _ => {
            if g_count == 0 {
                0
            } else {
                g_count - 1
            }
        }
    };
    if cursor.grapheme_offset > max_offset {
        cursor.grapheme_offset = max_offset;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use kjxlkj_core_text::TextBuffer;
    use kjxlkj_core_types::BufferId;

    #[test]
    fn test_clamp_normal_mode() {
        let mut buf = TextBuffer::new_scratch(BufferId(1));
        buf.insert_at_char(0, "hello\n");
        let mut pos = CursorPosition::new(0, 10);
        clamp_cursor(&mut pos, &buf, &Mode::Normal);
        // "hello" has 5 graphemes, max offset = 4
        assert_eq!(pos.grapheme_offset, 4);
    }

    #[test]
    fn test_clamp_insert_mode() {
        let mut buf = TextBuffer::new_scratch(BufferId(1));
        buf.insert_at_char(0, "hello\n");
        let mut pos = CursorPosition::new(0, 10);
        clamp_cursor(&mut pos, &buf, &Mode::Insert);
        // Insert mode: max offset = 5 (after last char)
        assert_eq!(pos.grapheme_offset, 5);
    }

    #[test]
    fn test_clamp_empty_line() {
        let buf = TextBuffer::new_scratch(BufferId(1));
        let mut pos = CursorPosition::new(0, 5);
        clamp_cursor(&mut pos, &buf, &Mode::Normal);
        assert_eq!(pos.grapheme_offset, 0);
    }
}
