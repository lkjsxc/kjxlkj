//! Cursor customization and rendering options.

use crate::Mode;

/// Cursor shape.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CursorShape {
    Block,
    Bar,
    Underline,
}

/// Cursor blink state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CursorBlink {
    /// Solid, no blinking.
    Steady,
    /// Blinking enabled.
    Blinking,
}

/// Cursor style configuration.
#[derive(Debug, Clone)]
pub struct CursorStyle {
    /// Shape in Normal mode.
    pub normal: CursorShape,
    /// Shape in Insert mode.
    pub insert: CursorShape,
    /// Shape in Replace mode.
    pub replace: CursorShape,
    /// Shape in Visual mode.
    pub visual: CursorShape,
    /// Shape in Command mode.
    pub command: CursorShape,
    /// Blink behavior.
    pub blink: CursorBlink,
    /// Blink interval in ms.
    pub blink_interval: u32,
}

impl Default for CursorStyle {
    fn default() -> Self {
        Self {
            normal: CursorShape::Block,
            insert: CursorShape::Bar,
            replace: CursorShape::Underline,
            visual: CursorShape::Block,
            command: CursorShape::Bar,
            blink: CursorBlink::Blinking,
            blink_interval: 530,
        }
    }
}

impl CursorStyle {
    /// Get cursor escape sequence for a mode.
    pub fn escape_for_mode(&self, mode: Mode) -> &'static str {
        let shape = match mode {
            Mode::Normal => self.normal,
            Mode::Insert => self.insert,
            Mode::Replace => self.replace,
            Mode::Visual(_) => self.visual,
            Mode::Command(_) => self.command,
            _ => self.normal,
        };
        match (shape, self.blink) {
            (CursorShape::Block, CursorBlink::Blinking) => "\x1b[1 q",
            (CursorShape::Block, CursorBlink::Steady) => "\x1b[2 q",
            (CursorShape::Underline, CursorBlink::Blinking) => "\x1b[3 q",
            (CursorShape::Underline, CursorBlink::Steady) => "\x1b[4 q",
            (CursorShape::Bar, CursorBlink::Blinking) => "\x1b[5 q",
            (CursorShape::Bar, CursorBlink::Steady) => "\x1b[6 q",
        }
    }
}

/// Indent guide configuration.
#[derive(Debug, Clone)]
pub struct IndentGuideConfig {
    /// Whether indent guides are enabled.
    pub enabled: bool,
    /// Character used for indent guide.
    pub char: char,
    /// Only show on active indent.
    pub active_only: bool,
}

impl Default for IndentGuideConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            char: '│',
            active_only: false,
        }
    }
}

/// Scroll customization.
#[derive(Debug, Clone)]
pub struct ScrollConfig {
    /// Number of lines for smooth scroll.
    pub smooth_lines: u16,
    /// Whether smooth scroll is enabled.
    pub smooth: bool,
    /// Minimum scrolloff.
    pub scrolloff: u16,
    /// Side scrolloff.
    pub sidescrolloff: u16,
}

impl Default for ScrollConfig {
    fn default() -> Self {
        Self {
            smooth_lines: 3,
            smooth: false,
            scrolloff: 5,
            sidescrolloff: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cursor_style_default() {
        let style = CursorStyle::default();
        assert_eq!(style.normal, CursorShape::Block);
        assert_eq!(style.insert, CursorShape::Bar);
    }

    #[test]
    fn cursor_escape_sequence() {
        let style = CursorStyle::default();
        let esc = style.escape_for_mode(Mode::Insert);
        assert_eq!(esc, "\x1b[5 q");
    }
}
