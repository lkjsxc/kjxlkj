//! Snapshot construction for the render pipeline.
//!
//! Extracts immutable EditorSnapshot from the mutable EditorState.
//! See /docs/spec/architecture/render-pipeline.md.

use kjxlkj_core_text::Buffer;
use kjxlkj_core_types::{
    ContentKind, EditorSnapshot, Rect, WindowContent,
};

use crate::editor::EditorState;
use crate::window_state::WindowState;

impl EditorState {
    /// Build an immutable snapshot for rendering.
    pub fn snapshot(&self) -> EditorSnapshot {
        let area = Rect::new(
            0,
            0,
            self.terminal_size.0,
            self.terminal_size.1,
        );
        let rects = self.layout.compute_rects(area);
        let mut window_contents = Vec::new();

        for (wid, content, rect) in &rects {
            let ws = self.windows.get(wid);
            let (lines, cursor_row, cursor_col, wtype, status) =
                match content {
                    ContentKind::Buffer(bid) => {
                        let buf = self
                            .buffers
                            .get(bid)
                            .map(|b| {
                                buffer_lines(b, ws, *rect)
                            })
                            .unwrap_or_default();
                        let cr = ws
                            .map(|w| {
                                w.cursor
                                    .line
                                    .saturating_sub(w.top_line)
                            })
                            .unwrap_or(0);
                        let cc =
                            ws.map(|w| w.cursor.col).unwrap_or(0);
                        let name = self
                            .buffers
                            .get(bid)
                            .map(|b| b.name.clone())
                            .unwrap_or_default();
                        let st = format!(
                            " {} | {} ",
                            self.mode.display_name(),
                            name
                        );
                        (buf, cr, cc, "buffer", st)
                    }
                    ContentKind::Explorer(_) => (
                        vec!["[Explorer]".into()],
                        0,
                        0,
                        "explorer",
                        " EXPLORER ".into(),
                    ),
                    ContentKind::Terminal(_) => (
                        vec!["[Terminal]".into()],
                        0,
                        0,
                        "terminal",
                        " TERMINAL ".into(),
                    ),
                };

            window_contents.push(WindowContent {
                window_id: *wid,
                rect: *rect,
                lines,
                cursor_row,
                cursor_col,
                window_type: wtype.to_string(),
                statusline: status,
            });
        }

        let layout_summary = format!(
            "windows={} focused={}",
            rects.len(),
            self.focus.focused.0
        );

        EditorSnapshot {
            sequence: self.sequence,
            mode: self.mode,
            terminal_size: self.terminal_size,
            cmdline: self.cmdline.clone(),
            notifications: Vec::new(),
            layout_summary,
            focused_window: self.focus.focused,
            window_contents,
        }
    }
}

/// Extract visible lines from a buffer for a given window viewport.
fn buffer_lines(
    buf: &Buffer,
    ws: Option<&WindowState>,
    rect: Rect,
) -> Vec<String> {
    let top = ws.map(|w| w.top_line).unwrap_or(0);
    let height = rect.height.saturating_sub(1) as usize;
    let mut lines = Vec::with_capacity(height);
    for i in 0..height {
        let line_idx = top + i;
        if let Some(content) = buf.line(line_idx) {
            lines.push(
                content.trim_end_matches('\n').to_string(),
            );
        } else {
            lines.push("~".to_string());
        }
    }
    lines
}
