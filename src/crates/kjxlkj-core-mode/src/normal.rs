//! Normal mode key handling.

use kjxlkj_core_types::{CursorDirection, Intent, Key, KeyCode, Mode};

use crate::{ModeState, PendingOperator};

/// Handle a key in normal mode.
pub fn handle_normal_key(state: &mut ModeState, key: Key) -> Vec<Intent> {
    let mut intents = Vec::new();

    match key.code {
        // Count accumulation
        KeyCode::Char(c @ '1'..='9') => {
            state.accumulate_count(c as u32 - '0' as u32);
        }
        KeyCode::Char('0') if state.count.is_some() => {
            state.accumulate_count(0);
        }

        // Movement
        KeyCode::Char('h') | KeyCode::Left => {
            let count = state.effective_count();
            for _ in 0..count {
                intents.push(Intent::CursorMove(CursorDirection::Left));
            }
            state.clear_pending();
        }
        KeyCode::Char('j') | KeyCode::Down => {
            let count = state.effective_count();
            for _ in 0..count {
                intents.push(Intent::CursorMove(CursorDirection::Down));
            }
            state.clear_pending();
        }
        KeyCode::Char('k') | KeyCode::Up => {
            let count = state.effective_count();
            for _ in 0..count {
                intents.push(Intent::CursorMove(CursorDirection::Up));
            }
            state.clear_pending();
        }
        KeyCode::Char('l') | KeyCode::Right => {
            let count = state.effective_count();
            for _ in 0..count {
                intents.push(Intent::CursorMove(CursorDirection::Right));
            }
            state.clear_pending();
        }

        // Word motions
        KeyCode::Char('w') => {
            let count = state.effective_count();
            for _ in 0..count {
                intents.push(Intent::CursorMove(CursorDirection::WordForward));
            }
            state.clear_pending();
        }
        KeyCode::Char('b') => {
            let count = state.effective_count();
            for _ in 0..count {
                intents.push(Intent::CursorMove(CursorDirection::WordBackward));
            }
            state.clear_pending();
        }
        KeyCode::Char('e') => {
            let count = state.effective_count();
            for _ in 0..count {
                intents.push(Intent::CursorMove(CursorDirection::WordEndForward));
            }
            state.clear_pending();
        }

        // Line motions
        KeyCode::Char('0') => {
            intents.push(Intent::CursorLineStart);
            state.clear_pending();
        }
        KeyCode::Char('^') => {
            intents.push(Intent::CursorFirstNonBlank);
            state.clear_pending();
        }
        KeyCode::Char('$') => {
            intents.push(Intent::CursorLineEnd);
            state.clear_pending();
        }

        // File motions
        KeyCode::Char('g') if state.pending_operator.is_none() => {
            // Start of gg
            state.pending_operator = Some(PendingOperator::Delete); // Reuse as marker
            // Actually we need a separate mechanism - simplified for now
            intents.push(Intent::CursorFileStart);
            state.clear_pending();
        }
        KeyCode::Char('G') => {
            if let Some(count) = state.count {
                intents.push(Intent::CursorGotoLine(count.saturating_sub(1)));
            } else {
                intents.push(Intent::CursorFileEnd);
            }
            state.clear_pending();
        }

        // Mode entry
        KeyCode::Char('i') => {
            intents.push(Intent::EnterMode(Mode::Insert));
            state.enter_mode(Mode::Insert);
        }
        KeyCode::Char('a') => {
            intents.push(Intent::CursorMove(CursorDirection::Right));
            intents.push(Intent::EnterMode(Mode::Insert));
            state.enter_mode(Mode::Insert);
        }
        KeyCode::Char('A') => {
            intents.push(Intent::CursorLineEnd);
            intents.push(Intent::CursorMove(CursorDirection::Right));
            intents.push(Intent::EnterMode(Mode::Insert));
            state.enter_mode(Mode::Insert);
        }
        KeyCode::Char('o') => {
            intents.push(Intent::InsertNewline);
            intents.push(Intent::EnterMode(Mode::Insert));
            state.enter_mode(Mode::Insert);
        }
        KeyCode::Char('v') => {
            intents.push(Intent::EnterMode(Mode::Visual));
            state.enter_mode(Mode::Visual);
        }
        KeyCode::Char('V') => {
            intents.push(Intent::EnterMode(Mode::VisualLine));
            state.enter_mode(Mode::VisualLine);
        }
        KeyCode::Char('R') => {
            intents.push(Intent::EnterMode(Mode::Replace));
            state.enter_mode(Mode::Replace);
        }
        KeyCode::Char(':') => {
            intents.push(Intent::EnterMode(Mode::Command));
            state.enter_mode(Mode::Command);
        }

        // Operators
        KeyCode::Char('d') => {
            if state.pending_operator == Some(PendingOperator::Delete) {
                // dd - delete line
                intents.push(Intent::DeleteLine);
                state.clear_pending();
            } else {
                state.pending_operator = Some(PendingOperator::Delete);
            }
        }
        KeyCode::Char('y') => {
            if state.pending_operator == Some(PendingOperator::Yank) {
                // yy - yank line
                intents.push(Intent::YankLine);
                state.clear_pending();
            } else {
                state.pending_operator = Some(PendingOperator::Yank);
            }
        }

        // Single-key edits
        KeyCode::Char('x') => {
            intents.push(Intent::DeleteChar);
            state.clear_pending();
        }
        KeyCode::Char('p') => {
            intents.push(Intent::PasteAfter);
            state.clear_pending();
        }
        KeyCode::Char('P') => {
            intents.push(Intent::PasteBefore);
            state.clear_pending();
        }

        // Undo/Redo
        KeyCode::Char('u') => {
            intents.push(Intent::Undo);
            state.clear_pending();
        }
        KeyCode::Char('r') if key.mods.ctrl() => {
            intents.push(Intent::Redo);
            state.clear_pending();
        }

        // Escape - clear pending
        KeyCode::Esc => {
            state.clear_pending();
        }

        _ => {
            state.clear_pending();
        }
    }

    intents
}
