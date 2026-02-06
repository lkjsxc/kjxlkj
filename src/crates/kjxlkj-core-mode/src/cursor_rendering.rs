/// Cursor rendering and customization — shape, blink, mode-dependent styling.

/// Cursor shape for terminal rendering.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CursorShape { Block, Line, Underline, Hidden }

/// Cursor blink state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlinkState { Steady, BlinkOn, BlinkOff }

/// Cursor rendering configuration.
#[derive(Debug, Clone)]
pub struct CursorConfig {
    pub shape: CursorShape,
    pub blink: bool,
    pub blink_interval_ms: u64,
    pub color_override: Option<String>,
}

impl Default for CursorConfig {
    fn default() -> Self {
        Self { shape: CursorShape::Block, blink: false, blink_interval_ms: 530, color_override: None }
    }
}

/// Mode-specific cursor appearance.
#[derive(Debug, Clone)]
pub struct ModeCursorMap {
    pub normal: CursorShape,
    pub insert: CursorShape,
    pub visual: CursorShape,
    pub replace: CursorShape,
    pub command: CursorShape,
    pub operator_pending: CursorShape,
}

impl Default for ModeCursorMap {
    fn default() -> Self {
        Self { normal: CursorShape::Block, insert: CursorShape::Line,
            visual: CursorShape::Block, replace: CursorShape::Underline,
            command: CursorShape::Line, operator_pending: CursorShape::Block }
    }
}

impl ModeCursorMap {
    pub fn shape_for_mode(&self, mode: &str) -> CursorShape {
        match mode {
            "normal" | "n" => self.normal, "insert" | "i" => self.insert,
            "visual" | "v" => self.visual, "replace" | "r" => self.replace,
            "command" | "c" => self.command, "operator_pending" | "o" => self.operator_pending,
            _ => CursorShape::Block,
        }
    }
}

/// Generate terminal escape sequence for cursor shape.
pub fn cursor_shape_escape(shape: CursorShape, blink: bool) -> &'static str {
    match (shape, blink) {
        (CursorShape::Block, true) => "\x1b[1 q",
        (CursorShape::Block, false) => "\x1b[2 q",
        (CursorShape::Underline, true) => "\x1b[3 q",
        (CursorShape::Underline, false) => "\x1b[4 q",
        (CursorShape::Line, true) => "\x1b[5 q",
        (CursorShape::Line, false) => "\x1b[6 q",
        (CursorShape::Hidden, _) => "\x1b[?25l",
    }
}

/// Compute blink state at a given timestamp.
pub fn blink_state(time_ms: u64, interval_ms: u64) -> BlinkState {
    if interval_ms == 0 { return BlinkState::Steady; }
    if (time_ms / interval_ms) % 2 == 0 { BlinkState::BlinkOn } else { BlinkState::BlinkOff }
}

/// Visibility check — hidden cursor or blink-off means invisible.
pub fn cursor_visible(shape: CursorShape, blink: BlinkState) -> bool {
    shape != CursorShape::Hidden && blink != BlinkState::BlinkOff
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_config() {
        let c = CursorConfig::default();
        assert_eq!(c.shape, CursorShape::Block); assert!(!c.blink);
    }

    #[test]
    fn mode_cursor_shapes() {
        let m = ModeCursorMap::default();
        assert_eq!(m.shape_for_mode("insert"), CursorShape::Line);
        assert_eq!(m.shape_for_mode("replace"), CursorShape::Underline);
    }

    #[test]
    fn unknown_mode_fallback() {
        let m = ModeCursorMap::default();
        assert_eq!(m.shape_for_mode("unknown"), CursorShape::Block);
    }

    #[test]
    fn escape_sequences() {
        assert_eq!(cursor_shape_escape(CursorShape::Block, false), "\x1b[2 q");
        assert_eq!(cursor_shape_escape(CursorShape::Line, true), "\x1b[5 q");
    }

    #[test]
    fn blink_alternates() {
        assert_eq!(blink_state(0, 500), BlinkState::BlinkOn);
        assert_eq!(blink_state(500, 500), BlinkState::BlinkOff);
        assert_eq!(blink_state(1000, 500), BlinkState::BlinkOn);
    }

    #[test]
    fn blink_zero_interval() { assert_eq!(blink_state(100, 0), BlinkState::Steady); }

    #[test]
    fn visibility() {
        assert!(cursor_visible(CursorShape::Block, BlinkState::BlinkOn));
        assert!(!cursor_visible(CursorShape::Hidden, BlinkState::BlinkOn));
        assert!(!cursor_visible(CursorShape::Block, BlinkState::BlinkOff));
    }

    #[test]
    fn steady_cursor_always_visible() {
        assert!(cursor_visible(CursorShape::Line, BlinkState::Steady));
    }
}
