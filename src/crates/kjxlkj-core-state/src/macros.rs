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
            if reg.is_ascii_uppercase() {
                let lower = reg.to_ascii_lowercase();
                self.macro_store.entry(lower).or_default().extend(keys);
            } else {
                self.macro_store.insert(reg, keys);
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
    pub(crate) fn play_macro(&mut self, reg: char, count: usize) {
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
        for _ in 0..count {
            for key in &keys {
                self.handle_key(key.clone());
            }
        }
    }

    /// Check if a key is the bare `q` key (no modifiers).
    pub(crate) fn is_q_key(key: &Key) -> bool {
        key.modifiers == Modifier::NONE && matches!(key.code, KeyCode::Char('q'))
    }
}
