//! Terminal emulator grid: cells, styles, and basic ANSI parsing.

use serde::{Deserialize, Serialize};

/// Terminal cell style.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct Style {
    pub fg: u8,
    pub bg: u8,
    pub bold: bool,
    pub italic: bool,
    pub underline: bool,
}

/// A single cell in the terminal grid.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Cell {
    pub ch: char,
    pub style: Style,
}

impl Default for Cell {
    fn default() -> Self {
        Self {
            ch: ' ',
            style: Style::default(),
        }
    }
}

/// ANSI action parsed from input.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AnsiAction {
    Print(char),
    CursorMove(u16, u16),
    ClearScreen,
    ClearToEol,
    SetStyle(Style),
}

/// Terminal grid with cells and cursor.
#[derive(Debug)]
pub struct TerminalGrid {
    pub width: u16,
    pub height: u16,
    pub cells: Vec<Vec<Cell>>,
    pub cursor_x: u16,
    pub cursor_y: u16,
    pub current_style: Style,
}

impl TerminalGrid {
    pub fn new(width: u16, height: u16) -> Self {
        let cells = vec![vec![Cell::default(); width as usize]; height as usize];
        Self {
            width,
            height,
            cells,
            cursor_x: 0,
            cursor_y: 0,
            current_style: Style::default(),
        }
    }

    /// Write a character at the cursor and advance.
    pub fn put_char(&mut self, ch: char) {
        let (x, y) = (self.cursor_x as usize, self.cursor_y as usize);
        if y < self.height as usize && x < self.width as usize {
            self.cells[y][x] = Cell {
                ch,
                style: self.current_style,
            };
            self.cursor_x += 1;
            if self.cursor_x >= self.width {
                self.cursor_x = 0;
                if self.cursor_y + 1 < self.height {
                    self.cursor_y += 1;
                } else {
                    self.scroll_up();
                }
            }
        }
    }

    /// Clear the entire grid.
    pub fn clear(&mut self) {
        for row in &mut self.cells {
            for cell in row.iter_mut() {
                *cell = Cell::default();
            }
        }
        self.cursor_x = 0;
        self.cursor_y = 0;
    }

    /// Clear from cursor to end of line.
    pub fn clear_to_eol(&mut self) {
        let y = self.cursor_y as usize;
        if y < self.height as usize {
            for x in (self.cursor_x as usize)..(self.width as usize) {
                self.cells[y][x] = Cell::default();
            }
        }
    }

    /// Scroll the grid up by one line.
    pub fn scroll_up(&mut self) {
        if !self.cells.is_empty() {
            self.cells.remove(0);
            self.cells.push(vec![Cell::default(); self.width as usize]);
        }
    }
}

/// Simplified ANSI parser (handles printable chars and basic CSI sequences).
pub fn parse_ansi_simple(input: &str) -> Vec<AnsiAction> {
    let mut actions = Vec::new();
    let mut chars = input.chars().peekable();

    while let Some(ch) = chars.next() {
        if ch == '\x1b' {
            if chars.peek() == Some(&'[') {
                chars.next(); // consume '['
                let mut param = String::new();
                while let Some(&c) = chars.peek() {
                    if c.is_ascii_digit() || c == ';' {
                        param.push(c);
                        chars.next();
                    } else {
                        break;
                    }
                }
                if let Some(cmd) = chars.next() {
                    match cmd {
                        'H' => {
                            let parts: Vec<u16> =
                                param.split(';').filter_map(|s| s.parse().ok()).collect();
                            let row = parts.first().copied().unwrap_or(1).saturating_sub(1);
                            let col = parts.get(1).copied().unwrap_or(1).saturating_sub(1);
                            actions.push(AnsiAction::CursorMove(col, row));
                        }
                        'J' => actions.push(AnsiAction::ClearScreen),
                        'K' => actions.push(AnsiAction::ClearToEol),
                        'm' => {
                            let mut style = Style::default();
                            for code in param.split(';').filter_map(|s| s.parse::<u8>().ok()) {
                                match code {
                                    1 => style.bold = true,
                                    3 => style.italic = true,
                                    4 => style.underline = true,
                                    30..=37 => style.fg = code - 30,
                                    40..=47 => style.bg = code - 40,
                                    _ => {}
                                }
                            }
                            actions.push(AnsiAction::SetStyle(style));
                        }
                        _ => {}
                    }
                }
            }
        } else if !ch.is_control() {
            actions.push(AnsiAction::Print(ch));
        }
    }
    actions
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn grid_put_and_clear() {
        let mut g = TerminalGrid::new(5, 3);
        g.put_char('A');
        assert_eq!(g.cells[0][0].ch, 'A');
        g.clear();
        assert_eq!(g.cells[0][0].ch, ' ');
    }

    #[test]
    fn parse_print_chars() {
        let actions = parse_ansi_simple("AB");
        assert_eq!(
            actions,
            vec![AnsiAction::Print('A'), AnsiAction::Print('B')]
        );
    }

    #[test]
    fn parse_cursor_move() {
        let actions = parse_ansi_simple("\x1b[5;10H");
        assert_eq!(actions, vec![AnsiAction::CursorMove(9, 4)]);
    }
}
