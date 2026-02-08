//! Scroll region and scroll operations for ScreenBuffer.

use kjxlkj_core_types::Cell;

use crate::screen::ScreenBuffer;

impl ScreenBuffer {
    /// Scroll up by n lines within scroll region.
    pub fn scroll_up(&mut self, n: u16) {
        let row_len = self.cols as usize;
        let n = n as usize;
        let top = self.scroll_top as usize;
        let bot = self.scroll_bottom as usize;
        let region_h = bot - top + 1;
        if n >= region_h {
            self.clear_region(top, bot);
            return;
        }
        // Save scrolled-off lines to scrollback.
        if top == 0 && !self.alt_screen_active {
            for r in 0..n {
                let start = r * row_len;
                let end = start + row_len;
                if end <= self.cells_ref().len() {
                    let line =
                        self.cells_ref()[start..end].to_vec();
                    self.scrollback.push(line);
                }
            }
            while self.scrollback.len() > self.scrollback_cap {
                self.scrollback.remove(0);
            }
        }
        // Shift rows up within region.
        for r in top..=bot - n {
            let dst = r * row_len;
            let src = (r + n) * row_len;
            for i in 0..row_len {
                self.cells_mut_raw()[dst + i] =
                    self.cells_mut_raw()[src + i].clone();
            }
        }
        self.clear_region(bot - n + 1, bot);
    }

    /// Scroll down by n lines within scroll region.
    pub fn scroll_down(&mut self, n: u16) {
        let row_len = self.cols as usize;
        let n = n as usize;
        let top = self.scroll_top as usize;
        let bot = self.scroll_bottom as usize;
        let region_h = bot - top + 1;
        if n >= region_h {
            self.clear_region(top, bot);
            return;
        }
        // Shift rows down within region.
        for r in (top + n..=bot).rev() {
            let dst = r * row_len;
            let src = (r - n) * row_len;
            for i in 0..row_len {
                self.cells_mut_raw()[dst + i] =
                    self.cells_mut_raw()[src + i].clone();
            }
        }
        self.clear_region(top, top + n - 1);
    }

    /// Set scroll region (DECSTBM). Params are 1-based.
    pub fn set_scroll_region(
        &mut self,
        top: u16,
        bottom: u16,
    ) {
        let t =
            top.saturating_sub(1).min(self.rows - 1);
        let b = bottom
            .saturating_sub(1)
            .min(self.rows - 1);
        if t < b {
            self.scroll_top = t;
            self.scroll_bottom = b;
        }
        self.cursor_col = 0;
        self.cursor_row = 0;
    }

    /// Clear rows [top..=bot] within the cell grid.
    fn clear_region(&mut self, top: usize, bot: usize) {
        let row_len = self.cols as usize;
        for r in top..=bot {
            let start = r * row_len;
            for c in
                &mut self.cells_mut_raw()[start..start + row_len]
            {
                *c = Cell::default();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use kjxlkj_core_types::TerminalId;

    use crate::screen::ScreenBuffer;

    #[test]
    fn scroll_region_up() {
        let mut buf =
            ScreenBuffer::new(TerminalId(1), 80, 24);
        buf.set_scroll_region(1, 10);
        buf.scroll_up(2);
        assert_eq!(buf.cursor_col, 0);
    }

    #[test]
    fn scroll_region_down() {
        let mut buf =
            ScreenBuffer::new(TerminalId(1), 80, 24);
        buf.set_scroll_region(1, 10);
        buf.scroll_down(2);
        assert_eq!(buf.cursor_col, 0);
    }

    #[test]
    fn scrollback_captures() {
        let mut buf =
            ScreenBuffer::new(TerminalId(1), 80, 24);
        buf.put_char('A');
        buf.scroll_up(1);
        assert_eq!(buf.scrollback.len(), 1);
    }
}
