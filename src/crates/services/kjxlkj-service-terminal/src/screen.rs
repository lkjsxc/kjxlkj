//! Terminal screen model — cell grid, cursor, styles.
//! See /docs/spec/features/terminal/terminal.md.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Attr { Bold, Dim, Italic, Underline, Reverse, Strikethrough }

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Color { Default, Basic(u16), Color256(u16), Rgb(u16, u16, u16) }

#[derive(Debug, Clone)]
pub struct Cell {
    pub ch: char,
    pub fg: Color, pub bg: Color,
    pub bold: bool, pub dim: bool, pub italic: bool,
    pub underline: bool, pub reverse: bool, pub strikethrough: bool,
}
impl Cell {
    pub fn blank() -> Self {
        Self { ch: ' ', fg: Color::Default, bg: Color::Default,
            bold: false, dim: false, italic: false,
            underline: false, reverse: false, strikethrough: false }
    }
}

pub struct Screen {
    pub cols: u16, pub rows: u16,
    pub cursor_row: u16, pub cursor_col: u16,
    cells: Vec<Cell>,
    pub title: String,
    saved_row: u16, saved_col: u16,
    scroll_top: u16, scroll_bottom: u16,
    cur_attr: Cell,
    pub cursor_visible: bool,
    pub alt_screen: bool,
    pub bracketed_paste: bool,
}
impl Screen {
    pub fn new(cols: u16, rows: u16) -> Self {
        let n = (cols as usize) * (rows as usize);
        Self {
            cols, rows, cursor_row: 0, cursor_col: 0,
            cells: vec![Cell::blank(); n], title: String::new(),
            saved_row: 0, saved_col: 0, scroll_top: 0, scroll_bottom: rows.saturating_sub(1),
            cur_attr: Cell::blank(), cursor_visible: true,
            alt_screen: false, bracketed_paste: false,
        }
    }
    fn idx(&self, row: u16, col: u16) -> usize { (row as usize) * (self.cols as usize) + col as usize }
    pub fn cell(&self, row: u16, col: u16) -> &Cell { &self.cells[self.idx(row, col)] }
    pub fn put_char(&mut self, ch: char) {
        if self.cursor_col >= self.cols { self.carriage_return(); self.linefeed(); }
        let i = self.idx(self.cursor_row, self.cursor_col);
        let a = &self.cur_attr;
        let c = &mut self.cells[i];
        c.ch = ch; c.bold = a.bold; c.dim = a.dim; c.italic = a.italic;
        c.underline = a.underline; c.reverse = a.reverse; c.strikethrough = a.strikethrough;
        c.fg = a.fg; c.bg = a.bg;
        self.cursor_col += 1;
    }
    pub fn linefeed(&mut self) {
        if self.cursor_row >= self.scroll_bottom { self.scroll_up(1); }
        else { self.cursor_row += 1; }
    }
    pub fn carriage_return(&mut self) { self.cursor_col = 0; }
    pub fn backspace(&mut self) { self.cursor_col = self.cursor_col.saturating_sub(1); }
    pub fn tab(&mut self) { self.cursor_col = ((self.cursor_col / 8) + 1) * 8; if self.cursor_col >= self.cols { self.cursor_col = self.cols - 1; } }
    pub fn reverse_index(&mut self) {
        if self.cursor_row <= self.scroll_top { self.scroll_down(1); }
        else { self.cursor_row -= 1; }
    }
    pub fn move_cursor_up(&mut self, n: u16) { self.cursor_row = self.cursor_row.saturating_sub(n).max(self.scroll_top); }
    pub fn move_cursor_down(&mut self, n: u16) { self.cursor_row = (self.cursor_row + n).min(self.scroll_bottom); }
    pub fn move_cursor_right(&mut self, n: u16) { self.cursor_col = (self.cursor_col + n).min(self.cols.saturating_sub(1)); }
    pub fn move_cursor_left(&mut self, n: u16) { self.cursor_col = self.cursor_col.saturating_sub(n); }
    pub fn set_cursor_col(&mut self, col: u16) { self.cursor_col = col.min(self.cols.saturating_sub(1)); }
    pub fn set_cursor_row(&mut self, row: u16) { self.cursor_row = row.min(self.rows.saturating_sub(1)); }
    pub fn set_cursor_pos(&mut self, row: u16, col: u16) { self.set_cursor_row(row); self.set_cursor_col(col); }
    pub fn save_cursor(&mut self) { self.saved_row = self.cursor_row; self.saved_col = self.cursor_col; }
    pub fn restore_cursor(&mut self) { self.cursor_row = self.saved_row; self.cursor_col = self.saved_col; }
    pub fn erase_display(&mut self, mode: u16) {
        match mode {
            0 => { // erase below
                for c in self.cursor_col..self.cols { let i = self.idx(self.cursor_row, c); self.cells[i] = Cell::blank(); }
                for r in (self.cursor_row + 1)..self.rows { for c in 0..self.cols { let i = self.idx(r, c); self.cells[i] = Cell::blank(); } }
            }
            1 => { // erase above
                for r in 0..self.cursor_row { for c in 0..self.cols { let i = self.idx(r, c); self.cells[i] = Cell::blank(); } }
                for c in 0..=self.cursor_col { let i = self.idx(self.cursor_row, c); self.cells[i] = Cell::blank(); }
            }
            2 | 3 => { self.cells.iter_mut().for_each(|c| *c = Cell::blank()); }
            _ => {}
        }
    }
    pub fn erase_line(&mut self, mode: u16) {
        let r = self.cursor_row;
        match mode {
            0 => { for c in self.cursor_col..self.cols { let i = self.idx(r, c); self.cells[i] = Cell::blank(); } }
            1 => { for c in 0..=self.cursor_col.min(self.cols - 1) { let i = self.idx(r, c); self.cells[i] = Cell::blank(); } }
            2 => { for c in 0..self.cols { let i = self.idx(r, c); self.cells[i] = Cell::blank(); } }
            _ => {}
        }
    }
    pub fn erase_chars(&mut self, n: u16) {
        for d in 0..n { let c = self.cursor_col + d; if c < self.cols { let i = self.idx(self.cursor_row, c); self.cells[i] = Cell::blank(); } }
    }
    pub fn insert_chars(&mut self, n: u16) {
        let r = self.cursor_row; let c = self.cursor_col as usize; let w = self.cols as usize; let n = n as usize;
        let start = self.idx(r, 0);
        let row = &mut self.cells[start..start + w];
        for i in (c + n..w).rev() { row[i] = row[i - n].clone(); }
        for i in c..(c + n).min(w) { row[i] = Cell::blank(); }
    }
    pub fn delete_chars(&mut self, n: u16) {
        let r = self.cursor_row; let c = self.cursor_col as usize; let w = self.cols as usize; let n = n as usize;
        let start = self.idx(r, 0);
        let row = &mut self.cells[start..start + w];
        for i in c..w { row[i] = if i + n < w { row[i + n].clone() } else { Cell::blank() }; }
    }
    pub fn scroll_up(&mut self, n: u16) {
        let top = self.scroll_top as usize; let bot = self.scroll_bottom as usize; let w = self.cols as usize;
        for _ in 0..n as usize {
            for r in top..bot { for c in 0..w { let s = (r + 1) * w + c; let d = r * w + c; self.cells[d] = self.cells[s].clone(); } }
            for c in 0..w { self.cells[bot * w + c] = Cell::blank(); }
        }
    }
    pub fn scroll_down(&mut self, n: u16) {
        let top = self.scroll_top as usize; let bot = self.scroll_bottom as usize; let w = self.cols as usize;
        for _ in 0..n as usize {
            for r in (top + 1..=bot).rev() { for c in 0..w { let s = (r - 1) * w + c; let d = r * w + c; self.cells[d] = self.cells[s].clone(); } }
            for c in 0..w { self.cells[top * w + c] = Cell::blank(); }
        }
    }
    pub fn insert_lines(&mut self, n: u16) {
        let saved = self.cursor_row; self.cursor_row = self.cursor_row; self.scroll_down(n); self.cursor_row = saved; self.cursor_col = 0;
    }
    pub fn delete_lines(&mut self, n: u16) {
        let saved = self.cursor_row; self.scroll_up(n); self.cursor_row = saved; self.cursor_col = 0;
    }
    pub fn set_scroll_region(&mut self, top: u16, bottom: u16) {
        self.scroll_top = top;
        self.scroll_bottom = if bottom == 0 { self.rows - 1 } else { (bottom - 1).min(self.rows - 1) };
        self.cursor_row = 0; self.cursor_col = 0;
    }
    pub fn reset(&mut self) { *self = Screen::new(self.cols, self.rows); }
    pub fn set_title(&mut self, title: String) { self.title = title; }
    pub fn set_private_mode(&mut self, mode: u16, on: bool) {
        match mode {
            25 => self.cursor_visible = on,
            47 | 1049 => self.alt_screen = on,
            2004 => self.bracketed_paste = on,
            _ => {} // track but ignore mouse modes etc.
        }
    }
    pub fn reset_attr(&mut self) { self.cur_attr = Cell::blank(); }
    pub fn set_attr(&mut self, attr: Attr, on: bool) {
        match attr {
            Attr::Bold => self.cur_attr.bold = on,
            Attr::Dim => self.cur_attr.dim = on,
            Attr::Italic => self.cur_attr.italic = on,
            Attr::Underline => self.cur_attr.underline = on,
            Attr::Reverse => self.cur_attr.reverse = on,
            Attr::Strikethrough => self.cur_attr.strikethrough = on,
        }
    }
    pub fn set_fg(&mut self, n: u16) { self.cur_attr.fg = Color::Basic(n); }
    pub fn set_bg(&mut self, n: u16) { self.cur_attr.bg = Color::Basic(n); }
    pub fn set_fg_256(&mut self, n: u16) { self.cur_attr.fg = Color::Color256(n); }
    pub fn set_bg_256(&mut self, n: u16) { self.cur_attr.bg = Color::Color256(n); }
    pub fn set_fg_rgb(&mut self, r: u16, g: u16, b: u16) { self.cur_attr.fg = Color::Rgb(r, g, b); }
    pub fn set_bg_rgb(&mut self, r: u16, g: u16, b: u16) { self.cur_attr.bg = Color::Rgb(r, g, b); }
    pub fn reset_fg(&mut self) { self.cur_attr.fg = Color::Default; }
    pub fn reset_bg(&mut self) { self.cur_attr.bg = Color::Default; }
}

#[cfg(test)]
#[path = "screen_tests.rs"]
mod tests;
