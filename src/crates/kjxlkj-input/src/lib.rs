//! Terminal input decoding — crossterm events to editor key events.

mod keyboard_layout;
mod keybinding_coverage;
mod timing_debounce;
mod mappings_engine;
mod keybinding_dsl;
mod layout_acceptance;
mod keybinding_tables;
mod leader_keys;
mod ux_coverage;

use kjxlkj_core_types::Size;

/// A decoded editor-level input event.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EditorEvent {
    /// A key press.
    Key(KeyEvent),
    /// Terminal was resized.
    Resize(Size),
    /// Mouse event (future).
    Mouse,
    /// Paste from bracket paste mode.
    Paste(String),
    /// Focus gained.
    FocusGained,
    /// Focus lost.
    FocusLost,
}

/// A normalised key event.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KeyEvent {
    pub code: KeyCode,
    pub modifiers: Modifiers,
}

/// Key code variants.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum KeyCode {
    Char(char),
    Esc,
    Enter,
    Backspace,
    Tab,
    BackTab,
    Delete,
    Up,
    Down,
    Left,
    Right,
    Home,
    End,
    PageUp,
    PageDown,
    F(u8),
}

/// Modifier flags.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Modifiers {
    pub ctrl: bool,
    pub alt: bool,
    pub shift: bool,
}

/// Decodes crossterm events into `EditorEvent`s.
pub struct InputDecoder;

impl InputDecoder {
    pub fn new() -> Self {
        Self
    }

    /// Decode a crossterm `Event` into an `EditorEvent`.
    pub fn decode(&self, event: crossterm::event::Event) -> Option<EditorEvent> {
        match event {
            crossterm::event::Event::Key(key) => self.decode_key(key),
            crossterm::event::Event::Resize(w, h) => {
                Some(EditorEvent::Resize(Size::new(w, h)))
            }
            crossterm::event::Event::Paste(text) => Some(EditorEvent::Paste(text)),
            crossterm::event::Event::FocusGained => Some(EditorEvent::FocusGained),
            crossterm::event::Event::FocusLost => Some(EditorEvent::FocusLost),
            _ => None,
        }
    }

    fn decode_key(&self, key: crossterm::event::KeyEvent) -> Option<EditorEvent> {
        use crossterm::event::{KeyCode as CK, KeyModifiers};

        let modifiers = Modifiers {
            ctrl: key.modifiers.contains(KeyModifiers::CONTROL),
            alt: key.modifiers.contains(KeyModifiers::ALT),
            shift: key.modifiers.contains(KeyModifiers::SHIFT),
        };

        let code = match key.code {
            CK::Char(c) => KeyCode::Char(c),
            CK::Esc => KeyCode::Esc,
            CK::Enter => KeyCode::Enter,
            CK::Backspace => KeyCode::Backspace,
            CK::Tab => KeyCode::Tab,
            CK::BackTab => KeyCode::BackTab,
            CK::Delete => KeyCode::Delete,
            CK::Up => KeyCode::Up,
            CK::Down => KeyCode::Down,
            CK::Left => KeyCode::Left,
            CK::Right => KeyCode::Right,
            CK::Home => KeyCode::Home,
            CK::End => KeyCode::End,
            CK::PageUp => KeyCode::PageUp,
            CK::PageDown => KeyCode::PageDown,
            CK::F(n) => KeyCode::F(n),
            _ => return None,
        };

        Some(EditorEvent::Key(KeyEvent { code, modifiers }))
    }
}

impl Default for InputDecoder {
    fn default() -> Self {
        Self::new()
    }
}
