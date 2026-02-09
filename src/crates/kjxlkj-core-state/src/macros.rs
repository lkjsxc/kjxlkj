//! Macro recording and playback for EditorState.
//!
//! `q{a-z}` starts recording into a register.
//! `q` stops recording.
//! `@{a-z}` plays back the macro stored in a register.
//! `@@` replays the last executed macro.

use kjxlkj_core_types::{Key, KeyCode, Modifier};

use crate::editor::EditorState;

impl EditorState {
    /// Start recording keystrokes into the given register.
    /// Lowercase: overwrite. Uppercase: append to lowercase reg.
    pub(crate) fn start_recording(&mut self, reg: char) {
        if reg.is_ascii_uppercase() {
            // Append mode: record into lowercase key.
            self.recording_macro = Some(reg);
            self.macro_buffer.clear();
            return;
        }
        let reg = reg.to_ascii_lowercase();
        if !reg.is_ascii_lowercase() {
            return;
        }
        self.recording_macro = Some(reg);
        self.macro_buffer.clear();
    }

    /// Stop recording and store the macro.
    pub(crate) fn stop_recording(&mut self) {
        if let Some(reg) = self.recording_macro.take() {
            let keys = std::mem::take(&mut self.macro_buffer);
            // Convert keys to string for register unification.
            let text: String = keys.iter().map(macro_key_to_char).collect();
            if reg.is_ascii_uppercase() {
                let lower = reg.to_ascii_lowercase();
                self.macro_store.entry(lower).or_default().extend(keys);
                // Append text to register.
                use kjxlkj_core_edit::{Register, RegisterName};
                if let Some(existing) = self.registers.get(RegisterName::Named(lower)) {
                    let mut s = existing.content.clone();
                    s.push_str(&text);
                    self.registers
                        .set(RegisterName::Named(lower), Register::new(s, false));
                } else {
                    self.registers
                        .set(RegisterName::Named(lower), Register::new(text, false));
                }
            } else {
                self.macro_store.insert(reg, keys);
                use kjxlkj_core_edit::{Register, RegisterName};
                self.registers
                    .set(RegisterName::Named(reg), Register::new(text, false));
            }
        }
    }

    /// Record a key during macro recording.
    /// Called for every key while recording EXCEPT the
    /// final `q` that stops recording.
    pub(crate) fn record_key(&mut self, key: &Key) {
        if self.recording_macro.is_some() {
            self.macro_buffer.push(key.clone());
        }
    }

    /// Whether we are currently recording a macro.
    pub(crate) fn is_recording(&self) -> bool {
        self.recording_macro.is_some()
    }

    /// Play back the macro stored in the given register.
    /// `count` is the number of times to replay.
    /// Enforces a maximum recursion depth of 100.
    pub(crate) fn play_macro(&mut self, reg: char, count: usize) {
        const MAX_MACRO_DEPTH: usize = 100;
        if self.macro_depth >= MAX_MACRO_DEPTH {
            return;
        }
        let reg = if reg == '@' {
            match self.last_macro {
                Some(r) => r,
                None => return,
            }
        } else {
            reg.to_ascii_lowercase()
        };
        let keys = match self.macro_store.get(&reg) {
            Some(k) => k.clone(),
            None => return,
        };
        self.last_macro = Some(reg);
        self.macro_depth += 1;
        self.macro_error = false;
        for _ in 0..count {
            if self.macro_depth > MAX_MACRO_DEPTH || self.macro_error {
                break;
            }
            for key in &keys {
                if self.macro_error {
                    break;
                }
                self.handle_key(key.clone());
            }
        }
        self.macro_depth -= 1;
    }

    /// Check if a key is the bare `q` key (no modifiers).
    pub(crate) fn is_q_key(key: &Key) -> bool {
        key.modifiers == Modifier::NONE && matches!(key.code, KeyCode::Char('q'))
    }

    /// Sync macro store → register: put macro as text into named register.
    #[allow(dead_code)]
    pub(crate) fn sync_macro_to_register(&mut self, reg: char) {
        let r = reg.to_ascii_lowercase();
        if let Some(keys) = self.macro_store.get(&r) {
            let text: String = keys.iter().map(macro_key_to_char).collect();
            self.registers.set(
                kjxlkj_core_edit::RegisterName::Named(r),
                kjxlkj_core_edit::Register::new(text, false),
            );
        }
    }

    /// Sync register → macro store: load text from named register as macro keys.
    #[allow(dead_code)]
    pub(crate) fn sync_register_to_macro(&mut self, reg: char) {
        let r = reg.to_ascii_lowercase();
        let text = self
            .registers
            .get(kjxlkj_core_edit::RegisterName::Named(r))
            .map(|reg| reg.content.clone());
        if let Some(text) = text {
            let keys: Vec<Key> = text.chars().map(|c| match c {
                '\n' => Key::new(KeyCode::Enter, Modifier::NONE),
                '\x1b' => Key::esc(),
                c => Key::char(c),
            }).collect();
            self.macro_store.insert(r, keys);
        }
    }
}

fn macro_key_to_char(key: &Key) -> char {
    match &key.code {
        KeyCode::Char(c) => *c,
        KeyCode::Enter => '\n',
        KeyCode::Esc => '\x1b',
        _ => '?',
    }
}
