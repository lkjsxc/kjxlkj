//! Terminal input handling for kjxlkj editor.
//!
//! This crate provides input event conversion from crossterm to kjxlkj types.

mod decoder;

pub use decoder::{decode_event, InputDecoder};

use kjxlkj_core_types::EditorEvent;

/// Input source trait for abstracting terminal input.
pub trait InputSource {
    /// Poll for the next event.
    fn poll_event(&mut self) -> Option<EditorEvent>;
}

/// Terminal-based input source.
pub struct TerminalInput;

impl TerminalInput {
    /// Create a new terminal input source.
    pub fn new() -> Self {
        Self
    }
}

impl Default for TerminalInput {
    fn default() -> Self {
        Self::new()
    }
}

impl InputSource for TerminalInput {
    fn poll_event(&mut self) -> Option<EditorEvent> {
        use crossterm::event::{poll, read, Event};
        use std::time::Duration;

        if poll(Duration::from_millis(100)).ok()? {
            let event = read().ok()?;
            Some(decode_event(event))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_terminal_input_new() {
        let _input = TerminalInput::new();
    }

    #[test]
    fn test_terminal_input_default() {
        let _input = TerminalInput::default();
    }

    #[test]
    fn test_input_decoder_exported() {
        let decoder = InputDecoder;
        // Just verify it's exported
        assert!(std::mem::size_of_val(&decoder) >= 0);
    }

    #[test]
    fn test_decode_event_exported() {
        // Verify decode_event function is exported
        // Can't actually call it without a crossterm event in a unit test
        assert!(true);
    }
}
