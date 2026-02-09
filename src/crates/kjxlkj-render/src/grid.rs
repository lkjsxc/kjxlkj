use crate::cell::{Cell, CellGrid};
use crate::grid_window::render_window;
use kjxlkj_core_types::ContentSource;
use kjxlkj_core_ui::{Color, EditorSnapshot, Style};

/// Build a cell grid from an editor snapshot.
pub fn build_grid(snapshot: &EditorSnapshot) -> CellGrid {
    let (cols, rows) = snapshot.terminal_size;
    let mut grid = CellGrid::new(cols, rows);

    if let Some(tab) = snapshot.tabs.get(snapshot.active_tab) {
        for (wid, ws) in &tab.windows {
            render_window(
                &mut grid,
                ws,
                &snapshot.buffers,
                *wid == snapshot.focused_window,
                &snapshot.theme.default_style,
                &snapshot.theme.line_number_style,
                &snapshot.theme.cursor_style,
                &snapshot.search.highlight_ranges,
            );
        }
    }

    if rows >= 2 {
        render_statusline(&mut grid, cols, rows - 2, snapshot);
    }

    if rows >= 1 {
        render_cmdline(&mut grid, cols, rows - 1, snapshot);
    }

    if let Some(ref pm) = snapshot.popup_menu {
        render_popup_menu(&mut grid, pm, &snapshot.theme.default_style);
    }

    grid
}

fn render_statusline(grid: &mut CellGrid, cols: u16, row: u16, snapshot: &EditorSnapshot) {
    let style = snapshot.theme.statusline_style;
    for col in 0..cols { grid.set(col, row, Cell { grapheme: " ".into(), width: 1, style, is_wide_continuation: false }); }
    let mode_str = format!(" {} ", snapshot.mode);
    grid.set_str(0, row, &mode_str, style);
    let tab = snapshot.tabs.get(snapshot.active_tab);
    let buf_name = tab.and_then(|t| t.windows.get(&snapshot.focused_window)).and_then(|ws| {
        if let ContentSource::Buffer(bid) = &ws.content { snapshot.buffers.get(bid).map(|b| { let m = if b.modified { "[+]" } else { "" }; format!(" {}{} ", b.name, m) }) } else { None }
    }).unwrap_or_default();
    grid.set_str(mode_str.len() as u16, row, &buf_name, style);
    if let Some(ws) = tab.and_then(|t| t.windows.get(&snapshot.focused_window)) {
        let pos_str = format!(" {}:{} ", ws.cursor.line + 1, ws.cursor.grapheme + 1);
        let pos_len = pos_str.len() as u16;
        if cols > pos_len { grid.set_str(cols - pos_len, row, &pos_str, style); }
    }
}

fn render_cmdline(grid: &mut CellGrid, cols: u16, row: u16, snapshot: &EditorSnapshot) {
    if snapshot.cmdline.active {
        let prefix = snapshot.cmdline.prefix.map(|c| c.to_string()).unwrap_or_default();
        grid.set_str(0, row, &format!("{}{}", prefix, snapshot.cmdline.content), snapshot.theme.cmdline_style);
        if !snapshot.cmdline.completions.is_empty() && row > 0 { render_wildmenu(grid, cols, row - 1, snapshot); }
    } else if let Some(notif) = snapshot.notifications.last() {
        let style = match notif.level {
            kjxlkj_core_ui::NotificationLevel::Error => Style { fg: Color::Rgb(255, 0, 0), ..Style::default() },
            kjxlkj_core_ui::NotificationLevel::Warning => Style { fg: Color::Rgb(255, 255, 0), ..Style::default() },
            _ => Style::default(),
        };
        grid.set_str(0, row, &notif.message, style);
    }
    if let Some((cur, total)) = snapshot.search.match_count {
        let cs = format!("[{}/{}]", cur, total); let len = cs.len() as u16;
        if cols > len { grid.set_str(cols - len, row, &cs, Style::default()); }
    }
}

#[rustfmt::skip]
fn render_wildmenu(grid: &mut CellGrid, cols: u16, row: u16, snapshot: &EditorSnapshot) {
    let sel = snapshot.cmdline.completion_index;
    let normal = snapshot.theme.statusline_style;
    let selected = Style { fg: Color::Rgb(0, 0, 0), bg: Color::Rgb(255, 255, 255), bold: false, italic: false, underline: false, reverse: false };
    let items: Vec<String> = snapshot.cmdline.completions.iter().map(|s| {
        let display = s.rsplit('/').next().unwrap_or(s);
        format!(" {} ", display)
    }).collect();
    let widths: Vec<u16> = items.iter().map(|s| s.len() as u16).collect();
    let total_w: u16 = widths.iter().sum();
    let scroll_start = if total_w <= cols { 0 } else {
        let si = sel.unwrap_or(0);
        let mut prefix: u16 = widths[..si].iter().sum();
        if prefix + widths[si] > cols { prefix = (prefix + widths[si]).saturating_sub(cols); }
        let mut start = 0usize; let mut acc = 0u16;
        for (i, &w) in widths.iter().enumerate() { if acc + w > prefix { start = i; break; } acc += w; }
        start
    };
    let ind = Style { fg: Color::Rgb(255, 255, 0), bg: normal.bg, bold: true, italic: false, underline: false, reverse: false };
    let mut col = if scroll_start > 0 { grid.set_str(0, row, "<", ind); 1u16 } else { 0u16 };
    for (i, item) in items.iter().enumerate().skip(scroll_start) {
        if col >= cols.saturating_sub(1) { break; }
        grid.set_str(col, row, item, if sel == Some(i) { selected } else { normal }); col += item.len() as u16;
    }
    let mut ve = scroll_start; let mut aw = if scroll_start > 0 { 1u16 } else { 0u16 };
    for (i, it) in items.iter().enumerate().skip(scroll_start) { if aw + it.len() as u16 > cols { break; } aw += it.len() as u16; ve = i + 1; }
    if ve < items.len() && cols > 0 { grid.set_str(cols - 1, row, ">", ind); }
}

#[rustfmt::skip]
fn render_popup_menu(grid: &mut CellGrid, pm: &kjxlkj_core_ui::PopupMenu, base: &Style) {
    let normal = Style { bg: Color::Rgb(40, 40, 40), fg: Color::Rgb(200, 200, 200), bold: false, italic: false, underline: false, reverse: false };
    let selected = Style { bg: Color::Rgb(80, 120, 200), fg: Color::Rgb(255, 255, 255), bold: true, italic: false, underline: false, reverse: false };
    let _ = base;
    let end = (pm.scroll_offset + pm.max_visible).min(pm.items.len());
    let max_w = (grid.width() as usize).saturating_sub(pm.col as usize).max(4);
    for (vi, idx) in (pm.scroll_offset..end).enumerate() {
        let r = pm.row.saturating_sub(vi as u16);
        let st = if pm.selected == Some(idx) { selected } else { normal };
        let raw = &pm.items[idx];
        let truncated = if raw.len() + 2 > max_w { format!(" {}…", &raw[..max_w.saturating_sub(3)]) } else { format!(" {} ", raw) };
        grid.set_str(pm.col, r, &truncated, st);
    }
}
