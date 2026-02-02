//! Visual mode key handling.

use crossterm::event::KeyCode;
use kjxlkj_core::Buffer;
use kjxlkj_core_types::VisualMode;

/// Visual selection state.
#[derive(Debug, Clone, Copy)]
pub struct VisualSelection {
    /// Anchor position (line, col).
    pub anchor_line: usize,
    pub anchor_col: usize,
}

/// Action from visual mode input.
#[derive(Debug)]
pub enum VisualAction {
    /// No action.
    None,
    /// Exit to normal mode.
    ExitToNormal,
    /// Exit and enter insert mode (after change).
    ExitToInsert,
    /// Switch visual mode variant.
    SwitchVariant(VisualMode),
    /// Delete selection.
    Delete,
    /// Yank selection.
    Yank,
    /// Change selection.
    Change,
}

/// Handles a key press in visual mode.
pub fn handle_visual_key(
    code: KeyCode,
    buffer: &mut Buffer,
    _current_mode: VisualMode,
) -> VisualAction {
    match code {
        KeyCode::Esc => VisualAction::ExitToNormal,
        // Motions
        KeyCode::Char('h') | KeyCode::Left => {
            buffer.cursor_left();
            VisualAction::None
        }
        KeyCode::Char('j') | KeyCode::Down => {
            buffer.cursor_down();
            VisualAction::None
        }
        KeyCode::Char('k') | KeyCode::Up => {
            buffer.cursor_up();
            VisualAction::None
        }
        KeyCode::Char('l') | KeyCode::Right => {
            buffer.cursor_right();
            VisualAction::None
        }
        KeyCode::Char('w') => {
            // Word forward motion
            buffer.cursor_right(); // Simplified
            VisualAction::None
        }
        KeyCode::Char('b') => {
            buffer.cursor_left(); // Simplified
            VisualAction::None
        }
        KeyCode::Char('0') => {
            buffer.move_cursor(buffer.cursor_line(), 0);
            VisualAction::None
        }
        KeyCode::Char('$') | KeyCode::End => {
            let len = buffer
                .line(buffer.cursor_line())
                .map(|l| l.trim_end_matches('\n').len().saturating_sub(1))
                .unwrap_or(0);
            buffer.move_cursor(buffer.cursor_line(), len);
            VisualAction::None
        }
        // Operators
        KeyCode::Char('d') | KeyCode::Char('x') => VisualAction::Delete,
        KeyCode::Char('y') => VisualAction::Yank,
        KeyCode::Char('c') | KeyCode::Char('s') => VisualAction::Change,
        // Mode switching
        KeyCode::Char('v') => VisualAction::SwitchVariant(VisualMode::Char),
        KeyCode::Char('V') => VisualAction::SwitchVariant(VisualMode::Line),
        _ => VisualAction::None,
    }
}
