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
