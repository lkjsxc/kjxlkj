//! Terminal capability detection.
//!
//! Detects color support, Unicode width, and other
//! terminal features at startup.

use std::env;

/// Detected terminal capabilities.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TerminalCapabilities {
    /// Color support level.
    pub color: ColorSupport,
    /// Whether the terminal supports Unicode.
    pub unicode: bool,
    /// Whether bracketed paste is supported.
    pub bracketed_paste: bool,
    /// Whether focus events are supported.
    pub focus_events: bool,
}

/// Color support levels.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ColorSupport {
    /// No color support (dumb terminal).
    None,
    /// Basic 16 colors (ANSI).
    Basic16,
    /// 256 colors (xterm-256color).
    Color256,
    /// 24-bit true color.
    TrueColor,
}

impl TerminalCapabilities {
    /// Detect terminal capabilities from environment.
    pub fn detect() -> Self {
        Self {
            color: detect_color_support(),
            unicode: detect_unicode_support(),
            bracketed_paste: true,
            focus_events: true,
        }
    }

    /// Whether the terminal supports at least basic colors.
    pub fn has_color(&self) -> bool {
        self.color >= ColorSupport::Basic16
    }

    /// Whether 24-bit color is available.
    pub fn has_true_color(&self) -> bool {
        self.color >= ColorSupport::TrueColor
    }
}

/// Detect color support from environment variables.
fn detect_color_support() -> ColorSupport {
    // COLORTERM is the most reliable for true-color.
    if let Ok(ct) = env::var("COLORTERM") {
        match ct.as_str() {
            "truecolor" | "24bit" => {
                return ColorSupport::TrueColor;
            }
            _ => {}
        }
    }

    // Check TERM for 256-color or basic.
    if let Ok(term) = env::var("TERM") {
        if term.contains("256color") {
            return ColorSupport::Color256;
        }
        if term == "dumb" {
            return ColorSupport::None;
        }
        // Most modern terminals support basic 16.
        return ColorSupport::Basic16;
    }

    // Fallback: assume basic if TERM is unset.
    ColorSupport::Basic16
}

/// Detect Unicode support from locale environment.
fn detect_unicode_support() -> bool {
    for var in ["LC_ALL", "LC_CTYPE", "LANG"] {
        if let Ok(val) = env::var(var) {
            let lower = val.to_lowercase();
            if lower.contains("utf-8")
                || lower.contains("utf8")
            {
                return true;
            }
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detect_does_not_panic() {
        let caps = TerminalCapabilities::detect();
        // We can't know what the CI env looks like,
        // but detect() must not panic.
        let _ = caps.has_color();
        let _ = caps.has_true_color();
    }

    #[test]
    fn color_ordering() {
        assert!(ColorSupport::None < ColorSupport::Basic16);
        assert!(ColorSupport::Basic16 < ColorSupport::Color256);
        assert!(
            ColorSupport::Color256 < ColorSupport::TrueColor
        );
    }

    #[test]
    fn true_color_implies_color() {
        let caps = TerminalCapabilities {
            color: ColorSupport::TrueColor,
            unicode: true,
            bracketed_paste: true,
            focus_events: true,
        };
        assert!(caps.has_color());
        assert!(caps.has_true_color());
    }

    #[test]
    fn no_color() {
        let caps = TerminalCapabilities {
            color: ColorSupport::None,
            unicode: false,
            bracketed_paste: false,
            focus_events: false,
        };
        assert!(!caps.has_color());
        assert!(!caps.has_true_color());
    }
}
