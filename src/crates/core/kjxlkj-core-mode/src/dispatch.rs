//! Key dispatch to mode handlers.

use kjxlkj_core_types::{KeyEvent, Mode, Key, SpecialKey, Modifiers};
use crate::{ModeState, HandleResult, ModeAction, InsertPosition};

/// Dispatch a key event based on current mode.
pub fn dispatch_key(state: &mut ModeState, key: &KeyEvent) -> HandleResult {
    match &state.mode {
        Mode::Normal => dispatch_normal(state, key),
        Mode::Insert => dispatch_insert(state, key),
        Mode::Visual(_) => dispatch_visual(state, key),
        Mode::Command(_) => dispatch_command(state, key),
        Mode::Replace => dispatch_replace(state, key),
        Mode::OperatorPending(_) => dispatch_operator_pending(state, key),
        Mode::InsertNormal => dispatch_insert_normal(state, key),
        Mode::TerminalInsert => dispatch_terminal(state, key),
    }
}

/// Dispatch in normal mode.
fn dispatch_normal(state: &mut ModeState, key: &KeyEvent) -> HandleResult {
    // Handle count prefix.
    if let Key::Char(c) = key.key {
        if c.is_ascii_digit() {
            if c != '0' || state.count.is_some() {
                let digit = c.to_digit(10).unwrap() as usize;
                state.count = Some(state.count.unwrap_or(0) * 10 + digit);
                return HandleResult::Consumed(vec![]);
            }
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
            HandleResult::Consumed(vec![ModeAction::EnterCommand(kjxlkj_core_types::CommandKind::Ex)])
        }
        (Key::Char('/'), Modifiers { ctrl: false, .. }) => {
            state.enter_command(kjxlkj_core_types::CommandKind::SearchForward);
            HandleResult::Consumed(vec![ModeAction::EnterCommand(kjxlkj_core_types::CommandKind::SearchForward)])
        }

        // Visual mode entry.
        (Key::Char('v'), Modifiers { ctrl: false, .. }) => {
            state.enter_visual(kjxlkj_core_types::VisualKind::Char);
            HandleResult::Consumed(vec![ModeAction::EnterVisual(kjxlkj_core_types::VisualKind::Char)])
        }
        (Key::Char('V'), Modifiers { ctrl: false, .. }) => {
            state.enter_visual(kjxlkj_core_types::VisualKind::Line);
            HandleResult::Consumed(vec![ModeAction::EnterVisual(kjxlkj_core_types::VisualKind::Line)])
        }

        _ => HandleResult::Ignored,
    }
}

/// Dispatch in insert mode.
fn dispatch_insert(state: &mut ModeState, key: &KeyEvent) -> HandleResult {
    match (&key.key, &key.modifiers) {
        (Key::Special(SpecialKey::Escape), _) => {
            state.enter_normal();
            HandleResult::Consumed(vec![ModeAction::ReturnNormal])
        }
        (Key::Char('o'), Modifiers { ctrl: true, .. }) => {
            state.enter_insert_normal();
            HandleResult::Consumed(vec![])
        }
        (Key::Char(c), Modifiers { ctrl: false, alt: false, .. }) => {
            HandleResult::Consumed(vec![ModeAction::InsertText(c.to_string())])
        }
        (Key::Special(SpecialKey::Enter), _) => {
            HandleResult::Consumed(vec![ModeAction::InsertText("\n".to_string())])
        }
        (Key::Special(SpecialKey::Tab), _) => {
            HandleResult::Consumed(vec![ModeAction::InsertText("\t".to_string())])
        }
        (Key::Special(SpecialKey::Backspace), _) => {
            HandleResult::Consumed(vec![ModeAction::DeleteAtCursor(kjxlkj_core_edit::Direction::Backward)])
        }
        (Key::Special(SpecialKey::Delete), _) => {
            HandleResult::Consumed(vec![ModeAction::DeleteAtCursor(kjxlkj_core_edit::Direction::Forward)])
        }
        _ => HandleResult::Ignored,
    }
}

/// Dispatch in visual mode.
fn dispatch_visual(state: &mut ModeState, key: &KeyEvent) -> HandleResult {
    match (&key.key, &key.modifiers) {
        (Key::Special(SpecialKey::Escape), _) => {
            state.enter_normal();
            HandleResult::Consumed(vec![ModeAction::ReturnNormal])
        }
        _ => HandleResult::Ignored,
    }
}

/// Dispatch in command mode.
fn dispatch_command(state: &mut ModeState, key: &KeyEvent) -> HandleResult {
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

/// Dispatch in replace mode.
fn dispatch_replace(state: &mut ModeState, key: &KeyEvent) -> HandleResult {
    match (&key.key, &key.modifiers) {
        (Key::Special(SpecialKey::Escape), _) => {
            state.enter_normal();
            HandleResult::Consumed(vec![ModeAction::ReturnNormal])
        }
        _ => HandleResult::Ignored,
    }
}

/// Dispatch in operator-pending mode.
fn dispatch_operator_pending(state: &mut ModeState, key: &KeyEvent) -> HandleResult {
    match (&key.key, &key.modifiers) {
        (Key::Special(SpecialKey::Escape), _) => {
            state.enter_normal();
            HandleResult::Consumed(vec![ModeAction::ReturnNormal])
        }
        _ => HandleResult::Ignored,
    }
}

/// Dispatch in insert-normal mode.
fn dispatch_insert_normal(state: &mut ModeState, key: &KeyEvent) -> HandleResult {
    // Execute one normal command then return to insert.
    let result = dispatch_normal(state, key);
    if matches!(result, HandleResult::Consumed(_)) {
        state.mode = kjxlkj_core_types::Mode::Insert;
    }
    result
}

/// Dispatch in terminal insert mode.
fn dispatch_terminal(state: &mut ModeState, key: &KeyEvent) -> HandleResult {
    // Check for escape sequence: Ctrl-\ Ctrl-n.
    match (&key.key, &key.modifiers) {
        (Key::Char('\\'), Modifiers { ctrl: true, .. }) => {
            HandleResult::Pending
        }
        (Key::Char('n'), Modifiers { ctrl: true, .. }) => {
            state.enter_normal();
            HandleResult::Consumed(vec![ModeAction::ReturnNormal])
        }
        _ => HandleResult::Ignored,
    }
}
