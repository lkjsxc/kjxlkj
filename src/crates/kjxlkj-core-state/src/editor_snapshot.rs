//! Snapshot production for the renderer.

use std::collections::HashMap;

use kjxlkj_core_ui::{
    BufferSnapshot, CmdlineState, EditorSnapshot,
    Rect, VisualRange, WindowLayout,
};

use crate::{BufferState, EditorState};

impl EditorState {
    /// Produce an immutable snapshot for render.
    pub fn snapshot(&mut self) -> EditorSnapshot {
        self.sequence += 1;
        let (cols, rows) = self.terminal_size;

        let mut buf_snaps = HashMap::new();
        for (id, buf) in &self.buffers {
            let snap =
                self.build_buffer_snapshot(buf);
            buf_snaps.insert(*id, snap);
        }

        let layout = WindowLayout::single(
            self.focused_window,
            Rect::new(
                0,
                0,
                cols,
                rows.saturating_sub(1),
            ),
        );

        let cmdline =
            if let Some(ref cs) = self.command_state {
                CmdlineState {
                    active: true,
                    prompt: cs.prompt_char(),
                    content: cs.content().to_string(),
                    cursor: cs.cursor,
                    completions: Vec::new(),
                    completion_index: None,
                }
            } else {
                CmdlineState::inactive()
            };

        let visual = self.visual_state.as_ref().map(|vs| {
            let (al, ac) = vs.anchor;
            let win = self.windows.get(&self.focused_window);
            let (cl, cc) = win
                .map(|w| (w.cursor.line, w.cursor.grapheme_offset))
                .unwrap_or((al, ac));
            let linewise = vs.kind == kjxlkj_core_types::VisualKind::Line;
            let block = vs.kind == kjxlkj_core_types::VisualKind::Block;
            let (start_line, start_col, end_line, end_col) =
                if (al, ac) <= (cl, cc) {
                    (al, ac, cl, cc)
                } else {
                    (cl, cc, al, ac)
                };
            VisualRange {
                start_line,
                start_col,
                end_line,
                end_col,
                linewise,
                block,
            }
        });

        EditorSnapshot {
            sequence: self.sequence,
            layout,
            buffers: buf_snaps,
            terminals: HashMap::new(),
            mode: self.mode.clone(),
            cmdline,
            notifications: Vec::new(),
            search: Default::default(),
            visual,
            theme: Default::default(),
            terminal_size: self.terminal_size,
        }
    }

    fn build_buffer_snapshot(
        &self,
        buf: &BufferState,
    ) -> BufferSnapshot {
        let win =
            self.windows.get(&self.focused_window);
        let (cursor_line, cursor_col) = win
            .map(|w| {
                (
                    w.cursor.line,
                    w.cursor.grapheme_offset,
                )
            })
            .unwrap_or((0, 0));
        let top_line = win
            .map(|w| w.viewport.top_line)
            .unwrap_or(0);

        let height = win
            .map(|w| w.viewport.height as usize)
            .unwrap_or(24);

        let mut visible_lines =
            Vec::with_capacity(height);
        for line_idx in top_line..top_line + height {
            if line_idx < buf.line_count() {
                visible_lines.push(
                    buf.content
                        .line_content(line_idx),
                );
            } else {
                visible_lines
                    .push(String::from("~"));
            }
        }

        BufferSnapshot {
            id: buf.id,
            version: buf.version,
            line_count: buf.line_count(),
            path: buf.path.clone(),
            name: buf.name.clone(),
            modified: buf.modified,
            readonly: buf.readonly,
            visible_lines,
            top_line,
            cursor_line,
            cursor_col,
            file_type: buf.file_type.clone(),
            line_ending: buf
                .line_ending
                .as_str()
                .to_string(),
            encoding: String::from("utf-8"),
        }
    }
}
