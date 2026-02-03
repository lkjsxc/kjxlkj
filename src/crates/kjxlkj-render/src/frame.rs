//! Frame rendering.

use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    Frame,
};

use kjxlkj_core_ui::EditorSnapshot;
use kjxlkj_core_types::CursorStyle;

use super::frame_widgets::{render_command_line, render_editor, render_status_line};

/// Render an editor snapshot to a terminal frame.
pub fn render_snapshot(frame: &mut Frame, snapshot: &EditorSnapshot) {
    let area = frame.area();

    // Layout: editor area + status line + command line
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(1),
            Constraint::Length(1),
            Constraint::Length(1),
        ])
        .split(area);

    let editor_area = chunks[0];
    let status_area = chunks[1];
    let cmdline_area = chunks[2];

    // Render editor content
    render_editor(frame, editor_area, snapshot);

    // Render status line
    render_status_line(frame, status_area, snapshot);

    // Render command line or message
    render_command_line(frame, cmdline_area, snapshot);

    // Position cursor
    let cursor_pos = calculate_cursor_position(editor_area, snapshot);
    frame.set_cursor_position(cursor_pos);
}

fn calculate_cursor_position(editor_area: Rect, snapshot: &EditorSnapshot) -> (u16, u16) {
    let window = &snapshot.active_window;
    let cursor = &window.cursor;
    let viewport = &window.viewport;

    let line_offset = cursor.line().saturating_sub(viewport.top_line) as u16;
    let col_offset = cursor.col().saturating_sub(viewport.left_col) as u16;

    // Account for line numbers (5 chars: 4 digits + space)
    let x = editor_area.x + 5 + col_offset;
    let y = editor_area.y + line_offset;

    (x.min(editor_area.right() - 1), y.min(editor_area.bottom() - 1))
}

/// Get the cursor shape for crossterm.
pub fn cursor_style_to_shape(style: CursorStyle) -> crossterm::cursor::SetCursorStyle {
    match style {
        CursorStyle::Block => crossterm::cursor::SetCursorStyle::SteadyBlock,
        CursorStyle::Bar => crossterm::cursor::SetCursorStyle::SteadyBar,
        CursorStyle::Hollow => crossterm::cursor::SetCursorStyle::SteadyBlock,
        CursorStyle::Underline => crossterm::cursor::SetCursorStyle::SteadyUnderScore,
    }
}
