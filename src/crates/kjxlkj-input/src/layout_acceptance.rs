/// Layout invariants — acceptance criteria for UI layout correctness.

/// Layout invariant kinds.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InvariantKind { NoOverlap, FullCoverage, MinSize, CursorVisible, CmdLinePresent, StatusLinePresent }

/// Result of an invariant check.
#[derive(Debug, Clone)]
pub struct InvariantResult { pub kind: InvariantKind, pub passed: bool, pub detail: String }

/// A region in the layout.
#[derive(Debug, Clone, Copy)]
pub struct LayoutRegion { pub row: u16, pub col: u16, pub width: u16, pub height: u16 }

impl LayoutRegion {
    pub fn new(row: u16, col: u16, w: u16, h: u16) -> Self { Self { row, col, width: w, height: h } }
    pub fn bottom(&self) -> u16 { self.row + self.height }
    pub fn right(&self) -> u16 { self.col + self.width }
    pub fn overlaps(&self, other: &LayoutRegion) -> bool {
        self.row < other.bottom() && other.row < self.bottom() && self.col < other.right() && other.col < self.right()
    }
    pub fn area(&self) -> u32 { self.width as u32 * self.height as u32 }
}

/// Check no two regions overlap.
pub fn check_no_overlap(regions: &[LayoutRegion]) -> InvariantResult {
    for i in 0..regions.len() {
        for j in (i + 1)..regions.len() {
            if regions[i].overlaps(&regions[j]) {
                return InvariantResult { kind: InvariantKind::NoOverlap, passed: false,
                    detail: format!("region {} overlaps region {}", i, j) };
            }
        }
    }
    InvariantResult { kind: InvariantKind::NoOverlap, passed: true, detail: "no overlaps".into() }
}

/// Check that regions cover the full screen area (approximate via total area).
pub fn check_coverage(regions: &[LayoutRegion], screen_w: u16, screen_h: u16) -> InvariantResult {
    let total: u32 = regions.iter().map(|r| r.area()).sum();
    let screen = screen_w as u32 * screen_h as u32;
    let passed = total >= screen;
    InvariantResult { kind: InvariantKind::FullCoverage, passed,
        detail: format!("covered {} of {} cells", total, screen) }
}

/// Check minimum editor area size (at least 1 line of editing).
pub fn check_min_size(editor_h: u16) -> InvariantResult {
    let passed = editor_h >= 1;
    InvariantResult { kind: InvariantKind::MinSize, passed,
        detail: format!("editor height: {}", editor_h) }
}

/// Check cursor is within a visible region.
pub fn check_cursor_visible(cursor_row: u16, cursor_col: u16, regions: &[LayoutRegion]) -> InvariantResult {
    let visible = regions.iter().any(|r| r.row <= cursor_row && cursor_row < r.bottom() && r.col <= cursor_col && cursor_col < r.right());
    InvariantResult { kind: InvariantKind::CursorVisible, passed: visible,
        detail: format!("cursor at ({}, {})", cursor_row, cursor_col) }
}

/// Run all layout invariants.
pub fn run_all_invariants(regions: &[LayoutRegion], screen_w: u16, screen_h: u16,
    cursor_row: u16, cursor_col: u16, editor_h: u16) -> Vec<InvariantResult> {
    vec![
        check_no_overlap(regions), check_coverage(regions, screen_w, screen_h),
        check_min_size(editor_h), check_cursor_visible(cursor_row, cursor_col, regions),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_overlap_pass() {
        let r = vec![LayoutRegion::new(0, 0, 80, 1), LayoutRegion::new(1, 0, 80, 22)];
        assert!(check_no_overlap(&r).passed);
    }

    #[test]
    fn overlap_detected() {
        let r = vec![LayoutRegion::new(0, 0, 80, 5), LayoutRegion::new(4, 0, 80, 5)];
        assert!(!check_no_overlap(&r).passed);
    }

    #[test]
    fn full_coverage() {
        let r = vec![LayoutRegion::new(0, 0, 80, 24)];
        assert!(check_coverage(&r, 80, 24).passed);
    }

    #[test]
    fn partial_coverage() {
        let r = vec![LayoutRegion::new(0, 0, 40, 12)];
        assert!(!check_coverage(&r, 80, 24).passed);
    }

    #[test]
    fn min_size_ok() { assert!(check_min_size(1).passed); }
    #[test]
    fn min_size_fail() { assert!(!check_min_size(0).passed); }

    #[test]
    fn cursor_visible_ok() {
        let r = vec![LayoutRegion::new(0, 0, 80, 24)];
        assert!(check_cursor_visible(10, 20, &r).passed);
    }

    #[test]
    fn cursor_not_visible() {
        let r = vec![LayoutRegion::new(0, 0, 80, 10)];
        assert!(!check_cursor_visible(15, 0, &r).passed);
    }

    #[test]
    fn run_all() {
        let r = vec![LayoutRegion::new(0, 0, 80, 24)];
        let results = run_all_invariants(&r, 80, 24, 5, 5, 22);
        assert!(results.iter().all(|r| r.passed));
    }
}
