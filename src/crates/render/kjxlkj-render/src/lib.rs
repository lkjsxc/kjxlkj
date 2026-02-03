#![forbid(unsafe_code)]

use anyhow::Context;
use kjxlkj_core_ui::EditorSnapshot;
use ratatui::backend::CrosstermBackend;
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::style::{Color, Style};
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::Terminal;
use std::io;

pub fn draw_frame(term: &mut Terminal<CrosstermBackend<io::Stdout>>, snap: &EditorSnapshot) -> anyhow::Result<()> {
    term.draw(|frame| {
        let area = frame.area();
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(1), Constraint::Length(1)].as_ref())
            .split(area);

        let main = Paragraph::new(render_buffer_lines(snap)).block(Block::default().borders(Borders::ALL));
        frame.render_widget(main, chunks[0]);

        let status = Paragraph::new(render_statusline(snap)).style(Style::default().fg(Color::White));
        frame.render_widget(status, chunks[1]);
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
    format!(" {}  {}", snap.mode.as_str(), snap.status)
}

