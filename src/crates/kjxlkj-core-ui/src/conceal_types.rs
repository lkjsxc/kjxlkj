//! Conceal types.

use std::ops::Range;

/// Conceal level.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConcealLevel {
    /// No concealment.
    None,
    /// Conceal and show replacement.
    Replace,
    /// Conceal completely.
    Hide,
}

impl Default for ConcealLevel {
    fn default() -> Self {
        Self::None
    }
}

/// A single conceal region.
#[derive(Debug, Clone)]
pub struct ConcealRegion {
    /// Byte range to conceal.
    pub range: Range<usize>,
    /// Replacement character (None = hide completely).
    pub replacement: Option<char>,
    /// Highlight group for the replacement.
    pub highlight: Option<String>,
}

impl ConcealRegion {
    /// Creates a new conceal region.
    pub fn new(range: Range<usize>) -> Self {
        Self {
            range,
            replacement: None,
            highlight: None,
        }
    }

    /// Sets the replacement character.
    pub fn with_replacement(mut self, ch: char) -> Self {
        self.replacement = Some(ch);
        self
    }

    /// Sets the highlight group.
    pub fn with_highlight(mut self, group: &str) -> Self {
        self.highlight = Some(group.to_string());
        self
    }
}

/// Line conceal state.
#[derive(Debug, Clone, Default)]
pub struct LineConceal {
    /// Concealed regions on this line.
    regions: Vec<ConcealRegion>,
}

impl LineConceal {
    /// Creates empty line conceal state.
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds a conceal region.
    pub fn add(&mut self, region: ConcealRegion) {
        self.regions.push(region);
        self.regions.sort_by_key(|r| r.range.start);
    }

    /// Returns the regions.
    pub fn regions(&self) -> &[ConcealRegion] {
        &self.regions
    }

    /// Returns whether a byte offset is concealed.
    pub fn is_concealed(&self, offset: usize) -> bool {
        self.regions.iter().any(|r| r.range.contains(&offset))
    }

    /// Clears all regions.
    pub fn clear(&mut self) {
        self.regions.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_conceal_region() {
        let region = ConcealRegion::new(0..5)
            .with_replacement('…')
            .with_highlight("Conceal");

        assert_eq!(region.replacement, Some('…'));
        assert_eq!(region.highlight.as_deref(), Some("Conceal"));
    }

    #[test]
    fn test_line_conceal() {
        let mut lc = LineConceal::new();
        lc.add(ConcealRegion::new(5..10));

        assert!(lc.is_concealed(7));
        assert!(!lc.is_concealed(3));
    }

    #[test]
    fn test_line_conceal_sorted() {
        let mut lc = LineConceal::new();
        lc.add(ConcealRegion::new(10..15));
        lc.add(ConcealRegion::new(0..5));

        let regions = lc.regions();
        assert_eq!(regions[0].range.start, 0);
        assert_eq!(regions[1].range.start, 10);
    }
}
