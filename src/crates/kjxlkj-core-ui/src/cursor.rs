//! Cursor customization types.
//!
//! Implements cursor appearance as specified in `/docs/spec/features/ui/cursor-customization.md`.

use kjxlkj_core_types::Mode;

/// Cursor shape options.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CursorShape {
    /// Block cursor (covers cell).
    #[default]
    Block,
    /// Vertical bar cursor.
    Bar,
    /// Underline cursor.
    Underline,
    /// Hollow block (outlined).
    Hollow,
}

impl CursorShape {
    /// Get the default cursor shape for a mode.
    pub fn for_mode(mode: Mode) -> Self {
        match mode {
            Mode::Normal => Self::Block,
            Mode::Insert => Self::Bar,
            Mode::Replace => Self::Underline,
            Mode::Visual | Mode::VisualLine | Mode::VisualBlock => Self::Hollow,
            Mode::Command => Self::Bar,
        }
    }

    /// Get terminal escape code for this shape.
    pub fn escape_code(&self) -> &'static str {
        match self {
            Self::Block => "\x1b[2 q",
            Self::Bar => "\x1b[6 q",
            Self::Underline => "\x1b[4 q",
            Self::Hollow => "\x1b[1 q",
        }
    }
}

/// Cursor blink configuration.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CursorBlink {
    /// Whether blinking is enabled.
    pub enabled: bool,
    /// Blink rate in milliseconds.
    pub rate_ms: u32,
}

impl Default for CursorBlink {
    fn default() -> Self {
        Self {
            enabled: true,
            rate_ms: 500,
        }
    }
}

impl CursorBlink {
    /// Create a new blink config.
    pub fn new(enabled: bool, rate_ms: u32) -> Self {
        // Clamp to safe range to avoid flicker
        let rate_ms = rate_ms.clamp(100, 2000);
        Self { enabled, rate_ms }
    }

    /// Disable blinking.
    pub fn disabled() -> Self {
        Self {
            enabled: false,
            rate_ms: 500,
        }
    }
}

/// Cursor line/column highlighting.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct CursorLine {
    /// Highlight current line.
    pub cursorline: bool,
    /// Highlight current column.
    pub cursorcolumn: bool,
}

impl CursorLine {
    /// Create a new config with cursorline enabled.
    pub fn line() -> Self {
        Self {
            cursorline: true,
            cursorcolumn: false,
        }
    }

    /// Create a crosshair config (both enabled).
    pub fn crosshair() -> Self {
        Self {
            cursorline: true,
            cursorcolumn: true,
        }
    }
}

/// Complete cursor configuration.
#[derive(Debug, Clone)]
pub struct CursorConfig {
    /// Shape per mode.
    shapes: [(Mode, CursorShape); 7],
    /// Blink settings.
    pub blink: CursorBlink,
    /// Line/column highlighting.
    pub highlight: CursorLine,
    /// Use terminal cursor (vs drawn).
    pub use_terminal_cursor: bool,
}

impl Default for CursorConfig {
    fn default() -> Self {
        Self {
            shapes: [
                (Mode::Normal, CursorShape::Block),
                (Mode::Insert, CursorShape::Bar),
                (Mode::Visual, CursorShape::Hollow),
                (Mode::VisualLine, CursorShape::Hollow),
                (Mode::VisualBlock, CursorShape::Hollow),
                (Mode::Replace, CursorShape::Underline),
                (Mode::Command, CursorShape::Bar),
            ],
            blink: CursorBlink::default(),
            highlight: CursorLine::default(),
            use_terminal_cursor: true,
        }
    }
}

impl CursorConfig {
    /// Get cursor shape for mode.
    pub fn shape_for_mode(&self, mode: Mode) -> CursorShape {
        self.shapes
            .iter()
            .find(|(m, _)| *m == mode)
            .map(|(_, s)| *s)
            .unwrap_or(CursorShape::Block)
    }

    /// Set cursor shape for a mode.
    pub fn set_shape(&mut self, mode: Mode, shape: CursorShape) {
        for (m, s) in &mut self.shapes {
            if *m == mode {
                *s = shape;
                return;
            }
        }
    }

    /// Enable cursorline.
    pub fn with_cursorline(mut self) -> Self {
        self.highlight.cursorline = true;
        self
    }

    /// Enable crosshair mode.
    pub fn with_crosshair(mut self) -> Self {
        self.highlight = CursorLine::crosshair();
        self
    }

    /// Disable blinking.
    pub fn with_no_blink(mut self) -> Self {
        self.blink.enabled = false;
        self
    }
}

/// Cursor state for rendering.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CursorState {
    /// Current shape.
    pub shape: CursorShape,
    /// Whether cursor is visible (blink state).
    pub visible: bool,
    /// Is primary cursor (vs secondary in multi-cursor).
    pub primary: bool,
}

impl CursorState {
    /// Create a new primary cursor state.
    pub fn primary(shape: CursorShape) -> Self {
        Self {
            shape,
            visible: true,
            primary: true,
        }
    }

    /// Create a secondary cursor state.
    pub fn secondary() -> Self {
        Self {
            shape: CursorShape::Block,
            visible: true,
            primary: false,
        }
    }

    /// Toggle visibility (for blink).
    pub fn toggle_blink(&mut self) {
        self.visible = !self.visible;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cursor_shape_for_mode_normal() {
        assert_eq!(CursorShape::for_mode(Mode::Normal), CursorShape::Block);
    }

    #[test]
    fn test_cursor_shape_for_mode_insert() {
        assert_eq!(CursorShape::for_mode(Mode::Insert), CursorShape::Bar);
    }

    #[test]
    fn test_cursor_shape_for_mode_visual() {
        assert_eq!(CursorShape::for_mode(Mode::Visual), CursorShape::Hollow);
    }

    #[test]
    fn test_cursor_shape_for_mode_replace() {
        assert_eq!(CursorShape::for_mode(Mode::Replace), CursorShape::Underline);
    }

    #[test]
    fn test_cursor_shape_escape_code() {
        assert!(CursorShape::Block.escape_code().contains("2 q"));
        assert!(CursorShape::Bar.escape_code().contains("6 q"));
    }

    #[test]
    fn test_cursor_blink_default() {
        let blink = CursorBlink::default();
        assert!(blink.enabled);
        assert_eq!(blink.rate_ms, 500);
    }

    #[test]
    fn test_cursor_blink_clamping() {
        let blink = CursorBlink::new(true, 50);
        assert_eq!(blink.rate_ms, 100);
        let blink = CursorBlink::new(true, 5000);
        assert_eq!(blink.rate_ms, 2000);
    }

    #[test]
    fn test_cursor_blink_disabled() {
        let blink = CursorBlink::disabled();
        assert!(!blink.enabled);
    }

    #[test]
    fn test_cursor_line_crosshair() {
        let cl = CursorLine::crosshair();
        assert!(cl.cursorline);
        assert!(cl.cursorcolumn);
    }

    #[test]
    fn test_cursor_line_default() {
        let cl = CursorLine::default();
        assert!(!cl.cursorline);
        assert!(!cl.cursorcolumn);
    }

    #[test]
    fn test_cursor_config_shape_for_mode() {
        let config = CursorConfig::default();
        assert_eq!(config.shape_for_mode(Mode::Normal), CursorShape::Block);
        assert_eq!(config.shape_for_mode(Mode::Insert), CursorShape::Bar);
    }

    #[test]
    fn test_cursor_config_set_shape() {
        let mut config = CursorConfig::default();
        config.set_shape(Mode::Normal, CursorShape::Bar);
        assert_eq!(config.shape_for_mode(Mode::Normal), CursorShape::Bar);
    }

    #[test]
    fn test_cursor_config_with_cursorline() {
        let config = CursorConfig::default().with_cursorline();
        assert!(config.highlight.cursorline);
    }

    #[test]
    fn test_cursor_config_with_crosshair() {
        let config = CursorConfig::default().with_crosshair();
        assert!(config.highlight.cursorline);
        assert!(config.highlight.cursorcolumn);
    }

    #[test]
    fn test_cursor_config_with_no_blink() {
        let config = CursorConfig::default().with_no_blink();
        assert!(!config.blink.enabled);
    }

    #[test]
    fn test_cursor_state_primary() {
        let state = CursorState::primary(CursorShape::Block);
        assert!(state.primary);
        assert!(state.visible);
    }

    #[test]
    fn test_cursor_state_secondary() {
        let state = CursorState::secondary();
        assert!(!state.primary);
    }

    #[test]
    fn test_cursor_state_toggle_blink() {
        let mut state = CursorState::primary(CursorShape::Block);
        assert!(state.visible);
        state.toggle_blink();
        assert!(!state.visible);
        state.toggle_blink();
        assert!(state.visible);
    }
}
