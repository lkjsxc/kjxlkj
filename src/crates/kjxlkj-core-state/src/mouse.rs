//! Mouse support per /docs/spec/features/config/mouse-support.md.

/// Mouse button.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
    ScrollUp,
    ScrollDown,
}

/// Mouse event.
#[derive(Debug, Clone, Copy)]
pub struct MouseEvent {
    /// Button pressed.
    pub button: MouseButton,
    /// Column (0-indexed).
    pub col: u16,
    /// Row (0-indexed).
    pub row: u16,
    /// Whether shift is held.
    pub shift: bool,
    /// Whether ctrl is held.
    pub ctrl: bool,
    /// Whether alt is held.
    pub alt: bool,
}

/// Mouse configuration.
#[derive(Debug, Clone)]
pub struct MouseConfig {
    /// Whether mouse is enabled.
    pub enabled: bool,
    /// Enable mouse in normal mode.
    pub normal: bool,
    /// Enable mouse in insert mode.
    pub insert: bool,
    /// Enable mouse in visual mode.
    pub visual: bool,
    /// Enable mouse in command mode.
    pub command: bool,
}

impl Default for MouseConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            normal: true,
            insert: true,
            visual: true,
            command: true,
        }
    }
}

impl MouseConfig {
    /// Create new config.
    pub fn new() -> Self {
        Self::default()
    }

    /// Parse mouse option string (e.g. "a", "nvi").
    pub fn from_option(opt: &str) -> Self {
        if opt == "a" {
            return Self::default();
        }
        Self {
            enabled: !opt.is_empty(),
            normal: opt.contains('n'),
            insert: opt.contains('i'),
            visual: opt.contains('v'),
            command: opt.contains('c'),
        }
    }
}

/// Keyboard layout support.
#[derive(Debug, Clone)]
pub struct KeyboardLayout {
    /// Layout name.
    pub name: String,
    /// Mapping from native keycode to character.
    pub mapping: std::collections::HashMap<char, char>,
}

impl KeyboardLayout {
    /// US QWERTY (identity mapping).
    pub fn qwerty() -> Self {
        Self {
            name: "qwerty".into(),
            mapping: std::collections::HashMap::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mouse_config_all() {
        let cfg = MouseConfig::from_option("a");
        assert!(cfg.enabled);
        assert!(cfg.normal);
        assert!(cfg.insert);
    }

    #[test]
    fn mouse_config_partial() {
        let cfg = MouseConfig::from_option("nv");
        assert!(cfg.enabled);
        assert!(cfg.normal);
        assert!(cfg.visual);
        assert!(!cfg.insert);
    }

    #[test]
    fn mouse_config_empty() {
        let cfg = MouseConfig::from_option("");
        assert!(!cfg.enabled);
    }
}
