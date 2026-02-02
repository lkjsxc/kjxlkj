//! Replace mode handling.

use crossterm::event::KeyCode;
use kjxlkj_core::Buffer;

/// Action from replace mode.
pub enum ReplaceAction {
    /// Exit to normal mode.
    ExitToNormal,
    /// Continue in replace mode.
    None,
}

/// Handles a key press in replace mode.
pub fn handle_replace_key(code: KeyCode, buffer: &mut Buffer) -> ReplaceAction {
    match code {
        KeyCode::Esc => ReplaceAction::ExitToNormal,
        KeyCode::Char(c) => {
            // Delete char at cursor and insert new char
            buffer.delete_char_at();
            buffer.insert_char(c);
            ReplaceAction::None
        }
        KeyCode::Backspace => {
            // In replace mode, backspace moves left and restores?
            // For simplicity, just move left
            buffer.cursor_left();
            ReplaceAction::None
        }
        KeyCode::Enter => {
            buffer.delete_char_at();
            buffer.insert_newline();
            ReplaceAction::None
        }
        KeyCode::Left => {
            buffer.cursor_left();
            ReplaceAction::None
        }
        KeyCode::Right => {
            buffer.cursor_right();
            ReplaceAction::None
        }
        KeyCode::Up => {
            buffer.cursor_up();
            ReplaceAction::None
        }
        KeyCode::Down => {
            buffer.cursor_down();
            ReplaceAction::None
        }
        _ => ReplaceAction::None,
    }
}
