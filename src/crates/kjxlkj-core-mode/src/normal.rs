//! Normal mode key dispatch.

use kjxlkj_core_types::{
    CommandKind, Key, KeyAction, KeyCode, KeyModifiers, Mode, Operator, VisualKind,
};

use crate::dispatch::{key_to_motion, DispatchResult, ModeDispatcher};

/// Handle a key in Normal mode.
pub fn dispatch_normal(disp: &mut ModeDispatcher, key: &Key) -> DispatchResult {
    // Count accumulation: digits 1-9 start count, 0 after digits
    if key.modifiers.is_empty() {
        if let KeyCode::Char(c) = &key.code {
            let c = *c;
            if c.is_ascii_digit() && (c != '0' || disp.count.is_some()) {
                disp.accumulate_count(c.to_digit(10).unwrap());
                return DispatchResult::Pending;
            }
        }
    }

    // Register prefix: "x
    if key.modifiers.is_empty() {
        if let KeyCode::Char('"') = &key.code {
            disp.pending_keys.push(key.clone());
            return DispatchResult::Pending;
        }
        // Leader key: Space prefix
        if let KeyCode::Char(' ') = &key.code {
            disp.pending_keys.push(key.clone());
            return DispatchResult::Pending;
        }
        // Viewport commands: z prefix
        if let KeyCode::Char('z') = &key.code {
            disp.pending_keys.push(key.clone());
            return DispatchResult::Pending;
        }
    }

    // Handle pending leader/z/" keys
    if !disp.pending_keys.is_empty() {
        if let Some(prev) = disp.pending_keys.last().cloned() {
            // Register prefix: " followed by register name
            if prev.code == KeyCode::Char('"') && key.modifiers.is_empty() {
                if let KeyCode::Char(c) = &key.code {
                    disp.register = Some(*c);
                    disp.pending_keys.clear();
                    return DispatchResult::Pending;
                }
                disp.pending_keys.clear();
                return DispatchResult::Noop;
            }
            // Leader sequences: <Space> followed by key(s)
            if prev.code == KeyCode::Char(' ') && key.modifiers.is_empty() {
                match &key.code {
                    KeyCode::Char('e') => {
                        disp.pending_keys.clear();
                        return DispatchResult::Action(KeyAction::ExplorerToggle);
                    }
                    KeyCode::Char('E') => {
                        disp.pending_keys.clear();
                        return DispatchResult::Action(KeyAction::ExplorerReveal);
                    }
                    KeyCode::Char('t') => {
                        // Could be <leader>t, <leader>th, or <leader>tv
                        disp.pending_keys.push(key.clone());
                        return DispatchResult::Pending;
                    }
                    _ => {
                        disp.pending_keys.clear();
                        return DispatchResult::Noop;
                    }
                }
            }
            // Leader-t sequences: <Space>t followed by h/v or timeout
            if disp.pending_keys.len() == 2 {
                let first = &disp.pending_keys[0];
                let second = &disp.pending_keys[1];
                if first.code == KeyCode::Char(' ') && second.code == KeyCode::Char('t') {
                    disp.pending_keys.clear();
                    return match &key.code {
                        KeyCode::Char('h') => DispatchResult::Action(KeyAction::TerminalSplitH),
                        KeyCode::Char('v') => DispatchResult::Action(KeyAction::TerminalSplitV),
                        _ => DispatchResult::Action(KeyAction::TerminalOpen),
                    };
                }
            }
            // z-prefix viewport commands
            if prev.code == KeyCode::Char('z') && key.modifiers.is_empty() {
                disp.pending_keys.clear();
                return match &key.code {
                    KeyCode::Char('z') => DispatchResult::Action(KeyAction::ViewportCenter),
                    KeyCode::Char('t') => DispatchResult::Action(KeyAction::ViewportTop),
                    KeyCode::Char('b') => DispatchResult::Action(KeyAction::ViewportBottom),
                    _ => DispatchResult::Noop,
                };
            }
        }
        disp.pending_keys.clear();
    }

    let count = disp.take_count();

    // Check motions first
    if let Some(motion) = key_to_motion(key) {
        return DispatchResult::Action(KeyAction::Motion(motion));
    }

    if key.modifiers.is_empty() {
        match &key.code {
            // Insert entry keys
            KeyCode::Char('i') => DispatchResult::ModeChange(Mode::Insert),
            KeyCode::Char('a') => DispatchResult::Action(KeyAction::InsertAppend),
            KeyCode::Char('A') => DispatchResult::Action(KeyAction::InsertAppendEol),
            KeyCode::Char('I') => DispatchResult::Action(KeyAction::InsertFirstNonBlank),
            KeyCode::Char('o') => DispatchResult::Action(KeyAction::OpenLineBelow),
            KeyCode::Char('O') => DispatchResult::Action(KeyAction::OpenLineAbove),
            // Operators
            KeyCode::Char('d') => {
                disp.count = Some(count);
                DispatchResult::ModeChange(Mode::OperatorPending(Operator::Delete))
            }
            KeyCode::Char('c') => {
                disp.count = Some(count);
                DispatchResult::ModeChange(Mode::OperatorPending(Operator::Change))
            }
            KeyCode::Char('y') => {
                disp.count = Some(count);
                DispatchResult::ModeChange(Mode::OperatorPending(Operator::Yank))
            }
            KeyCode::Char('>') => {
                disp.count = Some(count);
                DispatchResult::ModeChange(Mode::OperatorPending(Operator::Indent))
            }
            KeyCode::Char('<') => {
                disp.count = Some(count);
                DispatchResult::ModeChange(Mode::OperatorPending(Operator::Dedent))
            }
            // Visual modes
            KeyCode::Char('v') => DispatchResult::ModeChange(Mode::Visual(VisualKind::Char)),
            KeyCode::Char('V') => DispatchResult::ModeChange(Mode::Visual(VisualKind::Line)),
            // Command mode
            KeyCode::Char(':') => DispatchResult::ModeChange(Mode::Command(CommandKind::Ex)),
            KeyCode::Char('/') => {
                DispatchResult::ModeChange(Mode::Command(CommandKind::SearchForward))
            }
            KeyCode::Char('?') => {
                DispatchResult::ModeChange(Mode::Command(CommandKind::SearchBackward))
            }
            // Replace mode
            KeyCode::Char('R') => DispatchResult::ModeChange(Mode::Replace),
            // Single-key commands
            KeyCode::Char('x') => DispatchResult::Action(KeyAction::DeleteCharForward),
            KeyCode::Char('X') => DispatchResult::Action(KeyAction::DeleteCharBackward),
            KeyCode::Char('u') => DispatchResult::Action(KeyAction::Undo),
            KeyCode::Char('p') => DispatchResult::Action(KeyAction::PutAfter),
            KeyCode::Char('P') => DispatchResult::Action(KeyAction::PutBefore),
            KeyCode::Char('J') => DispatchResult::Action(KeyAction::JoinLines),
            KeyCode::Esc => DispatchResult::ModeChange(Mode::Normal),
            _ => DispatchResult::Noop,
        }
    } else if key.modifiers.contains(KeyModifiers::CTRL) {
        match &key.code {
            KeyCode::Char('r') => DispatchResult::Action(KeyAction::Redo),
            KeyCode::Char('v') => DispatchResult::ModeChange(Mode::Visual(VisualKind::Block)),
            KeyCode::Char('w') => {
                disp.window_pending = true;
                DispatchResult::Pending
            }
            _ => {
                // Try motions with ctrl
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
