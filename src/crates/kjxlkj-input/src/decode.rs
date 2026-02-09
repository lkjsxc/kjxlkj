//! Decode crossterm events into kjxlkj Key and Action types.

use kjxlkj_core_types::{Action, Key, KeyCode, KeyModifiers};

/// Convert a crossterm key event into our Key type.
pub fn decode_crossterm_key(event: &crossterm::event::KeyEvent) -> Key {
    let code = match event.code {
        crossterm::event::KeyCode::Char(c) => KeyCode::Char(c),
        crossterm::event::KeyCode::F(n) => KeyCode::F(n),
        crossterm::event::KeyCode::Backspace => KeyCode::Backspace,
        crossterm::event::KeyCode::Enter => KeyCode::Enter,
        crossterm::event::KeyCode::Tab => KeyCode::Tab,
        crossterm::event::KeyCode::BackTab => KeyCode::BackTab,
        crossterm::event::KeyCode::Esc => KeyCode::Esc,
        crossterm::event::KeyCode::Delete => KeyCode::Delete,
        crossterm::event::KeyCode::Insert => KeyCode::Insert,
        crossterm::event::KeyCode::Home => KeyCode::Home,
        crossterm::event::KeyCode::End => KeyCode::End,
        crossterm::event::KeyCode::PageUp => KeyCode::PageUp,
        crossterm::event::KeyCode::PageDown => KeyCode::PageDown,
        crossterm::event::KeyCode::Up => KeyCode::Up,
        crossterm::event::KeyCode::Down => KeyCode::Down,
        crossterm::event::KeyCode::Left => KeyCode::Left,
        crossterm::event::KeyCode::Right => KeyCode::Right,
        crossterm::event::KeyCode::Null => KeyCode::Null,
        _ => KeyCode::Null,
    };

    let mut modifiers = KeyModifiers::NONE;
    if event
        .modifiers
        .contains(crossterm::event::KeyModifiers::SHIFT)
    {
        modifiers |= KeyModifiers::SHIFT;
    }
    if event
        .modifiers
        .contains(crossterm::event::KeyModifiers::CONTROL)
    {
        modifiers |= KeyModifiers::CTRL;
    }
    if event
        .modifiers
        .contains(crossterm::event::KeyModifiers::ALT)
    {
        modifiers |= KeyModifiers::ALT;
    }

    Key { code, modifiers }
}

/// Convert a crossterm event into an Action.
pub fn decode_crossterm_event(event: &crossterm::event::Event) -> Option<Action> {
    match event {
        crossterm::event::Event::Key(key_event) => {
            // Keys are dispatched by the mode-aware layer,
            // we just decode them here. Return None since
            // mode handling is done elsewhere.
            let _key = decode_crossterm_key(key_event);
            None
        }
        crossterm::event::Event::Resize(cols, rows) => Some(Action::Resize(*cols, *rows)),
        crossterm::event::Event::Paste(text) => Some(Action::Paste(text.clone())),
        crossterm::event::Event::FocusGained => Some(Action::FocusGained),
        crossterm::event::Event::FocusLost => Some(Action::FocusLost),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decode_char_key() {
        let event = crossterm::event::KeyEvent::new(
            crossterm::event::KeyCode::Char('a'),
            crossterm::event::KeyModifiers::NONE,
        );
        let key = decode_crossterm_key(&event);
        assert_eq!(key.code, KeyCode::Char('a'));
        assert_eq!(key.modifiers, KeyModifiers::NONE);
    }

    #[test]
    fn decode_ctrl_key() {
        let event = crossterm::event::KeyEvent::new(
            crossterm::event::KeyCode::Char('w'),
            crossterm::event::KeyModifiers::CONTROL,
        );
        let key = decode_crossterm_key(&event);
        assert_eq!(key.code, KeyCode::Char('w'));
        assert!(key.modifiers.contains(KeyModifiers::CTRL));
    }
}
