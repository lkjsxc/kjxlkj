//! Main renderer – drives crossterm to paint frames from [`EditorSnapshot`].

use crossterm::{
    cursor::{Hide, MoveTo, SetCursorStyle, Show},
    execute,
    style::Print,
};
use kjxlkj_core_ui::snapshot::{CursorShape, EditorSnapshot};
use std::io::Write;

use crate::message_area::{render_message_area, MessageKind};

/// Terminal renderer holding viewport dimensions.
pub struct Renderer {
    pub width: u16,
    pub height: u16,
    pub last_cursor: Option<(u16, u16)>,
}

impl Renderer {
    /// Create a renderer for the given terminal size.
    pub fn new(width: u16, height: u16) -> Self {
        Self { width, height, last_cursor: None }
    }

    /// Render a complete frame from `snapshot` into `output`.
    pub fn render(&mut self, snapshot: &EditorSnapshot, output: &mut impl Write) -> std::io::Result<()> {
        execute!(output, Hide)?;

        let body_h = self.height.saturating_sub(3) as usize; // tab + status + cmd
        // Tab line (row 0)
        let tab = snapshot.tab_line.as_deref().unwrap_or("");
        let tab_padded = pad_or_truncate(tab, self.width as usize);
        execute!(output, MoveTo(0, 0), Print(&tab_padded))?;

        // Buffer content
        let win = snapshot.windows.first();
        let buf = snapshot.buffers.iter().find(|b| b.id == snapshot.active_buffer);
        let top = win.map_or(0, |w| w.top_line);
        let left = win.map_or(0, |w| w.left_col);
        let show_nums = true;
        let num_w: usize = 4;

        for row in 0..body_h {
            let line_idx = top + row;
            let screen_row = (row as u16) + 1;
            let mut line_text = String::new();
            if show_nums {
                if let Some(b) = buf {
                    if line_idx < b.lines.len() {
                        line_text.push_str(&format!("{:>3} ", line_idx + 1));
                    } else {
                        line_text.push_str("  ~ ");
                    }
                } else {
                    line_text.push_str("    ");
                }
            }
            if let Some(b) = buf {
                if line_idx < b.lines.len() {
                    let raw = &b.lines[line_idx];
                    let visible = safe_substr(raw, left, self.width as usize - num_w);
                    line_text.push_str(&visible);
                }
            }
            let padded = pad_or_truncate(&line_text, self.width as usize);
            execute!(output, MoveTo(0, screen_row), Print(&padded))?;
        }

        // Status line
        let status_row = 1 + body_h as u16;
        let sl = pad_or_truncate(&snapshot.status_line, self.width as usize);
        execute!(output, MoveTo(0, status_row), Print(&sl))?;

        // Command / message line
        let cmd_row = status_row + 1;
        let cmd = if let Some(ref msg) = snapshot.message {
            render_message_area(msg, MessageKind::Info, self.width as usize)
        } else {
            pad_or_truncate(&snapshot.command_line, self.width as usize)
        };
        execute!(output, MoveTo(cmd_row, cmd_row), Print(&cmd))?;

        // Cursor
        if snapshot.cursor.visible {
            let cr = snapshot.cursor.position.line.saturating_sub(top) as u16 + 1;
            let cc = snapshot.cursor.position.col.saturating_sub(left) as u16 + num_w as u16;
            execute!(output, MoveTo(cc, cr))?;
            let style = match snapshot.cursor.shape {
                CursorShape::Block => SetCursorStyle::SteadyBlock,
                CursorShape::Line => SetCursorStyle::SteadyBar,
                CursorShape::Underline => SetCursorStyle::SteadyUnderScore,
            };
            execute!(output, style, Show)?;
            self.last_cursor = Some((cr, cc));
        }
        output.flush()
    }

    /// Update terminal dimensions.
    pub fn resize(&mut self, width: u16, height: u16) {
        self.width = width;
        self.height = height;
    }
}

fn pad_or_truncate(s: &str, w: usize) -> String {
    let chars: Vec<char> = s.chars().collect();
    if chars.len() >= w {
        chars[..w].iter().collect()
    } else {
        let mut out: String = chars.into_iter().collect();
        out.extend(std::iter::repeat(' ').take(w - out.len()));
        out
    }
}

fn safe_substr(s: &str, start: usize, max_len: usize) -> String {
    let chars: Vec<char> = s.chars().collect();
    let end = (start + max_len).min(chars.len());
    if start >= chars.len() {
        String::new()
    } else {
        chars[start..end].iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pad_short() {
        assert_eq!(pad_or_truncate("hi", 5), "hi   ");
    }

    #[test]
    fn pad_long() {
        assert_eq!(pad_or_truncate("hello world", 5), "hello");
    }

    #[test]
    fn safe_substr_empty() {
        assert_eq!(safe_substr("abc", 10, 5), "");
    }

    #[test]
    fn renderer_new() {
        let r = Renderer::new(80, 24);
        assert_eq!(r.width, 80);
        assert_eq!(r.height, 24);
        assert!(r.last_cursor.is_none());
    }

    #[test]
    fn resize() {
        let mut r = Renderer::new(80, 24);
        r.resize(120, 40);
        assert_eq!(r.width, 120);
        assert_eq!(r.height, 40);
    }
}
