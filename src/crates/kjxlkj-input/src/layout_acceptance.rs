//! Layout invariant checking for UI regions.

use serde::{Deserialize, Serialize};

/// Kinds of layout invariants that can be checked.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum InvariantKind {
    NoOverlap,
    FullCoverage,
    MinSize,
    CursorVisible,
    CmdLinePresent,
    StatusLinePresent,
}

/// A rectangular layout region on screen.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LayoutRegion {
    pub x: u16,
    pub y: u16,
    pub w: u16,
    pub h: u16,
    pub name: String,
}

impl LayoutRegion {
    fn right(&self) -> u16 {
        self.x + self.w
    }
    fn bottom(&self) -> u16 {
        self.y + self.h
    }

    fn overlaps(&self, other: &Self) -> bool {
        self.x < other.right()
            && other.x < self.right()
            && self.y < other.bottom()
            && other.y < self.bottom()
    }
}

/// Check that no two regions overlap. Returns conflicting name pairs on failure.
pub fn check_no_overlap(regions: &[LayoutRegion]) -> Result<(), Vec<(String, String)>> {
    let mut conflicts = Vec::new();
    for i in 0..regions.len() {
        for j in (i + 1)..regions.len() {
            if regions[i].overlaps(&regions[j]) {
                conflicts.push((regions[i].name.clone(), regions[j].name.clone()));
            }
        }
    }
    if conflicts.is_empty() {
        Ok(())
    } else {
        Err(conflicts)
    }
}

/// Check that regions fully cover the total area (pixel-by-pixel).
pub fn check_coverage(regions: &[LayoutRegion], total_w: u16, total_h: u16) -> bool {
    for y in 0..total_h {
        for x in 0..total_w {
            let covered = regions
                .iter()
                .any(|r| x >= r.x && x < r.right() && y >= r.y && y < r.bottom());
            if !covered {
                return false;
            }
        }
    }
    true
}

/// Return names of regions smaller than `min_w` x `min_h`.
pub fn check_min_size(regions: &[LayoutRegion], min_w: u16, min_h: u16) -> Vec<String> {
    regions
        .iter()
        .filter(|r| r.w < min_w || r.h < min_h)
        .map(|r| r.name.clone())
        .collect()
}

/// Check if the cursor position is visible in any region.
pub fn check_cursor_visible(cursor_x: u16, cursor_y: u16, regions: &[LayoutRegion]) -> bool {
    regions.iter().any(|r| {
        cursor_x >= r.x && cursor_x < r.right() && cursor_y >= r.y && cursor_y < r.bottom()
    })
}

/// Run all layout invariants and return results.
pub fn run_all_invariants(
    regions: &[LayoutRegion],
    total_w: u16,
    total_h: u16,
    cursor_x: u16,
    cursor_y: u16,
) -> Vec<(InvariantKind, bool)> {
    let has_cmd = regions
        .iter()
        .any(|r| r.name.contains("cmd") || r.name.contains("command"));
    let has_status = regions.iter().any(|r| r.name.contains("status"));
    vec![
        (InvariantKind::NoOverlap, check_no_overlap(regions).is_ok()),
        (
            InvariantKind::FullCoverage,
            check_coverage(regions, total_w, total_h),
        ),
        (
            InvariantKind::MinSize,
            check_min_size(regions, 1, 1).is_empty(),
        ),
        (
            InvariantKind::CursorVisible,
            check_cursor_visible(cursor_x, cursor_y, regions),
        ),
        (InvariantKind::CmdLinePresent, has_cmd),
        (InvariantKind::StatusLinePresent, has_status),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn region(name: &str, x: u16, y: u16, w: u16, h: u16) -> LayoutRegion {
        LayoutRegion {
            x,
            y,
            w,
            h,
            name: name.into(),
        }
    }

    #[test]
    fn no_overlap_ok() {
        let r = vec![region("a", 0, 0, 10, 10), region("b", 10, 0, 10, 10)];
        assert!(check_no_overlap(&r).is_ok());
    }

    #[test]
    fn overlap_detected() {
        let r = vec![region("a", 0, 0, 10, 10), region("b", 5, 5, 10, 10)];
        assert!(check_no_overlap(&r).is_err());
    }

    #[test]
    fn coverage_full() {
        let r = vec![region("a", 0, 0, 5, 5), region("b", 5, 0, 5, 5)];
        assert!(check_coverage(&r, 10, 5));
    }

    #[test]
    fn coverage_gap() {
        let r = vec![region("a", 0, 0, 4, 5)];
        assert!(!check_coverage(&r, 10, 5));
    }

    #[test]
    fn min_size_check() {
        let r = vec![region("tiny", 0, 0, 1, 1), region("big", 1, 0, 10, 10)];
        let small = check_min_size(&r, 2, 2);
        assert!(small.contains(&"tiny".to_string()));
    }

    #[test]
    fn cursor_visible_check() {
        let r = vec![region("editor", 0, 0, 80, 24)];
        assert!(check_cursor_visible(5, 5, &r));
        assert!(!check_cursor_visible(80, 24, &r));
    }

    #[test]
    fn run_all() {
        let r = vec![
            region("editor", 0, 0, 80, 22),
            region("status", 0, 22, 80, 1),
            region("cmdline", 0, 23, 80, 1),
        ];
        let results = run_all_invariants(&r, 80, 24, 5, 5);
        assert_eq!(results.len(), 6);
        for (_, pass) in &results {
            assert!(pass);
        }
    }
}
