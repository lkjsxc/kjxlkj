//! Motion execution.

use crate::{Motion, MotionKind};
use kjxlkj_core_types::Position;

/// Result of a motion execution.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MotionResult {
    /// New position.
    pub position: Position,
    /// Whether motion found a valid target.
    pub found: bool,
}

impl MotionResult {
    /// Creates a successful result.
    pub fn ok(position: Position) -> Self {
        Self { position, found: true }
    }

    /// Creates a failed result (position unchanged).
    pub fn fail(position: Position) -> Self {
        Self { position, found: false }
    }
}

/// Executes a motion on text.
pub fn execute_motion(
    motion: &Motion,
    pos: Position,
    lines: &[&str],
) -> MotionResult {
    let mut result_pos = pos;
    
    for _ in 0..motion.count {
        match motion.kind {
            MotionKind::Left => {
                if result_pos.col > 0 {
                    result_pos.col -= 1;
                } else {
                    return MotionResult::fail(result_pos);
                }
            }
            MotionKind::Right => {
                let line_len = lines.get(result_pos.line).map(|l| l.len()).unwrap_or(0);
                if result_pos.col + 1 < line_len {
                    result_pos.col += 1;
                } else {
                    return MotionResult::fail(result_pos);
                }
            }
            MotionKind::Up => {
                if result_pos.line > 0 {
                    result_pos.line -= 1;
                    clamp_column(&mut result_pos, lines);
                } else {
                    return MotionResult::fail(result_pos);
                }
            }
            MotionKind::Down => {
                if result_pos.line + 1 < lines.len() {
                    result_pos.line += 1;
                    clamp_column(&mut result_pos, lines);
                } else {
                    return MotionResult::fail(result_pos);
                }
            }
            MotionKind::LineStart => {
                result_pos.col = 0;
            }
            MotionKind::LineEnd => {
                let line_len = lines.get(result_pos.line).map(|l| l.len()).unwrap_or(0);
                result_pos.col = line_len.saturating_sub(1);
            }
            MotionKind::FirstNonBlank => {
                if let Some(line) = lines.get(result_pos.line) {
                    result_pos.col = line.find(|c: char| !c.is_whitespace())
                        .unwrap_or(0);
                }
            }
            MotionKind::WordStart => {
                result_pos = find_word_start(result_pos, lines);
            }
            MotionKind::WordEnd => {
                result_pos = find_word_end(result_pos, lines);
            }
            MotionKind::WordBack => {
                result_pos = find_word_back(result_pos, lines);
            }
            MotionKind::BufferStart => {
                result_pos = Position::origin();
            }
            MotionKind::BufferEnd => {
                if !lines.is_empty() {
                    result_pos.line = lines.len() - 1;
                    result_pos.col = 0;
                }
            }
            MotionKind::GotoLine => {
                let target = motion.count.saturating_sub(1);
                result_pos.line = target.min(lines.len().saturating_sub(1));
                result_pos.col = 0;
            }
            MotionKind::FindChar => {
                if let Some(c) = motion.char_arg {
                    if let Some(new_pos) = find_char_forward(result_pos, lines, c) {
                        result_pos = new_pos;
                    } else {
                        return MotionResult::fail(result_pos);
                    }
                }
            }
            MotionKind::TillChar => {
                if let Some(c) = motion.char_arg {
                    if let Some(new_pos) = find_char_forward(result_pos, lines, c) {
                        if new_pos.col > 0 {
                            result_pos = Position::new(new_pos.line, new_pos.col - 1);
                        }
                    } else {
                        return MotionResult::fail(result_pos);
                    }
                }
            }
            _ => {}
        }
    }
    
    MotionResult::ok(result_pos)
}

fn clamp_column(pos: &mut Position, lines: &[&str]) {
    if let Some(line) = lines.get(pos.line) {
        let max_col = line.len().saturating_sub(1);
        if pos.col > max_col {
            pos.col = max_col;
        }
    }
}

fn find_word_start(pos: Position, lines: &[&str]) -> Position {
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

fn find_word_end(pos: Position, lines: &[&str]) -> Position {
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

fn find_word_back(pos: Position, lines: &[&str]) -> Position {
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

fn find_char_forward(pos: Position, lines: &[&str], c: char) -> Option<Position> {
    let line = lines.get(pos.line)?;
    let start = pos.col + 1;
    let remaining = &line[start..];
    remaining.find(c).map(|idx| Position::new(pos.line, start + idx))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn lines(s: &str) -> Vec<&str> {
        s.lines().collect()
    }

    #[test]
    fn test_motion_left() {
        let motion = Motion::new(MotionKind::Left);
        let result = execute_motion(&motion, Position::new(0, 5), &lines("hello world"));
        assert!(result.found);
        assert_eq!(result.position.col, 4);
    }

    #[test]
    fn test_motion_left_at_start() {
        let motion = Motion::new(MotionKind::Left);
        let result = execute_motion(&motion, Position::new(0, 0), &lines("hello"));
        assert!(!result.found);
        assert_eq!(result.position.col, 0);
    }

    #[test]
    fn test_motion_right() {
        let motion = Motion::new(MotionKind::Right);
        let result = execute_motion(&motion, Position::new(0, 0), &lines("hello world"));
        assert!(result.found);
        assert_eq!(result.position.col, 1);
    }

    #[test]
    fn test_motion_down() {
        let motion = Motion::new(MotionKind::Down);
        let text = "line one\nline two";
        let result = execute_motion(&motion, Position::new(0, 0), &lines(text));
        assert!(result.found);
        assert_eq!(result.position.line, 1);
    }

    #[test]
    fn test_motion_up() {
        let motion = Motion::new(MotionKind::Up);
        let text = "line one\nline two";
        let result = execute_motion(&motion, Position::new(1, 0), &lines(text));
        assert!(result.found);
        assert_eq!(result.position.line, 0);
    }

    #[test]
    fn test_motion_line_start() {
        let motion = Motion::new(MotionKind::LineStart);
        let result = execute_motion(&motion, Position::new(0, 5), &lines("hello world"));
        assert_eq!(result.position.col, 0);
    }

    #[test]
    fn test_motion_line_end() {
        let motion = Motion::new(MotionKind::LineEnd);
        let result = execute_motion(&motion, Position::new(0, 0), &lines("hello"));
        assert_eq!(result.position.col, 4); // 0-indexed
    }

    #[test]
    fn test_motion_first_nonblank() {
        let motion = Motion::new(MotionKind::FirstNonBlank);
        let result = execute_motion(&motion, Position::new(0, 0), &lines("   hello"));
        assert_eq!(result.position.col, 3);
    }

    #[test]
    fn test_motion_buffer_start() {
        let motion = Motion::new(MotionKind::BufferStart);
        let text = "line one\nline two\nline three";
        let result = execute_motion(&motion, Position::new(2, 5), &lines(text));
        assert_eq!(result.position, Position::origin());
    }

    #[test]
    fn test_motion_buffer_end() {
        let motion = Motion::new(MotionKind::BufferEnd);
        let text = "line one\nline two\nline three";
        let result = execute_motion(&motion, Position::new(0, 0), &lines(text));
        assert_eq!(result.position.line, 2);
    }

    #[test]
    fn test_motion_goto_line() {
        let motion = Motion::new(MotionKind::GotoLine).with_count(2);
        let text = "line one\nline two\nline three";
        let result = execute_motion(&motion, Position::new(0, 5), &lines(text));
        assert_eq!(result.position.line, 1); // 0-indexed
    }

    #[test]
    fn test_motion_find_char() {
        let motion = Motion::new(MotionKind::FindChar).with_char('o');
        let result = execute_motion(&motion, Position::new(0, 0), &lines("hello world"));
        assert!(result.found);
        assert_eq!(result.position.col, 4); // First 'o'
    }

    #[test]
    fn test_motion_with_count() {
        let motion = Motion::new(MotionKind::Right).with_count(3);
        let result = execute_motion(&motion, Position::new(0, 0), &lines("hello world"));
        assert_eq!(result.position.col, 3);
    }
}
