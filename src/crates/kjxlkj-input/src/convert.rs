//! Crossterm event conversion.

use crossterm::event::{Event, KeyEvent, KeyCode as CtKeyCode, KeyModifiers as CtMods};
use kjxlkj_core_types::{Key, KeyCode, KeyModifiers};

/// Convert a crossterm event to our Key type.
pub fn convert_event(event: Event) -> Option<Key> {
    match event {
        Event::Key(key_event) => Some(convert_key_event(key_event)),
        _ => None,
    }
}

/// Convert a crossterm KeyEvent to our Key type.
pub fn convert_key_event(event: KeyEvent) -> Key {
    let code = convert_key_code(event.code);
    let mods = convert_modifiers(event.modifiers);
    Key { code, mods }
}

fn convert_key_code(code: CtKeyCode) -> KeyCode {
    match code {
        CtKeyCode::Char(c) => KeyCode::Char(c),
        CtKeyCode::F(n) => KeyCode::F(n),
        CtKeyCode::Esc => KeyCode::Esc,
        CtKeyCode::Enter => KeyCode::Enter,
        CtKeyCode::Tab => KeyCode::Tab,
        CtKeyCode::Backspace => KeyCode::Backspace,
        CtKeyCode::Delete => KeyCode::Delete,
        CtKeyCode::Insert => KeyCode::Insert,
        CtKeyCode::Home => KeyCode::Home,
        CtKeyCode::End => KeyCode::End,
        CtKeyCode::PageUp => KeyCode::PageUp,
        CtKeyCode::PageDown => KeyCode::PageDown,
        CtKeyCode::Left => KeyCode::Left,
        CtKeyCode::Right => KeyCode::Right,
        CtKeyCode::Up => KeyCode::Up,
        CtKeyCode::Down => KeyCode::Down,
        _ => KeyCode::Char('\0'),
    }
}

fn convert_modifiers(mods: CtMods) -> KeyModifiers {
    let mut result = KeyModifiers::NONE;
    if mods.contains(CtMods::CONTROL) {
        result = result.union(KeyModifiers::CTRL);
    }
    if mods.contains(CtMods::ALT) {
        result = result.union(KeyModifiers::ALT);
    }
    if mods.contains(CtMods::SHIFT) {
        result = result.union(KeyModifiers::SHIFT);
    }
    result
}
