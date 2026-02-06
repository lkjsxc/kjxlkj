//! Keyboard layout detection and remapping.
//!
//! Supports QWERTY, Dvorak, Colemak, Workman layouts with optional
//! QWERTY-position preservation for hjkl navigation keys.

/// Known keyboard layouts.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyboardLayout {
    Qwerty,
    Dvorak,
    Colemak,
    Workman,
    Custom,
}

/// A key remapping entry.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KeyRemap {
    pub from: char,
    pub to: char,
}

/// Layout-specific remapping table.
#[derive(Debug, Clone)]
pub struct LayoutRemapper {
    layout: KeyboardLayout,
    remaps: Vec<KeyRemap>,
    preserve_hjkl: bool,
}

impl LayoutRemapper {
    /// Create a remapper for the given layout.
    pub fn new(layout: KeyboardLayout) -> Self {
        let (remaps, preserve_hjkl) = match layout {
            KeyboardLayout::Qwerty => (Vec::new(), false),
            KeyboardLayout::Dvorak => (dvorak_remaps(), true),
            KeyboardLayout::Colemak => (colemak_remaps(), true),
            KeyboardLayout::Workman => (workman_remaps(), true),
            KeyboardLayout::Custom => (Vec::new(), false),
        };
        Self { layout, remaps, preserve_hjkl }
    }

    /// Remap a character according to the layout.
    /// When `preserve_hjkl` is true, hjkl stay at QWERTY positions.
    pub fn remap(&self, ch: char) -> char {
        if !self.preserve_hjkl { return ch; }
        for r in &self.remaps {
            if r.from == ch { return r.to; }
        }
        ch
    }

    /// Get the current layout.
    pub fn layout(&self) -> KeyboardLayout { self.layout }

    /// Check if hjkl preservation is enabled.
    pub fn preserves_hjkl(&self) -> bool { self.preserve_hjkl }

    /// Set preserve_hjkl flag.
    pub fn set_preserve_hjkl(&mut self, preserve: bool) {
        self.preserve_hjkl = preserve;
    }

    /// Add a custom remap.
    pub fn add_remap(&mut self, from: char, to: char) {
        self.remaps.push(KeyRemap { from, to });
    }

    /// List all active remaps.
    pub fn list_remaps(&self) -> &[KeyRemap] { &self.remaps }
}

/// Parse a layout name from configuration.
pub fn parse_layout(name: &str) -> Option<KeyboardLayout> {
    match name.to_lowercase().as_str() {
        "qwerty" => Some(KeyboardLayout::Qwerty),
        "dvorak" => Some(KeyboardLayout::Dvorak),
        "colemak" => Some(KeyboardLayout::Colemak),
        "workman" => Some(KeyboardLayout::Workman),
        "custom" => Some(KeyboardLayout::Custom),
        _ => None,
    }
}

/// Dvorak: map QWERTY physical positions for hjkl to Dvorak chars.
/// On Dvorak, QWERTY h→d, j→h, k→t, l→n (physical positions).
fn dvorak_remaps() -> Vec<KeyRemap> {
    vec![
        KeyRemap { from: 'd', to: 'h' },
        KeyRemap { from: 'h', to: 'j' },
        KeyRemap { from: 't', to: 'k' },
        KeyRemap { from: 'n', to: 'l' },
    ]
}

/// Colemak: l moves to a different position.
fn colemak_remaps() -> Vec<KeyRemap> {
    vec![
        KeyRemap { from: 'n', to: 'j' },
        KeyRemap { from: 'e', to: 'k' },
        KeyRemap { from: 'i', to: 'l' },
    ]
}

/// Workman: hjkl equivalents.
fn workman_remaps() -> Vec<KeyRemap> {
    vec![
        KeyRemap { from: 'y', to: 'h' },
        KeyRemap { from: 'n', to: 'j' },
        KeyRemap { from: 'e', to: 'k' },
        KeyRemap { from: 'o', to: 'l' },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn qwerty_no_remap() {
        let r = LayoutRemapper::new(KeyboardLayout::Qwerty);
        assert_eq!(r.remap('h'), 'h');
        assert_eq!(r.remap('j'), 'j');
    }

    #[test]
    fn dvorak_remap_hjkl() {
        let r = LayoutRemapper::new(KeyboardLayout::Dvorak);
        assert_eq!(r.remap('d'), 'h');
        assert_eq!(r.remap('h'), 'j');
        assert_eq!(r.remap('t'), 'k');
        assert_eq!(r.remap('n'), 'l');
    }

    #[test]
    fn colemak_remap() {
        let r = LayoutRemapper::new(KeyboardLayout::Colemak);
        assert_eq!(r.remap('n'), 'j');
        assert_eq!(r.remap('e'), 'k');
        assert_eq!(r.remap('i'), 'l');
    }

    #[test]
    fn workman_remap() {
        let r = LayoutRemapper::new(KeyboardLayout::Workman);
        assert_eq!(r.remap('y'), 'h');
        assert_eq!(r.remap('n'), 'j');
        assert_eq!(r.remap('e'), 'k');
        assert_eq!(r.remap('o'), 'l');
    }

    #[test]
    fn disable_preserve_hjkl() {
        let mut r = LayoutRemapper::new(KeyboardLayout::Dvorak);
        r.set_preserve_hjkl(false);
        assert_eq!(r.remap('d'), 'd'); // no remapping
    }

    #[test]
    fn parse_layout_names() {
        assert_eq!(parse_layout("dvorak"), Some(KeyboardLayout::Dvorak));
        assert_eq!(parse_layout("COLEMAK"), Some(KeyboardLayout::Colemak));
        assert_eq!(parse_layout("Workman"), Some(KeyboardLayout::Workman));
        assert_eq!(parse_layout("unknown"), None);
    }

    #[test]
    fn custom_remap() {
        let mut r = LayoutRemapper::new(KeyboardLayout::Custom);
        r.set_preserve_hjkl(true);
        r.add_remap('a', 'h');
        r.add_remap('s', 'j');
        assert_eq!(r.remap('a'), 'h');
        assert_eq!(r.remap('s'), 'j');
        assert_eq!(r.list_remaps().len(), 2);
    }
}
