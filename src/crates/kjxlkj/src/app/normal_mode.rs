//! Normal mode key handling.

use crossterm::event::KeyCode;
use kjxlkj_core::{Buffer, Mode};
use kjxlkj_core_types::intent::Operator;
use kjxlkj_core_types::{CommandKind, VisualMode};

/// Action from normal mode input.
#[derive(Debug)]
pub enum NormalAction {
    /// No action.
    None,
    /// Change mode.
    ChangeMode(Mode),
    /// Quit application.
    Quit,
    /// Start operator-pending mode.
    StartOperator(Operator),
}

/// Handles a key press in normal mode.
pub fn handle_normal_key(code: KeyCode, buffer: &mut Buffer) -> NormalAction {
    match code {
        KeyCode::Char('q') => NormalAction::Quit,
        KeyCode::Char('i') => NormalAction::ChangeMode(Mode::Insert),
        KeyCode::Char('a') => {
            buffer.cursor_right();
            NormalAction::ChangeMode(Mode::Insert)
        }
        KeyCode::Char('I') => {
            buffer.move_cursor(buffer.cursor_line(), 0);
            NormalAction::ChangeMode(Mode::Insert)
        }
        KeyCode::Char('A') => {
            let len = buffer
                .line(buffer.cursor_line())
                .map(|l| l.trim_end_matches('\n').len())
                .unwrap_or(0);
            buffer.move_cursor(buffer.cursor_line(), len);
            NormalAction::ChangeMode(Mode::Insert)
        }
        KeyCode::Char('o') => {
            buffer.move_cursor(buffer.cursor_line(), 99999);
            buffer.insert_newline();
            NormalAction::ChangeMode(Mode::Insert)
        }
        KeyCode::Char('O') => {
            buffer.move_cursor(buffer.cursor_line(), 0);
            buffer.insert_newline();
            buffer.cursor_up();
            NormalAction::ChangeMode(Mode::Insert)
        }
        // Motions
        KeyCode::Char('h') | KeyCode::Left => {
            buffer.cursor_left();
            NormalAction::None
        }
        KeyCode::Char('j') | KeyCode::Down => {
            buffer.cursor_down();
            NormalAction::None
        }
        KeyCode::Char('k') | KeyCode::Up => {
            buffer.cursor_up();
            NormalAction::None
        }
        KeyCode::Char('l') | KeyCode::Right => {
            buffer.cursor_right();
            NormalAction::None
        }
        KeyCode::Char('0') => {
            buffer.move_cursor(buffer.cursor_line(), 0);
            NormalAction::None
        }
        KeyCode::Char('$') | KeyCode::End => {
            let len = buffer
                .line(buffer.cursor_line())
                .map(|l| l.trim_end_matches('\n').len().saturating_sub(1))
                .unwrap_or(0);
            buffer.move_cursor(buffer.cursor_line(), len);
            NormalAction::None
        }
        // Delete char
        KeyCode::Char('x') => {
            buffer.delete_char_at();
            NormalAction::None
        }
        // Operators
        KeyCode::Char('d') => NormalAction::StartOperator(Operator::Delete),
        KeyCode::Char('y') => NormalAction::StartOperator(Operator::Yank),
        KeyCode::Char('c') => NormalAction::StartOperator(Operator::Change),
        // Replace mode
        KeyCode::Char('R') => NormalAction::ChangeMode(Mode::Replace),
        // Visual modes
        KeyCode::Char('v') => NormalAction::ChangeMode(Mode::Visual(VisualMode::Char)),
        KeyCode::Char('V') => NormalAction::ChangeMode(Mode::Visual(VisualMode::Line)),
        // Command mode
        KeyCode::Char(':') => NormalAction::ChangeMode(Mode::Command(CommandKind::Ex)),
        _ => NormalAction::None,
    }
}
