//! Window and buffer rendering helpers for the
//! renderer.

use kjxlkj_core_types::{CellAttrs, Color, WindowId};
use kjxlkj_core_ui::{
    BufferSnapshot, EditorSnapshot, Rect,
};

use crate::cell_grid::CellGrid;
use crate::gutter;
use crate::statusline;

pub(crate) fn render_window(
    grid: &mut CellGrid,
    snapshot: &EditorSnapshot,
    win_id: WindowId,
    rect: Rect,
) {
    let is_focused =
        snapshot.layout.focused == win_id;

    if let Some((_id, buf)) =
        snapshot.buffers.iter().next()
    {
        let gutter_width = 4_u16;
        let text_x = rect.x + gutter_width;
        let text_width =
            rect.width.saturating_sub(gutter_width + 1);
        let text_height =
            rect.height.saturating_sub(1);

        gutter::render_gutter(
            grid,
            rect.x,
            rect.y,
            gutter_width,
            buf.top_line,
            buf.line_count,
            buf.cursor_line,
            text_height,
            snapshot.theme.line_nr,
            snapshot.theme.cursor_line_nr,
        );

        render_buffer_text(
            grid,
            buf,
            text_x,
            rect.y,
            text_width,
            text_height,
        );

        if is_focused {
            let cursor_row = buf
                .cursor_line
                .saturating_sub(buf.top_line)
                as u16;
            let cursor_col =
                text_x + buf.cursor_col as u16;
            if cursor_row < text_height
                && cursor_col < rect.x + rect.width
            {
                let cell = grid.get_mut(
                    cursor_col,
                    rect.y + cursor_row,
                );
                cell.attrs |= CellAttrs::REVERSE;
            }
        }

        let status_y = rect.y + rect.height - 1;
        let status_color = if is_focused {
            snapshot.theme.statusline
        } else {
            snapshot.theme.statusline_nc
        };
        statusline::render_statusline(
            grid,
            status_y,
            rect.width,
            &snapshot.mode,
            buf,
            is_focused,
            status_color,
            snapshot.theme.mode_indicator,
        );
    }
}

pub(crate) fn render_buffer_text(
    grid: &mut CellGrid,
    buf: &BufferSnapshot,
    x: u16,
    y: u16,
    width: u16,
    height: u16,
) {
    for row in 0..height {
        let line_idx = row as usize;
        if line_idx < buf.visible_lines.len() {
            let line = &buf.visible_lines[line_idx];
            let truncated =
                if line.len() > width as usize {
                    &line[..width as usize]
                } else {
                    line.as_str()
                };
            grid.write_str(
                x,
                y + row,
                truncated,
                Color::Default,
                Color::Default,
                CellAttrs::empty(),
            );
        }
    }
}
