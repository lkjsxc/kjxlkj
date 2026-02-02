//! Word motion implementations.

use kjxlkj_core_types::motion::MotionResult;
use super::MotionContext;

/// Executes word forward motion.
pub fn word_forward<C: MotionContext>(
    ctx: &C,
    mut line: usize,
    mut col: usize,
    big: bool,
) -> MotionResult {
    let content = ctx.line_content(line);
    let chars: Vec<char> = content.chars().collect();

    // Skip current word
    while col < chars.len() && is_word_char(chars[col], big) {
        col += 1;
    }
    // Skip whitespace/punctuation
    while col < chars.len() && !is_word_char(chars[col], big) {
        col += 1;
    }

    // If at end of line, go to next line
    if col >= chars.len() && line < ctx.line_count() - 1 {
        line += 1;
        let new_content = ctx.line_content(line);
        col = new_content.chars().position(|c| !c.is_whitespace()).unwrap_or(0);
        return MotionResult { line, column: col, wrapped: true, hit_boundary: false };
    }

    MotionResult {
        line,
        column: col.min(chars.len().saturating_sub(1)),
        wrapped: false,
        hit_boundary: col >= chars.len(),
    }
}

/// Executes word backward motion.
pub fn word_backward<C: MotionContext>(
    ctx: &C,
    mut line: usize,
    mut col: usize,
    big: bool,
) -> MotionResult {
    if col == 0 && line == 0 {
        return MotionResult { line: 0, column: 0, wrapped: false, hit_boundary: true };
    }

    // Move back one if not at start
    if col > 0 {
        col -= 1;
    } else if line > 0 {
        line -= 1;
        let new_content = ctx.line_content(line);
        col = new_content.chars().count().saturating_sub(1);
    }

    let chars: Vec<char> = ctx.line_content(line).chars().collect();

    // Skip whitespace/punctuation
    while col > 0 && !is_word_char(chars[col], big) {
        col -= 1;
    }
    // Skip word
    while col > 0 && is_word_char(chars[col - 1], big) {
        col -= 1;
    }

    MotionResult { line, column: col, wrapped: false, hit_boundary: false }
}

/// Executes word end motion.
pub fn word_end<C: MotionContext>(
    ctx: &C,
    line: usize,
    mut col: usize,
    big: bool,
) -> MotionResult {
    let content = ctx.line_content(line);
    let chars: Vec<char> = content.chars().collect();

    // Move forward one
    if col < chars.len().saturating_sub(1) {
        col += 1;
    }

    // Skip whitespace
    while col < chars.len() && !is_word_char(chars[col], big) {
        col += 1;
    }
    // Find end of word
    while col < chars.len().saturating_sub(1) && is_word_char(chars[col + 1], big) {
        col += 1;
    }

    MotionResult {
        line,
        column: col.min(chars.len().saturating_sub(1)),
        wrapped: false,
        hit_boundary: false,
    }
}

/// Checks if a character is a word character.
pub fn is_word_char(c: char, big: bool) -> bool {
    if big { !c.is_whitespace() } else { c.is_alphanumeric() || c == '_' }
}
