/// Keybinding DSL — full specification of keybinding notation and parsing.

/// Special key names recognized in DSL notation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpecialKey {
    Space, Enter, Escape, Tab, Backspace, Delete,
    Up, Down, Left, Right,
    Home, End, PageUp, PageDown,
    F(u8), // F1-F12
    Insert, Nul,
}

impl SpecialKey {
    pub fn from_name(name: &str) -> Option<Self> {
        match name.to_lowercase().as_str() {
            "space" | "spc" => Some(Self::Space), "cr" | "enter" | "return" => Some(Self::Enter),
            "esc" | "escape" => Some(Self::Escape), "tab" => Some(Self::Tab),
            "bs" | "backspace" => Some(Self::Backspace), "del" | "delete" => Some(Self::Delete),
            "up" => Some(Self::Up), "down" => Some(Self::Down),
            "left" => Some(Self::Left), "right" => Some(Self::Right),
            "home" => Some(Self::Home), "end" => Some(Self::End),
            "pageup" => Some(Self::PageUp), "pagedown" => Some(Self::PageDown),
            "insert" | "ins" => Some(Self::Insert), "nul" => Some(Self::Nul),
            s if s.starts_with('f') => s[1..].parse().ok().filter(|&n: &u8| n >= 1 && n <= 12).map(Self::F),
            _ => None,
        }
    }
}

/// Modifier flags.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Modifiers { pub ctrl: bool, pub alt: bool, pub shift: bool, pub meta: bool }

/// A parsed key sequence element.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum KeySpec { Char(char), Special(SpecialKey), Leader }

/// A fully parsed key chord.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KeyChord { pub key: KeySpec, pub mods: Modifiers }

/// Parse a VIM-style key notation like `<C-x>`, `<A-Space>`, `<leader>`.
pub fn parse_key_notation(input: &str) -> Option<KeyChord> {
    if !input.starts_with('<') || !input.ends_with('>') {
        if input.len() == 1 {
            return Some(KeyChord { key: KeySpec::Char(input.chars().next()?), mods: Modifiers::default() });
        }
        return None;
    }
    let inner = &input[1..input.len() - 1];
    if inner.eq_ignore_ascii_case("leader") {
        return Some(KeyChord { key: KeySpec::Leader, mods: Modifiers::default() });
    }
    let mut mods = Modifiers::default();
    let mut rest = inner;
    loop {
        if rest.len() >= 2 && rest[1..].starts_with('-') {
            match rest.chars().next()?.to_ascii_uppercase() {
                'C' => mods.ctrl = true, 'A' | 'M' => mods.alt = true,
                'S' => mods.shift = true, 'D' => mods.meta = true,
                _ => break,
            }
            rest = &rest[2..];
        } else { break; }
    }
    if let Some(special) = SpecialKey::from_name(rest) {
        Some(KeyChord { key: KeySpec::Special(special), mods })
    } else if rest.len() == 1 {
        Some(KeyChord { key: KeySpec::Char(rest.chars().next()?), mods })
    } else { None }
}

/// Parse a full key sequence string like `<C-w>h` or `<leader>ff`.
pub fn parse_key_sequence(input: &str) -> Vec<KeyChord> {
    let mut result = Vec::new();
    let mut i = 0;
    let bytes = input.as_bytes();
    while i < bytes.len() {
        if bytes[i] == b'<' {
            if let Some(end) = input[i..].find('>') {
                let notation = &input[i..i + end + 1];
                if let Some(chord) = parse_key_notation(notation) {
                    result.push(chord); i += end + 1; continue;
                }
            }
        }
        let ch = input[i..].chars().next().unwrap();
        result.push(KeyChord { key: KeySpec::Char(ch), mods: Modifiers::default() });
        i += ch.len_utf8();
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_plain_char() {
        let k = parse_key_notation("x").unwrap();
        assert_eq!(k.key, KeySpec::Char('x'));
    }

    #[test]
    fn parse_ctrl() {
        let k = parse_key_notation("<C-x>").unwrap();
        assert!(k.mods.ctrl); assert_eq!(k.key, KeySpec::Char('x'));
    }

    #[test]
    fn parse_alt_space() {
        let k = parse_key_notation("<A-Space>").unwrap();
        assert!(k.mods.alt); assert_eq!(k.key, KeySpec::Special(SpecialKey::Space));
    }

    #[test]
    fn parse_leader() {
        let k = parse_key_notation("<leader>").unwrap();
        assert_eq!(k.key, KeySpec::Leader);
    }

    #[test]
    fn parse_f_key() {
        let k = parse_key_notation("<F12>").unwrap();
        assert_eq!(k.key, KeySpec::Special(SpecialKey::F(12)));
    }

    #[test]
    fn parse_sequence() {
        let seq = parse_key_sequence("<C-w>h");
        assert_eq!(seq.len(), 2);
        assert!(seq[0].mods.ctrl);
    }

    #[test]
    fn parse_leader_sequence() {
        let seq = parse_key_sequence("<leader>ff");
        assert_eq!(seq.len(), 3);
        assert_eq!(seq[0].key, KeySpec::Leader);
    }

    #[test]
    fn invalid_notation() { assert!(parse_key_notation("<invalid>").is_none()); }
}
