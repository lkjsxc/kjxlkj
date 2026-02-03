#![forbid(unsafe_code)]

use anyhow::Context;
use kjxlkj_core_ui::EditorSnapshot;
use ratatui::backend::CrosstermBackend;
use ratatui::layout::{Constraint, Direction, Layout, Position};
use ratatui::style::{Color, Style};
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::Terminal;
use std::io;

pub fn draw_frame(term: &mut Terminal<CrosstermBackend<io::Stdout>>, snap: &EditorSnapshot) -> anyhow::Result<()> {
    term.draw(|frame| {
        let area = frame.area();
        let bottom_rows = if snap.cmdline.is_some() { 2 } else { 1 };
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(1), Constraint::Length(bottom_rows)].as_ref())
            .split(area);

        let block = Block::default().borders(Borders::ALL);
        let inner = block.inner(chunks[0]);
        let main = Paragraph::new(render_buffer_lines(snap)).block(block);
        frame.render_widget(main, chunks[0]);

        if let Some((x, y)) = cursor_xy(snap, inner) {
            frame.set_cursor_position(Position { x, y });
        }

        if let Some(cmdline) = &snap.cmdline {
            let bottom = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Length(1), Constraint::Length(1)].as_ref())
                .split(chunks[1]);
            let status = Paragraph::new(render_statusline(snap)).style(Style::default().fg(Color::White));
            frame.render_widget(status, bottom[0]);
            let cmd = Paragraph::new(cmdline.clone()).style(Style::default().fg(Color::Yellow));
            frame.render_widget(cmd, bottom[1]);
        } else {
            let status = Paragraph::new(render_statusline(snap)).style(Style::default().fg(Color::White));
            frame.render_widget(status, chunks[1]);
        }
    }).context("draw frame")?;
    Ok(())
}

fn render_buffer_lines(snap: &EditorSnapshot) -> String {
    let active_win = match snap.windows.iter().find(|w| w.id == snap.active_window) {
        Some(w) => w,
        None => return String::new(),
    };
    let buf = match snap.buffers.iter().find(|b| b.id == active_win.buffer_id) {
        Some(b) => b,
        None => return String::new(),
    };
    buf.lines.join("")
}

fn render_statusline(snap: &EditorSnapshot) -> String {
    let (name, modified, line, col) = active_buffer_status(snap).unwrap_or_else(|| ("[No Name]".to_string(), false, 1, 1));
    let mod_mark = if modified { " [+]" } else { "" };
    if snap.status.is_empty() {
        format!(" {}  {}{}  {}:{}", snap.mode.as_str(), name, mod_mark, line, col)
    } else {
        format!(" {}  {}{}  {}:{}  {}", snap.mode.as_str(), name, mod_mark, line, col, snap.status)
    }
}

fn active_buffer_status(snap: &EditorSnapshot) -> Option<(String, bool, usize, usize)> {
    let win = snap.windows.iter().find(|w| w.id == snap.active_window)?;
    let buf = snap.buffers.iter().find(|b| b.id == win.buffer_id)?;
    Some((
        buf.name.clone(),
        buf.modified,
        win.cursor.line.saturating_add(1),
        win.cursor.col.saturating_add(1),
    ))
}

fn cursor_xy(snap: &EditorSnapshot, inner: ratatui::layout::Rect) -> Option<(u16, u16)> {
    let win = snap.windows.iter().find(|w| w.id == snap.active_window)?;
    let y = inner.y.saturating_add((win.cursor.line.saturating_sub(win.viewport_top)) as u16);
    let x = inner.x.saturating_add((win.cursor.col.saturating_sub(win.viewport_left)) as u16);
    if x >= inner.x.saturating_add(inner.width) || y >= inner.y.saturating_add(inner.height) {
        return None;
    }
    Some((x, y))
}
