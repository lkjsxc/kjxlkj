//! Keybinding DSL parser — converts `<C-x>`, `<leader>e`, `<M-a>` notation to key sequences.

/// A parsed key chord from DSL notation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KeyChord {
    pub key: String,
    pub ctrl: bool,
    pub alt: bool,
    pub shift: bool,
}

impl KeyChord {
    pub fn plain(key: &str) -> Self { Self { key: key.into(), ctrl: false, alt: false, shift: false } }
    pub fn ctrl(key: &str) -> Self { Self { key: key.into(), ctrl: true, alt: false, shift: false } }
    pub fn alt(key: &str) -> Self { Self { key: key.into(), ctrl: false, alt: true, shift: false } }
    pub fn display(&self) -> String {
        let mut s = String::new();
        if self.ctrl || self.alt || self.shift { s.push('<'); }
        if self.ctrl { s.push_str("C-"); }
        if self.alt { s.push_str("M-"); }
        if self.shift { s.push_str("S-"); }
        s.push_str(&self.key);
        if self.ctrl || self.alt || self.shift { s.push('>'); }
        s
    }
}

/// Parse a single `<...>` notation token. Returns (KeyChord, bytes_consumed).
fn parse_angle(input: &str) -> Option<(KeyChord, usize)> {
    if !input.starts_with('<') { return None; }
    let end = input.find('>')?;
    let inner = &input[1..end];
    let mut ctrl = false;
    let mut alt = false;
    let mut shift = false;
    let mut rest = inner;
    loop {
        if rest.starts_with("C-") || rest.starts_with("c-") { ctrl = true; rest = &rest[2..]; }
        else if rest.starts_with("M-") || rest.starts_with("m-") || rest.starts_with("A-") || rest.starts_with("a-") {
            alt = true; rest = &rest[2..];
        } else if rest.starts_with("S-") || rest.starts_with("s-") { shift = true; rest = &rest[2..]; }
        else { break; }
    }
    let key = resolve_special(rest);
    Some((KeyChord { key, ctrl, alt, shift }, end + 1))
}

/// Resolve special key names to canonical form.
fn resolve_special(name: &str) -> String {
    match name.to_lowercase().as_str() {
        "cr" | "return" | "enter" => "Enter".into(),
        "esc" | "escape" => "Escape".into(),
        "bs" | "backspace" => "Backspace".into(),
        "tab" => "Tab".into(),
        "space" | "sp" => "Space".into(),
        "up" => "Up".into(), "down" => "Down".into(),
        "left" => "Left".into(), "right" => "Right".into(),
        "del" | "delete" => "Delete".into(),
        "home" => "Home".into(), "end" => "End".into(),
        "pageup" => "PageUp".into(), "pagedown" => "PageDown".into(),
        "bar" => "|".into(), "lt" => "<".into(), "gt" => ">".into(),
        "leader" => "leader".into(),
        "nop" => "Nop".into(),
        other => other.into(),
    }
}

/// Parse a full DSL key sequence (e.g. `<C-w>j`, `<leader>ff`, `gg`).
pub fn parse_key_sequence(input: &str) -> Vec<KeyChord> {
    let mut result = Vec::new();
    let mut i = 0;
    let bytes = input.as_bytes();
    while i < bytes.len() {
        if bytes[i] == b'<' {
            if let Some((chord, consumed)) = parse_angle(&input[i..]) {
                result.push(chord);
                i += consumed;
                continue;
            }
        }
        let ch = input[i..].chars().next().unwrap();
        result.push(KeyChord::plain(&ch.to_string()));
        i += ch.len_utf8();
    }
    result
}

/// Validate a key sequence string. Returns error message if invalid.
pub fn validate_key_sequence(input: &str) -> Result<(), String> {
    let mut i = 0;
    let bytes = input.as_bytes();
    while i < bytes.len() {
        if bytes[i] == b'<' {
            let rest = &input[i..];
            if let Some(end) = rest.find('>') {
                i += end + 1;
            } else {
                return Err(format!("unclosed '<' at position {}", i));
            }
        } else { i += input[i..].chars().next().map(|c| c.len_utf8()).unwrap_or(1); }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_plain() {
        let r = parse_key_sequence("gg");
        assert_eq!(r.len(), 2);
        assert_eq!(r[0], KeyChord::plain("g"));
    }

    #[test]
    fn parse_ctrl() {
        let r = parse_key_sequence("<C-w>");
        assert_eq!(r.len(), 1);
        assert_eq!(r[0], KeyChord::ctrl("w"));
    }

    #[test]
    fn parse_alt() {
        let r = parse_key_sequence("<M-a>");
        assert_eq!(r.len(), 1);
        assert_eq!(r[0], KeyChord::alt("a"));
    }

    #[test]
    fn parse_leader() {
        let r = parse_key_sequence("<leader>e");
        assert_eq!(r.len(), 2);
        assert_eq!(r[0].key, "leader");
        assert_eq!(r[1], KeyChord::plain("e"));
    }

    #[test]
    fn parse_special_keys() {
        assert_eq!(parse_key_sequence("<CR>")[0].key, "Enter");
        assert_eq!(parse_key_sequence("<Esc>")[0].key, "Escape");
        assert_eq!(parse_key_sequence("<BS>")[0].key, "Backspace");
        assert_eq!(parse_key_sequence("<Space>")[0].key, "Space");
        assert_eq!(parse_key_sequence("<Tab>")[0].key, "Tab");
    }

    #[test]
    fn parse_combined_modifiers() {
        let r = parse_key_sequence("<C-S-a>");
        assert_eq!(r.len(), 1);
        assert!(r[0].ctrl && r[0].shift);
    }

    #[test]
    fn display_round_trip() {
        let c = KeyChord::ctrl("w");
        assert_eq!(c.display(), "<C-w>");
        let p = KeyChord::plain("g");
        assert_eq!(p.display(), "g");
    }

    #[test]
    fn validate_ok() {
        assert!(validate_key_sequence("<C-w>j").is_ok());
        assert!(validate_key_sequence("gg").is_ok());
    }

    #[test]
    fn validate_unclosed() {
        assert!(validate_key_sequence("<C-w").is_err());
    }

    #[test]
    fn parse_complex_sequence() {
        let r = parse_key_sequence("<C-w>v<leader>ff");
        assert_eq!(r.len(), 5); // Ctrl-w, v, leader, f, f
    }
}
