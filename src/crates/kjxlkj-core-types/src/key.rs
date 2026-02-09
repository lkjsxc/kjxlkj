/// Internal key representation after decoding terminal events.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Key {
    pub code: KeyCode,
    pub modifiers: Modifier,
}

impl Key {
    pub fn new(code: KeyCode, modifiers: Modifier) -> Self {
        Self { code, modifiers }
    }

    pub fn char(c: char) -> Self {
        Self {
            code: KeyCode::Char(c),
            modifiers: Modifier::NONE,
        }
    }

    pub fn ctrl(c: char) -> Self {
        Self {
            code: KeyCode::Char(c),
            modifiers: Modifier::CTRL,
        }
    }

    pub fn esc() -> Self {
        Self {
            code: KeyCode::Esc,
            modifiers: Modifier::NONE,
        }
    }
}

/// Key code identity.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum KeyCode {
    Char(char),
    Enter,
    Esc,
    Backspace,
    Tab,
    BackTab,
    Delete,
    Up,
    Down,
    Left,
    Right,
    Home,
    End,
    PageUp,
    PageDown,
    F(u8),
}

/// Modifier bit flags.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Modifier(u8);

impl Modifier {
    pub const NONE: Self = Self(0);
    pub const CTRL: Self = Self(1);
    pub const ALT: Self = Self(2);
    pub const SHIFT: Self = Self(4);

    pub fn contains(self, other: Self) -> bool {
        (self.0 & other.0) == other.0
    }

    pub fn union(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}

impl std::ops::BitOr for Modifier {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        self.union(rhs)
    }
}
