//! Cursor shape configuration per mode and terminal escape sequences.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Terminal cursor shape.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CursorShape {
    Block,
    Line,
    Underline,
    Hidden,
}

/// Blink state for rendering.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BlinkState {
    On,
    Off,
}

/// Per-mode cursor configuration.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ModeCursorConfig {
    pub shape: CursorShape,
    pub blink_enabled: bool,
    pub blink_interval_ms: u64,
}

/// Map from mode name to cursor config.
pub type ModeCursorMap = HashMap<String, ModeCursorConfig>;

/// Return the cursor configuration for a given mode name.
pub fn cursor_for_mode(mode: &str) -> ModeCursorConfig {
    match mode {
        "Normal" | "NORMAL" => ModeCursorConfig {
            shape: CursorShape::Block,
            blink_enabled: false,
            blink_interval_ms: 0,
        },
        "Insert" | "INSERT" => ModeCursorConfig {
            shape: CursorShape::Line,
            blink_enabled: true,
            blink_interval_ms: 530,
        },
        "Replace" | "REPLACE" => ModeCursorConfig {
            shape: CursorShape::Underline,
            blink_enabled: true,
            blink_interval_ms: 530,
        },
        "Visual" | "VISUAL" | "VisualLine" | "V-LINE" | "VisualBlock" | "V-BLOCK" => {
            ModeCursorConfig {
                shape: CursorShape::Block,
                blink_enabled: false,
                blink_interval_ms: 0,
            }
        }
        "Command" | "COMMAND" => ModeCursorConfig {
            shape: CursorShape::Line,
            blink_enabled: true,
            blink_interval_ms: 530,
        },
        "OperatorPending" | "OP-PENDING" => ModeCursorConfig {
            shape: CursorShape::Block,
            blink_enabled: true,
            blink_interval_ms: 400,
        },
        "Terminal" | "TERMINAL" => ModeCursorConfig {
            shape: CursorShape::Block,
            blink_enabled: false,
            blink_interval_ms: 0,
        },
        _ => ModeCursorConfig {
            shape: CursorShape::Block,
            blink_enabled: false,
            blink_interval_ms: 0,
        },
    }
}

/// Emit DECSCUSR terminal escape sequence for the given cursor shape and blink.
///
/// DECSCUSR values:
///   0/1 = blinking block, 2 = steady block,
///   3 = blinking underline, 4 = steady underline,
///   5 = blinking bar, 6 = steady bar
pub fn cursor_shape_escape(shape: CursorShape, blink: bool) -> String {
    let n = match (shape, blink) {
        (CursorShape::Block, true) => 1,
        (CursorShape::Block, false) => 2,
        (CursorShape::Underline, true) => 3,
        (CursorShape::Underline, false) => 4,
        (CursorShape::Line, true) => 5,
        (CursorShape::Line, false) => 6,
        (CursorShape::Hidden, _) => return "\x1b[?25l".to_string(),
    };
    format!("\x1b[{n} q")
}

/// Determine blink state based on elapsed time and interval.
pub fn blink_state(elapsed_ms: u64, interval_ms: u64) -> BlinkState {
    if interval_ms == 0 {
        return BlinkState::On;
    }
    if (elapsed_ms / interval_ms).is_multiple_of(2) {
        BlinkState::On
    } else {
        BlinkState::Off
    }
}

/// Whether the cursor should be visible given shape and blink state.
pub fn cursor_visible(shape: CursorShape, blink: BlinkState) -> bool {
    if shape == CursorShape::Hidden {
        return false;
    }
    blink == BlinkState::On
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normal_is_block() {
        let cfg = cursor_for_mode("Normal");
        assert_eq!(cfg.shape, CursorShape::Block);
        assert!(!cfg.blink_enabled);
    }

    #[test]
    fn insert_is_line() {
        let cfg = cursor_for_mode("Insert");
        assert_eq!(cfg.shape, CursorShape::Line);
        assert!(cfg.blink_enabled);
    }

    #[test]
    fn replace_is_underline() {
        let cfg = cursor_for_mode("Replace");
        assert_eq!(cfg.shape, CursorShape::Underline);
    }

    #[test]
    fn escape_sequences() {
        assert_eq!(cursor_shape_escape(CursorShape::Block, false), "\x1b[2 q");
        assert_eq!(cursor_shape_escape(CursorShape::Line, true), "\x1b[5 q");
        assert_eq!(cursor_shape_escape(CursorShape::Hidden, false), "\x1b[?25l");
    }

    #[test]
    fn blink_cycle() {
        assert_eq!(blink_state(0, 500), BlinkState::On);
        assert_eq!(blink_state(500, 500), BlinkState::Off);
        assert_eq!(blink_state(1000, 500), BlinkState::On);
    }

    #[test]
    fn hidden_never_visible() {
        assert!(!cursor_visible(CursorShape::Hidden, BlinkState::On));
    }

    #[test]
    fn visible_when_on() {
        assert!(cursor_visible(CursorShape::Block, BlinkState::On));
        assert!(!cursor_visible(CursorShape::Block, BlinkState::Off));
    }
}
