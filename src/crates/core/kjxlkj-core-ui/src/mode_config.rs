//! Mode-specific UI configuration.
//! See /docs/spec/modes/configuration.md for normative spec.

use kjxlkj_core_types::Mode;

/// Cursor shape emitted via DECSCUSR escape codes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CursorShape {
    BlockBlink,    // \e[1 q]
    BlockSteady,   // \e[2 q]
    UnderBlink,    // \e[3 q]
    UnderSteady,   // \e[4 q]
    BarBlink,      // \e[5 q]
    BarSteady,     // \e[6 q]
    Default,       // \e[0 q] — restore terminal default
}

impl CursorShape {
    /// DECSCUSR parameter for this shape.
    pub fn decscusr_code(self) -> u8 {
        match self {
            Self::BlockBlink => 1,
            Self::BlockSteady => 2,
            Self::UnderBlink => 3,
            Self::UnderSteady => 4,
            Self::BarBlink => 5,
            Self::BarSteady => 6,
            Self::Default => 0,
        }
    }

    /// Escape sequence string for this cursor shape.
    pub fn escape_sequence(self) -> String {
        format!("\x1b[{} q", self.decscusr_code())
    }
}

/// Return the default cursor shape for a given mode.
/// See configuration.md § "Cursor shape per mode".
pub fn cursor_shape_for_mode(mode: &Mode) -> CursorShape {
    match mode {
        Mode::Normal => CursorShape::BlockBlink,
        Mode::Insert | Mode::InsertNormal => CursorShape::BarBlink,
        Mode::Visual(_) => CursorShape::BlockSteady,
        Mode::Replace => CursorShape::UnderBlink,
        Mode::Command(_) => CursorShape::BarBlink,
        Mode::OperatorPending(_) => CursorShape::BlockSteady,
        Mode::TerminalInsert => CursorShape::BarBlink,
    }
}

/// Return the mode indicator text shown in the statusline.
/// See configuration.md § "Mode indicator".
pub fn mode_indicator(mode: &Mode) -> &'static str {
    match mode {
        Mode::Normal => "",
        Mode::Insert => "-- INSERT --",
        Mode::Visual(kjxlkj_core_types::VisualKind::Char) => "-- VISUAL --",
        Mode::Visual(kjxlkj_core_types::VisualKind::Line) => "-- VISUAL LINE --",
        Mode::Visual(kjxlkj_core_types::VisualKind::Block) => "-- VISUAL BLOCK --",
        Mode::Replace => "-- REPLACE --",
        Mode::Command(_) => "",
        Mode::OperatorPending(_) => "",
        Mode::TerminalInsert => "-- TERMINAL --",
        Mode::InsertNormal => "",
    }
}

/// Escape sequence to restore cursor to terminal default on exit.
pub fn cursor_restore_sequence() -> &'static str { "\x1b[0 q" }

#[cfg(test)]
mod tests {
    use super::*;
    use kjxlkj_core_types::{CommandKind, Operator, VisualKind};

    #[test]
    fn normal_mode_block_blink() {
        assert_eq!(cursor_shape_for_mode(&Mode::Normal), CursorShape::BlockBlink);
        assert_eq!(CursorShape::BlockBlink.decscusr_code(), 1);
    }

    #[test]
    fn insert_mode_bar_blink() {
        assert_eq!(cursor_shape_for_mode(&Mode::Insert), CursorShape::BarBlink);
        assert_eq!(CursorShape::BarBlink.decscusr_code(), 5);
    }

    #[test]
    fn visual_mode_block_steady() {
        let m = Mode::Visual(VisualKind::Char);
        assert_eq!(cursor_shape_for_mode(&m), CursorShape::BlockSteady);
    }

    #[test]
    fn replace_mode_underline_blink() {
        assert_eq!(cursor_shape_for_mode(&Mode::Replace), CursorShape::UnderBlink);
        assert_eq!(CursorShape::UnderBlink.decscusr_code(), 3);
    }

    #[test]
    fn command_mode_bar() {
        let m = Mode::Command(CommandKind::Ex);
        assert_eq!(cursor_shape_for_mode(&m), CursorShape::BarBlink);
    }

    #[test]
    fn operator_pending_steady_block() {
        let m = Mode::OperatorPending(Operator::Delete);
        assert_eq!(cursor_shape_for_mode(&m), CursorShape::BlockSteady);
    }

    #[test]
    fn indicator_insert() { assert_eq!(mode_indicator(&Mode::Insert), "-- INSERT --"); }

    #[test]
    fn indicator_visual_line() {
        assert_eq!(mode_indicator(&Mode::Visual(VisualKind::Line)), "-- VISUAL LINE --");
    }

    #[test]
    fn indicator_visual_block() {
        assert_eq!(mode_indicator(&Mode::Visual(VisualKind::Block)), "-- VISUAL BLOCK --");
    }

    #[test]
    fn indicator_normal_empty() { assert_eq!(mode_indicator(&Mode::Normal), ""); }

    #[test]
    fn indicator_terminal() { assert_eq!(mode_indicator(&Mode::TerminalInsert), "-- TERMINAL --"); }

    #[test]
    fn escape_sequence_format() {
        assert_eq!(CursorShape::BarBlink.escape_sequence(), "\x1b[5 q");
        assert_eq!(CursorShape::Default.escape_sequence(), "\x1b[0 q");
    }

    #[test]
    fn restore_sequence() { assert_eq!(cursor_restore_sequence(), "\x1b[0 q"); }
}
