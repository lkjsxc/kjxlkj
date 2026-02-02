//! Status line segment types.

/// Status line segment for styling.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StatusSegment {
    /// Segment text.
    pub text: String,
    /// Segment style.
    pub style: SegmentStyle,
}

/// Style for a status segment.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SegmentStyle {
    /// Normal text.
    #[default]
    Normal,
    /// Mode indicator.
    Mode,
    /// File name.
    FileName,
    /// Modified indicator.
    Modified,
    /// Position.
    Position,
    /// File type.
    FileType,
}

impl StatusSegment {
    /// Creates a new segment.
    pub fn new(text: impl Into<String>, style: SegmentStyle) -> Self {
        Self {
            text: text.into(),
            style,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_segment_new() {
        let seg = StatusSegment::new("test", SegmentStyle::Mode);
        assert_eq!(seg.text, "test");
        assert_eq!(seg.style, SegmentStyle::Mode);
    }

    #[test]
    fn test_segment_style_default() {
        let style = SegmentStyle::default();
        assert_eq!(style, SegmentStyle::Normal);
    }
}
