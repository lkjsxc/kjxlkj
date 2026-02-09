/// Render a single window's buffer content into the cell grid.
use crate::cell::{Cell, CellGrid};
use kjxlkj_core_types::{BufferId, ContentSource};
use kjxlkj_core_ui::{BufferSnapshot, Color, Style, WindowSnapshot};
use std::collections::HashMap;
use unicode_segmentation::UnicodeSegmentation;

pub(crate) fn render_window(
    grid: &mut CellGrid,
    ws: &WindowSnapshot,
    buffers: &HashMap<BufferId, BufferSnapshot>,
    is_focused: bool,
    default_style: &Style,
    line_num_style: &Style,
    cursor_style: &Style,
) {
    let area = &ws.area;
    let buf_id = match &ws.content {
        ContentSource::Buffer(id) => id,
        ContentSource::Terminal(_) => return,
    };
    let buf = match buffers.get(buf_id) {
        Some(b) => b,
        None => return,
    };

    let gutter_width = if ws.show_line_numbers {
        let digits = format!("{}", buf.line_count).len();
        (digits + 1) as u16
    } else {
        0
    };

    let text_start_col = area.x + gutter_width;
    let text_width = area.width.saturating_sub(gutter_width);

    for row_offset in 0..area.height {
        let buf_line = ws.top_line + row_offset as usize;
        let screen_row = area.y + row_offset;
        if screen_row >= grid.height() {
            break;
        }

        if ws.show_line_numbers && buf_line < buf.line_count {
            let num_str = format!(
                "{:>width$} ",
                buf_line + 1,
                width = gutter_width as usize - 1
            );
            grid.set_str(area.x, screen_row, &num_str, *line_num_style);
        }

        if buf_line < buf.line_count {
            render_line_content(
                grid,
                buf,
                buf_line,
                screen_row,
                text_start_col,
                text_width,
                is_focused,
                ws,
                default_style,
                cursor_style,
            );
        } else {
            let screen_col = text_start_col;
            if screen_col < grid.width() {
                grid.set(
                    screen_col,
                    screen_row,
                    Cell {
                        grapheme: "~".to_string(),
                        width: 1,
                        style: Style {
                            fg: Color::Indexed(242),
                            ..Style::default()
                        },
                        is_wide_continuation: false,
                    },
                );
            }
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn render_line_content(
    grid: &mut CellGrid,
    buf: &BufferSnapshot,
    buf_line: usize,
    screen_row: u16,
    text_start_col: u16,
    text_width: u16,
    is_focused: bool,
    ws: &WindowSnapshot,
    default_style: &Style,
    cursor_style: &Style,
) {
    let line_slice = buf.content.line(buf_line);
    let line_str: std::borrow::Cow<str> = line_slice.into();
    let trimmed = line_str.trim_end_matches(&['\n', '\r'][..]);

    let mut col = 0u16;
    for (g_idx, grapheme) in trimmed.graphemes(true).enumerate() {
        if col >= text_width {
            break;
        }
        let w = unicode_width::UnicodeWidthStr::width(grapheme) as u8;
        let mut style = *default_style;
        if is_focused && buf_line == ws.cursor.line && g_idx == ws.cursor.grapheme {
            style = *cursor_style;
        }
        let screen_col = text_start_col + col;
        if screen_col < grid.width() {
            grid.set(
                screen_col,
                screen_row,
                Cell {
                    grapheme: grapheme.to_string(),
                    width: w,
                    style,
                    is_wide_continuation: false,
                },
            );
            if w == 2 && screen_col + 1 < grid.width() {
                grid.set(
                    screen_col + 1,
                    screen_row,
                    Cell {
                        grapheme: String::new(),
                        width: 0,
                        style,
                        is_wide_continuation: true,
                    },
                );
            }
        }
        col += w as u16;
    }

    if is_focused
        && buf_line == ws.cursor.line
        && ws.cursor.grapheme >= trimmed.graphemes(true).count()
    {
        let screen_col = text_start_col + col;
        if screen_col < grid.width() {
            grid.set(
                screen_col,
                screen_row,
                Cell {
                    grapheme: " ".to_string(),
                    width: 1,
                    style: *cursor_style,
                    is_wide_continuation: false,
                },
            );
        }
    }
}
