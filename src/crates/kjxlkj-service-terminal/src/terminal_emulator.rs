//! Terminal cell grid and basic escape sequence parsing.

/// Style attributes for a terminal cell.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct CellStyle {
    pub bold: bool, pub underline: bool, pub reverse: bool,
    pub fg: Option<u8>, pub bg: Option<u8>,
}

/// A single cell in the terminal grid.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Cell {
    pub ch: char,
    pub style: CellStyle,
}

impl Default for Cell {
    fn default() -> Self { Self { ch: ' ', style: CellStyle::default() } }
}

/// Terminal grid with cursor position.
pub struct TerminalGrid {
    pub width: usize,
    pub height: usize,
    pub cells: Vec<Vec<Cell>>,
    pub cursor_row: usize,
    pub cursor_col: usize,
    pub current_style: CellStyle,
}

impl TerminalGrid {
    pub fn new(width: usize, height: usize) -> Self {
        let cells = vec![vec![Cell::default(); width]; height];
        Self { width, height, cells, cursor_row: 0, cursor_col: 0, current_style: CellStyle::default() }
    }

    /// Write a character at the cursor position and advance.
    pub fn put_char(&mut self, ch: char) {
        if self.cursor_row < self.height && self.cursor_col < self.width {
            self.cells[self.cursor_row][self.cursor_col] = Cell { ch, style: self.current_style };
            self.cursor_col += 1;
            if self.cursor_col >= self.width {
                self.cursor_col = 0;
                self.cursor_row += 1;
            }
        }
    }

    /// Move cursor to (row, col), clamping to grid bounds.
    pub fn move_cursor(&mut self, row: usize, col: usize) {
        self.cursor_row = row.min(self.height.saturating_sub(1));
        self.cursor_col = col.min(self.width.saturating_sub(1));
    }

    /// Clear the entire grid.
    pub fn clear(&mut self) {
        for row in &mut self.cells {
            for cell in row.iter_mut() { *cell = Cell::default(); }
        }
        self.cursor_row = 0;
        self.cursor_col = 0;
    }

    /// Clear from cursor to end of line.
    pub fn clear_to_eol(&mut self) {
        if self.cursor_row < self.height {
            for col in self.cursor_col..self.width {
                self.cells[self.cursor_row][col] = Cell::default();
            }
        }
    }

    /// Scroll the grid up by one line.
    pub fn scroll_up(&mut self) {
        if self.height > 0 {
            self.cells.remove(0);
            self.cells.push(vec![Cell::default(); self.width]);
        }
    }

    /// Read a row as a string (trimming trailing spaces).
    pub fn row_text(&self, row: usize) -> String {
        if row >= self.height { return String::new(); }
        let text: String = self.cells[row].iter().map(|c| c.ch).collect();
        text.trim_end().to_string()
    }
}

/// Parsed ANSI escape action.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AnsiAction {
    Print(char),
    MoveCursor(usize, usize),
    ClearScreen,
    ClearToEol,
    SetBold(bool),
    SetFg(Option<u8>),
    SetBg(Option<u8>),
    Reset,
}

/// Parse a simplified sequence of ANSI escape tokens from raw bytes.
pub fn parse_ansi_simple(input: &str) -> Vec<AnsiAction> {
    let mut actions = Vec::new();
    let chars: Vec<char> = input.chars().collect();
    let mut i = 0;
    while i < chars.len() {
        if chars[i] == '\x1b' && i + 1 < chars.len() && chars[i + 1] == '[' {
            i += 2;
            let mut params = String::new();
            while i < chars.len() && !chars[i].is_ascii_alphabetic() {
                params.push(chars[i]); i += 1;
            }
            if i < chars.len() {
                let cmd = chars[i]; i += 1;
                match cmd {
                    'H' => {
                        let ps: Vec<usize> = params.split(';')
                            .filter_map(|p| p.parse().ok()).collect();
                        let r = ps.first().copied().unwrap_or(1).saturating_sub(1);
                        let c = ps.get(1).copied().unwrap_or(1).saturating_sub(1);
                        actions.push(AnsiAction::MoveCursor(r, c));
                    }
                    'J' => actions.push(AnsiAction::ClearScreen),
                    'K' => actions.push(AnsiAction::ClearToEol),
                    'm' => {
                        let code: usize = params.parse().unwrap_or(0);
                        match code {
                            0 => actions.push(AnsiAction::Reset),
                            1 => actions.push(AnsiAction::SetBold(true)),
                            _ => actions.push(AnsiAction::Reset),
                        }
                    }
                    _ => {}
                }
            }
        } else {
            actions.push(AnsiAction::Print(chars[i])); i += 1;
        }
    }
    actions
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn grid_put_char() {
        let mut g = TerminalGrid::new(10, 5);
        g.put_char('A');
        assert_eq!(g.cells[0][0].ch, 'A'); assert_eq!(g.cursor_col, 1);
    }

    #[test]
    fn grid_clear_to_eol() {
        let mut g = TerminalGrid::new(10, 5);
        g.put_char('A'); g.put_char('B'); g.put_char('C');
        g.move_cursor(0, 1); g.clear_to_eol();
        assert_eq!(g.row_text(0), "A");
    }

    #[test]
    fn grid_scroll_up() {
        let mut g = TerminalGrid::new(5, 3);
        g.put_char('X'); g.scroll_up();
        assert_eq!(g.row_text(0), "");
    }

    #[test]
    fn grid_row_text() {
        let mut g = TerminalGrid::new(10, 2);
        for c in "hello".chars() { g.put_char(c); }
        assert_eq!(g.row_text(0), "hello");
    }

    #[test]
    fn parse_print_chars() {
        let actions = parse_ansi_simple("AB");
        assert_eq!(actions.len(), 2); assert_eq!(actions[0], AnsiAction::Print('A'));
    }

    #[test]
    fn parse_cursor_move() {
        assert_eq!(parse_ansi_simple("\x1b[3;5H"), vec![AnsiAction::MoveCursor(2, 4)]);
    }

    #[test]
    fn parse_clear_screen() {
        assert_eq!(parse_ansi_simple("\x1b[2J"), vec![AnsiAction::ClearScreen]);
    }

    #[test]
    fn grid_wrap_at_width() {
        let mut g = TerminalGrid::new(3, 2);
        for c in "ABCD".chars() { g.put_char(c); }
        assert_eq!(g.row_text(0), "ABC");
        assert_eq!(g.cells[1][0].ch, 'D');
    }
}
