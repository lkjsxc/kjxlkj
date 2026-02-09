//! Screen buffer operations: DECSET/DECRST, erase,
//! and CSI dispatch.

use kjxlkj_core_types::Cell;

use crate::screen::ScreenBuffer;

impl ScreenBuffer {
    /// Handle DECSET private mode.
    pub fn decset(&mut self, mode: u16) {
        match mode {
            25 => self.cursor_visible = true,
            1049 => {
                self.alt_screen_active = true;
                self.clear();
            }
            _ => {}
        }
    }

    /// Handle DECRST private mode.
    pub fn decrst(&mut self, mode: u16) {
        match mode {
            25 => self.cursor_visible = false,
            1049 => {
                self.alt_screen_active = false;
                self.clear();
            }
            _ => {}
        }
    }

    /// Erase in display (ED).
    pub fn erase_display(&mut self, mode: u16) {
        let row_len = self.cols as usize;
        match mode {
            0 => {
                let start = self.cursor_row as usize * row_len + self.cursor_col as usize;
                for c in &mut self.cells_mut_raw()[start..] {
                    *c = Cell::default();
                }
            }
            1 => {
                let end = self.cursor_row as usize * row_len + self.cursor_col as usize + 1;
                let len = self.cells_ref().len();
                let end = end.min(len);
                for c in &mut self.cells_mut_raw()[..end] {
                    *c = Cell::default();
                }
            }
            2 | 3 => self.clear(),
            _ => {}
        }
    }

    /// Erase in line (EL).
    pub fn erase_line(&mut self, mode: u16) {
        let row_len = self.cols as usize;
        let row_start = self.cursor_row as usize * row_len;
        let len = self.cells_ref().len();
        match mode {
            0 => {
                let start = row_start + self.cursor_col as usize;
                let end = (row_start + row_len).min(len);
                for c in &mut self.cells_mut_raw()[start..end] {
                    *c = Cell::default();
                }
            }
            1 => {
                let end = (row_start + self.cursor_col as usize + 1).min(len);
                for c in &mut self.cells_mut_raw()[row_start..end] {
                    *c = Cell::default();
                }
            }
            2 => {
                let end = (row_start + row_len).min(len);
                for c in &mut self.cells_mut_raw()[row_start..end] {
                    *c = Cell::default();
                }
            }
            _ => {}
        }
    }

    /// Erase characters (ECH).
    pub fn erase_chars(&mut self, n: u16) {
        let row_len = self.cols as usize;
        let row_start = self.cursor_row as usize * row_len;
        let start = row_start + self.cursor_col as usize;
        let end = (start + n as usize)
            .min(row_start + row_len)
            .min(self.cells_ref().len());
        for c in &mut self.cells_mut_raw()[start..end] {
            *c = Cell::default();
        }
    }
}

#[cfg(test)]
mod tests {
    use kjxlkj_core_types::TerminalId;

    use crate::screen::ScreenBuffer;

    #[test]
    fn decset_cursor_visibility() {
        let mut buf = ScreenBuffer::new(TerminalId(1), 80, 24);
        buf.decrst(25);
        assert!(!buf.cursor_visible);
        buf.decset(25);
        assert!(buf.cursor_visible);
    }

    #[test]
    fn erase_display_clears() {
        let mut buf = ScreenBuffer::new(TerminalId(1), 80, 24);
        buf.put_char('A');
        buf.erase_display(2);
        assert_eq!(buf.cells()[0].grapheme.as_str(), " ");
    }

    #[test]
    fn alt_screen_toggle() {
        let mut buf = ScreenBuffer::new(TerminalId(1), 80, 24);
        buf.decset(1049);
        assert!(buf.alt_screen_active);
        buf.decrst(1049);
        assert!(!buf.alt_screen_active);
    }
}
