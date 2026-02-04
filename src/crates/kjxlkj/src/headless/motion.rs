//! Motion application for headless mode.

use kjxlkj_core::{EditorState, MotionKind};

/// Apply a motion to the editor state.
pub fn apply(state: &mut EditorState, motion: kjxlkj_core::Motion) {
    for _ in 0..motion.count {
        match motion.kind {
            MotionKind::Left => {
                if state.cursor.col() > 0 {
                    state.cursor.position.col -= 1;
                }
            }
            MotionKind::Right => {
                let max = state.buffer.line_len(state.cursor.line()).saturating_sub(1);
                if state.cursor.col() < max {
                    state.cursor.position.col += 1;
                }
            }
            MotionKind::Up => {
                if state.cursor.line() > 0 {
                    state.cursor.position.line -= 1;
                }
            }
            MotionKind::Down => {
                if state.cursor.line() + 1 < state.buffer.line_count() {
                    state.cursor.position.line += 1;
                }
            }
            _ => {}
        }
    }
}
