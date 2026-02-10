//! Normal mode handler.

use crate::{FocusDirection, HandleResult, InsertPosition, ModeAction, ModeState, PendingPrefix, ScrollAction, WindowTarget};
use kjxlkj_core_types::{Key, KeyEvent, PendingOperator, SpecialKey, SplitDirection};

/// Dispatch in normal mode.
pub fn dispatch_normal(state: &mut ModeState, key: &KeyEvent) -> HandleResult {
    if state.pending_prefix != PendingPrefix::None {
        return dispatch_with_prefix(state, key);
    }

    if let Key::Char(c) = key.key {
        if c.is_ascii_digit() && (c != '0' || state.count.is_some()) {
            let digit = c.to_digit(10).unwrap() as usize;
            state.count = Some(state.count.unwrap_or(0) * 10 + digit);
            return HandleResult::Consumed(vec![]);
        }
    }

    let count = state.effective_count();

    match &key.key {
        Key::Char('i') if !key.modifiers.ctrl => insert_mode(state, InsertPosition::Before),
        Key::Char('a') if !key.modifiers.ctrl => insert_mode(state, InsertPosition::After),
        Key::Char('A') if !key.modifiers.ctrl => insert_mode(state, InsertPosition::EndOfLine),
        Key::Char('I') if !key.modifiers.ctrl => insert_mode(state, InsertPosition::FirstNonBlank),
        Key::Char('o') if !key.modifiers.ctrl => insert_mode(state, InsertPosition::NewLineBelow),
        Key::Char('O') if !key.modifiers.ctrl => insert_mode(state, InsertPosition::NewLineAbove),
        Key::Char('R') if !key.modifiers.ctrl => {
            state.enter_replace();
            HandleResult::Consumed(vec![ModeAction::EnterReplace])
        }

        Key::Char('h') | Key::Special(SpecialKey::Left) if !key.modifiers.ctrl => motion(state, kjxlkj_core_edit::Motion::Left, count),
        Key::Char('l') | Key::Special(SpecialKey::Right) if !key.modifiers.ctrl => motion(state, kjxlkj_core_edit::Motion::Right, count),
        Key::Char('j') | Key::Special(SpecialKey::Down) if !key.modifiers.ctrl => motion(state, kjxlkj_core_edit::Motion::Down, count),
        Key::Char('k') | Key::Special(SpecialKey::Up) if !key.modifiers.ctrl => motion(state, kjxlkj_core_edit::Motion::Up, count),
        Key::Char('0') if !key.modifiers.ctrl && state.count.is_none() => motion(state, kjxlkj_core_edit::Motion::LineStart, 1),
        Key::Char('^') if !key.modifiers.ctrl => motion(state, kjxlkj_core_edit::Motion::FirstNonBlank, 1),
        Key::Char('$') if !key.modifiers.ctrl => motion(state, kjxlkj_core_edit::Motion::LineEnd, count),
        Key::Char('w') if !key.modifiers.ctrl => motion(state, kjxlkj_core_edit::Motion::WordStart(kjxlkj_core_edit::Direction::Forward), count),
        Key::Char('W') if !key.modifiers.ctrl => motion(state, kjxlkj_core_edit::Motion::BigWordStart(kjxlkj_core_edit::Direction::Forward), count),
        Key::Char('b') if !key.modifiers.ctrl => motion(state, kjxlkj_core_edit::Motion::WordStart(kjxlkj_core_edit::Direction::Backward), count),
        Key::Char('B') if !key.modifiers.ctrl => motion(state, kjxlkj_core_edit::Motion::BigWordStart(kjxlkj_core_edit::Direction::Backward), count),
        Key::Char('e') if !key.modifiers.ctrl => motion(state, kjxlkj_core_edit::Motion::WordEnd(kjxlkj_core_edit::Direction::Forward), count),
        Key::Char('E') if !key.modifiers.ctrl => motion(state, kjxlkj_core_edit::Motion::BigWordEnd(kjxlkj_core_edit::Direction::Forward), count),
        Key::Char('G') if !key.modifiers.ctrl => {
            let m = if state.count.is_some() { kjxlkj_core_edit::Motion::Line(count) } else { kjxlkj_core_edit::Motion::LastLine };
            motion(state, m, 1)
        }
        Key::Char('g') if !key.modifiers.ctrl => { state.pending_prefix = PendingPrefix::G; HandleResult::Pending }

        Key::Char('x') if !key.modifiers.ctrl => { state.reset_prefix(); HandleResult::Consumed(vec![ModeAction::DeleteAtCursor(kjxlkj_core_edit::Direction::Forward); count]) }
        Key::Char('X') if !key.modifiers.ctrl => { state.reset_prefix(); HandleResult::Consumed(vec![ModeAction::DeleteAtCursor(kjxlkj_core_edit::Direction::Backward); count]) }

        Key::Char('d') if !key.modifiers.ctrl => { state.enter_operator_pending(PendingOperator::Delete); HandleResult::Pending }
        Key::Char('c') if !key.modifiers.ctrl => { state.enter_operator_pending(PendingOperator::Change); HandleResult::Pending }
        Key::Char('y') if !key.modifiers.ctrl => { state.enter_operator_pending(PendingOperator::Yank); HandleResult::Pending }
        Key::Char('>') if !key.modifiers.ctrl => { state.enter_operator_pending(PendingOperator::IndentRight); HandleResult::Pending }
        Key::Char('<') if !key.modifiers.ctrl => { state.enter_operator_pending(PendingOperator::IndentLeft); HandleResult::Pending }

        Key::Char('p') if !key.modifiers.ctrl => { state.reset_prefix(); HandleResult::Consumed(vec![ModeAction::Put { before: false }]) }
        Key::Char('P') if !key.modifiers.ctrl => { state.reset_prefix(); HandleResult::Consumed(vec![ModeAction::Put { before: true }]) }
        Key::Char('u') if !key.modifiers.ctrl => { state.reset_prefix(); HandleResult::Consumed(vec![ModeAction::Undo]) }
        Key::Char('r') if key.modifiers.ctrl => { state.reset_prefix(); HandleResult::Consumed(vec![ModeAction::Redo]) }
        Key::Char('.') if !key.modifiers.ctrl => { state.reset_prefix(); HandleResult::Consumed(vec![ModeAction::Repeat]) }

        Key::Char('d') if key.modifiers.ctrl => { state.reset_prefix(); HandleResult::Consumed(vec![ModeAction::Scroll(ScrollAction::HalfPageDown)]) }
        Key::Char('u') if key.modifiers.ctrl => { state.reset_prefix(); HandleResult::Consumed(vec![ModeAction::Scroll(ScrollAction::HalfPageUp)]) }
        Key::Char('f') if key.modifiers.ctrl => { state.reset_prefix(); HandleResult::Consumed(vec![ModeAction::Scroll(ScrollAction::PageDown)]) }
        Key::Char('b') if key.modifiers.ctrl => { state.reset_prefix(); HandleResult::Consumed(vec![ModeAction::Scroll(ScrollAction::PageUp)]) }
        Key::Char('w') if key.modifiers.ctrl => { state.pending_prefix = PendingPrefix::Window; HandleResult::Pending }

        Key::Char(':') if !key.modifiers.ctrl => { state.enter_command(kjxlkj_core_types::CommandKind::Ex); HandleResult::Consumed(vec![ModeAction::EnterCommand(kjxlkj_core_types::CommandKind::Ex)]) }
        Key::Char('/') if !key.modifiers.ctrl => { state.enter_command(kjxlkj_core_types::CommandKind::SearchForward); HandleResult::Consumed(vec![ModeAction::EnterCommand(kjxlkj_core_types::CommandKind::SearchForward)]) }
        Key::Char('?') if !key.modifiers.ctrl => { state.enter_command(kjxlkj_core_types::CommandKind::SearchBackward); HandleResult::Consumed(vec![ModeAction::EnterCommand(kjxlkj_core_types::CommandKind::SearchBackward)]) }

        Key::Char('v') if !key.modifiers.ctrl => { state.enter_visual(kjxlkj_core_types::VisualKind::Char); HandleResult::Consumed(vec![ModeAction::EnterVisual(kjxlkj_core_types::VisualKind::Char)]) }
        Key::Char('V') if !key.modifiers.ctrl => { state.enter_visual(kjxlkj_core_types::VisualKind::Line); HandleResult::Consumed(vec![ModeAction::EnterVisual(kjxlkj_core_types::VisualKind::Line)]) }
        Key::Char('v') if key.modifiers.ctrl => { state.enter_visual(kjxlkj_core_types::VisualKind::Block); HandleResult::Consumed(vec![ModeAction::EnterVisual(kjxlkj_core_types::VisualKind::Block)]) }

        Key::Char('m') if !key.modifiers.ctrl => { state.pending_prefix = PendingPrefix::Mark; HandleResult::Pending }
        Key::Char('\'') if !key.modifiers.ctrl => { state.pending_prefix = PendingPrefix::MarkJump; HandleResult::Pending }
        Key::Char('Z') if !key.modifiers.ctrl => { state.pending_prefix = PendingPrefix::Z; HandleResult::Pending }
        Key::Char('"') if !key.modifiers.ctrl => { state.pending_prefix = PendingPrefix::Register; HandleResult::Pending }

        _ => HandleResult::Ignored,
    }
}

fn insert_mode(state: &mut ModeState, pos: InsertPosition) -> HandleResult {
    state.enter_insert();
    HandleResult::Consumed(vec![ModeAction::EnterInsert(pos)])
}

fn motion(state: &mut ModeState, m: kjxlkj_core_edit::Motion, count: usize) -> HandleResult {
    state.reset_prefix();
    HandleResult::Consumed(vec![ModeAction::MoveCursor(m, count)])
}

fn dispatch_with_prefix(state: &mut ModeState, key: &KeyEvent) -> HandleResult {
    let prefix = state.pending_prefix;
    let count = state.effective_count();

    match prefix {
        PendingPrefix::G => dispatch_g(state, key, count),
        PendingPrefix::Window => dispatch_window(state, key),
        PendingPrefix::Z => dispatch_z(state, key),
        PendingPrefix::Register => {
            if let Key::Char(c) = key.key { state.register = Some(c); state.pending_prefix = PendingPrefix::None; HandleResult::Consumed(vec![]) }
            else { state.reset_prefix(); HandleResult::Ignored }
        }
        PendingPrefix::Mark => {
            if let Key::Char(c) = key.key { state.reset_prefix(); HandleResult::Consumed(vec![ModeAction::SetMark(c)]) }
            else { state.reset_prefix(); HandleResult::Ignored }
        }
        PendingPrefix::MarkJump => {
            if let Key::Char(c) = key.key { state.reset_prefix(); HandleResult::Consumed(vec![ModeAction::JumpToMark(c)]) }
            else { state.reset_prefix(); HandleResult::Ignored }
        }
        PendingPrefix::None => HandleResult::Ignored,
    }
}

fn dispatch_g(state: &mut ModeState, key: &KeyEvent, count: usize) -> HandleResult {
    match &key.key {
        Key::Char('g') => { let m = if state.count.is_some() { kjxlkj_core_edit::Motion::Line(count) } else { kjxlkj_core_edit::Motion::Line(1) }; motion(state, m, 1) }
        Key::Char('e') => motion(state, kjxlkj_core_edit::Motion::WordEnd(kjxlkj_core_edit::Direction::Backward), count),
        Key::Char('E') => motion(state, kjxlkj_core_edit::Motion::BigWordEnd(kjxlkj_core_edit::Direction::Backward), count),
        _ => { state.reset_prefix(); HandleResult::Ignored }
    }
}

fn dispatch_window(state: &mut ModeState, key: &KeyEvent) -> HandleResult {
    state.reset_prefix();
    match &key.key {
        Key::Char('h') | Key::Special(SpecialKey::Left) => HandleResult::Consumed(vec![ModeAction::FocusWindow(FocusDirection::Left)]),
        Key::Char('j') | Key::Special(SpecialKey::Down) => HandleResult::Consumed(vec![ModeAction::FocusWindow(FocusDirection::Down)]),
        Key::Char('k') | Key::Special(SpecialKey::Up) => HandleResult::Consumed(vec![ModeAction::FocusWindow(FocusDirection::Up)]),
        Key::Char('l') | Key::Special(SpecialKey::Right) => HandleResult::Consumed(vec![ModeAction::FocusWindow(FocusDirection::Right)]),
        Key::Char('s') => HandleResult::Consumed(vec![ModeAction::SplitWindow(SplitDirection::Horizontal)]),
        Key::Char('v') => HandleResult::Consumed(vec![ModeAction::SplitWindow(SplitDirection::Vertical)]),
        Key::Char('c') | Key::Char('q') => HandleResult::Consumed(vec![ModeAction::CloseWindow(WindowTarget::Current)]),
        Key::Char('o') => HandleResult::Consumed(vec![ModeAction::CloseWindow(WindowTarget::Others)]),
        Key::Char('w') => HandleResult::Consumed(vec![ModeAction::FocusWindow(FocusDirection::Next)]),
        _ => HandleResult::Ignored,
    }
}

fn dispatch_z(state: &mut ModeState, key: &KeyEvent) -> HandleResult {
    state.reset_prefix();
    match &key.key {
        Key::Char('Z') => HandleResult::Consumed(vec![ModeAction::WriteAndQuit]),
        Key::Char('Q') => HandleResult::Consumed(vec![ModeAction::QuitWithoutSave]),
        _ => HandleResult::Ignored,
    }
}
