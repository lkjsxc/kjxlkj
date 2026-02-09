use kjxlkj_core_types::{CommandKind, Key, KeyCode, Mode, Modifier, Operator, VisualKind};

/// Result of a mode transition attempt.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ModeTransition {
    /// Stay in current mode.
    Stay,
    /// Transition to a new mode.
    To(Mode),
}

/// Determine the mode transition for a given mode and key.
///
/// Transitions are deterministic and never block on IO.
pub fn transition(mode: &Mode, key: &Key, in_terminal_window: bool) -> ModeTransition {
    match mode {
        Mode::Normal => normal_transition(key, in_terminal_window),
        Mode::Insert => insert_transition(key),
        Mode::Replace => replace_transition(key),
        Mode::Visual(kind) => visual_transition(key, *kind),
        Mode::Command(_) => command_transition(key),
        Mode::OperatorPending(_) => op_pending_transition(key),
        Mode::TerminalInsert => terminal_insert_transition(key),
        Mode::InsertNormal => ModeTransition::Stay,
    }
}

fn normal_transition(key: &Key, in_terminal_window: bool) -> ModeTransition {
    match (&key.code, key.modifiers) {
        // Insert entry keys
        (KeyCode::Char('i'), Modifier::NONE) if in_terminal_window => {
            ModeTransition::To(Mode::TerminalInsert)
        }
        (KeyCode::Char('i'), Modifier::NONE) => ModeTransition::To(Mode::Insert),
        (KeyCode::Char('a'), Modifier::NONE) => ModeTransition::To(Mode::Insert),
        (KeyCode::Char('I'), Modifier::NONE) => ModeTransition::To(Mode::Insert),
        (KeyCode::Char('A'), Modifier::NONE) => ModeTransition::To(Mode::Insert),
        (KeyCode::Char('o'), Modifier::NONE) => ModeTransition::To(Mode::Insert),
        (KeyCode::Char('O'), Modifier::NONE) => ModeTransition::To(Mode::Insert),
        (KeyCode::Char('s'), Modifier::NONE) => ModeTransition::To(Mode::Insert),
        (KeyCode::Char('S'), Modifier::NONE) => ModeTransition::To(Mode::Insert),
        (KeyCode::Char('C'), Modifier::NONE) => ModeTransition::To(Mode::Insert),
        // Visual entry
        (KeyCode::Char('v'), Modifier::NONE) => ModeTransition::To(Mode::Visual(VisualKind::Char)),
        (KeyCode::Char('V'), Modifier::NONE) => ModeTransition::To(Mode::Visual(VisualKind::Line)),
        (KeyCode::Char('v'), m) if m.contains(Modifier::CTRL) => {
            ModeTransition::To(Mode::Visual(VisualKind::Block))
        }
        // Command entry
        (KeyCode::Char(':'), Modifier::NONE) => ModeTransition::To(Mode::Command(CommandKind::Ex)),
        (KeyCode::Char('/'), Modifier::NONE) => {
            ModeTransition::To(Mode::Command(CommandKind::Search))
        }
        (KeyCode::Char('?'), Modifier::NONE) => {
            ModeTransition::To(Mode::Command(CommandKind::Search))
        }
        // Replace
        (KeyCode::Char('R'), Modifier::NONE) => ModeTransition::To(Mode::Replace),
        // Operators
        (KeyCode::Char('d'), Modifier::NONE) => {
            ModeTransition::To(Mode::OperatorPending(Operator::Delete))
        }
        (KeyCode::Char('c'), Modifier::NONE) => {
            ModeTransition::To(Mode::OperatorPending(Operator::Change))
        }
        (KeyCode::Char('y'), Modifier::NONE) => {
            ModeTransition::To(Mode::OperatorPending(Operator::Yank))
        }
        (KeyCode::Char('>'), Modifier::NONE) => {
            ModeTransition::To(Mode::OperatorPending(Operator::Indent))
        }
        (KeyCode::Char('<'), Modifier::NONE) => {
            ModeTransition::To(Mode::OperatorPending(Operator::Dedent))
        }
        (KeyCode::Char('='), Modifier::NONE) => {
            ModeTransition::To(Mode::OperatorPending(Operator::Reindent))
        }
        _ => ModeTransition::Stay,
    }
}

fn insert_transition(key: &Key) -> ModeTransition {
    match (&key.code, key.modifiers) {
        (KeyCode::Esc, _) => ModeTransition::To(Mode::Normal),
        (KeyCode::Char('o'), m) if m.contains(Modifier::CTRL) => {
            ModeTransition::To(Mode::InsertNormal)
        }
        _ => ModeTransition::Stay,
    }
}

fn replace_transition(key: &Key) -> ModeTransition {
    match &key.code {
        KeyCode::Esc => ModeTransition::To(Mode::Normal),
        _ => ModeTransition::Stay,
    }
}

fn visual_transition(key: &Key, current: VisualKind) -> ModeTransition {
    match (&key.code, key.modifiers) {
        (KeyCode::Esc, _) => ModeTransition::To(Mode::Normal),
        (KeyCode::Char('v'), Modifier::NONE) => {
            if current == VisualKind::Char {
                ModeTransition::To(Mode::Normal)
            } else {
                ModeTransition::To(Mode::Visual(VisualKind::Char))
            }
        }
        (KeyCode::Char('V'), Modifier::NONE) => {
            if current == VisualKind::Line {
                ModeTransition::To(Mode::Normal)
            } else {
                ModeTransition::To(Mode::Visual(VisualKind::Line))
            }
        }
        (KeyCode::Char('v'), m) if m.contains(Modifier::CTRL) => {
            if current == VisualKind::Block {
                ModeTransition::To(Mode::Normal)
            } else {
                ModeTransition::To(Mode::Visual(VisualKind::Block))
            }
        }
        _ => ModeTransition::Stay,
    }
}

fn command_transition(key: &Key) -> ModeTransition {
    match &key.code {
        KeyCode::Esc => ModeTransition::To(Mode::Normal),
        KeyCode::Enter => ModeTransition::To(Mode::Normal),
        _ => ModeTransition::Stay,
    }
}

fn op_pending_transition(key: &Key) -> ModeTransition {
    match &key.code {
        KeyCode::Esc => ModeTransition::To(Mode::Normal),
        _ => ModeTransition::Stay, // Resolved by dispatch
    }
}

fn terminal_insert_transition(key: &Key) -> ModeTransition {
    // Ctrl-\ Ctrl-n exits terminal insert
    // This is a two-key sequence; simplified here
    match (&key.code, key.modifiers) {
        (KeyCode::Char('n'), m) if m.contains(Modifier::CTRL) => ModeTransition::To(Mode::Normal),
        _ => ModeTransition::Stay,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normal_to_insert() {
        let key = Key::char('i');
        let result = transition(&Mode::Normal, &key, false);
        assert_eq!(result, ModeTransition::To(Mode::Insert));
    }

    #[test]
    fn test_insert_to_normal_on_esc() {
        let key = Key::esc();
        let result = transition(&Mode::Insert, &key, false);
        assert_eq!(result, ModeTransition::To(Mode::Normal));
    }

    #[test]
    fn test_normal_to_visual() {
        let key = Key::char('v');
        let result = transition(&Mode::Normal, &key, false);
        assert_eq!(result, ModeTransition::To(Mode::Visual(VisualKind::Char)));
    }

    #[test]
    fn test_normal_to_command() {
        let key = Key::char(':');
        let result = transition(&Mode::Normal, &key, false);
        assert_eq!(result, ModeTransition::To(Mode::Command(CommandKind::Ex)));
    }

    #[test]
    fn test_normal_to_operator_pending() {
        let key = Key::char('d');
        let result = transition(&Mode::Normal, &key, false);
        assert_eq!(
            result,
            ModeTransition::To(Mode::OperatorPending(Operator::Delete))
        );
    }
}
