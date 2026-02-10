//! Visual mode key dispatch.
//!
//! In visual mode, operators apply immediately to the selection
//! instead of entering operator-pending mode.

use kjxlkj_core_types::{Key, KeyAction, KeyCode, KeyModifiers, Mode, Operator, VisualKind};

use crate::dispatch::{key_to_motion, DispatchResult, ModeDispatcher};

/// Handle a key in Visual mode. Operators apply to the selection;
/// motions extend/shrink the selection; Esc/v/V toggle out.
pub fn dispatch_visual(disp: &mut ModeDispatcher, key: &Key, vk: &VisualKind) -> DispatchResult {
    // Count accumulation
    if key.modifiers.is_empty() {
        if let KeyCode::Char(c) = &key.code {
            if c.is_ascii_digit() && (*c != '0' || disp.count.is_some()) {
                disp.accumulate_count(c.to_digit(10).unwrap());
                return DispatchResult::Pending;
            }
        }
    }
    // Register prefix
    if key.modifiers.is_empty() {
        if let KeyCode::Char('"') = &key.code {
            disp.pending_keys.push(key.clone());
            return DispatchResult::Pending;
        }
    }
    // Handle pending " register
    if !disp.pending_keys.is_empty() {
        if let Some(prev) = disp.pending_keys.last().cloned() {
            if prev.code == KeyCode::Char('"') && key.modifiers.is_empty() {
                if let KeyCode::Char(c) = &key.code {
                    disp.register = Some(*c);
                    disp.pending_keys.clear();
                    return DispatchResult::Pending;
                }
                disp.pending_keys.clear();
                return DispatchResult::Noop;
            }
        }
        disp.pending_keys.clear();
    }
    // Motions extend the selection
    if let Some(motion) = key_to_motion(key) {
        return DispatchResult::Action(KeyAction::Motion(motion));
    }
    if key.modifiers.is_empty() {
        match &key.code {
            // Operators apply immediately on the visual selection
            KeyCode::Char('d') | KeyCode::Char('x') => {
                DispatchResult::Action(KeyAction::VisualOperator(Operator::Delete))
            }
            KeyCode::Char('c') | KeyCode::Char('s') => {
                DispatchResult::Action(KeyAction::VisualOperator(Operator::Change))
            }
            KeyCode::Char('y') => DispatchResult::Action(KeyAction::VisualOperator(Operator::Yank)),
            KeyCode::Char('>') => {
                DispatchResult::Action(KeyAction::VisualOperator(Operator::Indent))
            }
            KeyCode::Char('<') => {
                DispatchResult::Action(KeyAction::VisualOperator(Operator::Dedent))
            }
            // Toggle visual sub-modes or exit
            KeyCode::Char('v') => match vk {
                VisualKind::Char => DispatchResult::ModeChange(Mode::Normal),
                _ => DispatchResult::ModeChange(Mode::Visual(VisualKind::Char)),
            },
            KeyCode::Char('V') => match vk {
                VisualKind::Line => DispatchResult::ModeChange(Mode::Normal),
                _ => DispatchResult::ModeChange(Mode::Visual(VisualKind::Line)),
            },
            KeyCode::Char('J') => DispatchResult::Action(KeyAction::JoinLines),
            KeyCode::Char('p') => DispatchResult::Action(KeyAction::PutAfter),
            KeyCode::Char('P') => DispatchResult::Action(KeyAction::PutBefore),
            KeyCode::Char(':') => {
                DispatchResult::ModeChange(Mode::Command(kjxlkj_core_types::CommandKind::Ex))
            }
            KeyCode::Esc => DispatchResult::ModeChange(Mode::Normal),
            _ => DispatchResult::Noop,
        }
    } else if key.modifiers.contains(KeyModifiers::CTRL) {
        match &key.code {
            KeyCode::Char('v') => match vk {
                VisualKind::Block => DispatchResult::ModeChange(Mode::Normal),
                _ => DispatchResult::ModeChange(Mode::Visual(VisualKind::Block)),
            },
            KeyCode::Char('w') => {
                disp.window_pending = true;
                DispatchResult::Pending
            }
            _ => {
                if let Some(motion) = key_to_motion(key) {
                    DispatchResult::Action(KeyAction::Motion(motion))
                } else {
                    DispatchResult::Noop
                }
            }
        }
    } else {
        DispatchResult::Noop
    }
}
