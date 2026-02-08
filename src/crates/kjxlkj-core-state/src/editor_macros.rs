//! Macro recording and playback for EditorState.
//!
//! Implements `q{reg}` to record, `q` to stop, `@{reg}`
//! to play, and `@@` to replay the last macro.

use kjxlkj_core_types::{Key, RegisterName};

use crate::EditorState;

impl EditorState {
    /// Start recording keystrokes into register `reg`.
    pub(crate) fn do_record_macro(
        &mut self,
        reg: char,
    ) {
        if self.macro_recording.is_some() {
            self.do_stop_record_macro();
        }
        self.macro_recording = Some(reg);
        self.macro_keys.clear();
    }

    /// Stop recording and store keys in register.
    pub(crate) fn do_stop_record_macro(&mut self) {
        if let Some(reg) = self.macro_recording.take() {
            let content = keys_to_string(&self.macro_keys);
            self.register_file.store(
                RegisterName::Named(reg),
                content,
                false,
            );
            self.register_file.set_last_macro(reg);
            self.macro_keys.clear();
        }
    }

    /// Record a key if macro recording is active.
    pub(crate) fn record_key_if_needed(
        &mut self,
        key: &Key,
    ) {
        if self.macro_recording.is_some() {
            self.macro_keys.push(key.clone());
        }
    }

    /// Play a macro from register `reg`, `count` times.
    pub(crate) fn do_play_macro(
        &mut self,
        reg: char,
        count: u32,
    ) {
        let name = RegisterName::Named(reg);
        let content = match self.register_file.get(name) {
            Some(r) => r.content.clone(),
            None => return,
        };
        self.register_file.set_last_macro(reg);

        let keys = string_to_keys(&content);
        for _ in 0..count {
            for key in &keys {
                self.dispatch_key(key.clone());
            }
        }
    }
}

/// Encode a key sequence to a storable string.
fn keys_to_string(keys: &[Key]) -> String {
    use kjxlkj_core_types::KeyCode;
    let mut s = String::new();
    for k in keys {
        match &k.code {
            KeyCode::Char(c) => s.push(*c),
            KeyCode::Enter => s.push_str("<CR>"),
            KeyCode::Esc => s.push_str("<Esc>"),
            KeyCode::Backspace => s.push_str("<BS>"),
            KeyCode::Tab => s.push_str("<Tab>"),
            _ => s.push_str("<?>"),
        }
    }
    s
}

/// Decode a stored string back to keys.
fn string_to_keys(s: &str) -> Vec<Key> {
    let mut keys = Vec::new();
    let mut chars = s.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '<' {
            let mut name = String::new();
            for nc in chars.by_ref() {
                if nc == '>' {
                    break;
                }
                name.push(nc);
            }
            match name.as_str() {
                "CR" => keys.push(Key::enter()),
                "Esc" => keys.push(Key::esc()),
                "BS" => keys.push(Key::backspace()),
                "Tab" => keys.push(Key::tab()),
                _ => {}
            }
        } else {
            keys.push(Key::char(c));
        }
    }
    keys
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn record_and_play_macro() {
        let mut ed = EditorState::new(80, 24);
        ed.do_record_macro('a');
        assert!(ed.macro_recording.is_some());
        ed.record_key_if_needed(&Key::char('i'));
        ed.record_key_if_needed(&Key::char('x'));
        ed.record_key_if_needed(&Key::esc());
        ed.do_stop_record_macro();
        assert!(ed.macro_recording.is_none());
        let reg = ed.register_file.get(
            RegisterName::Named('a'),
        );
        assert!(reg.is_some());
    }

    #[test]
    fn keys_roundtrip() {
        let keys = vec![
            Key::char('a'),
            Key::enter(),
            Key::esc(),
        ];
        let s = keys_to_string(&keys);
        let back = string_to_keys(&s);
        assert_eq!(back.len(), 3);
    }
}
