//! Viewport state and cursor-follow logic.

/// Viewport tracking for a single view.
#[derive(Debug, Clone)]
pub struct ViewportState {
    pub top_line: usize,
    pub left_col: usize,
    pub visible_lines: usize,
    pub visible_cols: usize,
    pub scrolloff: usize,
    pub sidescrolloff: usize,
}

impl ViewportState {
    pub fn new(visible_lines: usize, visible_cols: usize) -> Self {
        Self {
            top_line: 0,
            left_col: 0,
            visible_lines,
            visible_cols,
            scrolloff: 5,
            sidescrolloff: 0,
        }
    }
}

impl Default for ViewportState {
    fn default() -> Self {
        Self::new(24, 80)
    }
}

/// Adjust top_line to keep cursor_line visible, respecting scrolloff.
pub fn follow_cursor_v(vp: &mut ViewportState, cursor_line: usize) {
    let off = vp.scrolloff;
    if vp.visible_lines == 0 {
        return;
    }
    if cursor_line < vp.top_line.saturating_add(off) {
        vp.top_line = cursor_line.saturating_sub(off);
    }
    let bottom = vp.top_line + vp.visible_lines.saturating_sub(1);
    if cursor_line + off > bottom {
        vp.top_line = (cursor_line + off).saturating_sub(vp.visible_lines.saturating_sub(1));
    }
}

/// Adjust left_col to keep cursor_col visible, respecting sidescrolloff.
pub fn follow_cursor_h(vp: &mut ViewportState, cursor_col: usize) {
    let off = vp.sidescrolloff;
    if cursor_col < vp.left_col.saturating_add(off) {
        vp.left_col = cursor_col.saturating_sub(off);
    }
    let right = vp.left_col + vp.visible_cols.saturating_sub(1);
    if cursor_col + off > right {
        vp.left_col = (cursor_col + off).saturating_sub(vp.visible_cols.saturating_sub(1));
    }
}

/// Center viewport on a line (zz).
pub fn center_on_line(vp: &mut ViewportState, line: usize) {
    vp.top_line = line.saturating_sub(vp.visible_lines / 2);
}

/// Position cursor line at top of viewport (zt).
pub fn cursor_to_top(vp: &mut ViewportState, cursor_line: usize) {
    vp.top_line = cursor_line;
}

/// Position cursor line at bottom of viewport (zb).
pub fn cursor_to_bottom(vp: &mut ViewportState, cursor_line: usize) {
    vp.top_line = cursor_line.saturating_sub(vp.visible_lines.saturating_sub(1));
}

/// Scroll viewport by delta lines (positive = down, negative = up).
pub fn scroll(vp: &mut ViewportState, delta: i64, max_line: usize) {
    if delta > 0 {
        vp.top_line = (vp.top_line + delta as usize).min(max_line);
    } else {
        vp.top_line = vp.top_line.saturating_sub((-delta) as usize);
    }
}

/// Compute scroll percentage (0–100).
pub fn scroll_percent(cursor_line: usize, total_lines: usize) -> u8 {
    if total_lines <= 1 {
        return 100;
    }
    ((cursor_line as f64 / (total_lines - 1) as f64) * 100.0) as u8
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn follow_v_scrolls_down() {
        let mut vp = ViewportState::new(10, 80);
        vp.scrolloff = 2;
        follow_cursor_v(&mut vp, 15);
        assert!(vp.top_line > 0);
    }

    #[test]
    fn follow_v_scrolls_up() {
        let mut vp = ViewportState::new(10, 80);
        vp.top_line = 20;
        vp.scrolloff = 2;
        follow_cursor_v(&mut vp, 18);
        assert!(vp.top_line < 20);
    }

    #[test]
    fn center_on_line_test() {
        let mut vp = ViewportState::new(20, 80);
        center_on_line(&mut vp, 50);
        assert_eq!(vp.top_line, 40);
    }

    #[test]
    fn scroll_percent_test() {
        assert_eq!(scroll_percent(50, 101), 50);
        assert_eq!(scroll_percent(0, 100), 0);
        assert_eq!(scroll_percent(0, 1), 100);
    }
}
