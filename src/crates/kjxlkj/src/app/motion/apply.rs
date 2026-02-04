//! Motion application functions.

use kjxlkj_core::{EditorState, Motion, MotionKind, Position};

/// Apply a motion to the editor state.
pub fn apply_motion(state: &mut EditorState, motion: Motion) {
    for _ in 0..motion.count {
        match motion.kind {
            MotionKind::Left => apply_left(state),
            MotionKind::Right => apply_right(state),
            MotionKind::Up => apply_up(state),
            MotionKind::Down => apply_down(state),
            MotionKind::LineStart => state.cursor.position.col = 0,
            MotionKind::LineEnd => apply_line_end(state),
            MotionKind::FirstNonBlank => apply_first_non_blank(state),
            MotionKind::WordStart => apply_word_start(state),
            MotionKind::WordStartBackward => apply_word_start_backward(state),
            MotionKind::WordEnd => apply_word_end(state),
            MotionKind::FileStart => state.cursor.position = Position::new(0, 0),
            MotionKind::FileEnd => apply_file_end(state),
            MotionKind::GoToLine(line) => apply_goto_line(state, line),
            MotionKind::ScreenTop => apply_screen_top(state),
            MotionKind::ScreenMiddle => apply_screen_middle(state),
            MotionKind::ScreenBottom => apply_screen_bottom(state),
            _ => {}
        }
    }
}

fn is_word_char(c: char) -> bool { c.is_alphanumeric() || c == '_' }

fn next_word_boundary(line: &str, col: usize) -> usize {
    let chars: Vec<char> = line.chars().collect();
    if col >= chars.len() { return chars.len(); }
    let mut i = col;
    while i < chars.len() && is_word_char(chars[i]) { i += 1; }
    while i < chars.len() && chars[i].is_whitespace() { i += 1; }
    i
}

fn prev_word_boundary(line: &str, col: usize) -> usize {
    let chars: Vec<char> = line.chars().collect();
    if col == 0 || chars.is_empty() { return 0; }
    let mut i = col.min(chars.len()) - 1;
    while i > 0 && chars[i].is_whitespace() { i -= 1; }
    while i > 0 && is_word_char(chars[i - 1]) { i -= 1; }
    i
}

fn find_word_end(line: &str, col: usize) -> usize {
    let chars: Vec<char> = line.chars().collect();
    if col >= chars.len() { return chars.len().saturating_sub(1); }
    let mut i = col + 1;
    while i < chars.len() && chars[i].is_whitespace() { i += 1; }
    while i < chars.len() && is_word_char(chars[i]) { i += 1; }
    i.saturating_sub(1)
}

fn apply_left(state: &mut EditorState) {
    if state.cursor.col() > 0 { state.cursor.position.col -= 1; }
}

fn apply_right(state: &mut EditorState) {
    let max_col = if state.mode().is_insert() {
        state.buffer.line_len(state.cursor.line())
    } else {
        state.buffer.line_len(state.cursor.line()).saturating_sub(1)
    };
    if state.cursor.col() < max_col { state.cursor.position.col += 1; }
}

fn apply_up(state: &mut EditorState) {
    if state.cursor.line() > 0 { state.cursor.position.line -= 1; }
}

fn apply_down(state: &mut EditorState) {
    if state.cursor.line() + 1 < state.buffer.line_count() {
        state.cursor.position.line += 1;
    }
}

fn apply_line_end(state: &mut EditorState) {
    let len = state.buffer.line_len(state.cursor.line());
    state.cursor.position.col = if state.mode().is_insert() { len } else { len.saturating_sub(1) };
}

fn apply_first_non_blank(state: &mut EditorState) {
    let Some(line) = state.buffer.line(state.cursor.line()) else { return; };
    let fnb = line.char_indices().find(|(_, c)| !c.is_whitespace()).map(|(i, _)| i).unwrap_or(0);
    state.cursor.position.col = fnb;
}

fn apply_word_start(state: &mut EditorState) {
    let line = state.buffer.line(state.cursor.line()).unwrap_or_default();
    let new_col = next_word_boundary(&line, state.cursor.col());
    if new_col <= line.chars().count() {
        state.cursor.position.col = new_col;
    } else if state.cursor.line() + 1 < state.buffer.line_count() {
        state.cursor.position.line += 1;
        state.cursor.position.col = 0;
    }
}

fn apply_word_start_backward(state: &mut EditorState) {
    if state.cursor.col() > 0 {
        let line = state.buffer.line(state.cursor.line()).unwrap_or_default();
        state.cursor.position.col = prev_word_boundary(&line, state.cursor.col());
    } else if state.cursor.line() > 0 {
        state.cursor.position.line -= 1;
        state.cursor.position.col = state.buffer.line_len(state.cursor.line()).saturating_sub(1);
    }
}

fn apply_word_end(state: &mut EditorState) {
    let line = state.buffer.line(state.cursor.line()).unwrap_or_default();
    state.cursor.position.col = find_word_end(&line, state.cursor.col());
}

fn apply_file_end(state: &mut EditorState) {
    let last_line = state.buffer.line_count().saturating_sub(1);
    state.cursor.position = Position::new(last_line, 0);
}

fn apply_goto_line(state: &mut EditorState, line: usize) {
    let target = (line.saturating_sub(1)).min(state.buffer.line_count().saturating_sub(1));
    state.cursor.position.line = target;
    if let Some(content) = state.buffer.line(target) {
        let fnb = content.char_indices().find(|(_, c)| !c.is_whitespace()).map(|(i, _)| i).unwrap_or(0);
        state.cursor.position.col = fnb;
    }
}

fn apply_screen_top(state: &mut EditorState) {
    state.cursor.position.line = state.viewport.first_line;
}

fn apply_screen_middle(state: &mut EditorState) {
    let middle = state.viewport.first_line + state.viewport.visible_lines() / 2;
    state.cursor.position.line = middle.min(state.buffer.line_count().saturating_sub(1));
}

fn apply_screen_bottom(state: &mut EditorState) {
    let bottom = state.viewport.last_line();
    state.cursor.position.line = bottom.min(state.buffer.line_count().saturating_sub(1));
}

#[cfg(test)]
mod tests {
    use super::*;
    use kjxlkj_core::Motion;
    
    #[test]
    fn apply_motion_left() {
        let mut state = EditorState::new();
        state.cursor.position.col = 5;
        apply_motion(&mut state, Motion::new(MotionKind::Left));
        assert_eq!(state.cursor.col(), 4);
    }

    #[test]
    fn apply_motion_down() {
        let mut state = EditorState::new();
        state.buffer.insert(Position::new(0, 0), "\nhello");
        apply_motion(&mut state, Motion::new(MotionKind::Down));
        assert_eq!(state.cursor.line(), 1);
    }

    #[test]
    fn apply_motion_line_start() {
        let mut state = EditorState::new();
        state.cursor.position.col = 5;
        apply_motion(&mut state, Motion::new(MotionKind::LineStart));
        assert_eq!(state.cursor.col(), 0);
    }

    #[test]
    fn apply_motion_with_count() {
        let mut state = EditorState::new();
        state.cursor.position.col = 10;
        apply_motion(&mut state, Motion::new(MotionKind::Left).with_count(3));
        assert!(state.cursor.col() <= 7);
    }

    #[test]
    fn apply_motion_right() {
        let mut state = EditorState::new();
        state.buffer.insert(Position::new(0, 0), "hello");
        apply_motion(&mut state, Motion::new(MotionKind::Right));
        assert_eq!(state.cursor.col(), 1);
    }

    #[test]
    fn apply_motion_file_start() {
        let mut state = EditorState::new();
        state.cursor.position.line = 5;
        apply_motion(&mut state, Motion::new(MotionKind::FileStart));
        assert_eq!(state.cursor.line(), 0);
    }
}
