//! Layout invariant checking for rendered frames.
//!
//! Validates that rendered output satisfies key layout constraints:
//! no overlapping regions, cursor within bounds, status line present.

/// A rectangular region in the terminal frame.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LayoutRect {
    pub label: String,
    pub row: usize,
    pub col: usize,
    pub width: usize,
    pub height: usize,
}

impl LayoutRect {
    /// Check if this rect overlaps with another.
    pub fn overlaps(&self, other: &LayoutRect) -> bool {
        if self.width == 0 || self.height == 0 || other.width == 0 || other.height == 0 {
            return false;
        }
        let self_right = self.col + self.width;
        let self_bottom = self.row + self.height;
        let other_right = other.col + other.width;
        let other_bottom = other.row + other.height;
        self.col < other_right && self_right > other.col
            && self.row < other_bottom && self_bottom > other.row
    }

    /// Check if this rect fits within the given terminal dimensions.
    pub fn fits_in(&self, term_width: usize, term_height: usize) -> bool {
        self.col + self.width <= term_width && self.row + self.height <= term_height
    }
}

/// Layout violation found during invariant checking.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LayoutViolation {
    pub kind: ViolationKind,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ViolationKind {
    Overlap,
    OutOfBounds,
    MissingStatusLine,
    CursorOutOfBounds,
    GapInLayout,
}

/// Check all layout invariants for a set of regions.
pub fn check_layout_invariants(
    regions: &[LayoutRect],
    cursor_row: usize,
    cursor_col: usize,
    term_width: usize,
    term_height: usize,
    expect_statusline: bool,
) -> Vec<LayoutViolation> {
    let mut violations = Vec::new();
    // Check overlaps
    for i in 0..regions.len() {
        for j in (i + 1)..regions.len() {
            if regions[i].overlaps(&regions[j]) {
                violations.push(LayoutViolation {
                    kind: ViolationKind::Overlap,
                    message: format!("'{}' overlaps '{}'", regions[i].label, regions[j].label),
                });
            }
        }
    }
    // Check bounds
    for r in regions {
        if !r.fits_in(term_width, term_height) {
            violations.push(LayoutViolation {
                kind: ViolationKind::OutOfBounds,
                message: format!("'{}' exceeds terminal bounds ({}x{})", r.label, term_width, term_height),
            });
        }
    }
    // Check cursor
    if cursor_row >= term_height || cursor_col >= term_width {
        violations.push(LayoutViolation {
            kind: ViolationKind::CursorOutOfBounds,
            message: format!("cursor ({},{}) outside terminal ({}x{})", cursor_row, cursor_col, term_width, term_height),
        });
    }
    // Check statusline
    if expect_statusline && !regions.iter().any(|r| r.label == "statusline") {
        violations.push(LayoutViolation {
            kind: ViolationKind::MissingStatusLine,
            message: "expected statusline region not found".into(),
        });
    }
    violations
}

/// Check that regions tile without gaps in the vertical direction.
pub fn check_vertical_coverage(regions: &[LayoutRect], term_height: usize) -> bool {
    let mut covered = vec![false; term_height];
    for r in regions {
        for row in r.row..(r.row + r.height).min(term_height) {
            covered[row] = true;
        }
    }
    covered.iter().all(|&c| c)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn rect(label: &str, row: usize, col: usize, w: usize, h: usize) -> LayoutRect {
        LayoutRect { label: label.into(), row, col, width: w, height: h }
    }

    #[test]
    fn no_overlap() {
        let a = rect("a", 0, 0, 10, 5);
        let b = rect("b", 5, 0, 10, 5);
        assert!(!a.overlaps(&b));
    }

    #[test]
    fn overlap_detected() {
        let a = rect("a", 0, 0, 10, 5);
        let b = rect("b", 3, 5, 10, 5);
        assert!(a.overlaps(&b));
    }

    #[test]
    fn fits_in_bounds() {
        let r = rect("r", 0, 0, 80, 24);
        assert!(r.fits_in(80, 24));
        assert!(!r.fits_in(79, 24));
    }

    #[test]
    fn check_no_violations() {
        let regions = vec![
            rect("buffer", 0, 0, 80, 23),
            rect("statusline", 23, 0, 80, 1),
        ];
        let v = check_layout_invariants(&regions, 5, 10, 80, 24, true);
        assert!(v.is_empty());
    }

    #[test]
    fn cursor_out_of_bounds() {
        let regions = vec![rect("buffer", 0, 0, 80, 24)];
        let v = check_layout_invariants(&regions, 25, 0, 80, 24, false);
        assert!(v.iter().any(|x| x.kind == ViolationKind::CursorOutOfBounds));
    }

    #[test]
    fn missing_statusline() {
        let regions = vec![rect("buffer", 0, 0, 80, 24)];
        let v = check_layout_invariants(&regions, 0, 0, 80, 24, true);
        assert!(v.iter().any(|x| x.kind == ViolationKind::MissingStatusLine));
    }

    #[test]
    fn vertical_coverage() {
        let regions = vec![rect("a", 0, 0, 80, 12), rect("b", 12, 0, 80, 12)];
        assert!(check_vertical_coverage(&regions, 24));
    }

    #[test]
    fn vertical_gap() {
        let regions = vec![rect("a", 0, 0, 80, 10), rect("b", 15, 0, 80, 9)];
        assert!(!check_vertical_coverage(&regions, 24));
    }
}
