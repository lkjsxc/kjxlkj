//! Line wrapping state.
//!
//! Manages soft-wrapped line display.

/// Wrap mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum WrapMode {
    /// No wrapping.
    #[default]
    None,
    /// Wrap at window edge.
    Wrap,
    /// Wrap at word boundary.
    WordWrap,
}

/// A wrapped line segment.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WrapSegment {
    /// Start column in original line.
    pub start_col: usize,
    /// End column in original line.
    pub end_col: usize,
    /// Display row offset (0 for first segment).
    pub row_offset: usize,
}

impl WrapSegment {
    /// Creates a new segment.
    pub fn new(start_col: usize, end_col: usize, row_offset: usize) -> Self {
        Self {
            start_col,
            end_col,
            row_offset,
        }
    }

    /// Returns the segment width.
    pub fn width(&self) -> usize {
        self.end_col.saturating_sub(self.start_col)
    }
}

/// Wrapped line.
#[derive(Debug, Clone, Default)]
pub struct WrappedLine {
    /// Segments of this line.
    pub segments: Vec<WrapSegment>,
}

impl WrappedLine {
    /// Creates new wrapped line.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates single-segment line.
    pub fn single(width: usize) -> Self {
        Self {
            segments: vec![WrapSegment::new(0, width, 0)],
        }
    }

    /// Adds a segment.
    pub fn add_segment(&mut self, segment: WrapSegment) {
        self.segments.push(segment);
    }

    /// Returns the number of display rows.
    pub fn row_count(&self) -> usize {
        self.segments.len().max(1)
    }

    /// Returns whether this line is wrapped.
    pub fn is_wrapped(&self) -> bool {
        self.segments.len() > 1
    }
}

/// Wrap state for a buffer.
#[derive(Debug, Clone, Default)]
pub struct WrapState {
    /// Wrap mode.
    pub mode: WrapMode,
    /// Wrap width.
    pub width: usize,
    /// Wrapped lines.
    lines: Vec<WrappedLine>,
}

impl WrapState {
    /// Creates new wrap state.
    pub fn new(mode: WrapMode, width: usize) -> Self {
        Self {
            mode,
            width,
            lines: Vec::new(),
        }
    }

    /// Calculates wrapping for a line.
    pub fn wrap_line(&self, line_width: usize) -> WrappedLine {
        if self.mode == WrapMode::None || line_width <= self.width || self.width == 0 {
            return WrappedLine::single(line_width);
        }

        let mut wrapped = WrappedLine::new();
        let mut col = 0;
        let mut row = 0;

        while col < line_width {
            let end = (col + self.width).min(line_width);
            wrapped.add_segment(WrapSegment::new(col, end, row));
            col = end;
            row += 1;
        }

        wrapped
    }

    /// Sets wrapped lines.
    pub fn set_lines(&mut self, lines: Vec<WrappedLine>) {
        self.lines = lines;
    }

    /// Gets wrapped line.
    pub fn get(&self, line: usize) -> Option<&WrappedLine> {
        self.lines.get(line)
    }

    /// Clears wrap state.
    pub fn clear(&mut self) {
        self.lines.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wrap_mode_default() {
        assert_eq!(WrapMode::default(), WrapMode::None);
    }

    #[test]
    fn test_wrap_segment() {
        let seg = WrapSegment::new(0, 80, 0);
        assert_eq!(seg.width(), 80);
    }

    #[test]
    fn test_wrapped_line_single() {
        let line = WrappedLine::single(100);
        assert_eq!(line.row_count(), 1);
        assert!(!line.is_wrapped());
    }

    #[test]
    fn test_wrapped_line_multiple() {
        let mut line = WrappedLine::new();
        line.add_segment(WrapSegment::new(0, 80, 0));
        line.add_segment(WrapSegment::new(80, 120, 1));
        assert!(line.is_wrapped());
    }

    #[test]
    fn test_wrap_state_no_wrap() {
        let state = WrapState::new(WrapMode::None, 80);
        let wrapped = state.wrap_line(100);
        assert_eq!(wrapped.row_count(), 1);
    }

    #[test]
    fn test_wrap_state_wrap() {
        let state = WrapState::new(WrapMode::Wrap, 40);
        let wrapped = state.wrap_line(100);
        assert_eq!(wrapped.row_count(), 3);
    }
}
