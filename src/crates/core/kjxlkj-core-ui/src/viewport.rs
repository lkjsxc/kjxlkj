//! Viewport state model.
//!
//! See /docs/spec/features/ui/viewport.md for normative spec.
//! Per-window viewport parameters controlling visible region.

/// Viewport state for a single window.
#[derive(Debug, Clone)]
pub struct ViewportState {
    /// First visible logical line.
    pub top_line: usize,
    /// First visible column (when wrap=false).
    pub left_col: usize,
    /// Whether soft-wrap is active.
    pub wrap: bool,
    /// Visible content rows.
    pub text_rows: u16,
    /// Visible content columns.
    pub text_cols: u16,
    /// Vertical focus margin.
    pub scrolloff: usize,
    /// Horizontal focus margin.
    pub sidescrolloff: usize,
}

impl ViewportState {
    /// Create with given dimensions and defaults.
    pub fn new(rows: u16, cols: u16) -> Self {
        Self {
            top_line: 0,
            left_col: 0,
            wrap: false,
            text_rows: rows,
            text_cols: cols,
            scrolloff: 5,
            sidescrolloff: 0,
        }
    }

    /// Ensure cursor line and column are visible,
    /// adjusting top_line and left_col as needed.
    pub fn ensure_visible(
        &mut self,
        cursor_line: usize,
        cursor_col: usize,
        total_lines: usize,
    ) {
        let rows = self.text_rows as usize;
        if rows == 0 {
            return;
        }
        let soff = self.scrolloff.min(rows / 2);
        // Vertical: cursor must be within [top+soff, top+rows-1-soff].
        if cursor_line < self.top_line.saturating_add(soff) {
            self.top_line =
                cursor_line.saturating_sub(soff);
        }
        let bottom_limit =
            self.top_line + rows.saturating_sub(1);
        if cursor_line + soff > bottom_limit {
            self.top_line = (cursor_line + soff)
                .saturating_sub(rows.saturating_sub(1));
        }
        // Clamp top_line to valid range.
        if total_lines > 0 {
            let max_top =
                total_lines.saturating_sub(1);
            if self.top_line > max_top {
                self.top_line = max_top;
            }
        }
        // Horizontal (only when wrap=false).
        if !self.wrap {
            let cols = self.text_cols as usize;
            if cols > 0 {
                let ssoff =
                    self.sidescrolloff.min(cols / 2);
                if cursor_col
                    < self.left_col.saturating_add(ssoff)
                {
                    self.left_col =
                        cursor_col.saturating_sub(ssoff);
                }
                let right_limit =
                    self.left_col + cols.saturating_sub(1);
                if cursor_col + ssoff > right_limit {
                    self.left_col = (cursor_col + ssoff)
                        .saturating_sub(
                            cols.saturating_sub(1),
                        );
                }
            }
        }
    }

    /// Center viewport on a given line (zz command).
    pub fn scroll_center(
        &mut self,
        cursor_line: usize,
        total_lines: usize,
    ) {
        let half = (self.text_rows as usize) / 2;
        self.top_line =
            cursor_line.saturating_sub(half);
        self.clamp_top(total_lines);
    }

    /// Scroll so cursor line is at top (zt command).
    pub fn scroll_top(
        &mut self,
        cursor_line: usize,
        total_lines: usize,
    ) {
        self.top_line = cursor_line;
        self.clamp_top(total_lines);
    }

    /// Scroll so cursor line is at bottom (zb command).
    pub fn scroll_bottom(
        &mut self,
        cursor_line: usize,
        total_lines: usize,
    ) {
        let rows = self.text_rows as usize;
        self.top_line =
            cursor_line.saturating_sub(
                rows.saturating_sub(1),
            );
        self.clamp_top(total_lines);
    }

    /// Last visible line (inclusive).
    pub fn bottom_line(&self) -> usize {
        self.top_line + (self.text_rows as usize)
            .saturating_sub(1)
    }

    /// Whether a line is currently visible.
    pub fn is_line_visible(&self, line: usize) -> bool {
        line >= self.top_line
            && line <= self.bottom_line()
    }

    fn clamp_top(&mut self, total_lines: usize) {
        if total_lines > 0 {
            let max =
                total_lines.saturating_sub(1);
            if self.top_line > max {
                self.top_line = max;
            }
        }
    }
}

#[cfg(test)]
#[path = "viewport_tests.rs"]
mod tests;
