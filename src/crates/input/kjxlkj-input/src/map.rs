use crossterm::event::{Event, KeyCode as CtKeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use kjxlkj_core_types::{Key, KeyCode, KeyMods};

use crate::InputEvent;

pub fn map_event(event: Event) -> Option<InputEvent> {
    match event {
        Event::Key(k) => map_key_event(k).map(InputEvent::Key),
        Event::Resize(cols, rows) => Some(InputEvent::Resize { cols, rows }),
        _ => None,
    }
}

pub fn map_key_event(key: KeyEvent) -> Option<Key> {
    if key.kind == KeyEventKind::Release {
        return None;
    }
    let code = match key.code {
        CtKeyCode::Char(c) => KeyCode::Char(c),
        CtKeyCode::Enter => KeyCode::Enter,
        CtKeyCode::Esc => KeyCode::Esc,
        CtKeyCode::Backspace => KeyCode::Backspace,
        CtKeyCode::Tab => KeyCode::Tab,
        CtKeyCode::Left => KeyCode::Left,
        CtKeyCode::Right => KeyCode::Right,
        CtKeyCode::Up => KeyCode::Up,
        CtKeyCode::Down => KeyCode::Down,
        CtKeyCode::Home => KeyCode::Home,
        CtKeyCode::End => KeyCode::End,
        CtKeyCode::PageUp => KeyCode::PageUp,
        CtKeyCode::PageDown => KeyCode::PageDown,
        CtKeyCode::Delete => KeyCode::Delete,
        CtKeyCode::Insert => KeyCode::Insert,
        _ => return None,
    };

    let mods = KeyMods {
        ctrl: key.modifiers.contains(KeyModifiers::CONTROL),
        alt: key.modifiers.contains(KeyModifiers::ALT),
        shift: key.modifiers.contains(KeyModifiers::SHIFT),
    };

    Some(Key { code, mods })
}

