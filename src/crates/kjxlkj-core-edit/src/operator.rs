use kjxlkj_core_types::CursorPosition;

/// Edit operation to be applied to a buffer.
#[derive(Debug, Clone)]
pub enum EditOp {
    /// Insert text at position.
    Insert {
        line: usize,
        grapheme: usize,
        text: String,
    },
    /// Delete range (start inclusive, end exclusive).
    Delete {
        start: CursorPosition,
        end: CursorPosition,
    },
    /// Delete entire lines (start..=end).
    DeleteLines { start_line: usize, end_line: usize },
    /// Replace range with new text.
    Replace {
        start: CursorPosition,
        end: CursorPosition,
        text: String,
    },
    /// Replace a single character under cursor.
    ReplaceChar {
        line: usize,
        grapheme: usize,
        ch: char,
    },
    /// Indent lines.
    IndentLines {
        start_line: usize,
        end_line: usize,
        width: usize,
    },
    /// Dedent lines.
    DedentLines {
        start_line: usize,
        end_line: usize,
        width: usize,
    },
    /// Join line with next.
    JoinLine { line: usize, with_space: bool },
    /// Toggle case of character.
    ToggleCase { line: usize, grapheme: usize },
}
