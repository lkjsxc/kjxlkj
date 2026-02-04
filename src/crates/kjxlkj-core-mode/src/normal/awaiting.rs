//! Awaiting char handlers.

use crate::intent::ScrollIntent;
use crate::Intent;
use kjxlkj_core_edit::{Motion, MotionKind};
use kjxlkj_core_types::{RegisterName, SelectionKind};

use super::state::{AwaitingChar, NormalModeState};

impl NormalModeState {
    /// Handle a character when awaiting char input.
    pub fn handle_awaiting_char(&mut self, awaiting: AwaitingChar, c: char) -> Intent {
        let count = self.get_count();
        self.reset();
        match awaiting {
            AwaitingChar::Replace => Intent::ReplaceChar(c),
            AwaitingChar::FindForward => {
                self.last_find = Some((MotionKind::FindChar(c), c));
                Intent::Move(Motion::new(MotionKind::FindChar(c)).with_count(count))
            }
            AwaitingChar::FindBackward => {
                self.last_find = Some((MotionKind::FindCharBackward(c), c));
                Intent::Move(Motion::new(MotionKind::FindCharBackward(c)).with_count(count))
            }
            AwaitingChar::TillForward => {
                self.last_find = Some((MotionKind::TillChar(c), c));
                Intent::Move(Motion::new(MotionKind::TillChar(c)).with_count(count))
            }
            AwaitingChar::TillBackward => {
                self.last_find = Some((MotionKind::TillCharBackward(c), c));
                Intent::Move(Motion::new(MotionKind::TillCharBackward(c)).with_count(count))
            }
            AwaitingChar::Mark => Intent::SetMark(c),
            AwaitingChar::JumpMark => Intent::JumpToMark {
                mark: c,
                line_start: false,
            },
            AwaitingChar::JumpMarkLine => Intent::JumpToMark {
                mark: c,
                line_start: true,
            },
            AwaitingChar::Register => {
                if let Some(reg) = RegisterName::from_char(c) {
                    self.pending_register = Some(reg);
                    Intent::SetRegister(reg)
                } else {
                    Intent::None
                }
            }
            AwaitingChar::MacroRecord => Intent::StartMacro(c),
            AwaitingChar::MacroPlay => Intent::PlayMacro(c),
        }
    }

    /// Handle Ctrl+key combinations.
    pub fn handle_ctrl_key(&mut self, key: char) -> Intent {
        let _count = self.get_count();
        self.reset();
        match key {
            'r' => Intent::Redo,
            'o' => Intent::JumpList { forward: false },
            'i' => Intent::JumpList { forward: true },
            'd' => Intent::Scroll(ScrollIntent::HalfPageDown),
            'u' => Intent::Scroll(ScrollIntent::HalfPageUp),
            'f' => Intent::Scroll(ScrollIntent::FullPageDown),
            'b' => Intent::Scroll(ScrollIntent::FullPageUp),
            'e' => Intent::Scroll(ScrollIntent::LineDown),
            'y' => Intent::Scroll(ScrollIntent::LineUp),
            'a' => Intent::IncrementNumber,
            'x' => Intent::DecrementNumber,
            'v' => Intent::StartVisual(SelectionKind::Block),
            _ => Intent::None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn ctrl_r_is_redo() {
        let mut state = NormalModeState::new();
        assert_eq!(state.handle_ctrl_key('r'), Intent::Redo);
    }

    #[test]
    fn ctrl_d_half_page_down() {
        let mut state = NormalModeState::new();
        assert_eq!(state.handle_ctrl_key('d'), Intent::Scroll(ScrollIntent::HalfPageDown));
    }

    #[test]
    fn ctrl_u_half_page_up() {
        let mut state = NormalModeState::new();
        assert_eq!(state.handle_ctrl_key('u'), Intent::Scroll(ScrollIntent::HalfPageUp));
    }

    #[test]
    fn ctrl_unknown_is_none() {
        let mut state = NormalModeState::new();
        assert_eq!(state.handle_ctrl_key('z'), Intent::None);
    }

    #[test]
    fn ctrl_a_increment() {
        let mut state = NormalModeState::new();
        assert_eq!(state.handle_ctrl_key('a'), Intent::IncrementNumber);
    }

    #[test]
    fn ctrl_x_decrement() {
        let mut state = NormalModeState::new();
        assert_eq!(state.handle_ctrl_key('x'), Intent::DecrementNumber);
    }

    #[test]
    fn ctrl_f_page_down() {
        let mut state = NormalModeState::new();
        assert_eq!(state.handle_ctrl_key('f'), Intent::Scroll(ScrollIntent::FullPageDown));
    }

    #[test]
    fn ctrl_b_page_up() {
        let mut state = NormalModeState::new();
        assert_eq!(state.handle_ctrl_key('b'), Intent::Scroll(ScrollIntent::FullPageUp));
    }

    #[test]
    fn ctrl_e_line_down() {
        let mut state = NormalModeState::new();
        assert_eq!(state.handle_ctrl_key('e'), Intent::Scroll(ScrollIntent::LineDown));
    }

    #[test]
    fn ctrl_y_line_up() {
        let mut state = NormalModeState::new();
        assert_eq!(state.handle_ctrl_key('y'), Intent::Scroll(ScrollIntent::LineUp));
    }

    #[test]
    fn ctrl_o_jump_back() {
        let mut state = NormalModeState::new();
        let intent = state.handle_ctrl_key('o');
        assert!(matches!(intent, Intent::JumpList { forward: false }));
    }

    #[test]
    fn ctrl_i_jump_forward() {
        let mut state = NormalModeState::new();
        let intent = state.handle_ctrl_key('i');
        assert!(matches!(intent, Intent::JumpList { forward: true }));
    }

    #[test]
    fn ctrl_v_visual_block() {
        let mut state = NormalModeState::new();
        let intent = state.handle_ctrl_key('v');
        assert!(matches!(intent, Intent::StartVisual(SelectionKind::Block)));
    }

    #[test]
    fn awaiting_replace() {
        let mut state = NormalModeState::new();
        let intent = state.handle_awaiting_char(AwaitingChar::Replace, 'x');
        assert_eq!(intent, Intent::ReplaceChar('x'));
    }

    #[test]
    fn awaiting_mark() {
        let mut state = NormalModeState::new();
        let intent = state.handle_awaiting_char(AwaitingChar::Mark, 'a');
        assert_eq!(intent, Intent::SetMark('a'));
    }

    #[test]
    fn awaiting_jump_mark() {
        let mut state = NormalModeState::new();
        let intent = state.handle_awaiting_char(AwaitingChar::JumpMark, 'a');
        assert!(matches!(intent, Intent::JumpToMark { mark: 'a', line_start: false }));
    }

    #[test]
    fn awaiting_macro_record() {
        let mut state = NormalModeState::new();
        let intent = state.handle_awaiting_char(AwaitingChar::MacroRecord, 'q');
        assert_eq!(intent, Intent::StartMacro('q'));
    }

    #[test]
    fn awaiting_macro_play() {
        let mut state = NormalModeState::new();
        let intent = state.handle_awaiting_char(AwaitingChar::MacroPlay, 'q');
        assert_eq!(intent, Intent::PlayMacro('q'));
    }
}
