//! Normal mode handler.

use crate::{HandleResult, InsertPosition, ModeAction, ModeState};
use kjxlkj_core_types::{Key, KeyEvent, Modifiers, SpecialKey};

/// Dispatch in normal mode.
pub fn dispatch_normal(state: &mut ModeState, key: &KeyEvent) -> HandleResult {
    // Handle count prefix.
    if let Key::Char(c) = key.key {
        if c.is_ascii_digit()
            && (c != '0' || state.count.is_some()) {
                let digit = c.to_digit(10).unwrap() as usize;
                state.count = Some(state.count.unwrap_or(0) * 10 + digit);
                return HandleResult::Consumed(vec![]);
            }
    }

    // Handle register prefix.
    if let Key::Char('"') = key.key {
        return HandleResult::Pending;
    }

    let count = state.effective_count();

    match (&key.key, &key.modifiers) {
        // Mode entry.
        (Key::Char('i'), Modifiers { ctrl: false, .. }) => {
            state.enter_insert();
            HandleResult::Consumed(vec![ModeAction::EnterInsert(InsertPosition::Before)])
        }
        (Key::Char('a'), Modifiers { ctrl: false, .. }) => {
            state.enter_insert();
            HandleResult::Consumed(vec![ModeAction::EnterInsert(InsertPosition::After)])
        }
        (Key::Char('A'), Modifiers { ctrl: false, .. }) => {
            state.enter_insert();
            HandleResult::Consumed(vec![ModeAction::EnterInsert(InsertPosition::EndOfLine)])
        }
        (Key::Char('I'), Modifiers { ctrl: false, .. }) => {
            state.enter_insert();
            HandleResult::Consumed(vec![ModeAction::EnterInsert(InsertPosition::FirstNonBlank)])
        }
        (Key::Char('o'), Modifiers { ctrl: false, .. }) => {
            state.enter_insert();
            HandleResult::Consumed(vec![ModeAction::EnterInsert(InsertPosition::NewLineBelow)])
        }
        (Key::Char('O'), Modifiers { ctrl: false, .. }) => {
            state.enter_insert();
            HandleResult::Consumed(vec![ModeAction::EnterInsert(InsertPosition::NewLineAbove)])
        }

        // Motion keys.
        (Key::Char('h'), m) | (Key::Special(SpecialKey::Left), m) if !m.ctrl => {
            let motion = kjxlkj_core_edit::Motion::Left;
            state.reset_prefix();
            HandleResult::Consumed(vec![ModeAction::MoveCursor(motion, count)])
        }
        (Key::Char('l'), m) | (Key::Special(SpecialKey::Right), m) if !m.ctrl => {
            let motion = kjxlkj_core_edit::Motion::Right;
            state.reset_prefix();
            HandleResult::Consumed(vec![ModeAction::MoveCursor(motion, count)])
        }
        (Key::Char('j'), m) | (Key::Special(SpecialKey::Down), m) if !m.ctrl => {
            let motion = kjxlkj_core_edit::Motion::Down;
            state.reset_prefix();
            HandleResult::Consumed(vec![ModeAction::MoveCursor(motion, count)])
        }
        (Key::Char('k'), m) | (Key::Special(SpecialKey::Up), m) if !m.ctrl => {
            let motion = kjxlkj_core_edit::Motion::Up;
            state.reset_prefix();
            HandleResult::Consumed(vec![ModeAction::MoveCursor(motion, count)])
        }
        (Key::Char('0'), Modifiers { ctrl: false, .. }) if state.count.is_none() => {
            let motion = kjxlkj_core_edit::Motion::LineStart;
            HandleResult::Consumed(vec![ModeAction::MoveCursor(motion, 1)])
        }
        (Key::Char('$'), Modifiers { ctrl: false, .. }) => {
            let motion = kjxlkj_core_edit::Motion::LineEnd;
            state.reset_prefix();
            HandleResult::Consumed(vec![ModeAction::MoveCursor(motion, count)])
        }
        (Key::Char('w'), Modifiers { ctrl: false, .. }) => {
            let motion = kjxlkj_core_edit::Motion::WordStart(kjxlkj_core_edit::Direction::Forward);
            state.reset_prefix();
            HandleResult::Consumed(vec![ModeAction::MoveCursor(motion, count)])
        }
        (Key::Char('b'), Modifiers { ctrl: false, .. }) => {
            let motion = kjxlkj_core_edit::Motion::WordStart(kjxlkj_core_edit::Direction::Backward);
            state.reset_prefix();
            HandleResult::Consumed(vec![ModeAction::MoveCursor(motion, count)])
        }
        (Key::Char('e'), Modifiers { ctrl: false, .. }) => {
            let motion = kjxlkj_core_edit::Motion::WordEnd(kjxlkj_core_edit::Direction::Forward);
            state.reset_prefix();
            HandleResult::Consumed(vec![ModeAction::MoveCursor(motion, count)])
        }

        // Undo/redo.
        (Key::Char('u'), Modifiers { ctrl: false, .. }) => {
            state.reset_prefix();
            HandleResult::Consumed(vec![ModeAction::Undo])
        }
        (Key::Char('r'), Modifiers { ctrl: true, .. }) => {
            state.reset_prefix();
            HandleResult::Consumed(vec![ModeAction::Redo])
        }

        // Command mode entry.
        (Key::Char(':'), Modifiers { ctrl: false, .. }) => {
            state.enter_command(kjxlkj_core_types::CommandKind::Ex);
            HandleResult::Consumed(vec![ModeAction::EnterCommand(
                kjxlkj_core_types::CommandKind::Ex,
            )])
        }
        (Key::Char('/'), Modifiers { ctrl: false, .. }) => {
            state.enter_command(kjxlkj_core_types::CommandKind::SearchForward);
            HandleResult::Consumed(vec![ModeAction::EnterCommand(
                kjxlkj_core_types::CommandKind::SearchForward,
            )])
        }

        // Visual mode entry.
        (Key::Char('v'), Modifiers { ctrl: false, .. }) => {
            state.enter_visual(kjxlkj_core_types::VisualKind::Char);
            HandleResult::Consumed(vec![ModeAction::EnterVisual(
                kjxlkj_core_types::VisualKind::Char,
            )])
        }
        (Key::Char('V'), Modifiers { ctrl: false, .. }) => {
            state.enter_visual(kjxlkj_core_types::VisualKind::Line);
            HandleResult::Consumed(vec![ModeAction::EnterVisual(
                kjxlkj_core_types::VisualKind::Line,
            )])
        }

        _ => HandleResult::Ignored,
    }
}
