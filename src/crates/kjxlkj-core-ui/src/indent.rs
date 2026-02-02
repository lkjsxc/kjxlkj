//! Indent guides.
//!
//! Visual indentation guides.

/// Indent guide character style.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GuideStyle {
    /// Solid vertical line.
    Solid,
    /// Dotted vertical line.
    Dotted,
    /// Dashed vertical line.
    Dashed,
}

impl Default for GuideStyle {
    fn default() -> Self {
        Self::Solid
    }
}

/// Indent guide configuration.
#[derive(Debug, Clone)]
pub struct IndentGuide {
    /// Whether enabled.
    pub enabled: bool,
    /// Guide character.
    pub char: char,
    /// Style.
    pub style: GuideStyle,
    /// Tab width for calculation.
    pub tab_width: usize,
}

impl Default for IndentGuide {
    fn default() -> Self {
        Self {
            enabled: false,
            char: '│',
            style: GuideStyle::Solid,
            tab_width: 4,
        }
    }
}

impl IndentGuide {
    /// Creates new indent guide config.
    pub fn new() -> Self {
        Self::default()
    }

    /// Enables indent guides.
    pub fn enable(mut self) -> Self {
        self.enabled = true;
        self
    }

    /// Sets tab width.
    pub fn with_tab_width(mut self, width: usize) -> Self {
        self.tab_width = width;
        self
    }

    /// Calculates guide positions for a line.
    pub fn positions(&self, indent: usize) -> Vec<usize> {
        if !self.enabled || self.tab_width == 0 {
            return Vec::new();
        }

        let mut positions = Vec::new();
        let mut col = self.tab_width;
        while col < indent {
            positions.push(col);
            col += self.tab_width;
        }
        positions
    }

    /// Returns the guide character.
    pub fn guide_char(&self) -> char {
        match self.style {
            GuideStyle::Solid => self.char,
            GuideStyle::Dotted => '┆',
            GuideStyle::Dashed => '┊',
        }
    }
}

/// Calculated indent level.
pub fn indent_level(line: &str, tab_width: usize) -> usize {
    let mut level = 0;
    for ch in line.chars() {
        match ch {
            ' ' => level += 1,
            '\t' => level += tab_width - (level % tab_width),
            _ => break,
        }
    }
    level
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_guide_style_default() {
        assert_eq!(GuideStyle::default(), GuideStyle::Solid);
    }

    #[test]
    fn test_indent_guide_new() {
        let guide = IndentGuide::new();
        assert!(!guide.enabled);
    }

    #[test]
    fn test_indent_guide_positions() {
        let guide = IndentGuide::new().enable().with_tab_width(4);
        let positions = guide.positions(12);
        assert_eq!(positions, vec![4, 8]);
    }

    #[test]
    fn test_indent_guide_positions_disabled() {
        let guide = IndentGuide::new();
        assert!(guide.positions(12).is_empty());
    }

    #[test]
    fn test_indent_level_spaces() {
        assert_eq!(indent_level("    hello", 4), 4);
        assert_eq!(indent_level("        hello", 4), 8);
    }

    #[test]
    fn test_indent_level_tabs() {
        assert_eq!(indent_level("\thello", 4), 4);
        assert_eq!(indent_level("\t\thello", 4), 8);
    }
}
