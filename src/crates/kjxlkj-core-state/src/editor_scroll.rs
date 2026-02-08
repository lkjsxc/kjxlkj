//! Scroll operations for EditorState.

use kjxlkj_core_types::ScrollDirection;

use crate::EditorState;

impl EditorState {
    /// Execute a scroll action.
    pub(crate) fn do_scroll(
        &mut self,
        direction: ScrollDirection,
        count: u32,
    ) {
        let line_count = self
            .active_buffer()
            .map(|b| b.line_count())
            .unwrap_or(1);

        let height = self
            .focused_window()
            .map(|w| w.viewport.height as usize)
            .unwrap_or(24);

        let amount = match direction {
            ScrollDirection::Up => count as usize,
            ScrollDirection::Down => count as usize,
            ScrollDirection::HalfUp => {
                (height / 2).max(1) * count as usize
            }
            ScrollDirection::HalfDown => {
                (height / 2).max(1) * count as usize
            }
            ScrollDirection::PageUp => {
                height.saturating_sub(2).max(1)
                    * count as usize
            }
            ScrollDirection::PageDown => {
                height.saturating_sub(2).max(1)
                    * count as usize
            }
        };

        if let Some(w) = self.focused_window_mut() {
            match direction {
                ScrollDirection::Up
                | ScrollDirection::HalfUp
                | ScrollDirection::PageUp => {
                    w.viewport.scroll_up(amount);
                    // Keep cursor in visible area.
                    let top = w.viewport.top_line;
                    if w.cursor.line
                        >= top + w.viewport.height as usize
                    {
                        w.cursor.line = top
                            + w.viewport.height as usize
                            - 1;
                    }
                }
                ScrollDirection::Down
                | ScrollDirection::HalfDown
                | ScrollDirection::PageDown => {
                    w.viewport
                        .scroll_down(amount, line_count);
                    // Keep cursor in visible area.
                    let top = w.viewport.top_line;
                    if w.cursor.line < top {
                        w.cursor.line = top;
                    }
                    let max_line =
                        line_count.saturating_sub(1);
                    if w.cursor.line > max_line {
                        w.cursor.line = max_line;
                    }
                }
            }
        }
    }

    /// Substitute character: delete char at cursor,
    /// enter Insert mode.
    pub(crate) fn do_substitute_char(&mut self) {
        self.delete_char_forward();
        self.mode = kjxlkj_core_types::Mode::Insert;
        self.insert_state.reset();
    }

    /// Substitute line: clear line content, enter Insert.
    pub(crate) fn do_substitute_line(&mut self) {
        let line = self.cursor_pos().0;
        if let Some(buf) = self.active_buffer_mut() {
            let start =
                buf.content.line_start_offset(line);
            let end = buf.content.line_end_offset(line);
            if end > start {
                buf.content.delete_range(start, end);
                buf.modified = true;
            }
        }
        if let Some(w) = self.focused_window_mut() {
            w.cursor.grapheme_offset = 0;
        }
        self.mode = kjxlkj_core_types::Mode::Insert;
        self.insert_state.reset();
    }

    /// Change to end of line: delete from cursor to EOL,
    /// enter Insert.
    pub(crate) fn do_change_to_end(&mut self) {
        let (line, col) = self.cursor_pos();
        if let Some(buf) = self.active_buffer_mut() {
            let start = buf
                .content
                .line_grapheme_to_offset(line, col);
            let end = buf.content.line_end_offset(line);
            if end > start {
                buf.content.delete_range(start, end);
                buf.modified = true;
            }
        }
        self.mode = kjxlkj_core_types::Mode::Insert;
        self.insert_state.reset();
    }

    /// Join lines without space (gJ).
    pub(crate) fn do_join_no_space(&mut self) {
        let line = self.cursor_pos().0;
        if let Some(buf) = self.active_buffer_mut() {
            if line + 1 < buf.line_count() {
                let end =
                    buf.content.line_end_offset(line);
                let next_start = end + 1;
                if next_start
                    <= buf.content.len_chars()
                {
                    buf.content
                        .delete_range(end, next_start);
                    buf.modified = true;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scroll_half_down() {
        let mut ed = EditorState::new(80, 24);
        // Add lines to the buffer so scrolling has effect.
        if let Some(buf) = ed.active_buffer_mut() {
            for _ in 0..50 {
                let len = buf.content.len_chars();
                buf.content.insert_char(len, '\n');
            }
        }
        ed.do_scroll(ScrollDirection::HalfDown, 1);
        let top = ed
            .focused_window()
            .map(|w| w.viewport.top_line)
            .unwrap_or(0);
        assert!(top > 0);
    }

    #[test]
    fn scroll_half_up_clamp() {
        let mut ed = EditorState::new(80, 24);
        ed.do_scroll(ScrollDirection::HalfUp, 1);
        let top = ed
            .focused_window()
            .map(|w| w.viewport.top_line)
            .unwrap_or(0);
        assert_eq!(top, 0);
    }
}
