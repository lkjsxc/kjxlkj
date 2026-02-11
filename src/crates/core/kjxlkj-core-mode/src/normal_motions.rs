//! Normal-mode motion key mappings.
//!
//! Extracted from normal.rs to keep each file ≤ 200 lines.
//! See /docs/spec/editing/motions/motions.md for the full table.

use kjxlkj_core_types::{Action, Key, Motion};

/// Map a normal-mode key to a motion action, if applicable.
pub(crate) fn motion_for_key(key: &Key) -> Option<Action> {
    match key {
        // Character motions.
        Key::Char('h') | Key::Left => {
            Some(Action::Motion(Motion::Left))
        }
        Key::Char('l') | Key::Right => {
            Some(Action::Motion(Motion::Right))
        }
        Key::Char('k') | Key::Up => {
            Some(Action::Motion(Motion::Up))
        }
        Key::Char('j') | Key::Down => {
            Some(Action::Motion(Motion::Down))
        }
        // Line motions.
        Key::Char('0') => {
            Some(Action::Motion(Motion::LineStart))
        }
        Key::Char('$') => {
            Some(Action::Motion(Motion::LineEnd))
        }
        Key::Char('^') => {
            Some(Action::Motion(Motion::FirstNonBlank))
        }
        Key::Char('+') | Key::Enter => {
            Some(Action::Motion(Motion::Down))
        }
        Key::Char('-') => {
            Some(Action::Motion(Motion::Up))
        }
        // Word/WORD motions.
        Key::Char('w') => {
            Some(Action::Motion(Motion::WordForward))
        }
        Key::Char('b') => {
            Some(Action::Motion(Motion::WordBackward))
        }
        Key::Char('e') => {
            Some(Action::Motion(Motion::WordEndForward))
        }
        Key::Char('W') => {
            Some(Action::Motion(Motion::BigWordForward))
        }
        Key::Char('B') => {
            Some(Action::Motion(Motion::BigWordBackward))
        }
        Key::Char('E') => {
            Some(Action::Motion(Motion::BigWordEndForward))
        }
        // Document motions.
        Key::Char('G') => {
            Some(Action::Motion(Motion::GotoLastLine))
        }
        // Sentence/paragraph motions.
        Key::Char('(') => {
            Some(Action::Motion(Motion::SentenceBackward))
        }
        Key::Char(')') => {
            Some(Action::Motion(Motion::SentenceForward))
        }
        Key::Char('{') => {
            Some(Action::Motion(Motion::ParagraphBackward))
        }
        Key::Char('}') => {
            Some(Action::Motion(Motion::ParagraphForward))
        }
        // Match paren.
        Key::Char('%') => {
            Some(Action::Motion(Motion::MatchParen))
        }
        // Window motions.
        Key::Char('H') => {
            Some(Action::Motion(Motion::WindowTop))
        }
        Key::Char('M') => {
            Some(Action::Motion(Motion::WindowMiddle))
        }
        Key::Char('L') => {
            Some(Action::Motion(Motion::WindowBottom))
        }
        // Page keys.
        Key::PageDown => {
            Some(Action::Motion(Motion::PageDown))
        }
        Key::PageUp => {
            Some(Action::Motion(Motion::PageUp))
        }
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn h_maps_to_left() {
        assert_eq!(
            motion_for_key(&Key::Char('h')),
            Some(Action::Motion(Motion::Left)),
        );
    }

    #[test]
    fn dollar_maps_to_line_end() {
        assert_eq!(
            motion_for_key(&Key::Char('$')),
            Some(Action::Motion(Motion::LineEnd)),
        );
    }

    #[test]
    fn open_paren_maps_to_sentence_backward() {
        assert_eq!(
            motion_for_key(&Key::Char('(')),
            Some(Action::Motion(Motion::SentenceBackward)),
        );
    }

    #[test]
    fn unknown_key_returns_none() {
        assert_eq!(motion_for_key(&Key::Char('q')), None);
    }
}
