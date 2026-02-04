//! Normal mode key processing.

use kjxlkj_core::{EditorState, Intent, Motion, MotionKind};
use kjxlkj_input::{Key, KeyCode};

use crate::app::apply_intent;

/// Process a key in normal mode.
pub fn process_normal_key(state: &mut EditorState, key: Key) {
    match key.code {
        KeyCode::Escape => {
            state.mode_state.normal.reset();
            return;
        }
        KeyCode::Enter => {
            apply_intent(state, Intent::Move(Motion::new(MotionKind::Down)));
            return;
        }
        KeyCode::Backspace => {
            apply_intent(state, Intent::Move(Motion::new(MotionKind::Left)));
            return;
        }
        KeyCode::Left => {
            apply_intent(state, Intent::Move(Motion::new(MotionKind::Left)));
            return;
        }
        KeyCode::Right => {
            apply_intent(state, Intent::Move(Motion::new(MotionKind::Right)));
            return;
        }
        KeyCode::Up => {
            apply_intent(state, Intent::Move(Motion::new(MotionKind::Up)));
            return;
        }
        KeyCode::Down => {
            apply_intent(state, Intent::Move(Motion::new(MotionKind::Down)));
            return;
        }
        _ => {}
    }

    if let KeyCode::Char(c) = key.code {
        let intent = state
            .mode_state
            .normal
            .process_key(c, key.mods.ctrl, key.mods.shift);
        apply_intent(state, intent);
    }
}
