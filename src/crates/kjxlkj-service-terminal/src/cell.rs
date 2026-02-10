//! Terminal cell type and style attributes.

/// A single cell in the terminal grid.
#[derive(Debug, Clone, PartialEq)]
pub struct Cell {
    /// The character displayed in this cell.
    pub ch: char,
    /// Display width of the grapheme (1 or 2).
    pub width: u8,
    /// True if this is a continuation cell of a wide character.
    pub is_wide_continuation: bool,
    /// Style attributes for this cell.
    pub attrs: CellAttrs,
}

impl Default for Cell {
    fn default() -> Self {
        Self {
            ch: ' ',
            width: 1,
            is_wide_continuation: false,
            attrs: CellAttrs::default(),
        }
    }
}

/// Style attributes for a cell.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct CellAttrs {
    pub bold: bool,
    pub dim: bool,
    pub italic: bool,
    pub underline: bool,
    pub reverse: bool,
    pub hidden: bool,
    pub strikethrough: bool,
    pub fg: Color,
    pub bg: Color,
}

impl CellAttrs {
    /// Reset all attributes to default.
    pub fn reset(&mut self) {
        *self = Self::default();
    }
}

/// Terminal color.
#[derive(Debug, Clone, PartialEq, Default)]
pub enum Color {
    #[default]
    Default,
    Indexed(u8),
    Rgb(u8, u8, u8),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cell_default() {
        let c = Cell::default();
        assert_eq!(c.ch, ' ');
        assert_eq!(c.width, 1);
        assert!(!c.is_wide_continuation);
    }

    #[test]
    fn test_attrs_reset() {
        let mut a = CellAttrs {
            bold: true,
            italic: true,
            ..Default::default()
        };
        a.reset();
        assert!(!a.bold);
        assert!(!a.italic);
    }
}
