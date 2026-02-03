//! Widget rendering for editor components.

use ratatui::{
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span},
    widgets::Paragraph,
    Frame,
};

use kjxlkj_core_ui::{EditorSnapshot, MessageLevel};
use kjxlkj_core_types::Mode;

pub fn render_editor(frame: &mut Frame, area: Rect, snapshot: &EditorSnapshot) {
    let window = &snapshot.active_window;

    let mut lines: Vec<Line> = Vec::new();
    let num_width = 4; // Fixed width for simplicity

    for line_snap in &window.lines {
        let mut spans = Vec::new();

        // Line number
        let num_style = if line_snap.is_cursor_line {
            Style::default().fg(Color::Yellow)
        } else {
            Style::default().fg(Color::DarkGray)
        };
        spans.push(Span::styled(
            format!("{:>width$} ", line_snap.number, width = num_width),
            num_style,
        ));

        // Line content with selection highlighting
        let text_style = Style::default();

        if let Some(sel) = &window.selection {
            let line_idx = line_snap.number - 1;
            if line_idx >= sel.start.line && line_idx <= sel.end.line {
                let start_col = if line_idx == sel.start.line {
                    sel.start.col as usize
                } else {
                    0
                };
                let end_col = if line_idx == sel.end.line {
                    sel.end.col as usize
                } else {
                    line_snap.text.len()
                };

                let before = &line_snap.text[..start_col.min(line_snap.text.len())];
                let selected = &line_snap.text
                    [start_col.min(line_snap.text.len())..end_col.min(line_snap.text.len())];
                let after = &line_snap.text[end_col.min(line_snap.text.len())..];

                spans.push(Span::raw(before));
                spans.push(Span::styled(
                    selected,
                    Style::default().bg(Color::Blue).fg(Color::White),
                ));
                spans.push(Span::raw(after));
            } else {
                spans.push(Span::styled(line_snap.text.clone(), text_style));
            }
        } else {
            spans.push(Span::styled(line_snap.text.clone(), text_style));
        }

        lines.push(Line::from(spans));
    }

    // Fill remaining lines with tildes
    let visible_lines = lines.len();
    for _ in visible_lines..area.height as usize {
        lines.push(Line::from(vec![Span::styled(
            format!("{:>width$} ", "~", width = num_width),
            Style::default().fg(Color::Blue),
        )]));
    }

    let paragraph = Paragraph::new(lines);
    frame.render_widget(paragraph, area);
}

pub fn render_status_line(frame: &mut Frame, area: Rect, snapshot: &EditorSnapshot) {
    let mode_style = match snapshot.mode {
        Mode::Normal => Style::default().bg(Color::Blue).fg(Color::White),
        Mode::Insert => Style::default().bg(Color::Green).fg(Color::Black),
        Mode::Visual | Mode::VisualLine | Mode::VisualBlock => {
            Style::default().bg(Color::Magenta).fg(Color::White)
        }
        Mode::Command => Style::default().bg(Color::Yellow).fg(Color::Black),
        Mode::Replace => Style::default().bg(Color::Red).fg(Color::White),
    };

    let mode_span = Span::styled(format!(" {} ", snapshot.mode.name()), mode_style);

    let file_info = Span::raw(format!(
        " {} ",
        snapshot.active_window.buffer_meta.name.as_str()
    ));

    let modified = if snapshot.active_window.buffer_meta.modified {
        Span::styled(" [+] ", Style::default().fg(Color::Red))
    } else {
        Span::raw("")
    };

    let position = Span::styled(
        format!(" {} ", snapshot.status.right),
        Style::default().fg(Color::Gray),
    );

    // Calculate padding
    let left_len = mode_span.width() + file_info.width() + modified.width();
    let right_len = position.width();
    let padding = area.width as usize - left_len - right_len;

    let line = Line::from(vec![
        mode_span,
        file_info,
        modified,
        Span::raw(" ".repeat(padding)),
        position,
    ]);

    let status = Paragraph::new(line).style(Style::default().bg(Color::DarkGray));
    frame.render_widget(status, area);
}

pub fn render_command_line(frame: &mut Frame, area: Rect, snapshot: &EditorSnapshot) {
    let content = if snapshot.mode == Mode::Command {
        format!(":{}", snapshot.command_line.as_deref().unwrap_or(""))
    } else if let Some(msg) = &snapshot.message {
        let style = match msg.level {
            MessageLevel::Error => Style::default().fg(Color::Red),
            MessageLevel::Warning => Style::default().fg(Color::Yellow),
            MessageLevel::Info => Style::default(),
        };
        let line = Line::from(Span::styled(&msg.text, style));
        let paragraph = Paragraph::new(line);
        frame.render_widget(paragraph, area);
        return;
    } else {
        String::new()
    };

    let paragraph = Paragraph::new(content);
    frame.render_widget(paragraph, area);
}
