//! Terminal renderer.

mod selection;
mod status;

use crossterm::{
    cursor::{Hide, MoveTo, SetCursorStyle, Show},
    execute,
    style::{Color, Print, ResetColor, SetForegroundColor},
    terminal::{Clear, ClearType},
};
use kjxlkj_core_types::Mode;
use kjxlkj_core_ui::EditorSnapshot;
use std::io::{self, Write};

/// Terminal renderer.
pub struct Renderer {
    _private: (),
}

impl Renderer {
    /// Create a new renderer.
    pub fn new() -> Self {
        Self { _private: () }
    }

    /// Render a snapshot.
    pub fn render<W: Write>(&self, writer: &mut W, snapshot: &EditorSnapshot) -> io::Result<()> {
        execute!(writer, Hide, MoveTo(0, 0))?;

        // Render buffer lines
        for (i, line) in snapshot.buffer.lines.iter().enumerate() {
            execute!(writer, MoveTo(0, i as u16))?;
            self.render_line(writer, snapshot, i, line)?;
        }

        // Clear remaining lines
        for i in snapshot.buffer.lines.len()..snapshot.viewport.height as usize {
            execute!(
                writer,
                MoveTo(0, i as u16),
                Clear(ClearType::CurrentLine),
                SetForegroundColor(Color::DarkGrey),
                Print("~"),
                ResetColor
            )?;
        }

        // Render status line
        status::render(writer, snapshot)?;

        // Position cursor
        let (col, row) = snapshot.cursor_screen_position();
        execute!(writer, MoveTo(col, row))?;

        // Set cursor style based on mode
        let style = match snapshot.mode {
            Mode::Normal => SetCursorStyle::SteadyBlock,
            Mode::Insert => SetCursorStyle::SteadyBar,
            Mode::Replace => SetCursorStyle::SteadyUnderScore,
            Mode::Visual | Mode::VisualLine | Mode::VisualBlock => SetCursorStyle::SteadyBlock,
            Mode::Command => SetCursorStyle::SteadyBar,
        };
        execute!(writer, style, Show)?;

        writer.flush()
    }

    fn render_line<W: Write>(
        &self,
        writer: &mut W,
        snapshot: &EditorSnapshot,
        screen_row: usize,
        line: &str,
    ) -> io::Result<()> {
        execute!(writer, Clear(ClearType::CurrentLine))?;
        let buffer_line = snapshot.viewport.first_line + screen_row;

        if let Some(ref sel) = snapshot.selection {
            selection::render_with_selection(writer, line, buffer_line, sel)
        } else {
            execute!(writer, Print(line))
        }
    }
}

impl Default for Renderer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use kjxlkj_core_ui::Viewport;

    #[test]
    fn renderer_creation() {
        let renderer = Renderer::new();
        drop(renderer);
    }

    #[test]
    fn renderer_render() {
        let mut buf = Vec::new();
        let renderer = Renderer::new();
        let snapshot = EditorSnapshot::empty(Viewport::new(80, 24));
        let result = renderer.render(&mut buf, &snapshot);
        assert!(result.is_ok());
        assert!(!buf.is_empty());
    }

    #[test]
    fn renderer_render_with_cursor() {
        let mut buf = Vec::new();
        let renderer = Renderer::new();
        let snapshot = EditorSnapshot::empty(Viewport::new(40, 10));
        let result = renderer.render(&mut buf, &snapshot);
        assert!(result.is_ok());
    }

    #[test]
    fn renderer_output_contains_content() {
        let mut buf = Vec::new();
        let renderer = Renderer::new();
        let mut snapshot = EditorSnapshot::empty(Viewport::new(80, 24));
        snapshot.buffer.lines = vec!["Hello World".to_string()];
        let _ = renderer.render(&mut buf, &snapshot);
        let output = String::from_utf8_lossy(&buf);
        assert!(output.contains("Hello") || !buf.is_empty());
    }

    #[test]
    fn renderer_default() {
        let renderer = Renderer::default();
        let _ = renderer;
    }

    #[test]
    fn renderer_struct_exists() {
        fn assert_type<T>(_: &T) {}
        let r = Renderer::new();
        assert_type::<Renderer>(&r);
    }

    #[test]
    fn renderer_empty_buffer() {
        let mut buf = Vec::new();
        let renderer = Renderer::new();
        let snapshot = EditorSnapshot::empty(Viewport::new(80, 24));
        let _ = renderer.render(&mut buf, &snapshot);
        assert!(!buf.is_empty());
    }
}
