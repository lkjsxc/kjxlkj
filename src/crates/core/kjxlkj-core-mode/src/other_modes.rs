//! Command mode handler.

use crate::{HandleResult, ModeAction, ModeState};
use kjxlkj_core_types::{Key, KeyEvent, Modifiers, SpecialKey};

/// Dispatch in command mode.
pub fn dispatch_command(state: &mut ModeState, key: &KeyEvent) -> HandleResult {
    match (&key.key, &key.modifiers) {
        (Key::Special(SpecialKey::Escape), _) => {
            state.enter_normal();
            HandleResult::Consumed(vec![ModeAction::ReturnNormal])
        }
        (Key::Special(SpecialKey::Enter), _) => {
            let cmd = state.cmdline.clone();
            state.enter_normal();
            HandleResult::Consumed(vec![ModeAction::ExecuteCommand(cmd)])
        }
        (Key::Char(c), Modifiers { ctrl: false, alt: false, .. }) => {
            state.cmdline.push(*c);
            state.cmdline_cursor += 1;
            HandleResult::Consumed(vec![])
        }
        (Key::Special(SpecialKey::Backspace), _) => {
            if state.cmdline_cursor > 0 {
                state.cmdline_cursor -= 1;
                state.cmdline.remove(state.cmdline_cursor);
            }
            HandleResult::Consumed(vec![])
        }
        _ => HandleResult::Ignored,
    }
}

/// Dispatch in visual mode.
pub fn dispatch_visual(state: &mut ModeState, key: &KeyEvent) -> HandleResult {
    match (&key.key, &key.modifiers) {
        (Key::Special(SpecialKey::Escape), _) => {
            state.enter_normal();
            HandleResult::Consumed(vec![ModeAction::ReturnNormal])
        }
        _ => HandleResult::Ignored,
    }
}

/// Dispatch in replace mode.
pub fn dispatch_replace(state: &mut ModeState, key: &KeyEvent) -> HandleResult {
    match (&key.key, &key.modifiers) {
        (Key::Special(SpecialKey::Escape), _) => {
            state.enter_normal();
            HandleResult::Consumed(vec![ModeAction::ReturnNormal])
        }
        _ => HandleResult::Ignored,
    }
}

/// Dispatch in operator-pending mode.
pub fn dispatch_operator_pending(state: &mut ModeState, key: &KeyEvent) -> HandleResult {
    match (&key.key, &key.modifiers) {
        (Key::Special(SpecialKey::Escape), _) => {
            state.enter_normal();
            HandleResult::Consumed(vec![ModeAction::ReturnNormal])
        }
        _ => HandleResult::Ignored,
    }
}

/// Dispatch in terminal insert mode.
pub fn dispatch_terminal(state: &mut ModeState, key: &KeyEvent) -> HandleResult {
    match (&key.key, &key.modifiers) {
        (Key::Char('\\'), Modifiers { ctrl: true, .. }) => HandleResult::Pending,
        (Key::Char('n'), Modifiers { ctrl: true, .. }) => {
            state.enter_normal();
            HandleResult::Consumed(vec![ModeAction::ReturnNormal])
        }
        _ => HandleResult::Ignored,
    }
}
