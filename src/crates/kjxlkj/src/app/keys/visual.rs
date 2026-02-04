//! Visual mode key processing.

use kjxlkj_core::{EditorState, Mode, Motion, MotionKind};
use kjxlkj_input::{Key, KeyCode};

use crate::app::apply_motion;

/// Process a key in visual mode.
pub fn process_visual_key(state: &mut EditorState, key: Key) {
    match key.code {
        KeyCode::Escape => {
            state.set_mode(Mode::Normal);
        }
        KeyCode::Char('d') | KeyCode::Char('x') => {
            process_visual_delete(state);
            state.set_mode(Mode::Normal);
        }
        KeyCode::Char('y') => {
            process_visual_yank(state);
            state.set_mode(Mode::Normal);
        }
        KeyCode::Char('o') => {
            if let Some(ref mut sel) = state.selection {
                sel.swap();
                state.cursor.position = sel.cursor;
            }
        }
        KeyCode::Char(c) => {
            if let Some(kind) = char_to_motion(c) {
                apply_motion(state, Motion::new(kind));
                if let Some(ref mut sel) = state.selection {
                    sel.cursor = state.cursor.position;
                }
            }
        }
        _ => {}
    }
}

fn process_visual_delete(state: &mut EditorState) {
    if let Some(sel) = state.selection {
        let start = sel.start();
        let mut end = sel.end();
        end.col += 1;
        if let Some(text) = state.buffer.text_range(start, end) {
            state.registers.set(
                kjxlkj_core::RegisterName::Unnamed,
                text,
                matches!(sel.kind, kjxlkj_core::SelectionKind::Line),
            );
        }
        state.buffer.delete_range(start, end);
        state.cursor.position = start;
    }
}

fn process_visual_yank(state: &mut EditorState) {
    if let Some(sel) = state.selection {
        let start = sel.start();
        let mut end = sel.end();
        end.col += 1;
        if let Some(text) = state.buffer.text_range(start, end) {
            state.registers.set(
                kjxlkj_core::RegisterName::Unnamed,
                text,
                matches!(sel.kind, kjxlkj_core::SelectionKind::Line),
            );
        }
    }
}

fn char_to_motion(c: char) -> Option<MotionKind> {
    match c {
        'h' => Some(MotionKind::Left),
        'j' => Some(MotionKind::Down),
        'k' => Some(MotionKind::Up),
        'l' => Some(MotionKind::Right),
        'w' => Some(MotionKind::WordStart),
        'b' => Some(MotionKind::WordStartBackward),
        'e' => Some(MotionKind::WordEnd),
        '0' => Some(MotionKind::LineStart),
        '^' => Some(MotionKind::FirstNonBlank),
        '$' => Some(MotionKind::LineEnd),
        'G' => Some(MotionKind::FileEnd),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn char_h_left() {
        assert!(matches!(char_to_motion('h'), Some(MotionKind::Left)));
    }

    #[test]
    fn char_j_down() {
        assert!(matches!(char_to_motion('j'), Some(MotionKind::Down)));
    }

    #[test]
    fn char_k_up() {
        assert!(matches!(char_to_motion('k'), Some(MotionKind::Up)));
    }

    #[test]
    fn char_l_right() {
        assert!(matches!(char_to_motion('l'), Some(MotionKind::Right)));
    }

    #[test]
    fn char_w_word() {
        assert!(matches!(char_to_motion('w'), Some(MotionKind::WordStart)));
    }

    #[test]
    fn char_unknown_none() {
        assert!(char_to_motion('z').is_none());
    }

    #[test]
    fn char_b_word_backward() {
        assert!(matches!(char_to_motion('b'), Some(MotionKind::WordStartBackward)));
    }

    #[test]
    fn char_e_word_end() {
        assert!(matches!(char_to_motion('e'), Some(MotionKind::WordEnd)));
    }

    #[test]
    fn char_0_line_start() {
        assert!(matches!(char_to_motion('0'), Some(MotionKind::LineStart)));
    }

    #[test]
    fn char_caret_first_non_blank() {
        assert!(matches!(char_to_motion('^'), Some(MotionKind::FirstNonBlank)));
    }

    #[test]
    fn char_dollar_line_end() {
        assert!(matches!(char_to_motion('$'), Some(MotionKind::LineEnd)));
    }

    #[test]
    fn char_G_file_end() {
        assert!(matches!(char_to_motion('G'), Some(MotionKind::FileEnd)));
    }

    #[test]
    fn char_x_none() {
        assert!(char_to_motion('x').is_none());
    }

    #[test]
    fn char_d_none() {
        assert!(char_to_motion('d').is_none());
    }

    #[test]
    fn char_y_none() {
        assert!(char_to_motion('y').is_none());
    }

    #[test]
    fn char_space_none() {
        assert!(char_to_motion(' ').is_none());
    }

    #[test]
    fn char_newline_none() {
        assert!(char_to_motion('\n').is_none());
    }

    #[test]
    fn process_visual_key_import() {
        let _ = std::any::type_name::<EditorState>();
    }

    #[test]
    fn char_tab_none() {
        assert!(char_to_motion('\t').is_none());
    }

    #[test]
    fn char_capital_h_none() {
        assert!(char_to_motion('H').is_none());
    }

    #[test]
    fn char_capital_j_none() {
        assert!(char_to_motion('J').is_none());
    }
}
