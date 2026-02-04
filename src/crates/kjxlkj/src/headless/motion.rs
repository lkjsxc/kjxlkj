//! Motion application for headless mode.

use kjxlkj_core::EditorState;

/// Apply a motion to the editor state.
pub fn apply(state: &mut EditorState, motion: kjxlkj_core::Motion) {
    crate::app::apply_motion(state, motion);
}
