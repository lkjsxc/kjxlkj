//! Scroll and viewport customization.
//!
//! Implements scroll settings as specified in `/docs/spec/features/ui/scroll-customization.md`.

/// Past-end scrolling mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PastEnd {
    /// No scrolling past end of file.
    #[default]
    None,
    /// Allow scrolling past end (virtual lines).
    Scroll,
}

/// Scroll configuration.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ScrollConfig {
    /// Vertical context margin around cursor (rows).
    pub scrolloff: usize,
    /// Horizontal context margin around cursor (cols, no-wrap only).
    pub sidescrolloff: usize,
    /// Default line count for half-page scroll (0 = floor(text_rows/2)).
    pub scroll: usize,
    /// Minimum horizontal scroll amount.
    pub sidescroll: usize,
    /// Past-end scrolling mode.
    pub past_end: PastEnd,
    /// Enable smooth scrolling animation.
    pub smooth_follow: bool,
}

impl Default for ScrollConfig {
    fn default() -> Self {
        Self {
            scrolloff: 0,
            sidescrolloff: 0,
            scroll: 0,
            sidescroll: 1,
            past_end: PastEnd::None,
            smooth_follow: false,
        }
    }
}

impl ScrollConfig {
    /// Create a config with scrolloff.
    pub fn with_scrolloff(mut self, lines: usize) -> Self {
        self.scrolloff = lines;
        self
    }

    /// Create a config with sidescrolloff.
    pub fn with_sidescrolloff(mut self, cols: usize) -> Self {
        self.sidescrolloff = cols;
        self
    }

    /// Enable smooth scrolling.
    pub fn with_smooth(mut self) -> Self {
        self.smooth_follow = true;
        self
    }

    /// Allow scrolling past end of file.
    pub fn with_past_end(mut self) -> Self {
        self.past_end = PastEnd::Scroll;
        self
    }

    /// Set half-page scroll amount.
    pub fn with_scroll(mut self, lines: usize) -> Self {
        self.scroll = lines;
        self
    }

    /// Calculate effective vertical margin for viewport.
    pub fn effective_v_margin(&self, text_rows: usize) -> usize {
        let max_margin = (text_rows.saturating_sub(1)) / 2;
        self.scrolloff.min(max_margin)
    }

    /// Calculate effective horizontal margin for viewport.
    pub fn effective_h_margin(&self, text_cols: usize) -> usize {
        let max_margin = (text_cols.saturating_sub(1)) / 2;
        self.sidescrolloff.min(max_margin)
    }

    /// Calculate half-page scroll amount.
    pub fn half_page_scroll(&self, text_rows: usize) -> usize {
        if self.scroll > 0 {
            self.scroll
        } else {
            text_rows / 2
        }
    }
}

/// Scroll position tracking.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct ScrollPosition {
    /// Top line (0-indexed).
    pub top_line: usize,
    /// Left column (0-indexed, no-wrap only).
    pub left_col: usize,
}

impl ScrollPosition {
    /// Create a new scroll position.
    pub fn new(top_line: usize, left_col: usize) -> Self {
        Self { top_line, left_col }
    }

    /// Scroll to ensure cursor is visible.
    pub fn follow_cursor(
        &mut self,
        cursor_line: usize,
        cursor_col: usize,
        text_rows: usize,
        text_cols: usize,
        config: &ScrollConfig,
        wrap: bool,
    ) {
        // Vertical follow
        let v_margin = config.effective_v_margin(text_rows);
        let min_row = v_margin;
        let max_row = text_rows.saturating_sub(1).saturating_sub(v_margin);

        let cursor_row = cursor_line.saturating_sub(self.top_line);
        if cursor_row < min_row {
            self.top_line = cursor_line.saturating_sub(min_row);
        } else if cursor_row > max_row && max_row >= min_row {
            self.top_line = cursor_line.saturating_sub(max_row);
        }

        // Horizontal follow (only when not wrapping)
        if !wrap {
            let h_margin = config.effective_h_margin(text_cols);
            let min_x = h_margin;
            let max_x = text_cols.saturating_sub(1).saturating_sub(h_margin);

            let cursor_x = cursor_col.saturating_sub(self.left_col);
            if cursor_x < min_x {
                self.left_col = cursor_col.saturating_sub(min_x);
            } else if cursor_x > max_x && max_x >= min_x {
                self.left_col = cursor_col.saturating_sub(max_x);
            }
        } else {
            self.left_col = 0;
        }
    }

    /// Center viewport on line.
    pub fn center_on(&mut self, line: usize, text_rows: usize) {
        let half = text_rows / 2;
        self.top_line = line.saturating_sub(half);
    }

    /// Put cursor line at top.
    pub fn cursor_to_top(&mut self, cursor_line: usize) {
        self.top_line = cursor_line;
    }

    /// Put cursor line at bottom.
    pub fn cursor_to_bottom(&mut self, cursor_line: usize, text_rows: usize) {
        self.top_line = cursor_line.saturating_sub(text_rows.saturating_sub(1));
    }

    /// Scroll down by lines.
    pub fn scroll_down(&mut self, lines: usize, buffer_lines: usize, text_rows: usize, past_end: PastEnd) {
        let max_top = match past_end {
            PastEnd::None => buffer_lines.saturating_sub(text_rows),
            PastEnd::Scroll => buffer_lines.saturating_sub(1),
        };
        self.top_line = (self.top_line + lines).min(max_top);
    }

    /// Scroll up by lines.
    pub fn scroll_up(&mut self, lines: usize) {
        self.top_line = self.top_line.saturating_sub(lines);
    }

    /// Scroll left by columns.
    pub fn scroll_left(&mut self, cols: usize) {
        self.left_col = self.left_col.saturating_sub(cols);
    }

    /// Scroll right by columns.
    pub fn scroll_right(&mut self, cols: usize, max_width: usize, text_cols: usize) {
        let max_left = max_width.saturating_sub(text_cols);
        self.left_col = (self.left_col + cols).min(max_left);
    }

    /// Clamp position to valid range.
    pub fn clamp(&mut self, buffer_lines: usize, max_width: usize, text_rows: usize, text_cols: usize, past_end: PastEnd) {
        let max_top = match past_end {
            PastEnd::None => buffer_lines.saturating_sub(text_rows),
            PastEnd::Scroll => buffer_lines.saturating_sub(1),
        };
        self.top_line = self.top_line.min(max_top);
        let max_left = max_width.saturating_sub(text_cols);
        self.left_col = self.left_col.min(max_left);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scroll_config_default() {
        let config = ScrollConfig::default();
        assert_eq!(config.scrolloff, 0);
        assert_eq!(config.sidescrolloff, 0);
        assert!(!config.smooth_follow);
    }

    #[test]
    fn test_scroll_config_with_scrolloff() {
        let config = ScrollConfig::default().with_scrolloff(5);
        assert_eq!(config.scrolloff, 5);
    }

    #[test]
    fn test_scroll_config_with_smooth() {
        let config = ScrollConfig::default().with_smooth();
        assert!(config.smooth_follow);
    }

    #[test]
    fn test_scroll_config_with_past_end() {
        let config = ScrollConfig::default().with_past_end();
        assert_eq!(config.past_end, PastEnd::Scroll);
    }

    #[test]
    fn test_scroll_config_effective_v_margin() {
        let config = ScrollConfig::default().with_scrolloff(10);
        // For a 20-row viewport, max margin is 9
        assert_eq!(config.effective_v_margin(20), 9);
        // For a 5-row viewport, max margin is 2
        assert_eq!(config.effective_v_margin(5), 2);
    }

    #[test]
    fn test_scroll_config_half_page() {
        let config = ScrollConfig::default();
        assert_eq!(config.half_page_scroll(24), 12);

        let config = ScrollConfig::default().with_scroll(10);
        assert_eq!(config.half_page_scroll(24), 10);
    }

    #[test]
    fn test_scroll_position_new() {
        let pos = ScrollPosition::new(10, 5);
        assert_eq!(pos.top_line, 10);
        assert_eq!(pos.left_col, 5);
    }

    #[test]
    fn test_scroll_position_center() {
        let mut pos = ScrollPosition::default();
        pos.center_on(50, 24);
        assert_eq!(pos.top_line, 38);
    }

    #[test]
    fn test_scroll_position_cursor_to_top() {
        let mut pos = ScrollPosition::default();
        pos.cursor_to_top(50);
        assert_eq!(pos.top_line, 50);
    }

    #[test]
    fn test_scroll_position_cursor_to_bottom() {
        let mut pos = ScrollPosition::default();
        pos.cursor_to_bottom(50, 24);
        assert_eq!(pos.top_line, 27);
    }

    #[test]
    fn test_scroll_position_scroll_down() {
        let mut pos = ScrollPosition::default();
        pos.scroll_down(10, 100, 24, PastEnd::None);
        assert_eq!(pos.top_line, 10);
    }

    #[test]
    fn test_scroll_position_scroll_down_clamp() {
        let mut pos = ScrollPosition::default();
        pos.scroll_down(100, 50, 24, PastEnd::None);
        assert_eq!(pos.top_line, 26);
    }

    #[test]
    fn test_scroll_position_scroll_up() {
        let mut pos = ScrollPosition::new(20, 0);
        pos.scroll_up(5);
        assert_eq!(pos.top_line, 15);
    }

    #[test]
    fn test_scroll_position_scroll_up_clamp() {
        let mut pos = ScrollPosition::new(2, 0);
        pos.scroll_up(10);
        assert_eq!(pos.top_line, 0);
    }

    #[test]
    fn test_scroll_position_follow_cursor_vertical() {
        let mut pos = ScrollPosition::default();
        let config = ScrollConfig::default().with_scrolloff(5);
        
        // Cursor below viewport
        pos.follow_cursor(50, 0, 24, 80, &config, true);
        assert!(pos.top_line > 0);
    }

    #[test]
    fn test_scroll_position_follow_cursor_horizontal() {
        let mut pos = ScrollPosition::default();
        let config = ScrollConfig::default().with_sidescrolloff(5);
        
        // Cursor to the right (no wrap)
        pos.follow_cursor(0, 100, 24, 80, &config, false);
        assert!(pos.left_col > 0);
    }

    #[test]
    fn test_scroll_position_wrap_resets_left() {
        let mut pos = ScrollPosition::new(0, 20);
        let config = ScrollConfig::default();
        
        pos.follow_cursor(0, 0, 24, 80, &config, true);
        assert_eq!(pos.left_col, 0);
    }

    #[test]
    fn test_scroll_position_clamp() {
        let mut pos = ScrollPosition::new(1000, 500);
        pos.clamp(100, 200, 24, 80, PastEnd::None);
        assert_eq!(pos.top_line, 76);
        assert_eq!(pos.left_col, 120);
    }
}
