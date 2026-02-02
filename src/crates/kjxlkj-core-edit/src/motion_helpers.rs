//! Motion helper functions.

use kjxlkj_core_types::Position;

/// Clamps column to valid line length.
pub fn clamp_column(pos: &mut Position, lines: &[&str]) {
    if let Some(line) = lines.get(pos.line) {
        let max_col = line.len().saturating_sub(1);
        if pos.col > max_col {
            pos.col = max_col;
        }
    }
}

/// Finds the start of the next word.
pub fn find_word_start(pos: Position, lines: &[&str]) -> Position {
    let line = lines.get(pos.line).unwrap_or(&"");
    let chars: Vec<char> = line.chars().collect();
    let mut col = pos.col;

    // Skip current word
    while col < chars.len() && !chars[col].is_whitespace() {
        col += 1;
    }
    // Skip whitespace
    while col < chars.len() && chars[col].is_whitespace() {
        col += 1;
    }

    if col >= chars.len() && pos.line + 1 < lines.len() {
        // Go to next line
        let next_line = lines[pos.line + 1];
        let start = next_line.find(|c: char| !c.is_whitespace()).unwrap_or(0);
        return Position::new(pos.line + 1, start);
    }

    Position::new(pos.line, col.min(chars.len().saturating_sub(1)))
}

/// Finds the end of the current/next word.
pub fn find_word_end(pos: Position, lines: &[&str]) -> Position {
    let line = lines.get(pos.line).unwrap_or(&"");
    let chars: Vec<char> = line.chars().collect();
    let mut col = pos.col + 1;

    // Skip whitespace
    while col < chars.len() && chars[col].is_whitespace() {
        col += 1;
    }
    // Skip word
    while col < chars.len() && !chars[col].is_whitespace() {
        col += 1;
    }

    Position::new(pos.line, col.saturating_sub(1))
}

/// Finds the start of the previous word.
pub fn find_word_back(pos: Position, lines: &[&str]) -> Position {
    let line = lines.get(pos.line).unwrap_or(&"");
    let chars: Vec<char> = line.chars().collect();
    let mut col = pos.col;

    // Skip whitespace backwards
    while col > 0 && chars[col - 1].is_whitespace() {
        col -= 1;
    }
    // Skip word backwards
    while col > 0 && !chars[col - 1].is_whitespace() {
        col -= 1;
    }

    Position::new(pos.line, col)
}

/// Finds a character forward on the current line.
pub fn find_char_forward(pos: Position, lines: &[&str], c: char) -> Option<Position> {
    let line = lines.get(pos.line)?;
    let start = pos.col + 1;
    let remaining = &line[start..];
    remaining.find(c).map(|idx| Position::new(pos.line, start + idx))
}

/// Finds a character backward on the current line.
#[allow(dead_code)]
pub fn find_char_backward(pos: Position, lines: &[&str], c: char) -> Option<Position> {
    let line = lines.get(pos.line)?;
    if pos.col == 0 {
        return None;
    }
    let prefix = &line[..pos.col];
    prefix.rfind(c).map(|idx| Position::new(pos.line, idx))
}
