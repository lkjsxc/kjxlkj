//! Key representation.

use crossterm::event::{KeyCode as CtKeyCode, KeyEvent, KeyModifiers};
use serde::{Deserialize, Serialize};

/// Key modifiers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct Modifiers {
    pub ctrl: bool,
    pub alt: bool,
    pub shift: bool,
}

impl Modifiers {
    pub fn none() -> Self {
        Self::default()
    }

    pub fn ctrl() -> Self {
        Self {
            ctrl: true,
            ..Default::default()
        }
    }

    pub fn from_crossterm(mods: KeyModifiers) -> Self {
        Self {
            ctrl: mods.contains(KeyModifiers::CONTROL),
            alt: mods.contains(KeyModifiers::ALT),
            shift: mods.contains(KeyModifiers::SHIFT),
        }
    }
}

/// Key codes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum KeyCode {
    Char(char),
    Backspace,
    Enter,
    Left,
    Right,
    Up,
    Down,
    Home,
    End,
    PageUp,
    PageDown,
    Tab,
    Delete,
    Insert,
    Escape,
    F(u8),
    Null,
}

impl KeyCode {
    pub fn from_crossterm(code: CtKeyCode) -> Self {
        match code {
            CtKeyCode::Char(c) => KeyCode::Char(c),
            CtKeyCode::Backspace => KeyCode::Backspace,
            CtKeyCode::Enter => KeyCode::Enter,
            CtKeyCode::Left => KeyCode::Left,
            CtKeyCode::Right => KeyCode::Right,
            CtKeyCode::Up => KeyCode::Up,
            CtKeyCode::Down => KeyCode::Down,
            CtKeyCode::Home => KeyCode::Home,
            CtKeyCode::End => KeyCode::End,
            CtKeyCode::PageUp => KeyCode::PageUp,
            CtKeyCode::PageDown => KeyCode::PageDown,
            CtKeyCode::Tab => KeyCode::Tab,
            CtKeyCode::Delete => KeyCode::Delete,
            CtKeyCode::Insert => KeyCode::Insert,
            CtKeyCode::Esc => KeyCode::Escape,
            CtKeyCode::F(n) => KeyCode::F(n),
            _ => KeyCode::Null,
        }
    }
}

/// A key press.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Key {
    pub code: KeyCode,
    pub mods: Modifiers,
}

impl Key {
    pub fn new(code: KeyCode, mods: Modifiers) -> Self {
        Self { code, mods }
    }

    pub fn char(c: char) -> Self {
        Self::new(KeyCode::Char(c), Modifiers::none())
    }

    pub fn ctrl_char(c: char) -> Self {
        Self::new(KeyCode::Char(c), Modifiers::ctrl())
    }

    pub fn from_crossterm(event: KeyEvent) -> Self {
        Self {
            code: KeyCode::from_crossterm(event.code),
            mods: Modifiers::from_crossterm(event.modifiers),
        }
    }

    pub fn is_escape(&self) -> bool {
        matches!(self.code, KeyCode::Escape)
    }

    pub fn is_enter(&self) -> bool {
        matches!(self.code, KeyCode::Enter)
    }

    pub fn is_backspace(&self) -> bool {
        matches!(self.code, KeyCode::Backspace)
            || (matches!(self.code, KeyCode::Char('h')) && self.mods.ctrl)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn key_is_escape() {
        let k = Key::new(KeyCode::Escape, Modifiers::none());
        assert!(k.is_escape());
    }

    #[test]
    fn key_is_not_escape() {
        let k = Key::char('a');
        assert!(!k.is_escape());
    }

    #[test]
    fn key_is_enter() {
        let k = Key::new(KeyCode::Enter, Modifiers::none());
        assert!(k.is_enter());
    }

    #[test]
    fn key_is_backspace() {
        let k = Key::new(KeyCode::Backspace, Modifiers::none());
        assert!(k.is_backspace());
    }

    #[test]
    fn key_ctrl_h_is_backspace() {
        let k = Key::ctrl_char('h');
        assert!(k.is_backspace());
    }

    #[test]
    fn modifiers_none_has_no_ctrl() {
        let mods = Modifiers::none();
        assert!(!mods.ctrl);
    }

    #[test]
    fn modifiers_ctrl_has_ctrl() {
        let mods = Modifiers::ctrl();
        assert!(mods.ctrl);
    }

    #[test]
    fn modifiers_shift_has_shift() {
        let mods = Modifiers { ctrl: false, alt: false, shift: true };
        assert!(mods.shift);
    }

    #[test]
    fn modifiers_alt_has_alt() {
        let mods = Modifiers { ctrl: false, alt: true, shift: false };
        assert!(mods.alt);
    }

    #[test]
    fn key_new_struct() {
        let k = Key { code: KeyCode::Tab, mods: Modifiers::none() };
        assert!(matches!(k.code, KeyCode::Tab));
    }

    #[test]
    fn key_code_debug() {
        let _ = format!("{:?}", KeyCode::Delete);
    }

    #[test]
    fn key_modifiers_default() {
        let mods = Modifiers::default();
        assert!(!mods.ctrl && !mods.alt && !mods.shift);
    }
}
