//! Input event decoding from crossterm to kjxlkj types.

use crossterm::event::{Event, KeyCode as CTKeyCode, KeyEvent as CTKeyEvent, KeyModifiers as CTMods};
use kjxlkj_core_types::{EditorEvent, KeyCode, KeyEvent, KeyModifiers};

/// Input decoder for crossterm events.
pub struct InputDecoder;

impl InputDecoder {
    /// Create a new input decoder.
    pub fn new() -> Self {
        Self
    }

    /// Decode a crossterm event.
    pub fn decode(&self, event: Event) -> EditorEvent {
        decode_event(event)
    }
}

impl Default for InputDecoder {
    fn default() -> Self {
        Self::new()
    }
}

/// Decode a crossterm event into an editor event.
pub fn decode_event(event: Event) -> EditorEvent {
    match event {
        Event::Key(key_event) => EditorEvent::Key(decode_key(key_event)),
        Event::Resize(width, height) => EditorEvent::Resize { width, height },
        Event::FocusGained => EditorEvent::Focus(true),
        Event::FocusLost => EditorEvent::Focus(false),
        Event::Mouse(_) => EditorEvent::Ignored,
        Event::Paste(text) => EditorEvent::Paste(text),
    }
}

/// Decode a crossterm key event.
fn decode_key(event: CTKeyEvent) -> KeyEvent {
    let code = decode_key_code(event.code);
    let modifiers = decode_modifiers(event.modifiers);
    KeyEvent { code, modifiers }
}

/// Decode crossterm key code.
fn decode_key_code(code: CTKeyCode) -> KeyCode {
    match code {
        CTKeyCode::Char(c) => KeyCode::Char(c),
        CTKeyCode::Backspace => KeyCode::Backspace,
        CTKeyCode::Enter => KeyCode::Enter,
        CTKeyCode::Left => KeyCode::Left,
        CTKeyCode::Right => KeyCode::Right,
        CTKeyCode::Up => KeyCode::Up,
        CTKeyCode::Down => KeyCode::Down,
        CTKeyCode::Home => KeyCode::Home,
        CTKeyCode::End => KeyCode::End,
        CTKeyCode::PageUp => KeyCode::PageUp,
        CTKeyCode::PageDown => KeyCode::PageDown,
        CTKeyCode::Tab => KeyCode::Tab,
        CTKeyCode::BackTab => KeyCode::BackTab,
        CTKeyCode::Delete => KeyCode::Delete,
        CTKeyCode::Insert => KeyCode::Insert,
        CTKeyCode::Esc => KeyCode::Escape,
        CTKeyCode::F(n) => KeyCode::F(n),
        CTKeyCode::Null => KeyCode::Null,
        CTKeyCode::CapsLock => KeyCode::CapsLock,
        CTKeyCode::ScrollLock => KeyCode::ScrollLock,
        CTKeyCode::NumLock => KeyCode::NumLock,
        CTKeyCode::PrintScreen => KeyCode::PrintScreen,
        CTKeyCode::Pause => KeyCode::Pause,
        CTKeyCode::Menu => KeyCode::Menu,
        CTKeyCode::KeypadBegin => KeyCode::KeypadBegin,
        CTKeyCode::Media(media) => KeyCode::Media(format!("{:?}", media)),
        CTKeyCode::Modifier(modifier) => KeyCode::Modifier(format!("{:?}", modifier)),
    }
}

/// Decode crossterm key modifiers.
fn decode_modifiers(mods: CTMods) -> KeyModifiers {
    KeyModifiers {
        ctrl: mods.contains(CTMods::CONTROL),
        alt: mods.contains(CTMods::ALT),
        shift: mods.contains(CTMods::SHIFT),
        meta: mods.contains(CTMods::META) || mods.contains(CTMods::SUPER),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode_char() {
        let ct_event = CTKeyEvent::new(CTKeyCode::Char('a'), CTMods::NONE);
        let decoded = decode_key(ct_event);
        assert_eq!(decoded.code, KeyCode::Char('a'));
        assert!(!decoded.modifiers.ctrl);
    }

    #[test]
    fn test_decode_ctrl() {
        let ct_event = CTKeyEvent::new(CTKeyCode::Char('c'), CTMods::CONTROL);
        let decoded = decode_key(ct_event);
        assert_eq!(decoded.code, KeyCode::Char('c'));
        assert!(decoded.modifiers.ctrl);
    }

    #[test]
    fn test_decode_escape() {
        let ct_event = CTKeyEvent::new(CTKeyCode::Esc, CTMods::NONE);
        let decoded = decode_key(ct_event);
        assert_eq!(decoded.code, KeyCode::Escape);
    }

    #[test]
    fn test_decode_arrow() {
        let ct_event = CTKeyEvent::new(CTKeyCode::Up, CTMods::NONE);
        let decoded = decode_key(ct_event);
        assert_eq!(decoded.code, KeyCode::Up);
    }

    #[test]
    fn test_decode_resize() {
        let event = Event::Resize(120, 40);
        let decoded = decode_event(event);
        assert!(matches!(decoded, EditorEvent::Resize { width: 120, height: 40 }));
    }

    #[test]
    fn test_decode_focus() {
        let event = Event::FocusGained;
        let decoded = decode_event(event);
        assert!(matches!(decoded, EditorEvent::Focus(true)));
    }

    #[test]
    fn test_decode_focus_lost() {
        let event = Event::FocusLost;
        let decoded = decode_event(event);
        assert!(matches!(decoded, EditorEvent::Focus(false)));
    }

    #[test]
    fn test_decode_alt_modifier() {
        let ct_event = CTKeyEvent::new(CTKeyCode::Char('x'), CTMods::ALT);
        let decoded = decode_key(ct_event);
        assert!(decoded.modifiers.alt);
        assert!(!decoded.modifiers.ctrl);
    }

    #[test]
    fn test_decode_shift_modifier() {
        let ct_event = CTKeyEvent::new(CTKeyCode::Char('A'), CTMods::SHIFT);
        let decoded = decode_key(ct_event);
        assert!(decoded.modifiers.shift);
    }

    #[test]
    fn test_decode_ctrl_alt() {
        let mods = CTMods::CONTROL | CTMods::ALT;
        let ct_event = CTKeyEvent::new(CTKeyCode::Char('c'), mods);
        let decoded = decode_key(ct_event);
        assert!(decoded.modifiers.ctrl);
        assert!(decoded.modifiers.alt);
    }

    #[test]
    fn test_decode_backspace() {
        let ct_event = CTKeyEvent::new(CTKeyCode::Backspace, CTMods::NONE);
        let decoded = decode_key(ct_event);
        assert_eq!(decoded.code, KeyCode::Backspace);
    }

    #[test]
    fn test_decode_enter() {
        let ct_event = CTKeyEvent::new(CTKeyCode::Enter, CTMods::NONE);
        let decoded = decode_key(ct_event);
        assert_eq!(decoded.code, KeyCode::Enter);
    }

    #[test]
    fn test_decode_tab() {
        let ct_event = CTKeyEvent::new(CTKeyCode::Tab, CTMods::NONE);
        let decoded = decode_key(ct_event);
        assert_eq!(decoded.code, KeyCode::Tab);
    }

    #[test]
    fn test_decode_function_key() {
        let ct_event = CTKeyEvent::new(CTKeyCode::F(5), CTMods::NONE);
        let decoded = decode_key(ct_event);
        assert_eq!(decoded.code, KeyCode::F(5));
    }

    #[test]
    fn test_decode_page_up() {
        let ct_event = CTKeyEvent::new(CTKeyCode::PageUp, CTMods::NONE);
        let decoded = decode_key(ct_event);
        assert_eq!(decoded.code, KeyCode::PageUp);
    }

    #[test]
    fn test_decode_page_down() {
        let ct_event = CTKeyEvent::new(CTKeyCode::PageDown, CTMods::NONE);
        let decoded = decode_key(ct_event);
        assert_eq!(decoded.code, KeyCode::PageDown);
    }

    #[test]
    fn test_decode_home() {
        let ct_event = CTKeyEvent::new(CTKeyCode::Home, CTMods::NONE);
        let decoded = decode_key(ct_event);
        assert_eq!(decoded.code, KeyCode::Home);
    }

    #[test]
    fn test_decode_end() {
        let ct_event = CTKeyEvent::new(CTKeyCode::End, CTMods::NONE);
        let decoded = decode_key(ct_event);
        assert_eq!(decoded.code, KeyCode::End);
    }

    #[test]
    fn test_decode_delete() {
        let ct_event = CTKeyEvent::new(CTKeyCode::Delete, CTMods::NONE);
        let decoded = decode_key(ct_event);
        assert_eq!(decoded.code, KeyCode::Delete);
    }

    #[test]
    fn test_decode_insert() {
        let ct_event = CTKeyEvent::new(CTKeyCode::Insert, CTMods::NONE);
        let decoded = decode_key(ct_event);
        assert_eq!(decoded.code, KeyCode::Insert);
    }

    #[test]
    fn test_decode_paste() {
        let event = Event::Paste("pasted text".to_string());
        let decoded = decode_event(event);
        assert!(matches!(decoded, EditorEvent::Paste(_)));
    }

    #[test]
    fn test_input_decoder_struct() {
        let decoder = InputDecoder::new();
        let event = Event::Key(CTKeyEvent::new(CTKeyCode::Char('a'), CTMods::NONE));
        let decoded = decoder.decode(event);
        assert!(matches!(decoded, EditorEvent::Key(_)));
    }

    #[test]
    fn test_input_decoder_default() {
        let decoder = InputDecoder::default();
        let event = Event::FocusGained;
        let decoded = decoder.decode(event);
        assert!(matches!(decoded, EditorEvent::Focus(true)));
    }

    #[test]
    fn test_decode_left() {
        let ct_event = CTKeyEvent::new(CTKeyCode::Left, CTMods::NONE);
        let decoded = decode_key(ct_event);
        assert_eq!(decoded.code, KeyCode::Left);
    }

    #[test]
    fn test_decode_right() {
        let ct_event = CTKeyEvent::new(CTKeyCode::Right, CTMods::NONE);
        let decoded = decode_key(ct_event);
        assert_eq!(decoded.code, KeyCode::Right);
    }

    #[test]
    fn test_decode_down() {
        let ct_event = CTKeyEvent::new(CTKeyCode::Down, CTMods::NONE);
        let decoded = decode_key(ct_event);
        assert_eq!(decoded.code, KeyCode::Down);
    }

    #[test]
    fn test_input_decoder_new() {
        let decoder = InputDecoder::new();
        let event = Event::Key(CTKeyEvent::new(CTKeyCode::Char('a'), CTMods::NONE));
        let decoded = decoder.decode(event);
        assert!(matches!(decoded, EditorEvent::Key(_)));
    }

    #[test]
    fn test_ctrl_alt_modifiers() {
        let ct_event = CTKeyEvent::new(CTKeyCode::Char('x'), CTMods::CONTROL | CTMods::ALT);
        let decoded = decode_key(ct_event);
        assert!(decoded.modifiers.ctrl);
        assert!(decoded.modifiers.alt);
    }

    #[test]
    fn test_decode_null() {
        let ct_event = CTKeyEvent::new(CTKeyCode::Null, CTMods::NONE);
        let decoded = decode_key(ct_event);
        assert_eq!(decoded.code, KeyCode::Null);
    }

    #[test]
    fn test_decode_backtab() {
        let ct_event = CTKeyEvent::new(CTKeyCode::BackTab, CTMods::NONE);
        let decoded = decode_key(ct_event);
        assert_eq!(decoded.code, KeyCode::BackTab);
    }

    #[test]
    fn test_decode_caps_lock() {
        let ct_event = CTKeyEvent::new(CTKeyCode::CapsLock, CTMods::NONE);
        let decoded = decode_key(ct_event);
        assert_eq!(decoded.code, KeyCode::CapsLock);
    }

    #[test]
    fn test_decode_scroll_lock() {
        let ct_event = CTKeyEvent::new(CTKeyCode::ScrollLock, CTMods::NONE);
        let decoded = decode_key(ct_event);
        assert_eq!(decoded.code, KeyCode::ScrollLock);
    }

    #[test]
    fn test_decode_num_lock() {
        let ct_event = CTKeyEvent::new(CTKeyCode::NumLock, CTMods::NONE);
        let decoded = decode_key(ct_event);
        assert_eq!(decoded.code, KeyCode::NumLock);
    }

    #[test]
    fn test_decode_print_screen() {
        let ct_event = CTKeyEvent::new(CTKeyCode::PrintScreen, CTMods::NONE);
        let decoded = decode_key(ct_event);
        assert_eq!(decoded.code, KeyCode::PrintScreen);
    }

    #[test]
    fn test_decode_pause() {
        let ct_event = CTKeyEvent::new(CTKeyCode::Pause, CTMods::NONE);
        let decoded = decode_key(ct_event);
        assert_eq!(decoded.code, KeyCode::Pause);
    }

    #[test]
    fn test_decode_menu() {
        let ct_event = CTKeyEvent::new(CTKeyCode::Menu, CTMods::NONE);
        let decoded = decode_key(ct_event);
        assert_eq!(decoded.code, KeyCode::Menu);
    }

    #[test]
    fn test_decode_keypad_begin() {
        let ct_event = CTKeyEvent::new(CTKeyCode::KeypadBegin, CTMods::NONE);
        let decoded = decode_key(ct_event);
        assert_eq!(decoded.code, KeyCode::KeypadBegin);
    }

    #[test]
    fn test_decode_meta_modifier() {
        let ct_event = CTKeyEvent::new(CTKeyCode::Char('m'), CTMods::META);
        let decoded = decode_key(ct_event);
        assert!(decoded.modifiers.meta);
    }

    #[test]
    fn test_decode_super_as_meta() {
        let ct_event = CTKeyEvent::new(CTKeyCode::Char('s'), CTMods::SUPER);
        let decoded = decode_key(ct_event);
        assert!(decoded.modifiers.meta);
    }

    #[test]
    fn test_decode_all_modifiers() {
        let mods = CTMods::CONTROL | CTMods::ALT | CTMods::SHIFT | CTMods::META;
        let ct_event = CTKeyEvent::new(CTKeyCode::Char('a'), mods);
        let decoded = decode_key(ct_event);
        assert!(decoded.modifiers.ctrl);
        assert!(decoded.modifiers.alt);
        assert!(decoded.modifiers.shift);
        assert!(decoded.modifiers.meta);
    }

    #[test]
    fn test_decode_f1_through_f12() {
        for n in 1..=12 {
            let ct_event = CTKeyEvent::new(CTKeyCode::F(n), CTMods::NONE);
            let decoded = decode_key(ct_event);
            assert_eq!(decoded.code, KeyCode::F(n));
        }
    }

    #[test]
    fn test_decode_uppercase_char() {
        let ct_event = CTKeyEvent::new(CTKeyCode::Char('Z'), CTMods::SHIFT);
        let decoded = decode_key(ct_event);
        assert_eq!(decoded.code, KeyCode::Char('Z'));
        assert!(decoded.modifiers.shift);
    }

    #[test]
    fn test_decode_digit_char() {
        let ct_event = CTKeyEvent::new(CTKeyCode::Char('5'), CTMods::NONE);
        let decoded = decode_key(ct_event);
        assert_eq!(decoded.code, KeyCode::Char('5'));
    }

    #[test]
    fn test_decode_special_char() {
        let ct_event = CTKeyEvent::new(CTKeyCode::Char('@'), CTMods::SHIFT);
        let decoded = decode_key(ct_event);
        assert_eq!(decoded.code, KeyCode::Char('@'));
    }

    #[test]
    fn test_decode_space_char() {
        let ct_event = CTKeyEvent::new(CTKeyCode::Char(' '), CTMods::NONE);
        let decoded = decode_key(ct_event);
        assert_eq!(decoded.code, KeyCode::Char(' '));
    }

    #[test]
    fn test_decode_unicode_char() {
        let ct_event = CTKeyEvent::new(CTKeyCode::Char('ñ'), CTMods::NONE);
        let decoded = decode_key(ct_event);
        assert_eq!(decoded.code, KeyCode::Char('ñ'));
    }

    #[test]
    fn test_decode_ctrl_shift() {
        let mods = CTMods::CONTROL | CTMods::SHIFT;
        let ct_event = CTKeyEvent::new(CTKeyCode::Char('A'), mods);
        let decoded = decode_key(ct_event);
        assert!(decoded.modifiers.ctrl);
        assert!(decoded.modifiers.shift);
    }
}
