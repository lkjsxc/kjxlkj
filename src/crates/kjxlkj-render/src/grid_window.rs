/// Render a single window's buffer content into the cell grid.
use crate::cell::{Cell, CellGrid};
use kjxlkj_core_types::{BufferId, ContentSource, VisualKind};
use kjxlkj_core_ui::{BufferSnapshot, Color, Style, VisualSelection, WindowSnapshot};
use std::collections::HashMap;
use unicode_segmentation::UnicodeSegmentation;

#[allow(clippy::too_many_arguments)]
pub(crate) fn render_window(
    grid: &mut CellGrid,
    ws: &WindowSnapshot,
    buffers: &HashMap<BufferId, BufferSnapshot>,
    is_focused: bool,
    default_style: &Style,
    line_num_style: &Style,
    cursor_style: &Style,
    hl_ranges: &[(usize, usize, usize)],
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
                hl_ranges,
            );
        } else {
            render_tilde(grid, text_start_col, screen_row);
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
    hl_ranges: &[(usize, usize, usize)],
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
        // Apply search highlight.
        if hl_ranges
            .iter()
            .any(|&(l, s, e)| l == buf_line && g_idx >= s && g_idx < e)
        {
            style = Style {
                fg: Color::Rgb(0, 0, 0),
                bg: Color::Rgb(255, 255, 0),
                ..style
            };
        }
        // Apply visual selection highlight.
        if is_in_visual_selection(ws.visual_selection.as_ref(), buf_line, g_idx) {
            style = Style {
                fg: Color::Rgb(0, 0, 0),
                bg: Color::Rgb(100, 100, 255),
                ..style
            };
        }
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

/// Render tilde placeholder for empty lines.
#[rustfmt::skip]
fn render_tilde(grid: &mut CellGrid, col: u16, row: u16) {
    if col < grid.width() {
        let st = Style { fg: Color::Indexed(242), ..Style::default() };
        grid.set(col, row, Cell { grapheme: "~".into(), width: 1, style: st, is_wide_continuation: false });
    }
}

/// Check if a cell at (line, col) is inside the visual selection.
#[rustfmt::skip]
fn is_in_visual_selection(sel: Option<&VisualSelection>, line: usize, col: usize) -> bool {
    let sel = match sel { Some(s) => s, None => return false };
    let (al, ac) = (sel.anchor.line, sel.anchor.grapheme);
    let (cl, cc) = (sel.cursor.line, sel.cursor.grapheme);
    let (sl, sc, el, ec) = if (al, ac) <= (cl, cc) { (al, ac, cl, cc) } else { (cl, cc, al, ac) };
    match sel.kind {
        VisualKind::Char => {
            if line < sl || line > el { return false; }
            if sl == el { return col >= sc && col <= ec; }
            if line == sl { return col >= sc; }
            if line == el { return col <= ec; }
            true
        }
        VisualKind::Line => line >= sl && line <= el,
        VisualKind::Block => {
            let (min_c, max_c) = if ac <= cc { (ac, cc) } else { (cc, ac) };
            line >= sl && line <= el && col >= min_c && col <= max_c
        }
    }
}
