//! Insert mode key handling.

use crossterm::event::KeyCode;
use kjxlkj_core::Buffer;

/// Action from insert mode input.
#[derive(Debug)]
pub enum InsertAction {
    /// No mode change.
    None,
    /// Exit to normal mode.
    ExitToNormal,
}

/// Handles a key press in insert mode.
pub fn handle_insert_key(code: KeyCode, buffer: &mut Buffer) -> InsertAction {
    match code {
        KeyCode::Esc => InsertAction::ExitToNormal,
        KeyCode::Char(c) => {
            buffer.insert_char(c);
            InsertAction::None
        }
        KeyCode::Enter => {
            buffer.insert_newline();
            InsertAction::None
        }
        KeyCode::Backspace => {
            buffer.delete_char_before();
            InsertAction::None
        }
        KeyCode::Delete => {
            buffer.delete_char_at();
            InsertAction::None
        }
        KeyCode::Left => {
            buffer.cursor_left();
            InsertAction::None
        }
        KeyCode::Right => {
            buffer.cursor_right();
            InsertAction::None
        }
        KeyCode::Up => {
            buffer.cursor_up();
            InsertAction::None
        }
        KeyCode::Down => {
            buffer.cursor_down();
            InsertAction::None
        }
        KeyCode::Home => {
            buffer.move_cursor(buffer.cursor_line(), 0);
            InsertAction::None
        }
        KeyCode::End => {
            let len = buffer
                .line(buffer.cursor_line())
                .map(|l| l.trim_end_matches('\n').len())
                .unwrap_or(0);
            buffer.move_cursor(buffer.cursor_line(), len);
            InsertAction::None
        }
        KeyCode::Tab => {
            buffer.insert_char('\t');
            InsertAction::None
        }
        _ => InsertAction::None,
    }
}
