//! IME key routing: maps keys to IME actions during composition.

use kjxlkj_core_types::KeyCode;

use crate::ime::{ImeComposition, ImeState};

/// Result of routing a key through the IME model.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ImeResult {
    /// IME consumed the key; no downstream dispatch.
    Consumed,
    /// IME committed text; insert this string.
    Commit(String),
    /// IME cancelled; no text to insert.
    Cancelled,
    /// IME is idle; pass the key to normal dispatch.
    PassThrough,
}

/// Route a key through the IME composition model.
///
/// Returns an `ImeResult` indicating how the key was handled.
/// This MUST be called before leader/normal-mode dispatch to ensure
/// Space doesn't trigger leader during composition.
pub fn route_ime_key(ime: &mut ImeComposition, code: &KeyCode) -> ImeResult {
    if !ime.is_composing() {
        return ImeResult::PassThrough;
    }

    match code {
        KeyCode::Esc => {
            ime.cancel();
            ImeResult::Cancelled
        }
        KeyCode::Enter => {
            let text = ime.commit();
            ImeResult::Commit(text)
        }
        KeyCode::Backspace => {
            if !ime.backspace_preedit() {
                ime.cancel();
                ImeResult::Cancelled
            } else {
                ImeResult::Consumed
            }
        }
        KeyCode::Char(' ') => {
            if ime.state == ImeState::CandidateSelect {
                ime.next_candidate();
            }
            // Space during composition MUST NOT trigger leader
            ImeResult::Consumed
        }
        KeyCode::Char(c) => {
            if ime.state == ImeState::Preedit {
                ime.feed_preedit(*c);
            }
            ImeResult::Consumed
        }
        _ => ImeResult::Consumed,
    }
}
