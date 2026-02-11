//! Mode dispatch and transition logic.
//!
//! See /docs/spec/modes/transitions.md for the normative transition table.

mod normal;
mod normal_motions;
mod resolver;

pub use normal::handle_normal_key;
pub use resolver::resolve_mode_transition;

use kjxlkj_core_types::{Action, Key, KeyModifiers, Mode};

/// Process a key event in the current mode context.
///
/// Returns the action to execute and optionally a new mode.
pub fn dispatch_key(
    mode: Mode,
    key: &Key,
    mods: &KeyModifiers,
) -> (Action, Option<Mode>) {
    match mode {
        Mode::Normal => normal::handle_normal_key(key, mods),
        Mode::Insert => handle_insert_key(key, mods),
        Mode::Command(_) => handle_command_key(key, mods),
        Mode::Visual(_) => handle_visual_key(key, mods, mode),
        Mode::Replace => handle_replace_key(key, mods),
        Mode::OperatorPending(_) => {
            handle_operator_pending(key, mods, mode)
        }
        Mode::TerminalInsert => handle_terminal_insert(key, mods),
        Mode::InsertNormal => normal::handle_normal_key(key, mods),
    }
}

fn handle_insert_key(
    key: &Key,
    _mods: &KeyModifiers,
) -> (Action, Option<Mode>) {
    match key {
        Key::Escape => (Action::ExitToNormal, Some(Mode::Normal)),
        Key::Char(c) => (Action::InsertChar(*c), None),
        Key::Enter => (Action::InsertChar('\n'), None),
        Key::Backspace => (Action::DeleteCharBackward, None),
        Key::Delete => (Action::DeleteCharForward, None),
        Key::Tab => (Action::InsertChar('\t'), None),
        _ => (Action::Noop, None),
    }
}

fn handle_command_key(
    key: &Key,
    _mods: &KeyModifiers,
) -> (Action, Option<Mode>) {
    match key {
        Key::Escape => (Action::ExitToNormal, Some(Mode::Normal)),
        Key::Enter => {
            // Command will be extracted from cmdline state.
            (Action::ExCommand(String::new()), Some(Mode::Normal))
        }
        _ => (Action::Noop, None),
    }
}

fn handle_visual_key(
    key: &Key,
    _mods: &KeyModifiers,
    _mode: Mode,
) -> (Action, Option<Mode>) {
    match key {
        Key::Escape => (Action::ExitToNormal, Some(Mode::Normal)),
        _ => (Action::Noop, None),
    }
}

fn handle_replace_key(
    key: &Key,
    _mods: &KeyModifiers,
) -> (Action, Option<Mode>) {
    match key {
        Key::Escape => (Action::ExitToNormal, Some(Mode::Normal)),
        Key::Char(c) => (Action::ReplaceChar(*c), None),
        _ => (Action::Noop, None),
    }
}

fn handle_operator_pending(
    key: &Key,
    _mods: &KeyModifiers,
    _mode: Mode,
) -> (Action, Option<Mode>) {
    match key {
        Key::Escape => (Action::ExitToNormal, Some(Mode::Normal)),
        _ => (Action::Noop, Some(Mode::Normal)),
    }
}

fn handle_terminal_insert(
    key: &Key,
    mods: &KeyModifiers,
) -> (Action, Option<Mode>) {
    // Ctrl-\ Ctrl-n exits terminal insert mode.
    if mods.ctrl {
        if let Key::Char('n') = key {
            return (Action::ExitToNormal, Some(Mode::Normal));
        }
    }
    (
        Action::ForwardKey(key.clone(), *mods),
        None,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn insert_escape_returns_normal() {
        let (action, mode) = dispatch_key(
            Mode::Insert,
            &Key::Escape,
            &KeyModifiers::default(),
        );
        assert_eq!(action, Action::ExitToNormal);
        assert_eq!(mode, Some(Mode::Normal));
    }

    #[test]
    fn insert_char_stays_in_insert() {
        let (action, mode) = dispatch_key(
            Mode::Insert,
            &Key::Char('x'),
            &KeyModifiers::default(),
        );
        assert_eq!(action, Action::InsertChar('x'));
        assert_eq!(mode, None);
    }

    #[test]
    fn normal_mode_a_enters_insert() {
        let (action, mode) = dispatch_key(
            Mode::Normal,
            &Key::Char('a'),
            &KeyModifiers::default(),
        );
        // 'a' = append after cursor (enters insert)
        assert_eq!(mode, Some(Mode::Insert));
    }
}
