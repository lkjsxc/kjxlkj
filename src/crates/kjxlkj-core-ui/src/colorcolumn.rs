//! Column marker (colorcolumn).
//!
//! Visual markers at specific columns.

use std::collections::HashSet;

/// Column marker configuration.
#[derive(Debug, Clone, Default)]
pub struct ColorColumn {
    /// Columns to mark (0-indexed).
    columns: HashSet<usize>,
    /// Relative offset from textwidth.
    relative: Vec<i32>,
    /// Text width for relative calculation.
    textwidth: usize,
}

impl ColorColumn {
    /// Creates a new column marker.
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds a column marker.
    pub fn add(&mut self, column: usize) {
        self.columns.insert(column);
    }

    /// Adds relative column from textwidth.
    pub fn add_relative(&mut self, offset: i32) {
        self.relative.push(offset);
    }

    /// Removes a column marker.
    pub fn remove(&mut self, column: usize) {
        self.columns.remove(&column);
    }

    /// Sets the textwidth for relative columns.
    pub fn set_textwidth(&mut self, width: usize) {
        self.textwidth = width;
    }

    /// Clears all column markers.
    pub fn clear(&mut self) {
        self.columns.clear();
        self.relative.clear();
    }

    /// Returns whether a column should be marked.
    pub fn is_marked(&self, column: usize) -> bool {
        if self.columns.contains(&column) {
            return true;
        }

        // Check relative columns
        for &offset in &self.relative {
            let target = if offset >= 0 {
                self.textwidth.saturating_add(offset as usize)
            } else {
                self.textwidth.saturating_sub((-offset) as usize)
            };
            if column == target {
                return true;
            }
        }

        false
    }

    /// Returns all marked columns.
    pub fn marked_columns(&self) -> Vec<usize> {
        let mut cols: Vec<usize> = self.columns.iter().copied().collect();
        for &offset in &self.relative {
            let target = if offset >= 0 {
                self.textwidth.saturating_add(offset as usize)
            } else {
                self.textwidth.saturating_sub((-offset) as usize)
            };
            if !cols.contains(&target) {
                cols.push(target);
            }
        }
        cols.sort();
        cols
    }

    /// Returns whether any columns are marked.
    pub fn is_enabled(&self) -> bool {
        !self.columns.is_empty() || !self.relative.is_empty()
    }

    /// Parses a colorcolumn string (e.g., "80,+1,-1").
    pub fn parse(spec: &str, textwidth: usize) -> Self {
        let mut cc = Self::new();
        cc.textwidth = textwidth;

        for part in spec.split(',') {
            let part = part.trim();
            if part.is_empty() {
                continue;
            }

            if part.starts_with('+') || part.starts_with('-') {
                if let Ok(offset) = part.parse::<i32>() {
                    cc.add_relative(offset);
                }
            } else if let Ok(col) = part.parse::<usize>() {
                cc.add(col);
            }
        }

        cc
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_column_new() {
        let cc = ColorColumn::new();
        assert!(!cc.is_enabled());
    }

    #[test]
    fn test_color_column_add() {
        let mut cc = ColorColumn::new();
        cc.add(80);
        assert!(cc.is_marked(80));
        assert!(!cc.is_marked(79));
    }

    #[test]
    fn test_color_column_relative() {
        let mut cc = ColorColumn::new();
        cc.set_textwidth(80);
        cc.add_relative(0);
        assert!(cc.is_marked(80));
    }

    #[test]
    fn test_color_column_parse() {
        let cc = ColorColumn::parse("80,100", 0);
        assert!(cc.is_marked(80));
        assert!(cc.is_marked(100));
    }

    #[test]
    fn test_color_column_parse_relative() {
        let cc = ColorColumn::parse("+1,-1", 80);
        assert!(cc.is_marked(81));
        assert!(cc.is_marked(79));
    }

    #[test]
    fn test_color_column_marked_columns() {
        let mut cc = ColorColumn::new();
        cc.add(80);
        cc.add(120);
        let cols = cc.marked_columns();
        assert_eq!(cols, vec![80, 120]);
    }
}
