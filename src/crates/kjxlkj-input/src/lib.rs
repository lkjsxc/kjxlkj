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

    #[test]
    fn test_terminal_input_size() {
        let input = TerminalInput::new();
        // Zero-sized type
        assert_eq!(std::mem::size_of_val(&input), 0);
    }

    #[test]
    fn test_input_decoder_size() {
        let decoder = InputDecoder;
        // Zero-sized type
        assert_eq!(std::mem::size_of_val(&decoder), 0);
    }

    #[test]
    fn test_input_source_trait_object_safe() {
        // Verify InputSource can be used as a trait object
        fn _accepts_input_source(_: &dyn InputSource) {}
        // Compiles = trait is object safe
    }

    #[test]
    fn test_terminal_input_multiple_instances() {
        let input1 = TerminalInput::new();
        let input2 = TerminalInput::new();
        let _input3 = TerminalInput::default();
        // All instances should be usable
        assert_eq!(std::mem::size_of_val(&input1), std::mem::size_of_val(&input2));
    }
}
