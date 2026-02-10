//! Insert mode key dispatch.

use kjxlkj_core_types::{Key, KeyAction, KeyCode, KeyModifiers, Mode};

use crate::dispatch::DispatchResult;

/// Handle a key in Insert mode.
pub fn dispatch_insert(key: &Key) -> DispatchResult {
    match &key.code {
        KeyCode::Esc => DispatchResult::ModeChange(Mode::Normal),
        KeyCode::Char(c) => {
            if key.modifiers.contains(KeyModifiers::CTRL) {
                match c {
                    'o' => DispatchResult::ModeChange(Mode::InsertNormal),
                    _ => DispatchResult::Noop,
                }
            } else {
                DispatchResult::Action(KeyAction::InsertChar(*c))
            }
        }
        KeyCode::Backspace => DispatchResult::Action(KeyAction::DeleteCharBackward),
        KeyCode::Delete => DispatchResult::Action(KeyAction::DeleteCharForward),
        KeyCode::Enter => DispatchResult::Action(KeyAction::InsertChar('\n')),
        KeyCode::Tab => DispatchResult::Action(KeyAction::InsertChar('\t')),
        _ => DispatchResult::Noop,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_char() {
        let result = dispatch_insert(&Key::char('a'));
        assert!(matches!(
            result,
            DispatchResult::Action(KeyAction::InsertChar('a'))
        ));
    }

    #[test]
    fn test_insert_esc() {
        let result = dispatch_insert(&Key::new(KeyCode::Esc, KeyModifiers::NONE));
        assert!(matches!(result, DispatchResult::ModeChange(Mode::Normal)));
    }
}
